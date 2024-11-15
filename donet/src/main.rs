/*
    This file is part of Donet.

    Copyright © 2024 Max Rodriguez <me@maxrdz.com>

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
//! If you're looking for the documentation of **donet-core**, click [here](https://docs.donet-server.org/donet_core).

#![deny(unused_extern_crates)]

#[macro_use]
extern crate cfg_if;
use donet_daemon::meson::*;

#[cfg(feature = "requires_dc")]
use donet_core::{dconfig::DCFileConfig, read_dc_files};
use donet_daemon::config::*;
use donet_daemon::logger;
use donet_daemon::logger::DaemonLogger;
use donet_daemon::service::*;
use log::*;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;

#[derive(Clone, Copy)]
enum FlagArguments {
    DCFilePath,
}

// Macro for defining global logger static and initializing it.
macro_rules! init_logger {
    ($level:expr) => {
        pub static GLOBAL_LOGGER: DaemonLogger = DaemonLogger { log_level: $level };
        logger::init_logger(&GLOBAL_LOGGER)?;

        info!("Log level set at {}.", $level);
    };
}

fn main() -> std::io::Result<()> {
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
    let mut conf_file: File = match File::open(config_file) {
        Err(err) => {
            println!("Could not load TOML configuration.");
            println!("Donet cannot start without a configuration file present.");
            return Err(err);
        }
        Ok(file) => file,
    };

    let mut contents: String = String::new();

    conf_file.read_to_string(&mut contents)?;
    drop(conf_file); // we're in the main scope, so lets drop manually here

    // Deserialize the TOML config file to our [`DonetConfig`] struct.
    let daemon_config: DonetConfig = match toml::from_str(contents.as_str()) {
        Ok(config) => config,
        Err(err) => {
            error!("An error occurred while parsing the TOML configuration.");
            return Err(Error::new(ErrorKind::InvalidInput, err.message()));
        }
    };
    drop(contents);

    // Now that configuration file is parsed, we can create the logger.
    if let Some(log_level) = &daemon_config.daemon.log_level {
        match log_level.as_str() {
            "error" => {
                init_logger!(log::Level::Error);
            }
            "warn" => {
                init_logger!(log::Level::Warn);
            }
            "info" => {
                init_logger!(log::Level::Info);
            }
            "debug" => {
                init_logger!(log::Level::Debug);
            }
            "trace" => {
                init_logger!(log::Level::Trace);
            }
            _ => panic!("Could not initialize logger. Error in log level string in TOML configuration."),
        }
    } else {
        init_logger!(log::Level::Info);
    }

    // If `--validate-dc` argument was received, parse DC files and exit.
    if want_dc_check {
        cfg_if! {
            if #[cfg(feature = "requires_dc")] {
                return validate_dc_files(&daemon_config, dc_check_files);
            } else {
                error!("This build of Donet does not include DC file support.");
                return Err(Error::new(ErrorKind::Unsupported, "No DC file support."));
            }
        }
    }

    // At this point in execution, the program has not exited, which
    // means all arguments have been read and executed, if executed,
    // and now we can start the process of booting the Donet daemon.
    drop(args);

    // First step is to read the DC files listed in the daemon configuration.
    // Services like the Event Logger and Message Director do not need the DC file.
    cfg_if! {
        if #[cfg(feature = "requires_dc")] {
            let conf: DCFileConfig = daemon_config.clone().into();
            let files: Vec<String> = daemon_config.global.dc_files.clone();

            let dc: DCFile = match read_dc_files(conf, files) {
                Ok(dc) => dc,
                Err(dc_err) => {
                    error!("Failed to parse DC file(s): {}", dc_err);
                    return Err(Error::new(ErrorKind::InvalidInput, "Failed to parse DC file."));
                }
            };
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

        // Tokio join handles for spawned tasks of services started.
        let mut service_handles: Vec<JoinHandle<std::io::Result<()>>> = vec![];

        let want_client_agent: bool = services.client_agent.is_some();
        let want_message_director: bool = services.message_director.is_some();
        let want_state_server: bool = services.state_server.is_some();
        let want_database_server: bool = services.database_server.is_some();
        let want_dbss: bool = services.dbss.is_some();
        let want_event_logger: bool = services.event_logger.is_some();

        cfg_if! {
            if #[cfg(feature = "client-agent")] {
                if want_client_agent {
                    info!("Booting Client Agent service.");
                    todo!("CA not yet implemented.")
                }
            } else {
                if want_client_agent {
                    feature_warn("Client Agent");
                }
            }
        }
        cfg_if! {
            if #[cfg(feature = "message-director")] {
                use donet_message_director::MessageDirector;

                if want_message_director {
                    info!("Booting Message Director service.");

                    let handle = MessageDirector::start(daemon_config.clone(), None).await?;
                    service_handles.push(handle);
                }
            } else {
                if want_message_director {
                    feature_warn("Message Director");
                }
            }
        }
        cfg_if! {
            if #[cfg(feature = "state-server")] {
                if want_state_server {
                    info!("Booting State Server service.");
                    todo!("SS not yet implemented.")
                }
            } else {
                if want_state_server {
                    feature_warn("State Server");
                }
            }
        }
        cfg_if! {
            if #[cfg(feature = "database-server")] {
                if want_database_server {
                    info!("Booting Database Server service.");
                    todo!("DB not yet implemented.")
                }
            } else {
                if want_database_server {
                    feature_warn("Database Server");
                }
            }
        }
        cfg_if! {
            if #[cfg(feature = "dbss")] {
                if want_dbss {
                    info!("Booting DBSS service.");
                    todo!("DBSS not yet implemented.")
                }
            } else {
                if want_dbss {
                    feature_warn("DBSS");
                }
            }
        }
        cfg_if! {
            if #[cfg(feature = "event-logger")] {
                use donet_event_logger::EventLogger;

                if want_event_logger {
                    info!("Booting Event Logger service.");

                    let handle = EventLogger::start(daemon_config.clone(), None).await?;
                    service_handles.push(handle);
                }
            } else {
                if want_event_logger {
                    feature_warn("Event Logger");
                }
            }
        }
        // spawned services were given copies of these; drop originals.
        #[cfg(feature = "requires_dc")]
        drop(dc);
        drop(daemon_config);

        if service_handles.is_empty() {
            warn!("No services spawned, exiting program.")
        } else {
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
    set_future_return_type::<std::io::Result<()>, _>(&daemon_async_main);

    tokio_runtime.block_on(daemon_async_main)
}

cfg_if! {
    // In a production environment, multiple Docker containers can
    // be deployed, each with a specific build of Donet for it to
    // perform one service and only one service.
    //
    // However, all containers may share the same configuration file,
    // so every daemon will print feature warns for all the services
    // that it was not built with. Can be avoided by stripping config
    // files customized to every daemon, but that's inconvenient.
    //
    // If we know that this build of Donet is specifically being made
    // for a docker image, disable these feature warnings.
    if #[cfg(feature = "dockerized")] {
        #[allow(dead_code)]
        fn feature_warn(_: &'static str) {}
    } else {
        #[allow(dead_code)]
        fn feature_warn(name: &'static str) {
            warn!("This build of Donet has no {}; skipping.", name);
        }
    }
}

/// Performs the operation for the `-h` flag, or the `--help`
/// GNU-style long flag in the daemon binary.
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

/// Performs the operation for the `-v` flag, or the `--version`
/// GNU-style long flag in the daemon binary.
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
        Revision (Git SHA1): {}\n\
        Compiled on {}\n\
        Build options: {}\n\
        Feature Flags: {}\n\n\
        Donet is free software; you can redistribute it and/or modify\n\
        it under the terms of the GNU Affero General Public License,\n\
        as published by the Free Software Foundation, either version 3\n\
        of the License, or (at your option) any later version.\n\n\
        Donet is distributed in the hope that it will be useful,\n\
        but WITHOUT ANY WARRANTY; without even the implied warranty of\n\
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the\n\
        GNU Affero General Public License for more details.\n\n\
        You should have received a copy of the GNU Affero General Public\n\
        License along with Donet. If not, see <https://www.gnu.org/licenses/>.\n\n\
        The source code is publicly available at {}\n",
        logger::_ANSI_MAGENTA, logger::_ANSI_RESET,
        VERSION, bin_arch, bin_platform, bin_env, VCS_TAG,
        COMPILE_TIME, BUILD_OPTIONS, FEATURE_FLAGS, GIT_URL
    );
}

/// Performs the operation for the `-c` flag, or the `--validate-dc`
/// GNU-style long flag in the daemon binary.
#[cfg(feature = "requires_dc")]
fn validate_dc_files(conf: &DonetConfig, files: Vec<String>) -> std::io::Result<()> {
    use donet_core::dconfig::DCFileConfig;
    use donet_core::read_dc_files;
    use log::{error, info};
    use std::io::{Error, ErrorKind};

    // DC parser pipeline requires configuration; Build from TOML config.
    let dc_config: DCFileConfig = conf.clone().into();

    match read_dc_files(dc_config, files.to_owned()) {
        Ok(dc_file) => {
            let hash: u32 = dc_file.get_legacy_hash();
            let signed: i32 = hash as i32;
            let pretty: String = dc_file.get_pretty_hash();

            info!(
                "No issues found. Legacy file hash is {} (signed {}, hex {})",
                hash, signed, pretty
            );
            Ok(())
        }
        Err(err) => {
            error!("Failed to parse DC file: {:?}", err);

            Err(Error::new(ErrorKind::InvalidInput, "Failed to parse DC file."))
        }
    }
}
