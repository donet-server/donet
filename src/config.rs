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

#[allow(dead_code)]
pub mod config {
    use serde::Deserialize;
    use std::vec::Vec;

    #[derive(Deserialize)]
    pub struct Daemon {
        name: String,
        id: Option<u32>,
        log_level: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct Global {
        eventlogger: String, // '<host>:<port>'
        dc_files: Vec<String>,
    }

    #[derive(Deserialize)]
    pub struct Gateway {
        bind: String, // '<host>:<port>'
    }

    #[derive(Deserialize)]
    pub struct ClientAgent {
        bind: String, // '<host>:<port>'
        protocol: String,
        dc_file_hash: String, // FIXME: Can we deserialize as hex literal?
        version_string: String,
    }

    #[derive(Deserialize)]
    pub struct StateServer {
        control_channel: u64,
    }

    #[derive(Deserialize)]
    pub struct SQL {
        host: String, // '<host>:<port>'
        user: String,
        pass: String,
        database: String,
    }

    #[derive(Deserialize)]
    pub struct DBServer {
        control_channel: u64,
        db_backend: String,
        sql: SQL,
    }

    #[derive(Deserialize)]
    pub struct DBSS {
        db_channel: u64,
        range_min: u64,
        range_max: u64,
    }

    #[derive(Deserialize)]
    pub struct EventLogger {
        bind: String,   // '<host>:<port>'
        output: String, // path, relative to root
    }

    #[derive(Deserialize)]
    pub struct Services {
        client_agent: ClientAgent,
        state_server: StateServer,
        database_server: DBServer,
        dbss: DBSS,
        event_logger: EventLogger,
    }

    #[derive(Deserialize)]
    pub struct DonetConfig {
        daemon: Daemon,
        global: Global,
        gateway: Gateway,
        services: Services,
    }
}
