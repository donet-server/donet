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

use libdonet::globals;
use log::{error, info};
use mysql::prelude::*;
use mysql::*;
use std::vec::Vec;

pub struct DBCredentials<'a> {
    pub host: &'a str,
    pub port: i16,
    pub database: &'a str,
    pub user: &'a str,
    pub password: &'a str,
}

// Rust representations of SQL db tables
#[derive(Debug, PartialEq, Eq)]
struct Object {
    doid: globals::DoId,       // INT UNSIGNED NOT NULL PRIMARY KEY
    dclass: globals::DClassId, // SMALLINT UNSIGNED NOT NULL
}

#[derive(Debug, PartialEq, Eq)]
struct DClass<'a> {
    dclass: globals::DClassId, // SMALLINT UNSIGNED NOT NULL PRIMARY KEY
    name: &'a str,             // VARCHAR(32) NOT NULL
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

pub struct DatabaseServer<'a> {
    _sql_pool: Pool, // NOTE: afaik, we don't need to use this, but we need it to live.
    sql_conn: PooledConn,
    _credentials: DBCredentials<'a>,
}

impl DatabaseServer<'_> {
    pub fn new(creds: DBCredentials) -> DatabaseServer {
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
        let p_res: Result<Pool, Error> = Pool::new(url_str);

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

        let c_res: Result<PooledConn, Error> = pool.get_conn();

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

        DatabaseServer {
            _sql_pool: pool,
            sql_conn: conn,
            _credentials: creds,
        }
    }

    pub fn init_service(&mut self) -> globals::SqlResult {
        self.check_database_tables()?;
        Ok(())
    }

    // If the Objects, DClasses, & Fields tables do not exist in the
    // database, then we will create the required tables automatically.
    pub fn check_database_tables(&mut self) -> globals::SqlResult {
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
