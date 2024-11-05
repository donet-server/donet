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
use tokio::net::UdpSocket;

pub struct Socket {
    pub socket: UdpSocket,
    pub address: String,
}

impl Socket {
    pub async fn bind(uri: &str) -> Result<Self> {
        let socket = UdpSocket::bind(uri).await?;

        info!("Opened new UDP socket at {}.", uri);

        Ok(Self {
            socket,
            address: String::from(uri),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Socket;

    #[tokio::test]
    async fn async_udp_socket() {
        let bind_address: String = String::from("127.0.0.1:7197");
        let res: Result<Socket, _> = Socket::bind(&bind_address).await;

        match res {
            Ok(binding) => {
                assert_eq!(binding.address, bind_address);
            }
            Err(err) => panic!("UDPSocket failed to bind: {:?}", err),
        }
    }
}
