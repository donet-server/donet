//! DONET SOFTWARE
//!
//! Copyright (c) 2024, Donet Authors.
//!
//! This program is free software; you can redistribute it and/or modify
//! it under the terms of the GNU Affero General Public License version 3.
//! You should have received a copy of this license along
//! with this source code in a file named "COPYING."
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
//! <img src="https://gitlab.com/donet-server/donet/-/raw/master/logo/donet_banner.png" height=10%>
//!
//! # donet
//! Donet is a free and open source network engine designed after the Distributed Networking
//! protocol, as defined in the high-level networking API of the [Panda3D](https://panda3d.org)
//! game engine, which was originally developed by Disney Interactive (*formerly known as Disney
//! VR Studios*) to connect with their in-house server technology, the OTP (*Online Theme Park*)
//! server, which was used to power their massive multiplayer online games, such as Toontown
//! Online and Pirates of the Caribbean Online, from 2001 to 2013.
//!
//! See the project source on [GitLab](https://gitlab.com/donet-server/donet). Feel free
//! to also visit the website, [donet-server.org](https://www.donet-server.org).
//!
//! If you're looking for the documentation of **libdonet**, click [here](https://docs.donet-server.org/libdonet).

#![allow(clippy::module_inception)]
#![deny(unused_extern_crates)]

mod config;
#[cfg(feature = "database-server")]
mod database_server;
mod event;
#[cfg(feature = "event-logger")]
mod event_logger;
mod logger;
mod meson;
#[cfg(feature = "message-director")]
mod message_director;
mod network;
mod service_factory;
mod utils;

#[macro_use]
extern crate cfg_if;
use meson::*;

#[derive(Clone, Copy)]
enum FlagArguments {
    DCFilePath,
}

fn main() -> std::io::Result<()> {
    use config::*;
    use libdonet::dcfile::DCFile;
    use libdonet::read_dc_files;
    use log::{error, info};
    use logger::DaemonLogger;
    use service_factory::*;
    use std::fs::File;
    use std::io::{Error, ErrorKind, Read};
    use tokio::runtime::{Builder, Runtime};
    use tokio::task::JoinHandle;

    let args: Vec<String> = std::env::args().collect();
    let mut config_file: &str = DEFAULT_TOML;
    let mut want_dc_check: bool = false;
    let mut dc_check_files: Vec<String> = vec![];
    let mut expecting_flag_argument: Option<FlagArguments> = None;

    if args.len() > 1 {
        for item in args.iter().enumerate() {
            let (index, argument): (usize, &String) = item;
            if index == 0 {
                continue; // skip invoked binary name
            }
            if argument.starts_with('-') {
                if argument == "-h" || argument == "--help" {
                    print_help_page();
                    return Ok(());
                } else if argument == "-v" || argument == "--version" {
                    print_version();
                    return Ok(());
                } else if argument == "-c" || argument == "--validate-dc" {
                    want_dc_check = true;
                    expecting_flag_argument = Some(FlagArguments::DCFilePath);
                    continue;
                } else {
                    println!("{}: {}: Invalid flag.\n", BINARY, argument);
                    print_help_page();
                    return Ok(());
                }
            } else if let Some(expect_flag_arg) = expecting_flag_argument {
                match expect_flag_arg {
                    FlagArguments::DCFilePath => {
                        dc_check_files.push(argument.to_owned());

                        // Look ahead to see if we should expect more args.
                        if let Some(lookahead) = args.get(index + 1) {
                            if !lookahead.ends_with(".dc") {
                                expecting_flag_argument = None;
                            }
                            continue;
                        }
                        expecting_flag_argument = None;
                    }
                }
            } else if index == (args.len() - 1) {
                // last argument given & we're not expecting more arguments,
                // so it must be the configuration file path given.
                config_file = argument.as_str();
                break;
            } else {
                println!("{}: {}: Invalid argument.\n", BINARY, argument);
                print_help_page();
                return Ok(());
            }
        }
        if expecting_flag_argument.is_some() {
            println!("{}: Expected more arguments.\n", BINARY);
            print_help_page();
            return Ok(());
        }
    }

    // Read the daemon configuration file
    let mut conf_file: File = File::open(config_file)?;
    let mut contents: String = String::new();

    conf_file.read_to_string(&mut contents)?;
    drop(conf_file); // we're in the main scope, so lets drop manually here

    let toml_parse: Result<DonetConfig, toml::de::Error> = toml::from_str(contents.as_str());
    drop(contents);

    if let Err(toml_error) = toml_parse {
        error!("An error occurred while parsing the TOML configuration.");
        return Err(Error::new(ErrorKind::InvalidInput, toml_error.message()));
    }
    let daemon_config: DonetConfig = toml_parse.unwrap();

    // Now that configuration file is parsed, we can create the logger.
    if let Some(log_level) = &daemon_config.daemon.log_level {
        match log_level.as_str() {
            "error" => {
                pub static GLOBAL_LOGGER: DaemonLogger = DaemonLogger {
                    log_level: log::Level::Error,
                };
                logger::init_logger(&GLOBAL_LOGGER)?;
            }
            "warn" => {
                pub static GLOBAL_LOGGER: DaemonLogger = DaemonLogger {
                    log_level: log::Level::Warn,
                };
                logger::init_logger(&GLOBAL_LOGGER)?;
            }
            "info" => {
                pub static GLOBAL_LOGGER: DaemonLogger = DaemonLogger {
                    log_level: log::Level::Info,
                };
                logger::init_logger(&GLOBAL_LOGGER)?;
            }
            "debug" => {
                pub static GLOBAL_LOGGER: DaemonLogger = DaemonLogger {
                    log_level: log::Level::Debug,
                };
                logger::init_logger(&GLOBAL_LOGGER)?;
            }
            "trace" => {
                pub static GLOBAL_LOGGER: DaemonLogger = DaemonLogger {
                    log_level: log::Level::Trace,
                };
                logger::init_logger(&GLOBAL_LOGGER)?;
            }
            _ => panic!("Could not initialize logger. Error in log level string in TOML configuration."),
        }
    } else {
        pub static GLOBAL_LOGGER: DaemonLogger = DaemonLogger {
            log_level: log::Level::Info,
        };
        logger::init_logger(&GLOBAL_LOGGER)?;
    }

    // If `--validate-dc` argument was received, parse DC files and exit.
    if want_dc_check {
        return validate_dc_files(dc_check_files);
    }
    drop(args);

    // At this point in execution, the program has not exited,
    // so we can start the process of booting the Donet daemon.
    //
    // First step is to read the DC files listed in the daemon configuration.
    // Services like the Event Logger and Message Director do not need the DC file.
    cfg_if! {
        if #[cfg(feature = "requires_dc")] {
            let files: Vec<String> = daemon_config.global.dc_files.clone();
            let dc_read = read_dc_files(files);

            if let Err(dc_err) = dc_read {
                error!("Failed to parse DC file(s): {:?}", dc_err);
                return Err(Error::new(ErrorKind::InvalidInput, "Failed to parse DC file."));
            }

            let dc: DCFile = dc_read.unwrap();
        }
    }

    // Everything is prepped for the daemon, so we
    // are safe to start the Tokio asynchronous runtime.
    let tokio_runtime: Runtime = Builder::new_multi_thread()
        .enable_io()
        .thread_stack_size(2 * 1024 * 1024) // default: 2MB
        .build()?;

    let daemon_async_main = async move {
        let services: Services = daemon_config.services.clone();

        // New service instances on the main thread's stack
        #[cfg(feature = "client-agent")]
        let ca_service: ClientAgentService;
        #[cfg(feature = "message-director")]
        let md_service: MessageDirectorService;
        #[cfg(feature = "state-server")]
        let ss_service: StateServerService;
        #[cfg(feature = "database-server")]
        let db_service: DatabaseServerService;
        #[cfg(feature = "dbss")]
        let dbss_service: DBSSService;
        #[cfg(feature = "event-logger")]
        let el_service: EventLoggerService;

        // Tokio join handles for spawned tasks of services started.
        let mut service_handles: Vec<JoinHandle<std::io::Result<()>>> = vec![];

        #[cfg(feature = "client-agent")]
        let want_client_agent: bool = services.client_agent.is_some();
        #[cfg(feature = "message-director")]
        let want_message_director: bool = services.message_director.is_some();
        #[cfg(feature = "state-server")]
        let want_state_server: bool = services.state_server.is_some();
        #[cfg(feature = "database-server")]
        let want_database_server: bool = services.database_server.is_some();
        #[cfg(feature = "dbss")]
        let want_dbss: bool = services.dbss.is_some();
        #[cfg(feature = "event-logger")]
        let want_event_logger: bool = services.event_logger.is_some();

        cfg_if! {
            if #[cfg(feature = "client-agent")] {
                if want_client_agent {
                    let ca_factory: ClientAgentService = ClientAgentService {};
                    ca_service = ca_factory.create()?;

                    ca_service.start(daemon_config.clone()).await?;
                }
            }
        }
        cfg_if! {
            if #[cfg(feature = "message-director")] {
                if want_message_director {
                    let md_factory: MessageDirectorService = MessageDirectorService {};
                    md_service = md_factory.create()?;

                    let handle = md_service.start(daemon_config.clone()).await?;
                    service_handles.push(handle);
                }
            }
        }
        cfg_if! {
            if #[cfg(feature = "state-server")] {
                if want_state_server {
                    let ss_factory: StateServerService = StateServerService {};
                    ss_service = ss_factory.create()?;

                    ss_service.start(daemon_config.clone()).await?;
                }
            }
        }
        cfg_if! {
            if #[cfg(feature = "database-server")] {
                if want_database_server {
                    let db_factory: DatabaseServerService = DatabaseServerService {};
                    db_service = db_factory.create()?;

                    db_service.start(daemon_config.clone()).await?;
                }
            }
        }
        cfg_if! {
            if #[cfg(feature = "dbss")] {
                if want_dbss {
                    let dbss_factory: DBSSService = DBSSService {};
                    dbss_service = dbss_factory.create()?;

                    dbss_service.start(daemon_config.clone()).await?;
                }
            }
        }
        cfg_if! {
            if #[cfg(feature = "event-logger")] {
                if want_event_logger {
                    let el_factory: EventLoggerService = EventLoggerService {};
                    el_service = el_factory.create()?;

                    let handle = el_service.start(daemon_config.clone()).await?;
                    service_handles.push(handle);
                }
            }
        }

        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                println!();
                info!("Received interrupt (Ctrl + C)");
            }
            Err(err) => {
                error!("Unable to listen for shutdown signal: {}", err);
                panic!("Tokio was not able to listen to the interrupt signal.")
            }
        }
        info!("Exiting...");

        // Abort all spawned Tokio tasks.
        for handle in &service_handles {
            handle.abort();
        }
        // Await task handles to wrap things up; Expect a cancellation error.
        for handle in service_handles {
            assert!(handle.await.unwrap_err().is_cancelled());
        }
        Ok(())
    };

    // Hack to reassure the compiler that I want to return an IO result.
    utils::set_future_return_type::<std::io::Result<()>, _>(&daemon_async_main);

    tokio_runtime.block_on(daemon_async_main)
}

fn validate_dc_files(files: Vec<String>) -> std::io::Result<()> {
    use libdonet::read_dc_files;
    use log::{error, info};
    use std::io::{Error, ErrorKind};

    let dc_read = read_dc_files(files.to_owned());

    if let Ok(dc_file) = dc_read {
        let hash: u32 = dc_file.get_hash();
        let signed: i32 = hash as i32;
        let pretty: String = dc_file.get_pretty_hash();

        info!(
            "No issues found. File hash is {} (signed {}, hex {})",
            hash, signed, pretty
        );
        return Ok(());
    }
    error!("Failed to parse DC file: {:?}", dc_read.unwrap_err());

    Err(Error::new(ErrorKind::InvalidInput, "Failed to parse DC file."))
}

fn print_help_page() {
    println!(
        "Usage:    {} [options] ... [CONFIG_FILE]\n\
        \n\
        Donet - Distributed Object Network Engine.\n\
        This binary will look for a configuration file (.toml)\n\
        in the current working directory as \"{}\".\n\
        \n\
        -h, --help          Print the help page.\n\
        -v, --version       Print Donet binary build version & info.\n\
        -c, --validate-dc   Run the libdonet DC parser on the given DC file.\n",
        BINARY, DEFAULT_TOML
    );
}

#[rustfmt::skip]
fn print_version() {
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
        This program is free software; you can redistribute it and/or modify\n\
        it under the terms of the GNU Affero General Public License version 3.\n\
        You should have received a copy of this license along\n\
        with this source code in a file named \"COPYING.\"\n\n\
        This program is distributed in the hope that it will be useful,\n\
        but WITHOUT ANY WARRANTY; without even the implied warranty of\n\
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the\n\
        GNU General Public License for more details.\n\n\
        You should have received a copy of the GNU Affero General Public License\n\
        along with this program; if not, write to the Free Software Foundation,\n\
        Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.",
        logger::_ANSI_MAGENTA, logger::_ANSI_RESET,
        VERSION, bin_arch, bin_platform, bin_env, VCS_TAG
    );
}
