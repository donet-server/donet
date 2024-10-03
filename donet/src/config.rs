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

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct DonetConfig {
    pub daemon: Daemon,
    pub global: Global,
    pub services: Services,
}

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
pub struct Services {
    pub client_agent: Option<ClientAgent>,
    pub message_director: Option<MessageDirector>,
    pub state_server: Option<StateServer>,
    pub database_server: Option<DBServer>,
    pub dbss: Option<DBSS>,
    pub event_logger: Option<EventLogger>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct ClientAgent {
    pub bind: String, // '<host>:<port>'
    pub dc_file_hash: Option<String>,
    pub version_string: String,
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
pub struct DBServer {
    pub control_channel: u64,
    pub db_backend: String,
    pub sql: Option<SQL>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct SQL {
    pub host: String, // '<host>:<port>'
    pub user: String,
    pub pass: String,
    pub database: String,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct DBSS {
    pub db_channel: u64,
    pub range_min: u64,
    pub range_max: u64,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct EventLogger {
    pub bind: String,            // '<host>:<port>'
    pub output: String,          // path, relative to fs root
    pub log_format: String,      // e.g. "el-%Y-%m-%d-%H-%M-%S.log"
    pub rotate_interval: String, // e.g. "1d"
}
