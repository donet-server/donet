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

use chrono;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::io::{Error, ErrorKind, Result};

pub static _ANSI_RESET: &str = "\x1b[0m";
pub static _ANSI_RED: &str = "\x1b[31m";
pub static _ANSI_GREEN: &str = "\x1b[32m";
pub static _ANSI_ORANGE: &str = "\x1b[33m";
pub static _ANSI_YELLOW: &str = "\x1b[33;2m";
pub static _ANSI_BLUE: &str = "\x1b[34m";
pub static _ANSI_CYAN: &str = "\x1b[36m";
pub static _ANSI_GRAY: &str = "\x1b[37;2m";
pub static _ANSI_MAGENTA: &str = "\x1b[95m";

pub struct DaemonLogger {
    pub log_level: Level,
}

pub static MAX_LOG_LEVEL: LevelFilter = LevelFilter::Debug;

impl log::Log for DaemonLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        let level_color: &str = match record.level() {
            Level::Info => _ANSI_MAGENTA, // themed to logo
            Level::Debug => _ANSI_CYAN,
            Level::Warn => _ANSI_ORANGE,
            Level::Error => _ANSI_RED,
            Level::Trace => _ANSI_GRAY,
        };

        if self.enabled(record.metadata()) {
            // TODO: Write to log file by daemon configuration
            let out_string: String = format!(
                "{}[{}]{} {}{}:{} {}: {}",
                _ANSI_GRAY,
                chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S"),
                _ANSI_RESET,
                level_color,
                record.level(),
                _ANSI_RESET,
                record.target(),
                record.args()
            );
            println!("{}", out_string.as_str()); // stdout
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(logger: &'static dyn log::Log) -> Result<()> {
    let res: core::result::Result<(), SetLoggerError> =
        log::set_logger(logger).map(|()| log::set_max_level(MAX_LOG_LEVEL));

    if res.is_err() {
        // catch result and transform into std::io error for main to handle.
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to initialize the logger utility!",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod unit_testing {
    use super::{init_logger, DaemonLogger};
    use log::{debug, error, info, trace, warn};
    use std::io::Result;

    #[test]
    fn logger_integrity() {
        pub static GLOBAL_LOGGER: DaemonLogger = DaemonLogger {
            log_level: log::Level::Trace,
        };

        let res: Result<()> = init_logger(&GLOBAL_LOGGER);

        if res.is_err() {
            panic!("{}", res.unwrap_err());
        }
        error!("This macro should not panic.");
        info!("This macro should not panic.");
        debug!("This macro should not panic.");
        warn!("This macro should not panic.");
        trace!("This macro should not panic.");
    }
}
