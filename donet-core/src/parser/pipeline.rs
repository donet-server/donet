/*
    This file is part of Donet.

    Copyright © 2024-2025 Max Rodriguez <me@maxrdz.com>

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

//! Defines the [`PipelineStage`] structure, which manages
//! data stored in memory throughout the DC parser pipeline.

use super::ast;
use super::error::Diagnostic as DCDiagnostic;
use super::error::SemanticError;
use super::lexer::Span;
use crate::globals;
use anyhow::{anyhow, Result};
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::diagnostic::Severity;
use codespan_reporting::files::{self, SimpleFiles};
use codespan_reporting::term;
use multimap::MultiMap;
use term::termcolor::{ColorChoice, StandardStream};

/// Used by the [`PipelineData`] structure to keep track
/// of the current pipeline stage to properly store its state.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) enum PipelineStage {
    #[default]
    Parser, // includes lexical and syntax analysis
    SemanticAnalyzer,
    Generation,
}

impl PipelineStage {
    pub(crate) fn next(&self) -> Self {
        match self {
            PipelineStage::Parser => PipelineStage::SemanticAnalyzer,
            PipelineStage::SemanticAnalyzer => PipelineStage::Generation,
            PipelineStage::Generation => panic!("No next stage in pipeline."),
        }
    }
}

#[derive(PartialEq)]
pub enum TopLevelSymbol {
    TypeDef,
    KeywordDef,
    Struct,
    DClass,
}

/// Data structure used to keep track of declarations'
/// symbols (their identifiers) for usage during
/// semantic analysis.
pub type SymbolMap = MultiMap<String, TopLevelSymbol>;

/// Globals that the semantic analyzer uses while
/// parsing the abstract syntax tree.
#[derive(Default)]
pub struct DCData {
    symbol_map: SymbolMap,
    next_dclass_id: globals::DClassId,
    next_field_id: globals::FieldId,
}

impl DCData {
    /// Inserts new key/value pair to the private symbol map.
    pub fn register_symbol(&mut self, identifier: String, symbol_type: TopLevelSymbol) {
        self.symbol_map.insert(identifier, symbol_type);
    }

    /// Check is a given symbol exists in our symbol map (a.k.a
    /// it is a registered type declaration in the DC file.)
    pub fn symbol_exists(&self, identifier: &String, symbol_type: TopLevelSymbol) -> bool {
        self.symbol_map
            .iter()
            .find(|&x| x == (&identifier, &symbol_type))
            .is_some()
    }

    /// Gets the next dclass ID based on the current allocated IDs.
    ///
    /// If an error is returned, this DC file has run out of dclass
    /// IDs to assign. This function will emit the error diagnostic.
    ///
    pub fn get_next_dclass_id(
        &mut self,
        pipeline: &mut PipelineData,
        dclass: ast::DClass, // current dclass ref for diagnostic span
    ) -> Result<globals::DClassId> {
        let next_id: globals::DClassId = self.next_dclass_id;

        if next_id == globals::DClassId::MAX {
            // We have reached the maximum number of dclass declarations.
            let diag: DCDiagnostic =
                DCDiagnostic::error(dclass.span, pipeline, SemanticError::DClassOverflow);

            pipeline
                .emit_diagnostic(diag.into())
                .expect("Failed to emit diagnostic.");

            return Err(anyhow!("Ran out of 16-bit DClass IDs!"));
        }

        self.next_dclass_id += 1; // increment
        Ok(next_id)
    }

    /// Gets the next field ID based on the current allocated IDs.
    ///
    /// If an error is returned, this DC file has run out of field
    /// IDs to assign. This function will emit the error diagnostic.
    ///
    pub fn get_next_field_id(
        &mut self,
        pipeline: &mut PipelineData,
        field_span: Span,
    ) -> Result<globals::FieldId> {
        let next_id: globals::FieldId = self.next_field_id;

        if next_id == globals::DClassId::MAX {
            // We have reached the maximum number of dclass declarations.
            let diag: DCDiagnostic = DCDiagnostic::error(field_span, pipeline, SemanticError::DClassOverflow);

            pipeline
                .emit_diagnostic(diag.into())
                .expect("Failed to emit diagnostic.");

            return Err(anyhow!("Ran out of 16-bit Field IDs!"));
        }

        self.next_field_id += 1; // increment
        Ok(next_id)
    }
}

/// Data stored in memory throughout the DC parser pipeline.
///
/// Sets up writer and codespan config for rendering diagnostics
/// to stderr & storing DC files that implement codespan's File trait.
pub(crate) struct PipelineData<'a> {
    stage: PipelineStage,
    _writer: StandardStream,
    _config: term::Config,
    diagnostics_enabled: bool,
    errors_emitted: usize,
    pub files: SimpleFiles<&'a str, &'a str>,
    current_file: usize,
    pub syntax_trees: Vec<ast::Root>,
    pub dc_data: DCData,
}

/// If the [`PipelineData`] structure is dropped, this means the
/// pipeline finished, either with success or error.
///
/// Upon drop, emit a final diagnostic with the finish status of the pipeline.
impl Drop for PipelineData<'_> {
    fn drop(&mut self) {
        if self.errors_emitted > 0 {
            let diag = Diagnostic::error().with_message(format!(
                "Failed to read DC files due to {} previous errors.",
                self.errors_emitted
            ));

            self.emit_diagnostic(diag).expect("Failed to emit diagnostic.");
        }
    }
}

impl Default for PipelineData<'_> {
    fn default() -> Self {
        Self {
            stage: PipelineStage::default(),
            _writer: StandardStream::stderr(ColorChoice::Always),
            _config: term::Config::default(),
            diagnostics_enabled: {
                // Disable diagnostics in unit tests
                cfg_if! {
                    if #[cfg(test)] {
                        false
                    } else {
                        true
                    }
                }
            },
            errors_emitted: 0,
            files: SimpleFiles::new(),
            current_file: 0,
            syntax_trees: vec![],
            dc_data: DCData::default(),
        }
    }
}

impl PipelineData<'_> {
    /// Thin wrapper for emitting a codespan diagnostic using `PipelineData` properties.
    pub(crate) fn emit_diagnostic(&mut self, diag: Diagnostic<usize>) -> Result<(), files::Error> {
        if diag.severity == Severity::Error {
            self.errors_emitted += 1;
        }
        if !self.diagnostics_enabled {
            return Ok(());
        }
        term::emit(&mut self._writer.lock(), &self._config, &self.files, &diag)
    }

    #[inline(always)]
    pub(crate) fn current_stage(&self) -> PipelineStage {
        self.stage.clone()
    }

    pub(crate) fn next_stage(&mut self) {
        self.stage = self.stage.next();
        self.current_file = 0;
    }

    #[inline(always)]
    pub(crate) fn current_file(&self) -> usize {
        self.current_file
    }

    pub(crate) fn next_file(&mut self) {
        self.current_file += 1
    }

    #[inline(always)]
    pub(crate) fn failing(&self) -> bool {
        self.errors_emitted > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_stage_state() {
        let mut pipeline: PipelineData = PipelineData::default();

        assert_eq!(pipeline.stage, PipelineStage::Parser);

        pipeline.next_file(); // increase file counter to 1

        pipeline.next_stage(); // should reset state for next stage

        assert_eq!(pipeline.stage, PipelineStage::SemanticAnalyzer);
        assert_eq!(pipeline.current_file, 0);

        pipeline.next_stage();
        assert_eq!(pipeline.stage, PipelineStage::Generation);
    }
}
