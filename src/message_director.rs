// DONET SOFTWARE
// Copyright (c) 2023, DoNet Authors.

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

use crate::network::{TCPAcceptor, TCPConnection};
use log::info;
use std::io::Result;

pub struct MessageDirector {
    binding: TCPAcceptor,
    upstream: Option<TCPConnection>,
}

impl MessageDirector {
    pub fn new(bind_uri: &str, upstream_uri: Option<String>) -> MessageDirector {
        let upstream_connection: Option<TCPConnection>;

        if upstream_uri.is_some() {
            // This Message Director will connect to an upstream MD.
            info!("Message Director will connect to upstream MD.");
            let unwrapped_uri: String = upstream_uri.unwrap();
            upstream_connection = Some(TCPConnection::connect(unwrapped_uri.as_str()));
        } else {
            upstream_connection = None;
        }
        return MessageDirector {
            binding: TCPAcceptor::bind(bind_uri),
            upstream: upstream_connection,
        };
    }
    pub fn init_network(&self) -> Result<()> {
        return Ok(());
    }
}
