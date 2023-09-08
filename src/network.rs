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

use log::info;
use std::io::Result;
use tokio::net::{TcpListener, TcpStream};

pub struct TCPAcceptor {
    _socket: Box<TcpListener>,
    _bind_address: String,
}

pub struct TCPConnection {
    _socket: Box<TcpStream>,
    _address: String,
}

impl TCPAcceptor {
    pub async fn bind(uri: &str) -> Result<TCPAcceptor> {
        let net_resp: TcpListener = TcpListener::bind(uri).await?;

        info!("Opened new TCP listening socket at {}.", uri);
        let new_binding: Box<TcpListener> = Box::new(net_resp);

        Ok(TCPAcceptor {
            _socket: new_binding,
            _bind_address: String::from(uri),
        })
    }

    pub fn start_listening(&self) -> Result<()> {
        Ok(())
    }
}

impl TCPConnection {
    pub async fn connect(uri: &str) -> Result<TCPConnection> {
        let net_resp: TcpStream = TcpStream::connect(uri).await?;

        info!("Opened new TCP connection to {}.", uri);
        let new_socket: Box<TcpStream> = Box::new(net_resp);

        Ok(TCPConnection {
            _socket: new_socket,
            _address: String::from(uri),
        })
    }
}
