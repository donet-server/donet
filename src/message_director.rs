// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.
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

//use crate::channel_map;
use crate::network::{TCPAcceptor, TCPConnection};
use log::info;
use std::io::Result;

pub struct MessageDirector {
    _binding: TCPAcceptor,
    _upstream: Option<TCPConnection>,
}

impl MessageDirector {
    pub fn new(bind_uri: &str, upstream_uri: Option<String>) -> MessageDirector {
        let upstream_con: Option<TCPConnection> =
            upstream_uri.map(|uri| TCPConnection::connect(uri.as_str()));

        if upstream_con.is_some() {
            // This Message Director will connect to an upstream MD.
            info!("Message Director will connect to upstream MD.");
        }

        MessageDirector {
            _binding: TCPAcceptor::bind(bind_uri),
            _upstream: upstream_con,
        }
    }
    pub fn init_network(&self) -> Result<()> {
        Ok(())
    }
}
