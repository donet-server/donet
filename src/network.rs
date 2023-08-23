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

use log::{error, info};
use std::io::Result;
use std::net::TcpListener;

pub struct TCPSocket {
    stream: Box<TcpListener>,
}

impl TCPSocket {
    pub fn connect(uri: &str) -> TCPSocket {
        let net_resp: Result<TcpListener> = TcpListener::bind(uri);

        if net_resp.is_err() {
            error!("An error occurred when trying to open a new TCP listener.");
            panic!("Failed to open a new TCP listener!");
        }
        info!("Opened new TCP listening socket at {}.", uri);
        let new_binding: Box<TcpListener> = Box::new(net_resp.unwrap());

        return TCPSocket { stream: new_binding };
    }
    pub fn init_socket(&self) -> Result<()> {
        return Ok(());
    }
}
