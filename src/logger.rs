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

pub mod logger {
    use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

    pub struct DaemonLogger;

    impl log::Log for DaemonLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Info
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                println!("[{}] :: {}", record.level(), record.args());
            }
        }
        fn flush(&self) {}
    }

    pub static LOGGER: DaemonLogger = DaemonLogger;

    pub fn initialize_logger() -> Result<(), SetLoggerError> {
        return log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
    }
}