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

//! Defines the [`PipelineStage`] structure, which manages
//! data stored in memory throughout the DC parser pipeline.

use super::ast;
use crate::dconfig::*;
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::diagnostic::Severity;
use codespan_reporting::files::{self, SimpleFiles};
use codespan_reporting::term;
use term::termcolor::{ColorChoice, StandardStream};

/// Used by the [`PipelineData`] structure to keep track
/// of the current pipeline stage to properly store its state.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) enum PipelineStage {
    #[default]
    Parser, // includes lexical and syntax analysis
    SemanticAnalyzer,
}

impl PipelineStage {
    pub(crate) fn next(&self) -> Self {
        match self {
            PipelineStage::Parser => PipelineStage::SemanticAnalyzer,
            PipelineStage::SemanticAnalyzer => panic!("No next stage in pipeline."),
        }
    }
}

/// Data stored in memory throughout the DC parser pipeline.
///
/// Sets up writer and codespan config for rendering diagnostics
/// to stderr & storing DC files that implement codespan's File trait.
pub(crate) struct PipelineData<'a> {
    dc_parser_config: DCFileConfig,
    stage: PipelineStage,
    _writer: StandardStream,
    _config: term::Config,
    diagnostics_enabled: bool,
    errors_emitted: usize,
    pub files: SimpleFiles<&'a str, &'a str>,
    current_file: usize,
    pub syntax_trees: Vec<ast::Root>,
}

/// If the [`PipelineData`] structure is dropped, this means the
/// pipeline finished, either with success or error.
///
/// Upon drop, emit a final diagnostic with the finish status of the pipeline.
impl<'a> Drop for PipelineData<'a> {
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

impl<'a> From<DCFileConfig> for PipelineData<'a> {
    fn from(value: DCFileConfig) -> Self {
        Self {
            dc_parser_config: value,
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
        }
    }
}

impl<'a> DCFileConfigAccessor for PipelineData<'a> {
    fn get_dc_config(&self) -> &DCFileConfig {
        &self.dc_parser_config
    }
}

impl<'a> PipelineData<'a> {
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
mod unit_testing {
    use super::*;

    #[test]
    fn next_stage_state() {
        let mut pipeline: PipelineData = DCFileConfig::default().into();

        pipeline.next_file(); // increase file counter to 1

        pipeline.next_stage(); // should reset state for next stage

        assert_eq!(pipeline.stage, PipelineStage::SemanticAnalyzer);
        assert_eq!(pipeline.current_file, 0);
    }
}
