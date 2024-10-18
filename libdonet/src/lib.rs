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

//! <img src="https://gitlab.com/donet-server/donet/-/raw/master/logo/donet_banner.png?ref_type=heads" height=10%>
//!
//! You can return to the Donet manual at [`docs.donet-server.org`].
//!
//! # libdonet
//! Provides the necessary utilities and definitions for using the Donet networking protocol.
//!
//! These utilities include a lexer, parser, and high-level representation of the parsed DC
//! file, as well as creating datagrams, iterating through datagrams, and the definition of
//! every message type in the Donet protocol.
//!
//! ### Getting Started
//! The recommended way to get started is to enable all features.
//! Do this by enabling the `full` feature flag:
//! ```toml
//! libdonet = { version = "0.1.0", features = ["full"] }
//! ```
//!
//! ### Feature Flags
//! The crate provides a set of feature flags to reduce the amount of compiled code.
//! It is possible to just enable certain features over others.
//! Below is a list of the available feature flags.
//!
//! - **`full`**: Enables all feature flags available for libdonet.
//! - **`datagram`**: Includes Datagram / Datagram Iterator source for writing network packets.
//! - **`dcfile`**: Includes the DC file lexer, parser, and DC element structures.
//!
//! [`docs.donet-server.org`]: https://docs.donet-server.org/

#![doc(
    html_logo_url = "https://gitlab.com/donet-server/donet/-/raw/master/logo/donet_logo_v3.png?ref_type=heads"
)]
#![allow(clippy::module_inception)]
//#![warn(missing_docs)]
#![deny(unused_extern_crates)]

pub mod globals;

#[macro_use]
extern crate cfg_if;

#[cfg(feature = "datagram")]
pub mod datagram;

cfg_if! {
    if #[cfg(feature = "dcfile")] {
        mod parser;
        pub mod dcarray;
        pub mod dcatomic;
        pub mod dcfield;
        pub mod dcfile;
        pub mod dckeyword;
        pub mod dclass;
        pub mod dcmolecular;
        pub mod dcnumeric;
        pub mod dcparameter;
        pub mod dcstruct;
        pub mod dctype;
        mod hashgen;

        use anyhow::Result;
        use dcfile::DCFile;
        use parser::error::DCReadError;
    }
}

/// Returns false if a [`log`] logger is not initialized.
///
/// [`log`]: https://docs.rs/log/latest/log/
///
fn logger_initialized() -> bool {
    use log::Level::*;

    let levels: &[log::Level] = &[Error, Warn, Info, Debug, Trace];

    for level in levels {
        if log::log_enabled!(*level) {
            return true;
        }
    }
    false
}

/// Creates a [`pretty_env_logger`] logger if no [`log`]
/// logger is found to be initialized in this process.
///
/// [`pretty_env_logger`]: https://docs.rs/pretty_env_logger/latest/pretty_env_logger/
/// [`log`]: https://docs.rs/log/latest/log/
///
fn init_logger() {
    if logger_initialized() {
        return;
    }
    pretty_env_logger::init();
}

/// Easy to use interface for the DC file parser. Handles reading
/// the DC files, instantiating the DC parsing pipeline, and either
/// returns the DCFile object or a Parse/File error.
#[cfg(feature = "dcfile")]
pub fn read_dc_files<'a>(file_paths: Vec<String>) -> Result<DCFile<'a>, DCReadError> {
    use log::info;
    use parser::InputFile;
    use std::fs::File;
    use std::io::{Error, ErrorKind, Read};
    use std::path::Path;

    init_logger();
    info!("DC read of {:?}", file_paths);

    let mut filenames: Vec<String> = vec![];
    let mut file_results: Vec<Result<File, std::io::Error>> = vec![];
    let mut pipeline_input: Vec<parser::InputFile> = vec![];

    assert!(!file_paths.is_empty(), "No DC files given!");

    for file_path in &file_paths {
        // Get filename from given path
        match Path::new(file_path).file_name() {
            Some(filename_osstr) => {
                // Convert OsStr to String and store filename
                filenames.push(filename_osstr.to_string_lossy().into_owned());
            }
            None => {
                // std::path::Path.file_name() **only** returns `None`
                // if the path terminates in '..'.
                let filename_err: Error = Error::new(
                    ErrorKind::InvalidInput,
                    "Failed to get filename from path because\
                    path terminates in '..'.",
                );
                return Err(DCReadError::FileError(filename_err));
            }
        }

        // Open file using path and store result
        file_results.push(File::open(file_path));
    }

    for (index, io_result) in file_results.into_iter().enumerate() {
        if let Ok(mut dcf) = io_result {
            // Prepare `InputFile` tuple for the pipeline function.
            let filename: String = filenames.get(index).unwrap().to_owned();
            let mut in_file: InputFile = (filename, String::default());

            let res: std::io::Result<usize> = dcf.read_to_string(&mut in_file.1);

            if let Err(res_err) = res {
                // DC file content may not be in proper UTF-8 encoding.
                return Err(DCReadError::FileError(res_err));
            }
            pipeline_input.push(in_file);
        } else {
            // Failed to open one of the DC files. (most likely permission error)
            return Err(DCReadError::FileError(io_result.unwrap_err()));
        }
    }

    parser::dcparse_pipeline(pipeline_input)
}

/// Front end to the libdonet DC parser pipeline.
///
/// ## Example Usage
/// The following is an example of parsing a simple DC file string,
/// printing its DC hash in hexadecimal notation, and accessing
/// the elements of a defined Distributed Class:
/// ```rust
/// use libdonet::dcfile::DCFile;
/// use libdonet::dclass::DClass;
/// use libdonet::globals::DCReadError;
/// use libdonet::read_dc;
///
/// let dc_file = "
///
/// from game.ai import AnonymousContact/UD
/// from game.ai import LoginManager/AI
/// from game.world import DistributedWorld/AI
/// from game.avatar import DistributedAvatar/AI/OV
///
/// typedef uint32 doId;
/// typedef uint32 zoneId;
/// typedef uint64 channel;
///
/// dclass AnonymousContact {
///   login(string username, string password) clsend airecv;
/// };
///
/// dclass LoginManager {
///   login(channel client, string username, string password) airecv;
/// };
///
/// dclass DistributedWorld {
///   create_avatar(channel client) airecv;
/// };
///
/// dclass DistributedAvatar {
///   set_xyzh(int16 x, int16 y, int16 z, int16 h) broadcast required;
///   indicate_intent(int16 / 10, int16 / 10) ownsend airecv;
/// };
///
/// ";
///
/// let dc_read: Result<DCFile, DCReadError> = read_dc(dc_file.to_owned());
///
/// if let Ok(dc_file) = dc_read {
///     // Print the DC File's 32-bit hash in hexadecimal format.
///     println!("{}", dc_file.get_pretty_hash());
///
///     // TODO: Retrieve the `DistributedAvatar` dclass by ID.
///     //let class: &DClass = dc_file.get_dclass_by_id(3);
///
///     // TODO: Print the identifier of the dclass.
///     //println!("{}", class.get_name());
/// }
/// ```
///
/// The output of the program would be the following:
/// ```txt
/// 0x9c737148
/// DistributedAvatar
/// ```
/// <br><img src="https://c.tenor.com/myQHgyWQQ9sAAAAd/tenor.gif">
///
#[cfg(feature = "dcfile")]
pub fn read_dc<'a>(input: String) -> Result<DCFile<'a>, DCReadError> {
    let dcparse_input: Vec<parser::InputFile> = vec![("input.dc".to_string(), input)];

    parser::dcparse_pipeline(dcparse_input)
}
