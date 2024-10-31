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

use crate::network::tcp;
use donet_core::datagram::datagram::Datagram;
use std::io::Result;
use tokio::sync::Mutex;

/// Represents a connection to an upstream Message Director service.
pub struct UpstreamMD {
    connection: tcp::Connection,
    is_sending: bool,
    queue: Mutex<Vec<Datagram>>,
}

impl UpstreamMD {
    async fn connect(address: &str) -> Result<Self> {
        Ok(Self {
            connection: tcp::Connection::connect(address).await?,
            is_sending: false,
            queue: Mutex::new(vec![]),
        })
    }
}
