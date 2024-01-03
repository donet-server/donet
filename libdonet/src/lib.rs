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

/* Simple interface to our DC file parser; Handles reading
 * the DC files, creating the lexer and parser, and either
 * returns the DCFile object or a Parse/File error.
 */
pub fn read_dc_file(file_path: &String) -> DCReadResult {
    use crate::dclexer::Lexer;
    use crate::dcparser::parse;
    use std::fs::File;
    use std::io::Read;

    // Synchronous read of file to string, then lex and parse.
    let file_read: Result<File, std::io::Error> = File::open(file_path);

    if let Ok(mut dc_file) = file_read {
        let mut contents: String = String::new();
        let _ = dc_file.read_to_string(&mut contents);

        let lexer: Lexer<'_> = Lexer::new(&contents);
        let res: Result<dcfile::DCFile, globals::ParseError> = parse(lexer);

        if let Ok(res_ok) = res {
            Ok(res_ok)
        } else {
            Err(DCReadError::ParseError(res.unwrap_err()))
        }
    } else {
        Err(DCReadError::FileError(file_read.unwrap_err()))
    }
}
