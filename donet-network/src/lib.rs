/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez

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

use donet_core::datagram::datagram::Datagram;
use log::debug;
use std::io::{Error, ErrorKind, Result};
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct Client {
    socket: Option<TcpStream>,
    remote: SocketAddr,
    local: SocketAddr,
    queue: Vec<Datagram>,
    is_sending: bool,
}

impl Client {
    pub async fn new(socket: TcpStream) -> Result<Self> {
        Ok(Self {
            remote: socket.peer_addr()?,
            local: socket.local_addr()?,
            socket: Some(socket),
            queue: vec![],
            is_sending: false,
        })
    }

    async fn disconnect(&mut self) -> Result<()> {
        if let Some(sock) = &mut self.socket {
            self.queue.clear();
            sock.shutdown().await?;
        } else {
            debug!("Tried to disconnect client with no existing socket!");
            return Err(Error::new(ErrorKind::NotConnected, "Client has no socket!"));
        }

        // Dropping the `TcpStream` will disconnect the client for us.
        // So, we just have to take ownership and it will be automatically
        // dropped for us as soon as we return out of this scope.
        let _: TcpStream = self.socket.take().unwrap();
        Ok(())
    }
}
