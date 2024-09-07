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

use crate::config;
#[cfg(feature = "database-server")]
use crate::database_server::dbserver::{DBCredentials, DatabaseServer};
#[cfg(feature = "event-logger")]
use crate::event_logger::event_logger::EventLogger;
#[cfg(feature = "message-director")]
use crate::message_director::message_director::MessageDirector;
use crate::utils;
use log::{error, info};
use std::io::{Error, ErrorKind, Result};
use tokio::task::JoinHandle;

// All Donet service types.
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

#[cfg(feature = "client-agent")]
impl ClientAgentService {
    // TODO: implement client agent xd
    pub async fn start(&self, _conf: config::DonetConfig) -> Result<()> {
        info!("Booting Client Agent service.");
        Ok(())
    }

    pub fn create(&self) -> Result<ClientAgentService> {
        Ok(ClientAgentService)
    }
}

#[cfg(feature = "message-director")]
impl MessageDirectorService {
    pub async fn start(&self, conf: config::DonetConfig) -> Result<JoinHandle<Result<()>>> {
        info!("Booting Message Director service.");

        // We can unwrap safely here since this function only is called if it is `Some`.
        let service_conf: config::MessageDirector = conf.services.message_director.unwrap();

        let mut upstream: Option<String> = None;

        if let Some(upstream_some) = service_conf.upstream {
            // This Message Director will connect to an upstream MD.
            upstream = Some(upstream_some);
        }

        let md: MessageDirector = MessageDirector::new(service_conf.bind.as_str(), upstream).await?;

        // Prepare the Message Director's main async loop to spawn a new Tokio task.
        let md_loop = async move { md.init_network().await };
        utils::set_future_return_type::<Result<()>, _>(&md_loop);

        Ok(tokio::task::spawn(md_loop))
    }

    pub fn create(&self) -> Result<MessageDirectorService> {
        Ok(MessageDirectorService)
    }
}

#[cfg(feature = "state-server")]
impl StateServerService {
    pub async fn start(&self, _conf: config::DonetConfig) -> Result<()> {
        info!("Booting State Server service.");
        Ok(())
    }

    pub fn create(&self) -> Result<StateServerService> {
        Ok(StateServerService)
    }
}

#[cfg(feature = "database-server")]
impl DatabaseServerService {
    pub async fn start(&self, _conf: config::DonetConfig) -> Result<()> {
        info!("Booting Database Server service.");

        // NOTE: We are unwrapping an Option without checking,
        // as this method can only be called if 'database_server'
        // is of a 'Some' type, which guarantees no panic scenario.
        let db_server_conf: config::DBServer = _conf.services.database_server.unwrap();

        // TODO: Check for db backend type once we
        // have multiple DB backend support.
        let sql_config: config::SQL;
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

    pub fn create(&self) -> Result<DatabaseServerService> {
        Ok(DatabaseServerService)
    }
}

#[cfg(feature = "dbss")]
impl DBSSService {
    pub async fn start(&self, _conf: config::DonetConfig) -> Result<()> {
        info!("Booting DBSS service.");
        Ok(())
    }

    pub fn create(&self) -> Result<DBSSService> {
        Ok(DBSSService)
    }
}

#[cfg(feature = "event-logger")]
impl EventLoggerService {
    pub async fn start(&self, conf: config::DonetConfig) -> Result<JoinHandle<Result<()>>> {
        info!("Booting Event Logger service.");

        // We can unwrap safely here since this function only is called if it is `Some`.
        let service_conf = conf.services.event_logger.unwrap();

        let mut service: EventLogger = EventLogger::new(service_conf).await?;

        let service_loop = async move { service.start_receive().await };
        utils::set_future_return_type::<Result<()>, _>(&service_loop);

        Ok(tokio::task::spawn(service_loop))
    }

    pub fn create(&self) -> Result<EventLoggerService> {
        Ok(EventLoggerService)
    }
}
