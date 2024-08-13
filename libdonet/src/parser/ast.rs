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

use super::lexer::DCToken;
use crate::dcfile;
use crate::dckeyword; // Avoid wildcard import due to conflict with DCToken variant.
use crate::dclass; // Same reason as comment above.
use crate::dcstruct;
use crate::dctype::*;

/// Paired with the `type_declarations` production in the Context Free Grammar.
pub type Root = Vec<TypeDeclaration>;

/// Paired with the `type_decl` production in the Context Free Grammar.
#[derive(Debug)]
pub enum TypeDeclaration {
    // A single Python-style DC Import line can translate to
    // multiple [`PythonImport`] structures per symbol imported.
    PythonImport(Vec<PythonImport>),
    KeywordType(dckeyword::DCKeyword),
    StructType(dcstruct::DCStruct),
    SwitchType(Option<u8>), // TODO
    DClassType(dclass::DClass),
    TypedefType(DCTypeDefinition),
}

#[derive(Debug, Clone)]
pub struct PythonImport {
    pub python_module: String,
    pub symbols: Vec<String>,
}

#[derive(Debug)]
pub struct Keyword {
    pub identifier: String,
    pub alias_type: DCTypeDefinition,
}

#[derive(Debug)]
pub struct Struct {
    pub identifier: String,
    pub fields: Vec<(Parameter, Vec<String>)>,
}

#[derive(Debug)]
pub struct Switch {
    pub cases: Vec<Case>,
}

#[derive(Debug)]
pub struct Case {
    // `None` condition means this is a default case.
    pub condition: Option<TypeValue>,
    pub fields: Vec<(Parameter, Vec<String>)>,
}

#[derive(Debug)]
pub struct DClass {
    pub identifier: String,
    pub parents: Option<Vec<String>>,
    pub fields: Vec<AtomicOrMolecular>,
}

#[derive(Debug)]
pub enum AtomicOrMolecular {
    AtomicField(AtomicField),
    MolecularField(MolecularField),
}

#[derive(Debug)]
pub struct AtomicField {
    pub identifier: String,
    pub keywords: Vec<String>,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub struct MolecularField {
    pub identifier: String,
    pub atomic_field_identifiers: Vec<String>,
}

/// Paired with the `method` production in the Context Free Grammar.
pub type Method = Vec<Parameter>;

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
