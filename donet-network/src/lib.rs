/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez <me@maxrdz.com>

    Donet is free software; you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License,
    as published by the Free Software Foundation, either version 3
    of the License, or (at your option) any later version.

    Donet is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public
    License along with Donet. If not, see <https://www.gnu.org/licenses/>.
*/

pub mod tcp;
pub mod udp;

use donet_core::datagram::datagram::*;
use donet_core::datagram::iterator::*;
use donet_core::globals::*;
use log::{info, warn};
use std::collections::VecDeque;
use std::error::Error;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;

/// Size of the byte buffer for incoming TCP packets.
///
/// Tokio gives us entire TCP messages (after reassembling
/// segments) so we should expect this buffer to fill above
/// the TCP max segment size (MSS).
const TCP_READ_BUFFER_SIZE: usize = 300 * 1024; // 300 kb

/// Data sent via an MPSC channel from a
/// client receive loop task to a service
/// handle receive task.
pub struct RecvData {
    /// Remote IPv4/6 address of the sender
    pub remote: SocketAddr,
    /// Original datagram received
    pub dg: Datagram,
    /// Datagram Iterator. Propagated upwards to keep
    /// track of what data has already been consumed.
    pub dgi: DatagramIterator,
}

pub type RecvSendHandles = (JoinHandle<io::Result<()>>, JoinHandle<io::Result<()>>);

/// Ensures the implementing type owns a reference
/// to a [`Client`] structure.
pub trait HasClient {
    fn get_client(&self) -> Arc<Mutex<Client>>;
}

/// Represents a network client connected over TCP.
#[derive(Debug)]
pub struct Client {
    remote: SocketAddr,
    local: SocketAddr,
    /// Queue of datagrams to be sent. Use this to
    /// queue datagrams to be sent to the remote address
    /// of this [`Client`]'s TCP stream.
    send_queue_channel: Option<mpsc::Sender<Datagram>>,
    /// Wrapped in `Option` as we will consume these halves for tasks
    tcp_read_half: Option<OwnedReadHalf>,
    tcp_write_half: Option<OwnedWriteHalf>,
}

impl From<TcpStream> for Client {
    fn from(value: TcpStream) -> Self {
        let remote = value.peer_addr().expect("Failed to get remote address.");
        let local = value.local_addr().expect("Failed to get local address.");

        let (read_half, write_half) = value.into_split();

        Self {
            remote,
            local,
            send_queue_channel: None,
            tcp_read_half: Some(read_half),
            tcp_write_half: Some(write_half),
        }
    }
}

/// Allows for upgrading a [`tcp::Connection`] structure, to
/// a [`Client`] structure for advanced functionality, such
/// as receiving and sending datagrams asynchronously.
impl From<tcp::Connection> for Client {
    fn from(value: tcp::Connection) -> Self {
        value.socket.into()
    }
}

impl Client {
    /// Returns the remote IPv4/6 address of this client.
    pub fn get_remote(&self) -> SocketAddr {
        self.remote
    }

    /// Returns the local IPv4/6 address of this client.
    pub fn get_local(&self) -> SocketAddr {
        self.local
    }

    /// Sends the given [`Datagram`] to the send loop task, via the
    /// [`Client`]'s [`mpsc::Sender<Datagram>`].
    pub async fn stage_datagram(&mut self, dg: Datagram) -> Result<(), impl Error> {
        let tx = self
            .send_queue_channel
            .as_mut()
            .expect("recv/send tasks dont exist");
        tx.send(dg).await
    }

    /// Spawns a tokio task for `Self::receive_loop` and `Self::send_loop`,
    /// and returns a tuple:
    ///
    /// - The first tuple element is the [`JoinHandle`] for the receive loop.
    ///
    /// - The second tuple element is the [`JoinHandle`] for the send loop.
    pub fn spawn_recv_send_tasks(
        &mut self,
        incoming_tx: mpsc::Sender<RecvData>,
    ) -> impl Future<Output = RecvSendHandles> + Send + '_ {
        async move {
            let read_half = self.tcp_read_half.take().unwrap();
            let write_half = self.tcp_write_half.take().unwrap();

            let recv_handle = tokio::spawn(Self::receive_loop(read_half, incoming_tx));

            // send channel.
            // queues datagrams to be sent to the remote address of this client.
            let (tx, rx) = mpsc::channel::<Datagram>(32);

            self.send_queue_channel = Some(tx);

            let send_handle = tokio::spawn(Self::send_loop(write_half, rx));

            (recv_handle, send_handle)
        }
    }

    /// Main asynchronous loop for handling receiving TCP packets
    /// from this client's TCP stream.
    async fn receive_loop(
        read_half: OwnedReadHalf,
        incoming_queue_tx: mpsc::Sender<RecvData>,
    ) -> io::Result<()> {
        let remote: SocketAddr = read_half.peer_addr()?;

        loop {
            read_half.readable().await?;

            // initializing this **after** the `await` point prevents
            // it from being stored in the async task.
            let mut buffer = [0_u8; TCP_READ_BUFFER_SIZE];

            match read_half.try_read(&mut buffer) {
                Ok(0) => {
                    info!("Lost connection from {}", remote);

                    return Ok(()); // client closed TCP connection
                }
                Ok(len) => {
                    let mut dg: Datagram = Datagram::default();

                    dg.override_cap(TCP_READ_BUFFER_SIZE);

                    // The buffer is always a fixed size. Let's make a slice that
                    // contains only the length of the datagram received.
                    let mut buf_slice: Vec<u8> = buffer.to_vec();
                    buf_slice.truncate(len);

                    // we can safely unwrap here, since the size cap for `dg` was
                    // overridden to be the size of the read buffer size.
                    dg.add_data(buf_slice).unwrap();

                    Self::split_datagrams(remote, &incoming_queue_tx, dg.into()).await;
                    continue;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }

    /// Handles separating TCP packets into separate Datagrams, if multiple
    /// found in the packet, and sends each individual datagram over the
    /// mpsc channel using the given [`mpsc::Sender`].
    async fn split_datagrams(
        remote: SocketAddr,
        incoming_tx: &mpsc::Sender<RecvData>,
        mut dgi: DatagramIterator,
    ) {
        loop {
            let sizetag: DgSizeTag = dgi.read_size();

            if sizetag == 0 {
                warn!("Received datagram with a size tag of 0. Skipping.");
                break;
            }

            let mut individual_dg: Datagram = Datagram::default();

            let payload: Vec<u8> = match dgi.read_data(sizetag.into()) {
                Ok(data) => data,
                Err(err) => {
                    warn!("Received truncated datagram from {}: {}", remote, err);

                    // no more bytes to read, break read loop
                    break;
                }
            };

            assert!(individual_dg.add_data(payload).is_ok());

            // send individual datagram to the receive ncoming queue
            incoming_tx
                .send(RecvData {
                    remote,
                    dg: individual_dg.clone(),
                    dgi: DatagramIterator::from(individual_dg),
                })
                .await
                .expect("Tried to send received packet, but MPSC channel closed.");

            let remaining: usize = dgi.get_remaining();

            // if this packet has at least another size tag ahead,
            // try separating another datagram
            if remaining < std::mem::size_of::<DgSizeTag>() {
                // we *should* have 0 bytes left to read, if this is a
                // good packet. if not, its truncated (or we read it wrong)
                if remaining != 0 {
                    warn!("Received truncated datagram from {}", remote);
                    break;
                }
                break;
            }
        }
    }

    /// Main asynchronous loop for handling sending TCP packets to the
    /// remote address of this [`Client`]'s TCP stream.
    ///
    /// The queue of datagrams to be sent is received by this task
    /// via the given [`mpsc::Receiver<Datagram>`] struct.
    async fn send_loop(
        mut write_half: OwnedWriteHalf,
        mut send_queue_rx: mpsc::Receiver<Datagram>,
    ) -> io::Result<()> {
        loop {
            let mut buffer: Vec<Datagram> = vec![];

            // await until notified that more packets was added to the queue
            let n = send_queue_rx.recv_many(&mut buffer, 1000).await;

            // if `recv_many` returns 0, it means the MPSC channel was closed.
            if n == 0 {
                todo!("unhandled error. tcp client dg queue receiver returned 0.")
            }

            let mut queue: VecDeque<Datagram> = VecDeque::from(buffer);

            // prepare write buffer by reading the send queue
            let mut write_buffer_dg: Datagram = Datagram::default();

            while queue.len() != 0 {
                let mut dgi: DatagramIterator = queue.pop_front().unwrap().into();

                // get the size of this datagram to append size tag
                let sizetag: usize = dgi.get_remaining();

                // read the next bytes based on the size tag
                let dg_payload: Result<Vec<u8>, IteratorError> = dgi.read_data(sizetag);

                assert!(dg_payload.is_ok(), "Tried to read past datagram.");

                write_buffer_dg.add_size(sizetag as DgSizeTag).unwrap();
                write_buffer_dg.add_data(dg_payload.unwrap()).unwrap();

                debug_assert!(
                    dgi.get_remaining() == 0,
                    "Did not read all bytes from received dg to send."
                );
            }

            // send staged datagrams to client
            write_half.writable().await?;
            write_half.write_all(write_buffer_dg.get_buffer()).await?;
            write_half.flush().await?;
        }
    }
}
