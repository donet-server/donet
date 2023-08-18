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

#[path = "config.rs"]
mod config;
#[path = "dbserver.rs"]
mod dbserver;

pub mod service_factory {
    use super::dbserver::dbserver::{DBCredentials, DatabaseServer};
    use crate::config::config::*;
    use log::{error, info};
    use std::io::{Error, ErrorKind, Result};

    // All DoNet service types
    // Each implement the 'DonetService' trait,
    // with their bootstrap code to start the service.
    pub struct ClientAgentService;
    pub struct MessageDirectorService;
    pub struct StateServerService;
    pub struct DatabaseServerService;
    pub struct DBSSService;
    pub struct EventLoggerService;

    pub trait DonetService {
        fn start(&self, _conf: DonetConfig) -> Result<()>;
        fn create(&self) -> Result<Box<dyn DonetService>>;
    }

    impl DonetService for ClientAgentService {
        // TODO: implement client agent xd
        fn start(&self, _conf: DonetConfig) -> Result<()> {
            info!("Booting Client Agent service.");
            return Ok(());
        }

        fn create(&self) -> Result<Box<dyn DonetService>> {
            return Ok(Box::new(ClientAgentService));
        }
    }

    impl DonetService for MessageDirectorService {
        // TODO: write the md lmbo. this is repetitive
        fn start(&self, _conf: DonetConfig) -> Result<()> {
            info!("Booting Message Director service.");
            return Ok(());
        }

        fn create(&self) -> Result<Box<dyn DonetService>> {
            return Ok(Box::new(MessageDirectorService));
        }
    }

    impl DonetService for StateServerService {
        fn start(&self, _conf: DonetConfig) -> Result<()> {
            info!("Booting State Server service.");
            return Ok(());
        }

        fn create(&self) -> Result<Box<dyn DonetService>> {
            return Ok(Box::new(StateServerService));
        }
    }

    impl DonetService for DatabaseServerService {
        fn start(&self, _conf: DonetConfig) -> Result<()> {
            info!("Booting Database Server service.");

            // NOTE: We are unwrapping an Option without checking,
            // as this method can only be called if 'database_server'
            // is of a 'Some' type, which guarantees no panic scenario.
            let db_server_conf: DBServer = _conf.services.database_server.unwrap();

            // TODO: Check for db backend type once we
            // have multiple DB backend support.
            let sql_config: SQL;
            let host_port: Vec<&str>;

            if db_server_conf.sql.is_some() {
                sql_config = db_server_conf.sql.unwrap().clone();
                // NOTE: .collect() returns the values backwards?
                // so first &str is the port, and the second is the host.
                host_port = sql_config.host.rsplit(':').collect();
            } else {
                error!("Incomplete configuration for DB server service.");
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Missing database backend credentials.",
                ));
            }

            let creds: DBCredentials = DBCredentials {
                host: host_port[1],
                port: host_port[0].parse::<i16>().unwrap(),
                database: sql_config.database.as_str(),
                user: sql_config.user.as_str(),
                password: sql_config.pass.as_str(),
            };
            let mut db: DatabaseServer = DatabaseServer::new(creds);
            let res = db.init_service();

            if res.is_err() {
                // TODO: avoid panic; result type consistency
                panic!("unhandled. F");
            }
            return Ok(());
        }

        fn create(&self) -> Result<Box<dyn DonetService>> {
            return Ok(Box::new(DatabaseServerService));
        }
    }

    impl DonetService for DBSSService {
        fn start(&self, _conf: DonetConfig) -> Result<()> {
            info!("Booting DBSS Service.");
            return Ok(());
        }

        fn create(&self) -> Result<Box<dyn DonetService>> {
            return Ok(Box::new(DBSSService));
        }
    }

    impl DonetService for EventLoggerService {
        fn start(&self, _conf: DonetConfig) -> Result<()> {
            info!("Booting Event Logger Service.");
            return Ok(());
        }

        fn create(&self) -> Result<Box<dyn DonetService>> {
            return Ok(Box::new(EventLoggerService));
        }
    }
}
