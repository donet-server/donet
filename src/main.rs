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

pub mod channel_map;
pub mod config;
pub mod datagram;
pub mod dbserver;
pub mod globals;
pub mod logger;
pub mod message_director;
pub mod network;
pub mod protocol;
pub mod service_factory;

fn main() -> std::io::Result<()> {
    use config::*;
    use log::error;
    use service_factory::*;
    use std::fs::File;
    use std::io::{Error, ErrorKind, Read};

    static VERSION_STRING: &str = "0.1.0";
    static GIT_SHA1: &str = env!("GIT_SHA1");
    let mut config_file: &str = "daemon.toml"; // default
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let mut index: usize = 0;
        for argument in &args {
            if index == 0 {
                index += 1; // skip binary name
                continue;
            }
            if argument.starts_with('-') {
                if argument == "-h" || argument == "--help" {
                    print_help_page(config_file);
                    return Ok(());
                } else if argument == "-v" || argument == "--version" {
                    print_version(VERSION_STRING, GIT_SHA1);
                    return Ok(());
                } else {
                    println!("donet: {}: Invalid argument.\n", argument);
                    print_help_page(config_file);
                    return Ok(());
                }
            } else if index == (args.len() - 1) {
                // last argument given & doesn't match any of the above,
                // so it must be the configuration file path given.
                config_file = argument.as_str();
                break;
            }
        }
    }

    // Read the daemon configuration file
    let mut conf_file: File = File::open(config_file)?;
    let mut contents: String = String::new();
    conf_file.read_to_string(&mut contents)?;

    let toml_parse: Result<DonetConfig, toml::de::Error> = toml::from_str(contents.as_str());

    if let Ok(daemon_config) = toml_parse {
        #[allow(clippy::redundant_clone)]
        let services: Services = daemon_config.services.clone();

        logger::initialize_logger()?;

        let want_client_agent: bool = services.client_agent.is_some();
        let want_message_director: bool = services.message_director.is_some();
        let want_state_server: bool = services.state_server.is_some();
        let want_database_server: bool = services.database_server.is_some();
        let want_dbss: bool = services.dbss.is_some();
        let want_event_logger: bool = services.event_logger.is_some();

        // FIXME: I'm using the fancy 'factories' software design pattern,
        // but I still ended up writing repetitive code here. Improve!
        // FIXME: clippy doesn't like redundant clones, but they're needed.
        // This is probably bad code but it works. Will fix later.

        if want_client_agent {
            // Initialize the Client Agent
            let ca_factory: ClientAgentService = ClientAgentService {};
            let ca_service: Box<dyn DonetService> = ca_factory.create()?;

            #[allow(clippy::redundant_clone)]
            ca_service.start(daemon_config.clone())?;
        }
        if want_message_director {
            // Initialize the Message Director
            let md_factory: MessageDirectorService = MessageDirectorService {};
            let md_service: Box<dyn DonetService> = md_factory.create()?;

            #[allow(clippy::redundant_clone)]
            md_service.start(daemon_config.clone())?;
        }
        if want_state_server {
            // Initialize the State Server
            let ss_factory: StateServerService = StateServerService {};
            let ss_service: Box<dyn DonetService> = ss_factory.create()?;

            #[allow(clippy::redundant_clone)]
            ss_service.start(daemon_config.clone())?;
        }
        if want_database_server {
            // Initialize the Database Server
            let db_factory: DatabaseServerService = DatabaseServerService {};
            let db_service: Box<dyn DonetService> = db_factory.create()?;

            #[allow(clippy::redundant_clone)]
            db_service.start(daemon_config.clone())?;
        }
        if want_dbss {
            // Initialize the Database State Server
            let dbss_factory: DBSSService = DBSSService {};
            let dbss_service: Box<dyn DonetService> = dbss_factory.create()?;

            #[allow(clippy::redundant_clone)]
            dbss_service.start(daemon_config.clone())?;
        }
        if want_event_logger {
            // Initialize the Event Logger
            let el_factory: EventLoggerService = EventLoggerService {};
            let el_service: Box<dyn DonetService> = el_factory.create()?;

            #[allow(clippy::redundant_clone)]
            el_service.start(daemon_config.clone())?;
        }
        return Ok(());
    }
    // if not ok, then parsing threw an error.
    error!("An error occurred while parsing the TOML configuration.");
    Err(Error::new(
        ErrorKind::InvalidInput,
        toml_parse.unwrap_err().message(),
    ))
}

fn print_help_page(config_path: &str) {
    println!(
        "Usage:    donet [options] ... [CONFIG_FILE]\n\
        \n\
        Donet - Distributed Object Network Engine.\n\
        This binary will look for a configuration file (.toml)\n\
        in the current working directory as \"{}\".\n\
        \n\
        -h, --help      Print the help page.\n\
        -v, --version   Print Donet binary build version & info.\n",
        config_path
    );
}

fn print_version(version_string: &str, git_sha1: &str) {
    let bin_arch: &str = if cfg!(target_arch = "x86") {
        "x86"
    } else if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "unknown" // aka not supported
    };
    let bin_platform: &str = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "freebsd") {
        "freebsd"
    } else {
        "unknown" // aka not supported
    };
    let bin_env: &str = if cfg!(target_env = "gnu") {
        "gnu"
    } else if cfg!(target_env = "msvc") {
        "msvc"
    } else {
        "other"
    };
    println!(
        "Donet, version {} ({} {}-{})\n\
        Revision (Git SHA1): {}\n\n\
        Released under the AGPL-3.0 license. <https://www.gnu.org/licenses/agpl-3.0.html>\n\
        View the source code on GitHub: https://github.com/donet-server/Donet\n",
        version_string, bin_arch, bin_platform, bin_env, git_sha1
    );
}
