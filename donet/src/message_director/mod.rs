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

use crate::config;
use crate::network::tcp;
use crate::service::*;
use channel_map::*;
use libdonet::globals::*;
use log::{error, info};
use std::collections::HashSet;
use std::io::Result;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;

pub struct MessageDirector {
    binding: tcp::Acceptor,
    upstream_md: Option<tcp::Connection>,
    channel_map: ChannelMap,
    subscribers: HashSet<subscriber::Subscriber>,
    removed_subscribers: HashSet<subscriber::Subscriber>,
}

impl DonetService for MessageDirector {
    type Service = Self;
    type Configuration = config::MessageDirector;

    async fn create(conf: Self::Configuration, _: Option<DCFile<'static>>) -> Result<Self::Service> {
        Ok(MessageDirector {
            binding: tcp::Acceptor::bind(conf.bind.as_str()).await?,
            upstream_md: {
                if let Some(u_uri) = conf.upstream {
                    info!("Message Director will connect to upstream MD.");
                    Some(tcp::Connection::connect(u_uri.as_str()).await?)
                } else {
                    None
                }
            },
            channel_map: ChannelMap::default(),
            subscribers: HashSet::default(),
            removed_subscribers: HashSet::default(),
        })
    }

    async fn start(conf: config::DonetConfig, _: Option<DCFile<'static>>) -> Result<JoinHandle<Result<()>>> {
        // We can unwrap safely here since this function only is called if it is `Some`.
        let service_conf: config::MessageDirector = conf.services.message_director.unwrap();

        let mut md: MessageDirector = MessageDirector::create(service_conf, None).await?;

        Ok(Self::spawn_async_task(async move { md.main().await }))
    }

    async fn main(&mut self) -> Result<()> {
        loop {
            match self.binding.socket.accept().await {
                Ok((socket, address)) => {
                    info!("Received incoming connection from {:?}.", address);

                    self.route_datagram(&socket).await?;
                }
                Err(socket_err) => error!("Failed to get client: {:?}", socket_err),
            }
        }
    }
}

impl ChannelCoordinator for MessageDirector {
    fn get_channel_map(&mut self) -> &mut ChannelMap {
        &mut self.channel_map
    }

    async fn on_add_channel(&self, channel: Channel) {
        todo!()
    }

    async fn on_add_range(&self, range: std::ops::Range<Channel>) {
        todo!()
    }

    async fn on_remove_channel(&self, channel: Channel) {
        todo!()
    }

    async fn on_remove_range(&self, range: std::ops::Range<Channel>) {
        todo!()
    }
}

impl MessageDirector {
    pub async fn route_datagram(&self, _socket: &TcpStream) -> Result<()> {
        Ok(())
    }
}
