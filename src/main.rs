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

//#[path = "config.rs"]
//mod config;
#[path = "dbserver.rs"]
mod dbserver;
#[path = "logger.rs"]
mod logger;

fn main() {
    use self::logger::logger;
    use log::SetLoggerError;

    // Initialize the logger utility
    let res: Result<(), SetLoggerError> = logger::initialize_logger();

    if res.is_err() {
        panic!("Failed to initialize the logger utility!");
    }

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let _option: &String = &args[1];
    }

    // FIXME: Remove temporary bootstrap code for prototype dbserver
    use dbserver::dbserver::DBCredentials;
    use dbserver::dbserver::DatabaseServer;

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
        panic!("error haha");
    }
}
