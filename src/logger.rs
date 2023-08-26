// DONET SOFTWARE
// Copyright (c) 2023, DoNet Authors.
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
pub static _ANSI_GRAY: &str = "\x1b[37m";
pub static _ANSI_MAGENTA: &str = "\x1b[95;1m";

pub struct DaemonLogger;
pub static LOGGER: DaemonLogger = DaemonLogger;

impl log::Log for DaemonLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        let level_color: &str;
        match record.level() {
            Level::Info => level_color = _ANSI_MAGENTA, // themed to logo
            Level::Debug => level_color = _ANSI_CYAN,
            Level::Warn => level_color = _ANSI_ORANGE,
            Level::Error => level_color = _ANSI_RED,
            Level::Trace => level_color = _ANSI_GRAY,
        }
        if self.enabled(record.metadata()) {
            // TODO: Write to log file by daemon configuration
            let out_string: String = format!(
                "[{}] [{}{}{}] :: {}",
                chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_color,
                record.level(),
                _ANSI_RESET,
                record.args()
            );
            println!("{}", out_string.as_str()); // stdout
        }
    }
    fn flush(&self) {}
}

pub fn initialize_logger() -> Result<()> {
    let res: core::result::Result<(), SetLoggerError> =
        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
    if res.is_err() {
        // catch result and transform into std::io error for main to handle.
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to initialize the logger utility!",
        ));
    }
    return Ok(());
}
