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

#![warn(unused_extern_crates)]

pub mod byte_order;
pub mod datagram;
pub mod dcfield;
pub mod dcfile;
pub mod dckeyword;
pub mod dclass;
pub mod dclexer;
pub mod dcparser;
pub mod dcstruct;
pub mod dctype;
pub mod globals;
mod hashgen;

#[derive(Debug)]
pub enum DCReadError {
    ParseError(globals::ParseError),
    FileError(std::io::Error),
}
pub type DCReadResult = Result<dcfile::DCFile, DCReadError>;

/* Easy to use interface for the DC file parser; Handles reading
 * the DC files, instantiating the lexer and parser, and either
 * returns the DCFile object or a Parse/File error.
 */
pub fn read_dc_files(file_paths: Vec<String>) -> DCReadResult {
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
            if res.is_err() {
                // DC file content may not be in proper UTF-8 encoding.
                return Err(DCReadError::FileError(res.unwrap_err()));
            }
        } else {
            // Failed to open one of the DC files. (most likely permission error)
            return Err(DCReadError::FileError(io_result.unwrap_err()));
        }
    }

    let lexer: Lexer<'_> = Lexer::new(&lexer_input);
    let res: Result<dcfile::DCFile, globals::ParseError> = parse(lexer);

    if let Ok(res_ok) = res {
        Ok(res_ok)
    } else {
        Err(DCReadError::ParseError(res.unwrap_err()))
    }
}
