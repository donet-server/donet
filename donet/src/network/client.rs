// DONET SOFTWARE
// Copyright (c) 2024, Donet Authors.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License version 3.
// You should have received a copy of this license along
// with this source code in a file named "LICENSE."
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use libdonet::datagram::datagram::Datagram;
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
