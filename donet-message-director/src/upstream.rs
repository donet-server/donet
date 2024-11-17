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

use donet_core::datagram::datagram::*;
use donet_core::{globals::*, Protocol};
use donet_network::{tcp, Client, HasClient};
use std::io::Result;
use std::ops::Range;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a connection to an upstream Message Director service.
pub struct UpstreamMD {
    connection: Arc<Mutex<Client>>,
}

impl HasClient for UpstreamMD {
    fn get_client(&self) -> Arc<Mutex<Client>> {
        self.connection.clone()
    }
}

impl UpstreamMD {
    pub async fn connect(address: &str) -> Result<Self> {
        Ok(Self {
            connection: Arc::new(Mutex::new(tcp::Connection::connect(address).await?.into())),
        })
    }

    /// Pushes the given [`Datagram`] into the send queue channel
    /// for the send loop Tokio task for this TCP stream.
    ///
    /// This is a thin wrapper of [`Client::stage_datagram()`].
    pub async fn stage_datagram(&self, dg: Datagram) {
        self.connection.lock().await.stage_datagram(dg).await.unwrap()
    }

    /// Sends a `CONTROL_ADD_CHANNEL` control message uplink.
    pub async fn stage_add_channel(&self, channel: Channel) {
        let mut dg: Datagram = Datagram::default();

        dg.add_control_header(Protocol::MDAddChannel.into()).unwrap();
        dg.add_channel(channel).unwrap();

        self.stage_datagram(dg).await;
    }

    /// Sends a `CONTROL_ADD_RANGE` control message uplink.
    pub async fn stage_add_range(&self, range: Range<Channel>) {
        let mut dg: Datagram = Datagram::default();

        dg.add_control_header(Protocol::MDAddRange.into()).unwrap();

        dg.add_channel(range.start).unwrap();
        dg.add_channel(range.end).unwrap();

        self.stage_datagram(dg).await;
    }

    /// Sends a `CONTROL_REMOVE_CHANNEL` control message uplink.
    pub async fn stage_remove_channel(&self, channel: Channel) {
        let mut dg: Datagram = Datagram::default();

        dg.add_control_header(Protocol::MDRemoveChannel.into()).unwrap();
        dg.add_channel(channel).unwrap();

        self.stage_datagram(dg).await;
    }

    /// Sends a `CONTROL_REMOVE_RANGE` control message uplink.
    pub async fn stage_remove_range(&self, range: Range<Channel>) {
        let mut dg: Datagram = Datagram::default();

        dg.add_control_header(Protocol::MDRemoveRange.into()).unwrap();

        dg.add_channel(range.start).unwrap();
        dg.add_channel(range.end).unwrap();

        self.stage_datagram(dg).await;
    }

    /// Sends a `CONTROL_ADD_POST_REMOVE` control message uplink.
    pub async fn stage_post_remove(&self, sender: Channel, post_remove: Datagram) {
        let mut dg: Datagram = Datagram::default();

        dg.add_control_header(Protocol::MDAddPostRemove.into()).unwrap();

        dg.add_channel(sender).unwrap();
        dg.add_blob(post_remove.get_data()).unwrap();

        self.stage_datagram(dg).await;
    }

    /// Sends a `CONTROL_CLEAR_POST_REMOVES` control message uplink.
    pub async fn recall_post_removes(&self, sender: Channel) {
        let mut dg: Datagram = Datagram::default();

        dg.add_control_header(Protocol::MDClearPostRemoves.into())
            .unwrap();

        dg.add_channel(sender).unwrap();

        self.stage_datagram(dg).await;
    }
}
