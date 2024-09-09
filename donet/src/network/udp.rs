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
mod unit_testing {
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
