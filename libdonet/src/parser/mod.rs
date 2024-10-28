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
pub mod error;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod pipeline;
mod semantics;

use crate::dcfile::DCFile;
use anyhow::Result;
use error::DCReadError;
use pipeline::PipelineData;

/// Tuple that represents an input file for the DC parser.
/// The first item is the filename, the second item is the file content.
pub(crate) type InputFile = (String, String);

/// Runs the entire DC parser pipeline. The input is an array of strings
/// that represent the input DC files in UTF-8, and the output is the final
/// DC element tree data structure to be used by Donet.
pub(crate) fn dcparse_pipeline<'a>(inputs: Vec<InputFile>) -> Result<DCFile<'a>, DCReadError> {
    let mut pipeline_data: PipelineData<'_> = PipelineData::default();

    // Create codespan files for each DC file
    for input in &inputs {
        let _: usize = pipeline_data.files.add(&input.0, &input.1);
    }

    // Create an abstract syntax tree per DC file
    for input in &inputs {
        let lexer: lexer::Lexer<'_> = lexer::Lexer::new(&input.1);

        let ast: ast::Root = match parser::parse(lexer) {
            // See issue #19 for why LALR parser cannot return custom errors.
            Err(err) => {
                if let Some(parser_err) = err.clone().0 {
                    // Extract parser error details
                    let span: lexer::Span = parser_err.1;
                    let token: lexer::DCToken = parser_err.0;
                    let msg: String = err.1.to_owned();

                    let diag: error::Diagnostic = error::Diagnostic::error(
                        span,
                        &mut pipeline_data,
                        error::PipelineError::Parser(error::ParseError::Error(token, msg)),
                    );

                    pipeline_data
                        .emit_diagnostic(diag.into())
                        .expect("Failed to emit diagnostic.");
                }

                return Err(DCReadError::Syntax);
            }
            Ok(ast) => ast,
        };

        pipeline_data.syntax_trees.push(ast);
        pipeline_data.next_file();
    }

    // Process all abstract syntax trees in semantic analyzer.
    semantics::semantic_analyzer(&mut pipeline_data)
}
