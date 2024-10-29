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

mod channel_map;
mod subscriber;
mod upstream;

use crate::network::tcp;
use channel_map::ChannelMap;
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
        Ok(MessageDirector {
            binding: tcp::Acceptor::bind(bind_uri).await?,
            upstream: {
                if let Some(u_uri) = upstream_uri {
                    info!("Message Director will connect to upstream MD.");
                    Some(tcp::Connection::connect(u_uri.as_str()).await?)
                } else {
                    None
                }
            },
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
