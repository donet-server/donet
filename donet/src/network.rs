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

//! Thin wrappers around Tokio async TCP/UDP sockets for logging.

use log::info;
use std::io::Result;
use tokio::net::{TcpListener, TcpStream, UdpSocket};

pub struct TCPAcceptor {
    pub socket: TcpListener,
    pub address: String,
}

pub struct TCPConnection {
    pub socket: TcpStream,
    pub address: String,
}

impl TCPAcceptor {
    pub async fn bind(uri: &str) -> Result<TCPAcceptor> {
        let socket: TcpListener = TcpListener::bind(uri).await?;

        info!("Opened new TCP listening socket at {}.", uri);

        Ok(TCPAcceptor {
            socket,
            address: String::from(uri),
        })
    }
}

impl TCPConnection {
    pub async fn connect(uri: &str) -> Result<TCPConnection> {
        let socket: TcpStream = TcpStream::connect(uri).await?;

        info!("Opened new TCP connection to {}.", uri);

        Ok(TCPConnection {
            socket,
            address: String::from(uri),
        })
    }
}

pub struct UDPSocket {
    pub socket: UdpSocket,
    pub address: String,
}

impl UDPSocket {
    pub async fn bind(uri: &str) -> Result<UDPSocket> {
        let socket = UdpSocket::bind(uri).await?;

        info!("Opened new UDP socket at {}.", uri);

        Ok(UDPSocket {
            socket,
            address: String::from(uri),
        })
    }
}

#[cfg(test)]
mod unit_testing {
    use super::{TCPAcceptor, TCPConnection};

    #[tokio::test]
    async fn async_tcp_listener() {
        let bind_address: String = String::from("127.0.0.1:7199");
        let res: Result<TCPAcceptor, _> = TCPAcceptor::bind(&bind_address).await;

        match res {
            Ok(binding) => {
                assert_eq!(binding.address, bind_address);
            }
            Err(err) => panic!("TCPAcceptor failed to bind: {:?}", err),
        }
    }

    #[tokio::test]
    async fn async_tcp_connection() {
        let bind_address: String = String::from("127.0.0.1:7198");
        let bind_res: Result<TCPAcceptor, _> = TCPAcceptor::bind(&bind_address).await;

        match bind_res {
            Ok(listener) => {
                tokio::spawn(async move {
                    loop {
                        let _ = listener.socket.accept().await;
                    }
                });
            }
            Err(err) => panic!("Failed to set up listener for test: {:?}", err),
        }

        // This should make a TCP connection with the listener created above.
        let dst_address: String = String::from("127.0.0.1:7198");
        let res: Result<TCPConnection, _> = TCPConnection::connect(&dst_address).await;

        match res {
            Ok(binding) => {
                assert_eq!(binding.address, dst_address);
            }
            Err(err) => panic!("TCPConnection failed to establish: {:?}", err),
        }
    }
}
