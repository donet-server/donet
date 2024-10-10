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

//! Enum and Struct definitions that are used to build the DC File [`AST`].
//! Used by [`crate::parser::parser`].
//!
//! [`AST`]: https://en.wikipedia.org/wiki/Abstract_syntax_tree

use super::lexer::{DCToken, Span};
use crate::dctype::*;

/// Paired with the `type_declarations` production in the Context Free Grammar.
#[derive(Debug)]
pub struct Root {
    pub type_declarations: Vec<TypeDeclaration>,
}

/// Paired with the `type_decl` production in the Context Free Grammar.
#[derive(Debug)]
pub enum TypeDeclaration {
    // A single Python-style DC Import line can translate to
    // multiple [`PythonImport`] structures per symbol imported.
    PythonImport(PythonImport),
    KeywordType(KeywordDefinition),
    StructType(Struct),
    DClassType(DClass),
    TypedefType(DCTypeDefinition),
}

/// Paired with the `python_style_import` production in the Context Free Grammar.
#[derive(Debug, Clone)]
pub struct PythonImport {
    pub span: Span,
    pub imports: Vec<PyModuleImport>,
}

#[derive(Debug, Clone)]
pub struct PyModuleImport {
    pub python_module: String,
    pub symbols: Vec<String>,
}

/// Paired with the `keyword_type` production in the Context Free Grammar.
#[derive(Debug)]
pub struct KeywordDefinition {
    pub span: Span,
    pub identifier: String,
}

/// Paired with the `struct_type` production in the Context Free Grammar.
#[derive(Debug)]
pub struct Struct {
    pub span: Span,
    pub identifier: String,
    pub fields: Vec<(Parameter, Vec<String>)>,
}

/// Paired with the `struct_fields` production in the Context Free Grammar.
pub type StructFields = Vec<(Parameter, Vec<String>)>;

/// Paired with the `switch_type` production in the Context Free Grammar.
#[derive(Debug)]
pub struct Switch {
    pub span: Span,
    pub cases: Vec<Case>,
}

/// Paired with the `switch_case` production in the Context Free Grammar.
#[derive(Debug)]
pub struct Case {
    pub span: Span,
    // `None` condition means this is a default case.
    pub condition: Option<TypeValue>,
    pub fields: Vec<(Parameter, Vec<String>)>,
}

/// Paired with the `distributed_class_type` production in the Context Free Grammar.
#[derive(Debug)]
pub struct DClass {
    pub span: Span,
    pub identifier: String,
    pub parents: Vec<String>,
    pub fields: ClassFields,
}

/// Paired with the `optional_class_fields` production in the Context Free Grammar.
pub type ClassFields = Vec<AtomicOrMolecular>;

/// Paired with the `class_field` production in the Context Free Grammar.
#[derive(Debug)]
pub enum AtomicOrMolecular {
    Atomic(AtomicField),
    Molecular(MolecularField),
}

/// The Atomic Field variant of the [`AtomicOrMolecular`] enum.
#[derive(Debug)]
pub struct AtomicField {
    pub span: Span,
    pub identifier: String,
    pub keywords: Vec<String>,
    pub parameters: MethodBody,
}

/// Paired with the `molecular_field` production in the Context Free Grammar.
#[derive(Debug)]
pub struct MolecularField {
    pub span: Span,
    pub identifier: String,
    pub atomic_field_identifiers: Vec<String>,
}

/// Paired with the `method_body` production in the Context Free Grammar.
pub type MethodBody = Vec<Parameter>;

/// Paired with the `parameter_values` production in the Context Free Grammar.
pub type ParameterValues = Vec<TypeValue>;

/// Paired with the `method_as_field` production in the Context Free Grammar.
#[derive(Debug)]
pub struct MethodAsField {
    pub span: Span,
    pub identifier: String,
    pub parameters: MethodBody,
}

/// Paired with the `parameter` production in the Context Free Grammar.
#[derive(Debug)]
pub struct Parameter {
    pub span: Span,
    pub data_type: DCTypeEnum,
    pub identifier: String,
    pub default_value: Option<TypeValue>,
}

/// Paired with the `array_expansion` production in the Context Free Grammar.
pub type ArrayExpansion = (TypeValue, u32);

/// Paired with the `type_or_sized_value` production in the Context Free Grammar.
#[derive(Debug)]
pub enum TypeOrSizedValue {
    TypeValue(TypeValue),
    SizedValue(DCToken),
}

/// Paired with the `type_value` production in the Context Free Grammar.
#[derive(Debug)]
pub enum TypeValue {
    I64(i64),
    Char(char),
    String(String),
    ArrayValue(Vec<ArrayExpansion>),
}

/// Paired with the `char_or_u16` production in the Context Free Grammar.
#[derive(Clone, Copy)]
pub enum CharOrU16 {
    Char(char),
    U16(u16),
}

/// Paired with the `char_or_number` production in the Context Free Grammar.
#[derive(Clone, Copy)]
pub enum CharOrNumber {
    Char(char),
    I64(i64),
    F64(f64),
}

pub struct DataType {
    pub span: Span,
    pub token: DCToken,
    pub dctype: DCTypeEnum,
}

impl DataType {
    pub fn from_token(value: DCToken, span: Span) -> Self {
        Self {
            span,
            token: value.clone(),
            dctype: match value {
                DCToken::Float32T => DCTypeEnum::TFloat32,
                DCToken::Float64T => DCTypeEnum::TFloat64,
                DCToken::Int8T => DCTypeEnum::TInt8,
                DCToken::Int16T => DCTypeEnum::TInt16,
                DCToken::Int32T => DCTypeEnum::TInt32,
                DCToken::Int64T => DCTypeEnum::TInt64,
                DCToken::UInt8T => DCTypeEnum::TUInt8,
                DCToken::UInt16T => DCTypeEnum::TUInt16,
                DCToken::UInt32T => DCTypeEnum::TUInt32,
                DCToken::UInt64T => DCTypeEnum::TUInt64,
                DCToken::Int8ArrayT => DCTypeEnum::TArray,
                DCToken::Int16ArrayT => DCTypeEnum::TArray,
                DCToken::Int32ArrayT => DCTypeEnum::TArray,
                DCToken::UInt8ArrayT => DCTypeEnum::TArray,
                DCToken::UInt16ArrayT => DCTypeEnum::TArray,
                DCToken::UInt32ArrayT => DCTypeEnum::TArray,
                DCToken::UInt32UInt8ArrayT => DCTypeEnum::TArray,
                _ => panic!("DC token matches no production in CFG."),
            },
        }
    }
}
