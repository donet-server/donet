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

use log::info;
use std::io::Result;
use tokio::net::{TcpListener, TcpStream};

pub struct Acceptor {
    pub socket: TcpListener,
    pub address: String,
}

pub struct Connection {
    pub socket: TcpStream,
    pub address: String,
}

impl Acceptor {
    pub async fn bind(uri: &str) -> Result<Self> {
        let socket: TcpListener = TcpListener::bind(uri).await?;

        info!("Opened new TCP listening socket at {}.", uri);

        Ok(Self {
            socket,
            address: String::from(uri),
        })
    }
}

impl Connection {
    pub async fn connect(uri: &str) -> Result<Self> {
        let socket: TcpStream = TcpStream::connect(uri).await?;

        info!("Opened new TCP connection to {}.", uri);

        Ok(Self {
            socket,
            address: String::from(uri),
        })
    }
}

#[cfg(test)]
mod unit_testing {
    use super::{Acceptor, Connection};

    #[tokio::test]
    async fn async_tcp_listener() {
        let bind_address: String = String::from("127.0.0.1:7199");
        let res: Result<Acceptor, _> = Acceptor::bind(&bind_address).await;

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
        let bind_res: Result<Acceptor, _> = Acceptor::bind(&bind_address).await;

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
        let res: Result<Connection, _> = Connection::connect(&dst_address).await;

        match res {
            Ok(binding) => {
                assert_eq!(binding.address, dst_address);
            }
            Err(err) => panic!("TCPConnection failed to establish: {:?}", err),
        }
    }
}
