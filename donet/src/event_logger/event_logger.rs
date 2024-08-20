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

use crate::config;
use crate::network::UDPSocket;
use chrono::DateTime;
use libdonet::datagram::datagram::{Datagram, DatagramIterator};
use log::{debug, error, info};
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tokio::fs::File;

/// The `EventLogger` is a Donet service in the daemon that opens
/// up a socket and reads UDP packets from that socket. Received
/// UDP packets will be logged as configured in the daemon TOML file.
pub struct EventLogger {
    binding: UDPSocket,
    log_format: String,
    log_level: log::Level,
    log_file: Arc<Mutex<Option<File>>>,
}

impl EventLogger {
    pub async fn new(conf: config::EventLogger) -> Result<Self> {
        Ok(Self {
            binding: UDPSocket::bind(&conf.bind).await?,
            log_format: conf.log_format,
            log_level: log::Level::Info,
            log_file: Arc::new(Mutex::new(None)),
        })
    }

    /// This is Event Logger's main asynchronous loop.
    /// Spawned as a Tokio task by the service factory.
    pub async fn start_receive(&mut self) -> Result<()> {
        self.rotate_log().await?;

        let mut buffer = [0_u8; 1024]; // bytes
        loop {
            let (len, addr) = self.binding.socket.recv_from(&mut buffer).await?;
        }
    }

    fn process_datagram(&self, mut dg: Datagram) {
        let mut dgi: DatagramIterator = DatagramIterator::new(dg.clone());
        let data: String = String::default();

        if dgi.tell() != dg.size() {
            error!("Received packet with extraneous data from {}:{}", "", "");
            return;
        }
    }

    async fn rotate_log(&mut self) -> Result<()> {
        let unix_time: i64 = Self::get_unix_time();
        let date = DateTime::from_timestamp(unix_time, 0).expect("Invalid unix time!");

        // `chrono::DateTime.format()` has the same behavior as C/C++ ctime `strftime()`.
        let filename: String = format!("{}", date.format(&self.log_format));

        debug!("New log filename: {}", filename);

        {
            let mut file_guard = self.log_file.lock().unwrap();

            if let Some(_) = file_guard.take() {
                // We consume the file and the Option is set to `None`.
                // At the end of this scope, the file is dropped, which closes the file.
            }
        }

        let new_log: File = File::create_new(filename).await?;

        let mut file_guard = self.log_file.lock().unwrap();
        file_guard.replace(new_log); // replace `None` with new log file

        info!("Opened a new log.");
        Ok(())
    }

    #[inline(always)]
    fn get_unix_time() -> i64 {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(time) => time.as_secs().try_into().unwrap(),
            Err(e) => {
                error!("An error occurred trying to get a Unix timestamp: {}", e);
                panic!("The Event Logger had to panic unexpectedly.");
            }
        }
    }
}
