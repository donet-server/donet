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
