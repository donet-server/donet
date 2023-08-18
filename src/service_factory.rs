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
    use log::info;
    use std::io::Result;

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

            // FIXME: pull credentials from configuration file
            let creds: DBCredentials = DBCredentials {
                host: "192.168.1.252",
                port: 3306,
                database: "test",
                user: "root",
                password: "",
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
