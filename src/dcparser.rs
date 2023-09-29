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

// The following suppress linting warnings, which are okay to ignore
// as they go off in the parser grammar definitions, which we are writing
// just as the plex crate readme says we should, so everything is okay.
#![allow(clippy::type_complexity, clippy::redundant_field_names, clippy::ptr_arg)]
#![allow(clippy::redundant_closure_call, clippy::enum_variant_names)]

use crate::dclexer::DCToken::*;
use crate::dclexer::{DCToken, Span};
use plex::parser;

mod ast {
    // In this module we store all the structures and enums
    // that make up the final generated abstract syntax tree.
    use super::{DCToken, Span};
    use std::ops::Range;
    pub type IdentifierString = String; // type alias

    #[derive(Debug, PartialEq)]
    pub struct DCFile {
        pub type_decl: Vec<TypeDecl>,
    }

    #[derive(Debug, PartialEq)]
    pub struct TypeDecl {
        pub span: Span,
        pub node: TypeDecl_,
    }

    #[derive(Debug, PartialEq)]
    pub enum TypeDecl_ {
        KeywordType(KeywordType),
        StructType(StructType),
        DistributedClassType(DistributedClassType),
        DCImport(DCImport),
        TypeDefinition(TypeDefinition),
    }

    #[derive(Debug, PartialEq)]
    pub struct KeywordType {
        pub span: Span,
        pub identifier: IdentifierString,
    }

    #[derive(Debug, PartialEq)]
    pub struct StructType {
        pub span: Span,
        pub identifier: IdentifierString,
        pub parameters: Vec<ParameterField>,
    }

    #[derive(Debug, PartialEq)]
    pub struct DistributedClassType {
        pub span: Span,
        pub identifier: IdentifierString,
        pub field_declarations: Vec<FieldDecl>,
    }

    #[derive(Debug, PartialEq)]
    pub struct DCImport {
        pub span: Span,
        pub module: Vec<String>, // python filename, or module(s)
        pub module_views: Vec<String>,
        pub class: IdentifierString,
        pub class_views: Vec<String>, // AI, UD, OV ...
    }

    #[derive(Debug, PartialEq)]
    pub struct TypeDefinition {
        pub span: Span,
        pub dc_type: DataType,
        pub alias: IdentifierString,
    }

    #[derive(Debug, PartialEq)]
    pub struct FieldDecl {
        pub span: Span,
        pub node: FieldDecl_,
    }

    #[derive(Debug, PartialEq)]
    pub enum FieldDecl_ {
        MolecularField(MolecularField),
        AtomicField(AtomicField),
        ParameterField(ParameterField),
    }

    #[derive(Debug, PartialEq)]
    pub struct MolecularField {
        pub identifier: IdentifierString,
        pub field_type: FieldType,
    }

    #[derive(Debug, PartialEq)]
    pub enum FieldType {
        Atomic(AtomicField),
        Parameter(ParameterField),
    }

    #[derive(Debug, PartialEq)]
    pub struct AtomicField {
        pub identifier: IdentifierString,
        pub parameters: Vec<Parameter>,
        pub keyword_list: Vec<KeywordType>,
    }

    #[derive(Debug, PartialEq)]
    pub struct ParameterField {
        pub parameter: Parameter,
        pub keyword_list: Vec<KeywordType>,
    }

    #[derive(Debug, PartialEq)]
    pub enum Parameter {
        Char(CharParameter),
        Int(IntParameter),
        Float(FloatParameter),
        String(StringParameter),
        Blob(BlobParameter),
        Struct(StructParameter),
        Array(ArrayParameter),
    }

    #[derive(Debug, PartialEq)]
    pub struct CharParameter {
        pub identifier: Option<IdentifierString>,
        pub char_literal: Option<char>,
    }

    #[derive(Debug, PartialEq)]
    pub struct IntParameter {
        pub identifier: IdentifierString,
        pub int_type: Option<IdentifierString>,
        pub int_range: Option<Range<i64>>,
        pub int_transform: Option<IntTransform>,
        pub int_constant: Option<i64>,
    }

    #[derive(Debug, PartialEq)]
    pub struct FloatParameter {
        pub identifier: IdentifierString,
        pub float_type: Option<IdentifierString>,
        pub float_range: Option<Range<f64>>,
        pub float_transform: Option<FloatTransform>,
        pub float_constant: Option<f64>,
    }

    // NOTE: StringParameter and BlobParameter hold the same information,
    // as by specification they are both 'SizedParameter' types. We use
    // separate structs for strings and blobs so Donet knows exactly
    // what data type they are and make optimizations.

    #[derive(Debug, PartialEq)]
    pub struct StringParameter {
        pub identifier: Option<IdentifierString>,
        pub string_literal: Option<String>,
        pub size_constraint: Option<i64>,
    }

    #[derive(Debug, PartialEq)]
    pub struct BlobParameter {
        pub identifier: Option<IdentifierString>,
        pub string_literal: Option<String>,
        pub size_constraint: Option<i64>,
    }

    #[derive(Debug, PartialEq)]
    pub struct StructParameter {
        pub struct_type: IdentifierString,
        pub identifier: Option<IdentifierString>,
    }

    #[derive(Debug, PartialEq)]
    pub struct ArrayParameter {
        pub data_type: DataType,
        pub identifier: Option<IdentifierString>,
        pub array_range: Range<i64>,
    }

    #[derive(Debug, PartialEq)]
    pub struct DataType {
        pub base_type: BaseType,
        pub type_identifier: Option<String>, // used for IntType (unsigned/signed + bits)
    }

    #[rustfmt::skip]
    #[derive(Debug, PartialEq)]
    pub enum BaseType {
        CharType, IntType, FloatType,
        StringType, BlobType, StructType,
    }

    #[derive(Debug, PartialEq)]
    pub enum IntTransform {
        OperatorIntLiteral { operator: DCToken, int_literal: i32 },
        ParenthesizedIntTransform(Box<IntTransform>),
    }

    #[derive(Debug, PartialEq)]
    pub enum FloatTransform {
        OperatorFloatLiteral { operator: DCToken, float_literal: f32 },
        ParenthesizedFloatTransform(Box<FloatTransform>),
    }
}

// Plex macro to start defining our grammar & productions
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
        type_declarations[tds] => ast::DCFile { type_decl: tds },
    }

    // Collect all our Type Declarations into a vector for the DCFile.
    type_declarations: Vec<ast::TypeDecl> {
        => vec![],
        type_declarations[mut td_vec] type_decl[next_td] => {
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
        dc_import[dci] => ast::TypeDecl {
            span: span!(),
            node: ast::TypeDecl_::DCImport(dci),
        },
        type_definition[td] => ast::TypeDecl {
            span: span!(),
            node: ast::TypeDecl_::TypeDefinition(td),
        },
    }

    keyword_type: ast::KeywordType {
        KeywordType Identifier(id) Semicolon => ast::KeywordType {
            span: span!(),
            identifier: id,
        }
    }

    struct_type: ast::StructType {
        StructType Identifier(id) OpenBraces parameter_fields[ps]
        CloseBraces Semicolon => ast::StructType {
            span: span!(),
            identifier: id,
            parameters: ps,
        }
    }

    distributed_class_type: ast::DistributedClassType {
        DClassType Identifier(id) OpenBraces field_declarations[fds]
        CloseBraces Semicolon => ast::DistributedClassType {
            span: span!(),
            identifier: id,
            field_declarations: fds,
        }
    }

    // Donet does not make use of python-style import statements,
    // as this is a feature used by Donet clients and AI/UD processes.
    // We still have our production rules defined to avoid a parser panic.
    dc_import: ast::DCImport {
        py_mod[(m, ms)] dc_imp[(c, cs)] => ast::DCImport {
            span: span!(),
            module: vec![m],
            module_views: ms,
            class: c,
            class_views: cs,
        },
        nested_py_mod[(nm, ms)] dc_imp[(c, cs)] => ast::DCImport {
            span: span!(),
            module: nm,
            module_views: ms,
            class: c,
            class_views: cs,
        },
    }

    type_definition: ast::TypeDefinition {
        TypeDefinition CharType Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: ast::DataType {
                base_type: ast::BaseType::CharType,
                type_identifier: None,
            },
            alias: alias,
        },
        TypeDefinition IntType(int_id) Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: ast::DataType {
                base_type: ast::BaseType::IntType,
                type_identifier: Some(int_id), // unsigned/signed + bits
            },
            alias: alias,
        },
        TypeDefinition FloatType Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: ast::DataType {
                base_type: ast::BaseType::FloatType,
                type_identifier: None,
            },
            alias: alias,
        },
        TypeDefinition StringType Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: ast::DataType {
                base_type: ast::BaseType::StringType,
                type_identifier: None,
            },
            alias: alias,
        },
        TypeDefinition BlobType Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: ast::DataType {
                base_type: ast::BaseType::BlobType,
                type_identifier: None,
            },
            alias: alias,
        },
    }

    // e.g. "from views ..."
    py_mod: (String, Vec<String>) {
        From import_with_suffix[(m, ms)] => (m, ms),
    }

    // e.g. "from game.views.Donut/AI ..."
    nested_py_mod: (Vec<String>, Vec<String>) {
        From nested_modules_with_suffix[(nm, ms)] => (nm, ms),
    }

    // e.g. "import DistributedDonut/AI/OV"
    dc_imp: (String, Vec<String>) {
        Import import_with_suffix[(c, cs)] => (c, cs)
    }

    import_with_suffix: (String, Vec<String>) {
        // e.g. "from views/AI/OV import DistributedDonut/AI/OV"
        // e.g. "from my_views/AI/OV import DistributedDonut/AI/OV"
        Identifier(i) view_suffixes[is] => (i, is),
        Module(i) view_suffixes[is] => (i, is),
    }

    nested_modules_with_suffix: (Vec<String>, Vec<String>) {
        nested_py_modules[nm] view_suffixes[ms] => (nm, ms),
    }

    // Bundles module names in 'from' statements, e.g. "myviews.Donut".
    nested_py_modules: Vec<String> {
        => vec![],
        nested_py_modules[mut nm] py_module[m] => {
            nm.push(m);
            nm
        }
    }

    // NOTE: Module names may be lexed as identifiers or module tokens.
    py_module: String {
        Period Identifier(m) => m,
        Period Module(m) => m,
    }

    // Bundle up all views of a dclass/module to be imported, into a vector
    // of strings, each corresponding to a view suffix. (AI, UD, OV..)
    view_suffixes: Vec<String> {
        => vec![],
        view_suffixes[mut vs] view_suffix[s] => {
            vs.push(s);
            vs
        }
    }

    // Matches '/AI' '/OV' from, example, "DistributedDonut/AI/OV"
    view_suffix: String {
        ForwardSlash Identifier(s) => s
    }

    field_declarations: Vec<ast::FieldDecl> {
        => vec![],
        field_declarations[mut fds] field_declaration[fd] => {
            fds.push(fd);
            fds
        }
    }

    field_declaration: ast::FieldDecl {
        molecular_field[mf] => ast::FieldDecl {
            span: span!(),
            node: ast::FieldDecl_::MolecularField(mf),
        },
        atomic_field[af] => ast::FieldDecl {
            span: span!(),
            node: ast::FieldDecl_::AtomicField(af),
        },
        parameter_field[pf] => ast::FieldDecl {
            span: span!(),
            node: ast::FieldDecl_::ParameterField(pf),
        },
    }

    molecular_field: ast::MolecularField {
        Identifier(id) Colon atomic_field[af] => ast::MolecularField {
            identifier: id,
            field_type: ast::FieldType::Atomic(af),
        },
        // maybe separate atomic/parameter fields match to other production rule??
        //
        //Identifier(id) Colon parameter_fields[pfs] => ast::MolecularField {
        //    identifier: id,
        //    field_type: ast::FieldType::Parameter(pfs),
        //}
    }

    atomic_fields: Vec<ast::AtomicField> {
        => vec![],
        atomic_fields[mut afs] Comma atomic_field[af] => {
            afs.push(af);
            afs
        }
    }

    atomic_field: ast::AtomicField {
        // FIXME: solve shift-reduce conflict
        //Identifier(id) OpenParenthesis parameters[ps]
        //CloseParenthesis Semicolon => ast::AtomicField {
        //    identifier: id,
        //    parameters: ps,
        //    keyword_list: vec![],
        //},
        Identifier(id) OpenParenthesis parameters[ps]
        CloseParenthesis dc_keyword_list[kl] Semicolon => ast::AtomicField {
            identifier: id,
            parameters: ps,
            keyword_list: kl,
        }
    }

    // The 'parameter_fields' production is made up of the current parameters
    // plus a new parameter following, and returns a vector of all
    // parameters parsed so far. This bundles them all up for other productions.
    parameter_fields: Vec<ast::ParameterField> {
        => vec![],
        parameter_fields[mut ps] Comma parameter_field[p] => {
            ps.push(p);
            ps
        }
    }

    parameter_field: ast::ParameterField {
        parameter[p] dc_keyword_list[kl] => ast::ParameterField {
            parameter: p,
            keyword_list: kl,
        }
    }

    parameters: Vec<ast::Parameter> {
        => vec![],
        parameters[mut ps] parameter[p] => {
            ps.push(p);
            ps
        }
    }

    parameter: ast::Parameter {
        char_param[cp] => ast::Parameter::Char(cp),
        int_param[ip] => ast::Parameter::Int(ip),
        float_param[fp] => ast::Parameter::Float(fp),
        string_param[sp] => ast::Parameter::String(sp),
        blob_param[bp] => ast::Parameter::Blob(bp),
        struct_param[sp] => ast::Parameter::Struct(sp),
        array_param[ap] => ast::Parameter::Array(ap),
    }

    char_param: ast::CharParameter {
        // FIXME: solve shift-reduce conflict
        //CharType => ast::CharParameter {
        //    identifier: None,
        //    char_literal: None,
        //},
        CharType Identifier(id) => ast::CharParameter {
            identifier: Some(id),
            char_literal: None,
        },
        CharType Identifier(id) Equals CharacterLiteral(c) => ast::CharParameter {
            identifier: Some(id),
            char_literal: Some(c),
        },
    }

    int_param: ast::IntParameter {
        Colon Colon => ast::IntParameter {
            identifier: String::from("id"),
            int_type: None,
            int_range: None,
            int_transform: None,
            int_constant: None,
        }
    }

    int_range: Range<i64> {
        OpenParenthesis DecimalLiteral(a) Hyphen DecimalLiteral(b) CloseParenthesis => a .. b
    }

    float_param: ast::FloatParameter {
        Hyphen Hyphen Hyphen Hyphen Hyphen => ast::FloatParameter {
            identifier: String::from("id"),
            float_type: None,
            float_range: None,
            float_transform: None,
            float_constant: None,
        }
    }

    float_range: (f64, f64) {
        OpenParenthesis FloatLiteral(a) Hyphen FloatLiteral(b) CloseParenthesis => (a, b)
    }

    string_param: ast::StringParameter {
        // FIXME: Stops at this match for any string type.
        //StringType => ast::StringParameter {
        //    identifier: None,
        //    string_literal: None,
        //    size_constraint: None,
        //},
        // FIXME: solve shift-reduce conflict
        //sized_string[sc] => ast::StringParameter {
        //    identifier: None,
        //    string_literal: None,
        //    size_constraint: sc,
        //},
        named_sized_string[(id, sc)] => ast::StringParameter {
            identifier: Some(id),
            string_literal: None,
            size_constraint: Some(sc),
        },
        named_sized_string[(id, sc)] Equals StringLiteral(sl) => ast::StringParameter {
            identifier: Some(id),
            string_literal: Some(sl),
            size_constraint: Some(sc),
        }
    }

    named_sized_string: (String, i64) {
        sized_string[sc] Identifier(id) => (id, sc)
    }

    sized_string: i64 {
        StringType size_constraint[sc] => sc
    }

    blob_param: ast::BlobParameter {
        // FIXME: Stops at this match for any blob type.
        //BlobType => ast::BlobParameter {
        //    identifier: None,
        //    string_literal: None,
        //    size_constraint: None,
        //},
        // FIXME: solve shift-reduce conflict
        //sized_blob[sc] => ast::BlobParameter {
        //    identifier: None,
        //    string_literal: None,
        //    size_constraint: sc,
        //},
        named_sized_blob[(id, sc)] => ast::BlobParameter {
            identifier: Some(id),
            string_literal: None,
            size_constraint: Some(sc),
        },
        named_sized_blob[(id, sc)] Equals StringLiteral(sl) => ast::BlobParameter {
            identifier: Some(id),
            string_literal: Some(sl),
            size_constraint: Some(sc),
        }
    }

    named_sized_blob: (String, i64) {
        sized_blob[sc] Identifier(id) => (id, sc)
    }

    sized_blob: i64 {
        BlobType size_constraint[sc] => sc
    }

    size_constraint: i64 {
        OpenParenthesis DecimalLiteral(s) CloseParenthesis => s
    }

    struct_param: ast::StructParameter {
        // FIXME: solve shift-reduce conflict
        //Identifier(struct_type) => ast::StructParameter {
        //    struct_type: struct_type,
        //    identifier: None,
        //},
        Identifier(struct_type) Identifier(struct_id) => ast::StructParameter {
            struct_type: struct_type,
            identifier: Some(struct_id),
        },
    }

    // FIXME: Implement me
    array_param: ast::ArrayParameter {
        CloseBraces CloseBraces => ast::ArrayParameter {
            data_type: ast::DataType {
                base_type: ast::BaseType::CharType,
                type_identifier: None,
            },
            identifier: None,
            array_range: 1 .. 3,
        }
    }

    // Bundle up all dc_keyword productions into one vector.
    dc_keyword_list: Vec<ast::KeywordType> {
        => vec![],
        dc_keyword_list[mut kl] dc_keyword[k] => {
            kl.push(k);
            kl
        }
    }

    // Wrap hardcoded DC keyword tokens into ast::KeywordType node.
    dc_keyword: ast::KeywordType {
        dc_keyword_[k] => ast::KeywordType {
            span: span!(),
            identifier: k,
        }
    }

    // We use hardcoded DC keyword tokens, since using a plain
    // identifier token causes a shift-reduce conflict in parsing.
    dc_keyword_: String {
        RAM => String::from("ram"),
        REQUIRED => String::from("required"),
        DB => String::from("db"),
        AIRECV => String::from("airecv"),
        OWNRECV => String::from("ownrecv"),
        CLRECV => String::from("clrecv"),
        BROADCAST => String::from("broadcast"),
        OWNSEND => String::from("ownsend"),
        CLSEND => String::from("clsend"),
    }

}

// This is the interface to our parser; Provides an iterator.
pub fn parse<I: Iterator<Item = (DCToken, Span)>>(
    i: I,
) -> Result<ast::DCFile, (Option<(DCToken, Span)>, &'static str)> {
    parse_(i)
}

#[cfg(test)]
mod unit_testing {
    use super::{ast, parse, Span};
    use crate::dclexer::Lexer;

    // Utility function for verifying the parser output to the target AST.
    fn parse_for_ast_target(input: &str, target_ast: ast::DCFile) {
        let lexer = Lexer::new(input).inspect(|tok| eprintln!("token: {:?}", tok));
        let dc_file_ast: ast::DCFile = parse(lexer).unwrap();

        eprintln!("{:#?}", dc_file_ast); // Pretty print output AST
        assert_eq!(dc_file_ast, target_ast);
    }

    #[test]
    fn type_definition_production() {
        let dc_file: &str = "typedef char test;\n";
        let target_ast: ast::DCFile = ast::DCFile {
            type_decl: vec![ast::TypeDecl {
                span: Span {
                    min: 0,
                    max: 18,
                    line: 1,
                },
                node: ast::TypeDecl_::TypeDefinition(ast::TypeDefinition {
                    span: Span {
                        min: 0,
                        max: 18,
                        line: 1,
                    },
                    dc_type: ast::DataType {
                        base_type: ast::BaseType::CharType,
                        type_identifier: None,
                    },
                    alias: "test".to_string(),
                }),
            }],
        };
        parse_for_ast_target(dc_file, target_ast);
    }

    #[test]
    fn python_style_imports() {
        let dc_file: &str = "from example_views import DistributedDonut\n\
                             from views import DistributedDonut/AI/OV\n\
                             from views/AI/OV import DistributedDonut/AI/OV\n";
        let target_ast: ast::DCFile = ast::DCFile {
            type_decl: vec![
                // from example_views import DistributedDonut
                ast::TypeDecl {
                    span: Span {
                        min: 0,
                        max: 42,
                        line: 1,
                    },
                    node: ast::TypeDecl_::DCImport(ast::DCImport {
                        span: Span {
                            min: 0,
                            max: 42,
                            line: 1,
                        },
                        module: vec!["example_views".to_string()],
                        module_views: vec![],
                        class: "DistributedDonut".to_string(),
                        class_views: vec![],
                    }),
                },
                // from views import DistributedDonut/AI/OV
                ast::TypeDecl {
                    span: Span {
                        min: 43,
                        max: 83,
                        line: 2,
                    },
                    node: ast::TypeDecl_::DCImport(ast::DCImport {
                        span: Span {
                            min: 43,
                            max: 83,
                            line: 2,
                        },
                        module: vec!["views".to_string()],
                        module_views: vec![],
                        class: "DistributedDonut".to_string(),
                        class_views: vec!["AI".to_string(), "OV".to_string()],
                    }),
                },
                // from views/AI/OV import DistributedDonut/AI/OV
                ast::TypeDecl {
                    span: Span {
                        min: 84,
                        max: 130,
                        line: 3,
                    },
                    node: ast::TypeDecl_::DCImport(ast::DCImport {
                        span: Span {
                            min: 84,
                            max: 130,
                            line: 3,
                        },
                        module: vec!["views".to_string()],
                        module_views: vec!["AI".to_string(), "OV".to_string()],
                        class: "DistributedDonut".to_string(),
                        class_views: vec!["AI".to_string(), "OV".to_string()],
                    }),
                },
            ],
        };
        parse_for_ast_target(dc_file, target_ast);
    }

    //#[test]
    //fn distributed_class_production() {
    //    let dc_file: &str = "dclass DistributedDonut {\n\
    //                            set_foo(string(10) bar = \"aa\") ram;\n
    //                            set_bar(blob(10) foo);\n
    //                         };\n";
    //    let target_ast: ast::DCFile = ast::DCFile {
    //        type_decl: vec![],
    //    };
    //    //parse_for_ast_target(dc_file, target_ast);
    //}
}
