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

//! Module of libdonet that contains the [`Context Free Grammar`] definition
//! of the DC file language, the process of generating the DC file
//! [`Abstract Syntax Tree`], and the process of converting the AST into the
//! final DC file class hierarchy structure that is used by the Donet daemon
//! at runtime to be able to interpret network messages that follow the
//! network contract defined in the DC file(s).
//!
//! [`Context Free Grammar`]: https://en.wikipedia.org/wiki/Context-free_grammar
//! [`Abstract Syntax Tree`]: https://en.wikipedia.org/wiki/Abstract_syntax_tree

pub(crate) mod ast;
mod error;
mod generate;
pub(crate) mod lexer;
pub(crate) mod parser;

use crate::dcfile::DCFile;
use crate::globals::ParseError;

/// Runs the entire DC parser pipeline. The input is a single string slice
/// that represents the raw DC file in UTF-8, and the output is the final
/// DC element tree data structure to be used by Donet.
#[inline]
pub(crate) fn dcparse_pipeline<'a>(input: String) -> Result<DCFile<'a>, ParseError> {
    let lexer: lexer::Lexer<'_> = lexer::Lexer::new(&input);
    let ast: ast::Root = parser::parse(lexer)?;

    let dc_file: DCFile = generate::generate_dcf_structure(ast);

    Ok(dc_file)
}
