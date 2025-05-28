/*
    This file is part of Donet.

    Copyright © 2024-2025 Max Rodriguez <me@maxrdz.com>

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

use donet_core::datagram::datagram::Datagram;
use donet_core::Protocol;
use donet_network::*;
use std::future::Future;
use std::io::{Error, ErrorKind, Result};

/// The [`ClusterSubscriber`] trait must be implemented to
/// interact with the rest of the Donet cluster of services
/// via a message director service instance.
///
/// It is called a subscriber, as its indirectly a subscriber
/// to the Donet cluster via its message director service.
pub trait ClusterSubscriber
where
    Self: HasClient,
{
    /// Here is where the Donet service receives incoming
    /// messages from the cluster, provided by a message director.
    fn receive_datagram(dg: Datagram) -> impl Future<Output = Result<()>>;

    /// Sends a log message (blob in msgpack format) to the message
    /// director, which then routes it to an event logger service.
    fn send_log(&mut self, msgpack_blob: Datagram) -> impl Future<Output = Result<()>> {
        async move {
            let mut dg: Datagram = Datagram::default();

            // TODO: fix clashing result types (core result and IO result)
            dg.add_control_header(Protocol::MDLogMessage.into())?;
            dg.add_blob(msgpack_blob.get_data())?;

            if let Err(err) = self.get_client().lock().await.stage_datagram(dg).await {
                return Err(Error::new(ErrorKind::Other, err.to_string()));
            }
            Ok(())
        }
    }

    /// Sends a `CONTROL_SET_CON_NAME` message to this service's MD.
    fn set_connection_name(&mut self, name: String) -> impl Future<Output = Result<()>> {
        async move {
            let mut dg: Datagram = Datagram::default();

            dg.add_control_header(Protocol::MDSetConName.into())?;
            dg.add_string(&name)?;

            if let Err(err) = self.get_client().lock().await.stage_datagram(dg).await {
                return Err(Error::new(ErrorKind::Other, err.to_string()));
            }
            Ok(())
        }
    }

    /// Sends a `CONTROL_SET_CON_URL` message to this service's MD.
    fn set_connection_url(&mut self, url: String) -> impl Future<Output = Result<()>> {
        async move {
            let mut dg: Datagram = Datagram::default();

            dg.add_control_header(Protocol::MDSetConName.into())?;
            dg.add_string(&url)?;

            if let Err(err) = self.get_client().lock().await.stage_datagram(dg).await {
                return Err(Error::new(ErrorKind::Other, err.to_string()));
            }
            Ok(())
        }
    }
}
