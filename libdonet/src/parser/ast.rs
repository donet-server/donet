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

use crate::dcfile::*;
use crate::dckeyword; // Avoid wildcard import due to conflict with DCToken variant.
use crate::dclass; // Same reason as comment above.
use crate::dcstruct;
use crate::dctype::*;

/// Paired with the `type_decl` production in the Context Free Grammar.
pub enum TypeDeclaration {
    PythonImport(Vec<DCImport>),
    KeywordType(dckeyword::DCKeyword),
    StructType(dcstruct::DCStruct),
    SwitchType(Option<u8>),
    DClassType(dclass::DClass),
    TypedefType(DCTypeDefinition),
}

/// Paired with the `type_value` production in the Context Free Grammar.
pub enum TypeValue {
    I64(i64),
    Char(char),
    String(String),
    ArrayValue(Vec<(TypeValue, u32)>),
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

/// Paired with the `parameter` production in the Context Free Grammar.
#[derive(Debug, Default)]
pub struct Parameter {
    pub base_type: DCTypeDefinition,
    pub identifier: String,
    pub type_alias: String,
    pub default_value: Option<Vec<u8>>,
}
