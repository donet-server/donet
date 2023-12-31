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

use serde::Deserialize;
use std::vec::Vec;

// NOTE: We're deriving Clone trait instead of Copy,
// because most of our structs here use String types
// which use the Drop trait, which does not allow us
// to derive from the Copy trait.
#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Daemon {
    pub name: String,
    pub id: Option<u32>,
    pub log_level: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Global {
    pub eventlogger: String, // '<host>:<port>'
    pub dc_files: Vec<String>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Filtering {
    pub mode: String, // 'blacklist', 'whitelist'
    pub file: String, // <file_path>
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ClientAgent {
    pub bind: String, // '<host>:<port>'
    //pub protocol: String,
    pub dc_file_hash: Option<String>, // FIXME: Can we deserialize as hex literal?
    pub version_string: String,
    pub filtering: Filtering,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct MessageDirector {
    pub bind: String,             // '<host>:<port>'
    pub upstream: Option<String>, // '<host>:<port>'
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct StateServer {
    pub control_channel: u64,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct SQL {
    pub host: String, // '<host>:<port>'
    pub user: String,
    pub pass: String,
    pub database: String,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct DBServer {
    pub control_channel: u64,
    pub db_backend: String,
    pub sql: Option<SQL>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct DBSS {
    pub db_channel: u64,
    pub range_min: u64,
    pub range_max: u64,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct EventLogger {
    pub bind: String,   // '<host>:<port>'
    pub output: String, // path, relative to root
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Services {
    pub client_agent: Option<ClientAgent>,
    pub message_director: Option<MessageDirector>,
    pub state_server: Option<StateServer>,
    pub database_server: Option<DBServer>,
    pub dbss: Option<DBSS>,
    pub event_logger: Option<EventLogger>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct DonetConfig {
    pub daemon: Daemon,
    pub global: Global,
    pub services: Services,
}
