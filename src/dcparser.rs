// DONET SOFTWARE
// Copyright (c) 2023, Donet Authors.
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

use crate::dclexer::DCToken::*;
use crate::dclexer::{DCToken, Span};
use plex::parser;

mod ast {
    // In this module we store all the structures and enums
    // that make up the final generated abstract syntax tree.
    use super::{DCToken, Span};
    use std::ops::Range;
    pub type IdentifierString = String; // type alias

    #[derive(Debug)]
    pub struct DCFile {
        pub type_decl: Vec<TypeDecl>,
    }

    #[derive(Debug)]
    pub struct TypeDecl {
        pub span: Span,
        pub node: TypeDecl_,
    }

    #[derive(Debug)]
    pub enum TypeDecl_ {
        KeywordType(KeywordType),
        StructType(StructType),
        DistributedClassType(DistributedClassType),
    }

    #[derive(Debug)]
    pub struct KeywordType {
        pub span: Span,
        pub node: KeywordType_,
    }

    #[derive(Debug)]
    pub enum KeywordType_ {
        KeywordType(IdentifierString),
        KeywordList(Vec<IdentifierString>),
    }

    #[derive(Debug)]
    pub struct StructType {
        pub span: Span,
        pub identifier: IdentifierString,
        pub parameters: Vec<Parameter>,
    }

    #[derive(Debug)]
    pub struct DistributedClassType {
        pub span: Span,
        pub identifier: IdentifierString,
        pub field_declarations: Vec<FieldDecl>,
    }

    #[derive(Debug)]
    pub struct FieldDecl {
        pub span: Span,
        pub node: FieldDecl_,
    }

    #[derive(Debug)]
    pub enum FieldDecl_ {
        MolecularField(MolecularField),
        AtomicField(AtomicField),
        ParameterField(ParameterField),
    }

    #[derive(Debug)]
    pub struct MolecularField {
        pub identifier: IdentifierString,
        pub field_type: FieldType,
    }

    #[derive(Debug)]
    pub enum FieldType {
        Atomic(AtomicField),
        Parameter(ParameterField),
    }

    #[derive(Debug)]
    pub struct AtomicField {
        pub identifier: IdentifierString,
        pub parameters: Vec<Parameter>,
        pub keyword_list: Option<KeywordType>,
    }

    #[derive(Debug)]
    pub struct ParameterField {
        pub parameter: Parameter,
        pub keyword_list: Option<KeywordType>,
    }

    #[derive(Debug)]
    pub enum Parameter {
        Char(CharParameter),
        Int(IntParameter),
        Float(FloatParameter),
        Sized(SizedParameter),
        Struct(StructParameter),
        Array(ArrayParameter),
    }

    #[derive(Debug)]
    pub struct CharParameter {
        pub char_type: Option<IdentifierString>,
        pub char_literal: Option<char>,
    }

    #[derive(Debug)]
    pub struct IntParameter {
        pub identifier: Option<IdentifierString>,
        pub int_type: Option<IdentifierString>,
        pub int_range: Option<Range<i64>>,
        pub int_transform: Option<IntTransform>,
        pub int_constant: Option<i64>,
    }

    #[derive(Debug)]
    pub struct FloatParameter {
        pub identifier: Option<IdentifierString>,
        pub float_type: Option<IdentifierString>,
        pub float_range: Option<Range<f64>>,
        pub float_transform: Option<FloatTransform>,
        pub float_constant: Option<f64>,
    }

    #[derive(Debug)]
    pub struct SizedParameter {
        pub sized_type: Option<IdentifierString>,
        pub size_constraint: Option<i64>,
        pub identifier: Option<IdentifierString>,
        pub string_literal: Option<String>,
    }

    #[derive(Debug)]
    pub struct StructParameter {
        pub identifier1: IdentifierString,
        pub identifier2: Option<IdentifierString>,
    }

    #[derive(Debug)]
    pub struct ArrayParameter {
        pub data_type: DataType,
        pub identifier: Option<IdentifierString>,
        pub array_range: Range<i64>,
    }

    #[derive(Debug)]
    pub enum DataType {
        BaseType(BaseType),
        Identifier(IdentifierString),
    }

    #[rustfmt::skip]
    #[derive(Debug)]
    pub enum BaseType {
        CharType, IntType, FloatType,
        StringType, BlobType, StructType,
    }

    #[derive(Debug)]
    pub enum IntTransform {
        OperatorIntLiteral { operator: DCToken, int_literal: i32 },
        ParenthesizedIntTransform(Box<IntTransform>),
    }

    #[derive(Debug)]
    pub enum FloatTransform {
        OperatorFloatLiteral { operator: DCToken, float_literal: f32 },
        ParenthesizedFloatTransform(Box<FloatTransform>),
    }
}

// Plex macro to start defining our grammar
parser! {
    fn parse_(DCToken, Span);

    // Instruct parser how to combine two spans
    (a, b) {
        Span {
            min: a.min,
            max: b.max,
            line: a.line, // only keep a's line number
        }
    }

    // DC File (root production of the grammar)
    dc_file: ast::DCFile {
        type_declarations[s] => ast::DCFile { type_decl: s }
    }

    // Collect all our Type Declarations into a vector for the DCFile.
    type_declarations: Vec<ast::TypeDecl> {
        => vec![],
        type_declarations[mut td_vec] type_decl[next_td] Semicolon => {
            td_vec.push(next_td);
            td_vec
        }
    }

    type_decl: ast::TypeDecl {
        keyword_type[k] => ast::TypeDecl {
            span: span!(),
            node: ast::TypeDecl_::KeywordType(k),
        },
        struct_type[s] => ast::TypeDecl {
            span: span!(),
            node: ast::TypeDecl_::StructType(s),
        },
        distributed_class_type[dc] => ast::TypeDecl {
            span: span!(),
            node: ast::TypeDecl_::DistributedClassType(dc),
        },
    }

    keyword_type: ast::KeywordType {
        KeywordType Identifier(id) Semicolon => ast::KeywordType {
            span: span!(),
            node: ast::KeywordType_::KeywordType(id),
        }
    }

    struct_type: ast::StructType {

    }

    distributed_class_type: ast::DistributedClassType {

    }

}

// This is the interface to our parser; Provides an iterator.
pub fn parse<I: Iterator<Item = (DCToken, Span)>>(
    i: I,
) -> Result<ast::DCFile, (Option<(DCToken, Span)>, &'static str)> {
    parse_(i)
}
