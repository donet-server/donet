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

#![doc(html_logo_url = "https://raw.githubusercontent.com/donet-server/donet/master/logo/donet_logo_v3.png")]
//#![warn(missing_docs)]
#![deny(unused_extern_crates)]

pub mod globals;

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(feature = "datagram")] {
        pub mod byte_order;
        pub mod datagram;
    }
}
cfg_if! {
    if #[cfg(feature = "dcfile")] {
        pub mod dcarray;
        pub mod dcatomic;
        pub mod dcfield;
        pub mod dcfile;
        pub mod dckeyword;
        pub mod dclass;
        pub mod dclexer;
        pub mod dcmolecular;
        pub mod dcnumeric;
        pub mod dcparameter;
        pub mod dcparser;
        pub mod dcstruct;
        pub mod dctype;
        mod hashgen;
    }
}

/// Easy to use interface for the DC file parser. Handles reading
/// the DC files, instantiating the lexer and parser, and either
/// returns the DCFile object or a Parse/File error.
///
/// ## Example Usage
/// The following is an example of parsing a simple DC file string,
/// printing its DC hash in hexadecimal notation, and accessing
/// the elements of a defined Distributed Class:
/// ```rust
/// use libdonet::dcfile::DCFileInterface;
/// use libdonet::dclass::{DClass, DClassInterface};
/// use libdonet::globals::DCReadResult;
/// use libdonet::read_dc_files;
///
/// // All elements in a DC File object are thread-safe!
/// use std::sync::{Arc, Mutex, MutexGuard};
///
/// let dc_file: &str = "from game.ai import AnonymousContact/UD
///                      from game.ai import LoginManager/AI
///                      from game.world import DistributedWorld/AI
///                      from game.avatar import DistributedAvatar/AI/OV
///
///                      dclass AnonymousContact {
///                        login(string username, string password) clsend airecv;
///                      };
///
///                      dclass LoginManager {
///                        login(channel client, string username, string password) airecv;
///                      };
///
///                      dclass DistributedWorld {
///                        create_avatar(channel client) airecv;
///                      };
///
///                      dclass DistributedAvatar {
///                        set_xyzh(int16 x, int16 y, int16 z, int16 h) broadcast required;
///                        indicate_intent(int16 / 10, int16 / 10) ownsend airecv;
///                      };";
///
/// let dc_read: DCReadResult = read_dc_files(vec![dc_file.into()]);
///
/// if let Ok(mut dc_file) = dc_read {
///     println!("{}", dc_file.get_pretty_hash()); // Print the DC File Hash
///     
///     let avatar_class: Arc<Mutex<DClass>> = dc_file.get_dclass_by_id(3);
///     let mut locked_class: MutexGuard<'_, DClass> = avatar_class.lock().unwrap();
///
///     println!("{}", locked_class.get_name());
/// }
/// ```
///
/// The output of the program would be the following:
/// ```txt
/// 0x01a5Fb0c
/// DistributedAvatar
/// ```
/// <br><img src="https://c.tenor.com/myQHgyWQQ9sAAAAd/tenor.gif">
///
#[cfg(feature = "dcfile")]
pub fn read_dc_files(file_paths: Vec<String>) -> globals::DCReadResult {
    use crate::dclexer::Lexer;
    use crate::dcparser::parse;
    use std::fs::File;
    use std::io::Read;

    let mut file_results: Vec<Result<File, std::io::Error>> = vec![];
    let mut lexer_input: String = String::new();

    for file_path in &file_paths {
        file_results.push(File::open(file_path));
    }

    for io_result in file_results {
        if let Ok(mut dcf) = io_result {
            let res: std::io::Result<usize> = dcf.read_to_string(&mut lexer_input);
            if let Err(res_err) = res {
                // DC file content may not be in proper UTF-8 encoding.
                return Err(globals::DCReadError::FileError(res_err));
            }
        } else {
            // Failed to open one of the DC files. (most likely permission error)
            return Err(globals::DCReadError::FileError(io_result.unwrap_err()));
        }
    }

    let lexer: Lexer<'_> = Lexer::new(&lexer_input);
    let res: Result<dcfile::DCFile, globals::ParseError> = parse(lexer);

    if let Ok(res_ok) = res {
        Ok(res_ok)
    } else {
        Err(globals::DCReadError::ParseError(res.unwrap_err()))
    }
}
