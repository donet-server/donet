//! DONET SOFTWARE
//!
//! Copyright (c) 2024, Donet Authors.
//!
//! This program is free software; you can redistribute it and/or modify
//! it under the terms of the GNU Affero General Public License version 3.
//! You should have received a copy of this license along
//! with this source code in a file named "LICENSE."
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU Affero General Public License
//! along with this program; if not, write to the Free Software Foundation,
//! Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
//!
//! <img src="https://github.com/donet-server/donet/blob/master/logo/donet_banner.png?raw=true" height=10%>
//!
//! # donet
//! Donet is a free and open source network engine designed after the Distributed Networking
//! protocol, as defined in the high-level networking API of the [Panda3D](https://panda3d.org)
//! game engine, which was originally developed by Disney Interactive (*formerly known as Disney
//! VR Studios*) to connect with their in-house server technology, the OTP (*Online Theme Park*)
//! server, which was used to power their massive multiplayer online games, such as Toontown
//! Online and Pirates of the Caribbean Online, from 2001 to 2013.
//!
//! See the project source on [Github](https://github.com/donet-server/donet). Feel free
//! to also visit the website, [donet-server.org](https://www.donet-server.org).
//!
//! If you're looking for the documentation of **libdonet**, click [here](https://libdonet.rs).

pub mod channel_map;
pub mod config;
pub mod dbserver;
pub mod logger;
pub mod message_director;
pub mod network;
pub mod service_factory;
pub mod utils;

fn main() -> std::io::Result<()> {
    use config::*;
    use libdonet::dcfile::DCFileInterface;
    use libdonet::globals::DCReadResult;
    use libdonet::read_dc_files;
    use log::{error, info};
    use service_factory::*;
    use std::fs::File;
    use std::io::{Error, ErrorKind, Read};
    use tokio::runtime::{Builder, Runtime};
    use tokio::task::JoinHandle;

    static VERSION_STRING: &str = "0.1.0";
    static GIT_SHA1: &str = env!("GIT_SHA1");
    let mut config_file: &str = "daemon.toml"; // default
    let args: Vec<String> = std::env::args().collect();

    let mut want_dc_check: bool = false;
    let mut dc_check_file: Option<&String> = None;
    let mut consumed_next_arg: bool = false;

    if args.len() > 1 {
        for item in args.iter().enumerate() {
            let (index, argument): (usize, &String) = item;
            if (index == 0) || consumed_next_arg {
                consumed_next_arg = false;
                continue;
            }
            if argument.starts_with('-') {
                if argument == "-h" || argument == "--help" {
                    print_help_page(config_file);
                    return Ok(());
                } else if argument == "-v" || argument == "--version" {
                    print_version(VERSION_STRING, GIT_SHA1);
                    return Ok(());
                } else if argument == "-c" || argument == "--check-dc" {
                    // Consume extra argument, catch error if out of arguments.
                    if let Some(dc_filename) = args.get(index + 1) {
                        consumed_next_arg = true;
                        want_dc_check = true;
                        dc_check_file = Some(dc_filename);
                    } else {
                        println!("donet: {}: Missing filename argument.\n", argument);
                        print_help_page(config_file);
                        return Ok(());
                    }
                    continue;
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
    logger::initialize_logger()?;

    if want_dc_check {
        if let Some(dc_file) = dc_check_file {
            let dc_read: DCReadResult = read_dc_files(vec![dc_file.to_owned()]);

            if let Ok(mut dc_file) = dc_read {
                info!("No issues found. DC File Hash: {}", dc_file.get_pretty_hash());
                return Ok(());
            }
            error!("Failed to parse DC file: {:?}", dc_read.unwrap_err());
            return Err(Error::new(ErrorKind::InvalidInput, "Failed to parse DC file."));
        } else {
            error!("No DC filename provided. Cannot do DC read.");
            return Err(Error::new(ErrorKind::InvalidInput, "No DC file given."));
        }
    }

    // Read the daemon configuration file
    let mut conf_file: File = File::open(config_file)?;
    let mut contents: String = String::new();
    conf_file.read_to_string(&mut contents)?;

    let toml_parse: Result<DonetConfig, toml::de::Error> = toml::from_str(contents.as_str());

    if let Err(toml_error) = toml_parse {
        error!("An error occurred while parsing the TOML configuration.");
        return Err(Error::new(ErrorKind::InvalidInput, toml_error.message()));
    }
    let daemon_config: DonetConfig = toml_parse.unwrap();

    // Once we've got arguments parsed and configuration read,
    // we are safe to start the Tokio asynchronous runtime.
    let tokio_runtime: Runtime = Builder::new_multi_thread()
        .enable_io()
        .thread_stack_size(2 * 1024 * 1024) // default: 2MB
        .build()?;

    let daemon_main = async move {
        #[allow(clippy::redundant_clone)]
        let services: Services = daemon_config.services.clone();

        // Smart pointers to new service instances on heap
        let ca_service: Box<ClientAgentService>;
        let md_service: Box<MessageDirectorService>;
        let ss_service: Box<StateServerService>;
        let db_service: Box<DatabaseServerService>;
        let dbss_service: Box<DBSSService>;
        let el_service: Box<EventLoggerService>;

        // Tokio join handles for spawned tasks of services started.
        let mut service_handles: Vec<JoinHandle<std::io::Result<()>>> = Vec::new();

        let want_client_agent: bool = services.client_agent.is_some();
        let want_message_director: bool = services.message_director.is_some();
        let want_state_server: bool = services.state_server.is_some();
        let want_database_server: bool = services.database_server.is_some();
        let want_dbss: bool = services.dbss.is_some();
        let want_event_logger: bool = services.event_logger.is_some();

        if want_client_agent {
            let ca_factory: ClientAgentService = ClientAgentService {};
            ca_service = ca_factory.create()?;

            #[allow(clippy::redundant_clone)]
            ca_service.start(daemon_config.clone()).await?;
        }
        if want_message_director {
            let md_factory: MessageDirectorService = MessageDirectorService {};
            md_service = md_factory.create()?;

            #[allow(clippy::redundant_clone)]
            service_handles.push(md_service.start(daemon_config.clone()).await?);
        }
        if want_state_server {
            let ss_factory: StateServerService = StateServerService {};
            ss_service = ss_factory.create()?;

            #[allow(clippy::redundant_clone)]
            ss_service.start(daemon_config.clone()).await?;
        }
        if want_database_server {
            let db_factory: DatabaseServerService = DatabaseServerService {};
            db_service = db_factory.create()?;

            #[allow(clippy::redundant_clone)]
            db_service.start(daemon_config.clone()).await?;
        }
        if want_dbss {
            let dbss_factory: DBSSService = DBSSService {};
            dbss_service = dbss_factory.create()?;

            #[allow(clippy::redundant_clone)]
            dbss_service.start(daemon_config.clone()).await?;
        }
        if want_event_logger {
            let el_factory: EventLoggerService = EventLoggerService {};
            el_service = el_factory.create()?;

            #[allow(clippy::redundant_clone)]
            el_service.start(daemon_config.clone()).await?;
        }

        loop {
            // TODO: Iterate through services' join handles
            // and abort them once an interrupt signal is received.
        }
    };
    // Hack to reassure the compiler that I want to return an IO result.
    utils::set_future_return_type::<std::io::Result<()>, _>(&daemon_main);

    tokio_runtime.block_on(daemon_main)
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
        -v, --version   Print Donet binary build version & info.\n\
        -c, --check-dc  Run the DC parser on the given DC file.\n",
        config_path
    );
}

#[rustfmt::skip]
fn print_version(version_string: &str, git_sha1: &str) {
    let bin_arch: &str = if cfg!(target_arch = "x86") { "x86" }
    else if cfg!(target_arch = "x86_64") { "x86_64" }
    else if cfg!(target_arch = "aarch64") { "aarch64" }
    else { "unknown" };

    let bin_platform: &str = if cfg!(target_os = "linux") { "linux" }
    else if cfg!(target_os = "windows") { "windows" }
    else if cfg!(target_os = "macos") { "macos" }
    else if cfg!(target_os = "freebsd") { "freebsd" }
    else { "unknown" };

    let bin_env: &str = if cfg!(target_env = "gnu") { "gnu" }
    else if cfg!(target_env = "msvc") { "msvc" }
    else { "other" };

    println!(
        "{}Donet{}, version {} ({} {}-{})\n\
        Revision (Git SHA1): {}\n\n\
        Released under the AGPL-3.0 license. <https://www.gnu.org/licenses/agpl-3.0.html>\n\
        View the source code on GitHub: https://github.com/donet-server/donet\n",
        logger::_ANSI_MAGENTA, logger::_ANSI_RESET,
        version_string, bin_arch, bin_platform, bin_env, git_sha1
    );
}
