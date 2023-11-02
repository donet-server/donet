// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.
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

use crate::config::*;
use crate::dbserver::{DBCredentials, DatabaseServer};
use crate::message_director::MessageDirector;
use crate::utils;
use log::{error, info};
use std::io::{Error, ErrorKind, Result};
use tokio::task::JoinHandle;

// All DoNet service types
// Each implement bootstrap code to start a service.
pub struct ClientAgentService;
pub struct MessageDirectorService;
pub struct StateServerService;
pub struct DatabaseServerService;
pub struct DBSSService;
pub struct EventLoggerService;

//pub trait DonetService {
//    fn start(&self, _conf: DonetConfig) -> Result<()>;
//    fn create(&self) -> Result<Box<dyn DonetService>>;
//}
// NOTE: Removed above due to async not allowed in traits.
//       Hoping to add back on a future rust release?

impl ClientAgentService {
    // TODO: implement client agent xd
    pub async fn start(&self, _conf: DonetConfig) -> Result<()> {
        info!("Booting Client Agent service.");
        Ok(())
    }

    pub fn create(&self) -> Result<Box<ClientAgentService>> {
        Ok(Box::new(ClientAgentService))
    }
}

impl MessageDirectorService {
    pub async fn start(&self, conf: DonetConfig) -> Result<JoinHandle<Result<()>>> {
        info!("Booting Message Director service.");

        // Use 'MessageDirector' config repr, not *THE* MessageDirector.
        let md_conf: crate::config::MessageDirector;

        if let Some(md_some) = conf.services.message_director {
            md_conf = md_some;
        } else {
            error!("Missing required Message Director configuration.");
            panic!("Cannot initialize Donet daemon without MD.");
        }
        let mut upstream: Option<String> = None;

        if let Some(upstream_some) = md_conf.upstream {
            // This Message Director will connect to an upstream MD.
            upstream = Some(upstream_some);
        }

        let md: MessageDirector = MessageDirector::new(md_conf.bind.as_str(), upstream).await?;

        let md_loop = async move { md.init_network().await };
        utils::set_future_return_type::<Result<()>, _>(&md_loop);

        Ok(tokio::task::spawn(md_loop))
    }

    pub fn create(&self) -> Result<Box<MessageDirectorService>> {
        Ok(Box::new(MessageDirectorService))
    }
}

impl StateServerService {
    pub async fn start(&self, _conf: DonetConfig) -> Result<()> {
        info!("Booting State Server service.");
        Ok(())
    }

    pub fn create(&self) -> Result<Box<StateServerService>> {
        Ok(Box::new(StateServerService))
    }
}

impl DatabaseServerService {
    pub async fn start(&self, _conf: DonetConfig) -> Result<()> {
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
            sql_config = db_server_conf.sql.unwrap();
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
        let res: utils::SqlResult = db.init_service();

        if res.is_err() {
            error!("Failed to initialize the Database Server.");
        }
        Ok(())
    }

    pub fn create(&self) -> Result<Box<DatabaseServerService>> {
        Ok(Box::new(DatabaseServerService))
    }
}

impl DBSSService {
    pub async fn start(&self, _conf: DonetConfig) -> Result<()> {
        info!("Booting DBSS service.");
        Ok(())
    }

    pub fn create(&self) -> Result<Box<DBSSService>> {
        Ok(Box::new(DBSSService))
    }
}

impl EventLoggerService {
    pub async fn start(&self, _conf: DonetConfig) -> Result<()> {
        info!("Booting Event Logger service.");
        Ok(())
    }

    pub fn create(&self) -> Result<Box<EventLoggerService>> {
        Ok(Box::new(EventLoggerService))
    }
}
