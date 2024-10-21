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

//! Errors that can be returned by the DC parser pipeline.

use super::lexer::{DCToken, Span};
use super::pipeline::PipelineStage;
use codespan_diag::Label;
use codespan_diag::LabelStyle;
use codespan_reporting::diagnostic as codespan_diag;
use std::mem::discriminant;
use thiserror::Error;

/// Convert `self` type to an error code.
/// Used with DC parser pipeline error types.
pub trait ToErrorCode
where
    Self: std::error::Error,
{
    fn error_code(&self) -> &str;
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum DCReadError {
    #[error("parser error")]
    ParseError,
    #[error("semantics error")]
    SemanticError,
    FileError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum PipelineError {
    ParseError(#[from] ParseError),
    SemanticError(#[from] SemanticError),
}

impl ToErrorCode for PipelineError {
    fn error_code(&self) -> &str {
        // Get the error code from the underlying error type.
        match self {
            Self::ParseError(err) => err.error_code(),
            Self::SemanticError(err) => err.error_code(),
        }
    }
}

/// Error type for the semantic analysis stage of the pipeline.
#[derive(Debug, Error)]
pub enum SemanticError {
    // generic
    #[error("`{0}` is already defined")]
    AlreadyDefined(String),
    #[error("`{0}` is not defined")]
    NotDefined(String),

    // dc file
    #[error("multiple inheritance is not allowed")]
    MultipleInheritanceDisabled,
    #[error("maximum number of dclasses declared")]
    DClassOverflow,
    #[error("maximum number of fields declared")]
    FieldOverflow,

    // python-style imports
    #[error("redundant view suffix `{0}`")]
    RedundantViewSuffix(String),

    // keywords
    #[error("redundant keyword `{0}`")]
    RedundantKeyword(String),

    // structs
    #[error("dc keywords are not allowed in struct fields")]
    KeywordsInStructField,

    // switches
    #[error("duplicate case value")]
    RedundantCase,
    #[error("default case is already defined")]
    RedundantDefault,

    // molecular fields
    #[error("`mismatched dc keywords in molecule between `{atom1}` and `{atom2}`")]
    MismatchedKeywords { atom1: String, atom2: String },
    #[error("`{0}` is not an atomic field")]
    ExpectedAtomic(String),

    // numeric ranges
    #[error("invalid range for type")]
    InvalidRange,
    #[error("overlapping range")]
    OverlappingRange,
    #[error("value out of range")]
    ValueOutOfRange,

    // transforms
    #[error("invalid divisor")]
    InvalidDivisor,
    #[error("invalid modulus")]
    InvalidModulus,

    // default
    #[error("invalid default value for type")]
    InvalidDefault,

    // struct type
    #[error("`{0}` is not a struct")]
    ExpectedStruct(String),
}

impl ToErrorCode for SemanticError {
    fn error_code(&self) -> &str {
        match self {
            // generic
            Self::AlreadyDefined(_) => "E0200",
            Self::NotDefined(_) => "E0201",
            // dc file
            Self::MultipleInheritanceDisabled => "E0210",
            Self::DClassOverflow => "E0211",
            Self::FieldOverflow => "E0212",
            // python-style imports
            Self::RedundantViewSuffix(_) => "E0220",
            // keywords
            Self::RedundantKeyword(_) => "E0230",
            // structs
            Self::KeywordsInStructField => "E0240",
            // switches
            Self::RedundantCase => "E0250",
            Self::RedundantDefault => "E0251",
            // molecular fields
            Self::MismatchedKeywords { atom1: _, atom2: _ } => "E0260",
            Self::ExpectedAtomic(_) => "E0261",
            // numeric ranges
            Self::InvalidRange => "E0270",
            Self::OverlappingRange => "E0271",
            Self::ValueOutOfRange => "E0272",
            // transforms
            Self::InvalidDivisor => "E0280",
            Self::InvalidModulus => "E0281",
            // default
            Self::InvalidDefault => "E0290",
            // struct type
            Self::ExpectedStruct(_) => "E0300",
        }
    }
}

/// Error type for the parser stage of the pipeline.
/// Currently, it only stores one error type, which is
/// the standard error type for the parser. Due to a
/// plex limitation, I am unable to propagate custom
/// errors from grammar productions. See Donet issue #19.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("syntax error; {1}, found `{0:?}`")]
    Error(DCToken, String),
}

impl ToErrorCode for ParseError {
    fn error_code(&self) -> &str {
        match self {
            Self::Error(_, _) => "E0100",
        }
    }
}

pub(crate) struct Diagnostic {
    span: Span,
    stage: PipelineStage,
    file_id: usize,
    severity: codespan_diag::Severity,
    error: PipelineError,
}

impl Diagnostic {
    pub fn error(
        span: Span,
        pipeline_stage: PipelineStage,
        file_id: usize,
        err: impl Into<PipelineError>,
    ) -> Self {
        Self {
            span,
            stage: pipeline_stage,
            file_id,
            severity: codespan_diag::Severity::Error,
            error: err.into(),
        }
    }
}

/// Allows converting our Diagnostic type into a codespan Diagnostic type.
impl Into<codespan_diag::Diagnostic<usize>> for Diagnostic {
    fn into(self) -> codespan_diag::Diagnostic<usize> {
        codespan_diag::Diagnostic::new(self.severity)
            .with_message(self.error.to_string())
            .with_code(self.error.error_code())
            .with_labels(vec![Label::new(
                LabelStyle::Primary,
                self.file_id,
                self.span.min..self.span.max,
            )])
            .with_notes({
                // If error type is from the Plex parser stage, emit the following notice.
                if discriminant(&self.stage) == discriminant(&PipelineStage::Parser) {
                    vec![
                        "Syntax errors are limited. Please see issue #19.".into(),
                        "https://gitlab.com/donet-server/donet/-/issues/19".into(),
                    ]
                } else {
                    vec![]
                }
            })
    }
}
