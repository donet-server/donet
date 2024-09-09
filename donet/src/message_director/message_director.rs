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

use super::channel_map::ChannelMap;
use crate::network::tcp;
use log::{error, info};
use std::io::Result;
use tokio::net::TcpStream;

pub struct MessageDirector {
    binding: tcp::Acceptor,
    upstream: Option<tcp::Connection>,
    channel_map: ChannelMap,
}

impl MessageDirector {
    pub async fn new(bind_uri: &str, upstream_uri: Option<String>) -> Result<MessageDirector> {
        let mut upstream_con: Option<tcp::Connection> = None;

        if let Some(u_uri) = upstream_uri {
            info!("Message Director will connect to upstream MD.");
            upstream_con = Some(tcp::Connection::connect(u_uri.as_str()).await?);
        }

        Ok(MessageDirector {
            binding: tcp::Acceptor::bind(bind_uri).await?,
            upstream: upstream_con,
            channel_map: ChannelMap::default(),
        })
    }

    /// This is the Message Director's main asynchronous loop.
    /// Spawned as a Tokio task by the service factory.
    pub async fn init_network(&self) -> Result<()> {
        loop {
            match self.binding.socket.accept().await {
                Ok((socket, address)) => {
                    info!("Received incoming connection from {:?}.", address);

                    self.handle_datagram(&socket).await?;
                }
                Err(socket_err) => error!("Failed to get client: {:?}", socket_err),
            }
        }
    }

    pub async fn handle_datagram(&self, _socket: &TcpStream) -> Result<()> {
        Ok(())
    }
}
