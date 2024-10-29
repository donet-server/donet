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

use crate::config;
use crate::service::DonetService;
use libdonet::dcfile::DCFile;
use libdonet::globals;
use log::{error, info};
use mysql::prelude::*;
use mysql::*;
use std::io::{Error, ErrorKind, Result};
use tokio::task::JoinHandle;

// MySQL Result (mysql crate API response)
pub type SqlResult = std::result::Result<(), Box<dyn std::error::Error>>;

pub struct DBCredentials {
    pub host: String,
    pub port: i16,
    pub database: String,
    pub user: String,
    pub password: String,
}

/// Native representation of SQL db tables
#[derive(Debug, PartialEq, Eq)]
struct Object {
    doid: globals::DoId,       // INT UNSIGNED NOT NULL PRIMARY KEY
    dclass: globals::DClassId, // SMALLINT UNSIGNED NOT NULL
}

#[derive(Debug, PartialEq, Eq)]
struct DClass {
    dclass: globals::DClassId, // SMALLINT UNSIGNED NOT NULL PRIMARY KEY
    name: String,              // VARCHAR(32) NOT NULL
    storable: bool,            // BOOLEAN NOT NULL
}

// FIXME: Every dclass field that has the 'db' keyword has its
// own SQL table created in the database. Not sure if this struct
// will be able to represent all field tables.
#[derive(Debug, PartialEq, Eq)]
struct Field {
    doid: globals::DoId,       // INT UNSIGNED NOT NULL PRIMARY KEY
    dclass: globals::DClassId, // SMALLINT UNSIGNED NOT NULL
    field: globals::FieldId,   // SMALLINT UNSIGNED NOT NULL
    parameters: Vec<Vec<u8>>,  // NOT NULL
}

pub struct DatabaseServer {
    dc_file: DCFile<'static>,
    _sql_pool: Pool,
    sql_conn: PooledConn,
    _credentials: DBCredentials,
}

impl DonetService for DatabaseServer {
    type Service = Self;
    type Configuration = config::DBServer;

    async fn create(conf: Self::Configuration, dc: DCFile<'static>) -> Result<Self::Service> {
        // TODO: Check for db backend type once we have multiple DB backend support.
        let sql_config: config::SQL;
        let host_port: Vec<&str>;

        if conf.sql.is_some() {
            sql_config = conf.sql.unwrap();
            host_port = sql_config.host.rsplit(':').collect();
        } else {
            error!("Incomplete configuration for DB server service.");
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Missing database backend credentials.",
            ));
        }

        let creds: DBCredentials = DBCredentials {
            host: host_port[1].to_owned(),
            port: host_port[0].parse::<i16>().unwrap(),
            database: sql_config.database.to_owned(),
            user: sql_config.user.to_owned(),
            password: sql_config.pass.to_owned(),
        };

        let port_str: &str = &creds.port.to_string();
        let url: String = format!(
            "mysql://{}:{}@{}:{}/{}",
            creds.user, creds.password, creds.host, port_str, creds.database
        );
        let url_str: &str = url.as_str(); // can't do `as_str()` in line above, due to lifetime

        info!(
            "Connecting to SQL database backend with URL: {}",
            format!(
                "mysql://{}:****@{}:{}/{}",
                creds.user, creds.host, port_str, creds.database
            )
        );
        let p_res: std::result::Result<Pool, mysql::Error> = Pool::new(url_str); // FIXME: This is not async!

        // FIXME: Clippy recommends bad code, so we're ignoring, but we need to fix later.
        #[allow(clippy::needless_late_init)]
        let pool: Pool;

        if let Ok(res_ok) = p_res {
            pool = res_ok;
        } else {
            // FIXME: I cannot find a solution to returning this error. Since this is
            // the constructor, I can only return a `DatabaseServer` struct, meaning I
            // can't pass the error over to whoever is calling this method. So if issues
            // occur with establishing the conn, the service will simply panic and halt.
            error!("Failed to create SQL conn pool: {}", p_res.unwrap_err());
            panic!("An error occurred while connecting to the SQL database.");
        }

        let c_res: std::result::Result<PooledConn, mysql::Error> = pool.get_conn();

        #[allow(clippy::needless_late_init)]
        let conn: PooledConn;

        if let Ok(res_ok) = c_res {
            conn = res_ok;
        } else {
            error!(
                "Failed to get SQL conn from pooled connection: {}",
                c_res.unwrap_err()
            );
            panic!("An error occurred while connecting to the SQL database.");
        }

        Ok(DatabaseServer {
            dc_file: dc,
            _sql_pool: pool,
            sql_conn: conn,
            _credentials: creds,
        })
    }

    async fn start(conf: config::DonetConfig, dc: DCFile<'static>) -> Result<JoinHandle<Result<()>>> {
        // NOTE: We are unwrapping an Option without checking,
        // as this method can only be called if 'database_server'
        // is of a 'Some' type, which guarantees no panic scenario.
        let db_server_conf: config::DBServer = conf.services.database_server.unwrap();

        let mut db: DatabaseServer = DatabaseServer::create(db_server_conf, dc).await?;
        let res: Result<()> = db.main().await;

        if res.is_err() {
            error!("Failed to initialize the Database Server.");
        }
        Ok(Self::spawn_async_task(async move { db.main().await }))
    }

    async fn main(&mut self) -> Result<()> {
        self.check_database_tables().unwrap(); // FIXME
        Ok(())
    }
}

impl DatabaseServer {
    // If the Objects, DClasses, & Fields tables do not exist in the
    // database, then we will create the required tables automatically.
    fn check_database_tables(&mut self) -> SqlResult {
        self.sql_conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS objects (
                                    doid INT UNSIGNED NOT NULL PRIMARY KEY,
                                    dclass SMALLINT UNSIGNED NOT NULL
                                );",
        )?;
        // NOTE: dclasses table restricts dclass names to be at max 32 chars.
        self.sql_conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS dclasses (
                                    dclass SMALLINT UNSIGNED NOT NULL PRIMARY KEY,
                                    name VARCHAR(32) NOT NULL,
                                    storable BOOLEAN NOT NULL
                                );",
        )?;
        Ok(())
    }
}

// DBServer Unit Testing
//#[cfg(test)]
//mod tests {
//    #[allow(unused_imports)] // FIXME: remove once we write tests
//    use super::*;
//}
