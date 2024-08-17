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

//! Enum and Struct definitions that are used to build the DC File [`AST`].
//! Used by [`crate::parser::parser`].
//!
//! [`AST`]: https://en.wikipedia.org/wiki/Abstract_syntax_tree

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
    PythonImport(Vec<PythonImport>),
    KeywordType(KeywordDefinition),
    StructType(Struct),
    SwitchType(Switch),
    DClassType(DClass),
    TypedefType(DCTypeDefinition),
}

/// Paired with the `python_style_import` production in the Context Free Grammar.
#[derive(Debug, Clone)]
pub struct PythonImport {
    pub python_module: String,
    pub symbols: Vec<String>,
}

/// Paired with the `keyword_type` production in the Context Free Grammar.
#[derive(Debug)]
pub struct KeywordDefinition {
    pub identifier: String,
}

/// Paired with the `struct_type` production in the Context Free Grammar.
#[derive(Debug)]
pub struct Struct {
    pub identifier: String,
    pub fields: Vec<(Parameter, Vec<String>)>,
}

/// Paired with the `struct_fields` production in the Context Free Grammar.
pub type StructFields = Vec<(Parameter, Vec<String>)>;

/// Paired with the `switch_type` production in the Context Free Grammar.
#[derive(Debug)]
pub struct Switch {
    pub cases: Vec<Case>,
}

/// Paired with the `switch_case` production in the Context Free Grammar.
#[derive(Debug)]
pub struct Case {
    // `None` condition means this is a default case.
    pub condition: Option<TypeValue>,
    pub fields: Vec<(Parameter, Vec<String>)>,
}

/// Paired with the `distributed_class_type` production in the Context Free Grammar.
#[derive(Debug)]
pub struct DClass {
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
    pub identifier: String,
    pub keywords: Vec<String>,
    pub parameters: Vec<Parameter>,
}

/// Paired with the `molecular_field` production in the Context Free Grammar.
#[derive(Debug)]
pub struct MolecularField {
    pub identifier: String,
    pub atomic_field_identifiers: Vec<String>,
}

/// Paired with the `method_body` production in the Context Free Grammar.
pub type MethodBody = Vec<Parameter>;

/// Paired with the `method_as_field` production in the Context Free Grammar.
#[derive(Debug)]
pub struct MethodAsField {
    pub identifier: String,
    pub parameters: MethodBody,
}

/// Paired with the `parameter` production in the Context Free Grammar.
#[derive(Debug)]
pub struct Parameter {
    pub data_type: DCTypeEnum,
    pub identifier: String,
    pub default_value: Option<TypeValue>,
}

/// Paired with the `array_expansion` production in the Context Free Grammar.
pub type ArrayExpansion = (TypeValue, u32);

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
