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

mod msgpack;

use crate::config;
use crate::event::LoggedEvent;
use crate::network::udp;
use chrono::{DateTime, Duration, Local, TimeZone};
use libdonet::datagram::datagram::Datagram;
use libdonet::datagram::iterator::DatagramIterator;
use log::{debug, error, info, trace};
use regex::Regex;
use std::io::{Error, ErrorKind, Result};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

/// Interval unit types for log rotation intervals.
#[derive(Debug, PartialEq, Eq)]
pub enum IntervalUnit {
    Minutes, // min
    Hours,   // h, hr
    Days,    // d
    Months,  // mo
}

/// Represents the configured log rotation interval.
/// First item is the unit quantity, second item is the unit type.
pub type Interval = (i64, IntervalUnit);

/// The `EventLogger` is a Donet service in the daemon that opens
/// up a socket and reads UDP packets from that socket. Received
/// UDP packets will be logged as configured in the daemon TOML file.
pub struct EventLogger {
    binding: udp::Socket,
    log_format: String,
    log_file: Arc<Mutex<Option<File>>>,
    rotation_interval: Interval,
    next_rotation: i64, // unix timestamp
}

impl EventLogger {
    pub async fn new(conf: config::EventLogger) -> Result<Self> {
        Ok(Self {
            binding: udp::Socket::bind(&conf.bind).await?,
            log_format: format!("{}{}", conf.output, conf.log_format),
            log_file: Arc::new(Mutex::new(None)),
            rotation_interval: Self::str_to_interval(&conf.rotate_interval),
            next_rotation: 0_i64, // set on first log opened
        })
    }

    /// This is Event Logger's main asynchronous loop.
    /// Spawned as a Tokio task by the service factory.
    pub async fn start_receive(&mut self) -> Result<()> {
        self.open_log().await?;

        let mut buffer = [0_u8; 1024]; // 1 kb
        let mut data: String = String::default();

        let mut dg: Datagram;
        let mut dgi: DatagramIterator;

        {
            let mut event = LoggedEvent::new("log-opened", "EventLogger");
            event.add("msg", "Log opened upon Event Logger startup.");

            dgi = event.make_datagram().into();

            let ip = core::net::Ipv4Addr::new(127, 0, 0, 1);
            let v4addr = core::net::SocketAddrV4::new(ip, 0);
            let addr = std::net::SocketAddr::V4(v4addr);

            self.process_datagram(addr, &mut data, &mut dgi)
                .await
                .expect("Failed to process log opened event!");
        }

        loop {
            let (len, addr) = self.binding.socket.recv_from(&mut buffer).await?;
            trace!("Got packet from {}.", addr);

            dg = Datagram::default();

            // The buffer is always 1 kb in size. Let's make a slice that
            // contains only the length of the datagram received.
            let mut buf_slice = buffer.to_vec();
            buf_slice.truncate(len);

            dg.add_data(buf_slice)
                .expect("Failed to create dg from buffer slice!");

            dgi = dg.clone().into();

            // Check Unix timestamp for next rotation and cycle log if expired.
            let unix_time: i64 = Self::get_unix_time();

            if self.next_rotation <= unix_time {
                self.rotate_log(&mut data, &mut dgi).await?
            }

            match self.process_datagram(addr, &mut data, &mut dgi).await {
                Ok(txt) => txt,
                Err(err) => {
                    error!("Failed to process datagram from {}: {}", addr, err);
                    continue;
                }
            };
        }
    }

    /// Takes in `DatagramIterator` with packet data and modifies output string stream.
    /// Expects datagram bytes to follow the [`MessagePack`] format.
    ///
    /// [`MessagePack`]: https://msgpack.org
    async fn process_datagram(
        &self,
        addr: core::net::SocketAddr,
        data: &mut String,
        dgi: &mut DatagramIterator,
    ) -> Result<()> {
        // new datagram being processed, clear previous data
        data.clear();

        msgpack::decode_to_json(data, dgi);

        // Verify the msgpack contains a Map from the beginning.
        if let Some(ch) = data.get(0..1) {
            if ch != "{" {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Received non-map event log. Data: {}", &data),
                ));
            }
        }

        // Remaining bytes should equal the remaining unused buffer.
        if dgi.get_remaining() as usize != 0 {
            error!("Received packet with extraneous data from {}", addr);
        }
        trace!("Received: {}", data);

        let unix_time: i64 = Self::get_unix_time();
        let date: DateTime<Local> = Local.timestamp_opt(unix_time, 0).unwrap();

        // Insert timestamp as the first element of the map for this log entry.
        data.insert_str(
            1,
            &format!("{}", date.format("\"_time\": \"%Y-%m-%d %H:%M:%S%z\", ")),
        );

        let mut guard = self.log_file.lock().await;
        let file = guard.as_mut().expect("");

        data.push('\n');
        file.write_all(data.as_bytes()).await?;

        Ok(())
    }

    /// Opens a new log file on disk once any writes to the current log
    /// file are finished, and creates a next log rotation timestamp.
    async fn open_log(&mut self) -> Result<()> {
        let unix_time: i64 = Self::get_unix_time();
        let date = DateTime::from_timestamp(unix_time, 0).expect("Invalid unix time!");

        // `chrono::DateTime.format()` has the same behavior as C/C++ ctime `strftime()`.
        let filename: String = format!("{}", date.format(&self.log_format));

        debug!("New log filename: {}", filename);

        {
            let mut file_guard = self.log_file.lock().await;

            #[allow(clippy::redundant_pattern_matching)]
            if let Some(_) = file_guard.take() {
                // We consume the file and the Option is set to `None`.
                // At the end of this scope, the file is dropped, which closes the file.
            }
        }

        let new_log: File = File::create_new(filename).await?;

        let mut file_guard = self.log_file.lock().await;
        file_guard.replace(new_log); // replace `None` with new log file

        info!("Opened a new log.");

        // Create new chrono `DateTime` to represent the expiration time for this log.
        let next_rotation_date = match self.rotation_interval.1 {
            IntervalUnit::Minutes => date + Duration::minutes(self.rotation_interval.0),
            IntervalUnit::Hours => date + Duration::hours(self.rotation_interval.0),
            IntervalUnit::Days => date + Duration::days(self.rotation_interval.0),
            IntervalUnit::Months => date + Duration::days(self.rotation_interval.0 * 30),
        };

        // Convert `DateTime` to Unix timestamp & set as next rotation timestamp
        self.next_rotation = next_rotation_date.timestamp();

        Ok(())
    }

    /// Rotates the log file. The current log file is closed once all writes
    /// to the file are finished, and a new log file is opened.
    async fn rotate_log(&mut self, data: &mut String, dgi: &mut DatagramIterator) -> Result<()> {
        self.open_log().await?;

        let mut event = LoggedEvent::new("log-opened", "EventLogger");
        event.add("msg", "Log cycled.");

        *dgi = DatagramIterator::from(event.make_datagram());

        let ip = core::net::Ipv4Addr::new(127, 0, 0, 1);
        let v4addr = core::net::SocketAddrV4::new(ip, 0);
        let addr = std::net::SocketAddr::V4(v4addr);

        self.process_datagram(addr, data, dgi)
            .await
            .expect("Failed to process log cycled event!");
        Ok(())
    }

    /// Parses a string (from TOML config) into an [`Interval`] tuple.
    #[inline(always)]
    pub(self) fn str_to_interval(input: &str) -> Interval {
        let quantity_re = Regex::new(r"-{0,}(0|([1-9][0-9]*))").unwrap(); // decimal
        let unit_re = Regex::new(r"((min)|h|(hr)|d|(mo))$").unwrap(); // see `IntervalUnit`

        let quantity: i64 = quantity_re
            .find(input)
            .expect("Regex for interval unit quantity did not match.")
            .as_str()
            .parse::<i64>()
            .unwrap();

        // `Duration` prefers i64, but we won't accept signed integers.
        assert!(
            quantity >= 1,
            "Log rotation interval unit quantity cannot be negative or zero."
        );

        let unit_match: &str = unit_re
            .find(input)
            .expect("Regex for interval unit type did not match.")
            .as_str();

        let unit_type: IntervalUnit = match unit_match {
            "min" => IntervalUnit::Minutes,
            "h" => IntervalUnit::Hours,
            "hr" => IntervalUnit::Hours,
            "d" => IntervalUnit::Days,
            "mo" => IntervalUnit::Months,
            _ => panic!("Regex invalid"),
        };

        (quantity, unit_type)
    }

    /// Returns the current unix timestamp as a 64-bit signed integer.
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

#[cfg(test)]
mod unit_testing {
    use super::{EventLogger, Interval, IntervalUnit};
    use crate::config;
    use crate::event::LoggedEvent;
    use crate::network::udp;
    use libdonet::datagram::datagram::Datagram;
    use std::io::Error;
    use std::result::Result;

    #[tokio::test]
    async fn basic_message_test() -> Result<(), Error> {
        let conf = config::EventLogger {
            bind: "127.0.0.0:7197".to_string(),
            output: "./".to_string(),
            log_format: "el-%Y-%m-%d-%H-%M-%S.log".to_string(),
            rotate_interval: "1d".to_string(),
        };

        let _: EventLogger = EventLogger::new(conf).await?;

        let sock: udp::Socket = udp::Socket::bind("127.0.0.1:2816").await?;
        let mut dg: Datagram;

        let mut new_log = LoggedEvent::new("test", "Unit Test Socket");
        new_log.add("msg", "This is a test log message.");

        dg = new_log.make_datagram();

        sock.socket.send_to(&dg.get_data(), "127.0.0.1:7197").await?;
        Ok(())
    }

    #[test]
    fn str_to_interval() {
        let inputs: [&str; 5] = ["1min", "10h", "999hr", "42d", "3mo"];
        let outputs: [Interval; 5] = [
            (1, IntervalUnit::Minutes),
            (10, IntervalUnit::Hours),
            (999, IntervalUnit::Hours),
            (42, IntervalUnit::Days),
            (3, IntervalUnit::Months),
        ];

        for (i, input) in inputs.iter().enumerate() {
            assert_eq!(EventLogger::str_to_interval(input), outputs[i]);
        }
    }

    #[test]
    #[should_panic]
    fn negative_or_zero_interval() {
        let _: Interval = EventLogger::str_to_interval("-1d");
        _ = EventLogger::str_to_interval("0d");
    }
}
