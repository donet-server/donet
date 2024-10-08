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
pub(crate) mod lexer;
pub(crate) mod parser;
mod semantics;

use crate::dcfile::DCFile;
use crate::globals::ParseError;
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files::{self, SimpleFiles};
use codespan_reporting::term;
use multimap::MultiMap;
use term::termcolor::{ColorChoice, StandardStream};

/// Data stored in memory throughout the DC parser pipeline.
///
/// Sets up writer and codespan config for rendering diagnostics
/// to stderr & storing DC files that implement codespan's File trait.
struct PipelineData<'a> {
    _writer: StandardStream,
    _config: term::Config,
    pub files: SimpleFiles<&'a str, &'a str>,
    pub filename_to_id: MultiMap<&'a str, usize>,
    pub syntax_trees: Vec<ast::Root>,
}

impl<'a> Default for PipelineData<'a> {
    fn default() -> Self {
        Self {
            _writer: StandardStream::stderr(ColorChoice::Always),
            _config: term::Config::default(),
            files: SimpleFiles::new(),
            filename_to_id: MultiMap::default(),
            syntax_trees: vec![],
        }
    }
}

impl<'a> PipelineData<'a> {
    /// Thin wrapper for emitting a codespan diagnostic using `PipelineData` properties.
    pub fn emit_diagnostic(&mut self, diag: Diagnostic<usize>) -> Result<(), files::Error> {
        term::emit(&mut self._writer.lock(), &self._config, &self.files, &diag)
    }
}

/// Tuple that represents an input file for the DC parser.
/// The first item is the filename, the second item is the file content.
pub(crate) type InputFile = (String, String);

/// Runs the entire DC parser pipeline. The input is a single string slice
/// that represents the raw DC file in UTF-8, and the output is the final
/// DC element tree data structure to be used by Donet.
pub(crate) fn dcparse_pipeline<'a>(inputs: Vec<InputFile>) -> Result<DCFile<'a>, ParseError> {
    let mut pipeline_data: PipelineData<'_> = PipelineData::default();

    // Create codespan files for each DC file
    for input in &inputs {
        let file_id: usize = pipeline_data.files.add(&input.0, &input.1);

        pipeline_data.filename_to_id.insert(&input.0, file_id);
    }

    // Create an abstract syntax tree per DC file
    for input in &inputs {
        let lexer: lexer::Lexer<'_> = lexer::Lexer::new(&input.1);
        let ast: ast::Root = parser::parse(lexer)?;

        pipeline_data.syntax_trees.push(ast);
    }

    // Process all abstract syntax trees in semantic analyzer.
    Ok(semantics::semantic_analyzer(pipeline_data))
}
