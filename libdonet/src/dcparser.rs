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
use std::ops::Range;

pub mod ast {
    // In this module we store all the structures and enums
    // that make up the final generated abstract syntax tree.
    use super::{DCToken, Range, Span};
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
        KeywordType(IdentifierString),
        StructType(StructType),
        DistributedClassType(DistributedClassType),
        DCImport(DCImport),
        TypeDefinition(TypeDefinition),
    }

    #[derive(Debug, PartialEq)]
    pub struct StructType {
        pub span: Span,
        pub identifier: IdentifierString,
        pub parameters: Vec<Parameter>,
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
        pub keyword_list: Vec<IdentifierString>,
    }

    #[derive(Debug, PartialEq)]
    pub struct ParameterField {
        pub parameter: Parameter,
        pub keyword_list: Vec<IdentifierString>,
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
        pub identifier: Option<IdentifierString>,
        pub int_type: IdentifierString,
        pub int_range: Option<Range<i64>>,
        pub int_transform: Option<IntTransform>,
        pub int_constant: Option<i64>,
    }

    #[derive(Debug, PartialEq)]
    pub struct FloatParameter {
        pub identifier: Option<IdentifierString>,
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
        OperatorIntLiteral { operator: DCToken, int_literal: i64 },
        ParenthesizedIntTransform(Box<IntTransform>),
    }

    #[derive(Debug, PartialEq)]
    pub enum FloatTransform {
        OperatorFloatLiteral { operator: DCToken, float_literal: f64 },
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
        python_import[dci] => ast::TypeDecl {
            span: span!(),
            node: ast::TypeDecl_::DCImport(dci),
        },
        type_definition[td] => ast::TypeDecl {
            span: span!(),
            node: ast::TypeDecl_::TypeDefinition(td),
        },
    }

    // ----- Keyword Type ----- //

    keyword_type: String {
        KeywordType Identifier(id) Semicolon => id
    }

    // ----- Struct Type ----- //

    struct_type: ast::StructType {
        StructType Identifier(id) OpenBraces parameters[ps]
        CloseBraces Semicolon => ast::StructType {
            span: span!(),
            identifier: id,
            parameters: ps,
        }
    }

    // ----- Distributed Class Type ----- //

    distributed_class_type: ast::DistributedClassType {
        DClassType Identifier(id) OpenBraces field_declarations[fds]
        CloseBraces Semicolon => ast::DistributedClassType {
            span: span!(),
            identifier: id,
            field_declarations: fds,
        }
    }

    // ----- Type Definition Type ----- //

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

    // ----- Python-style Import ----- //

    // Donet does not make use of python-style import statements,
    // as this is a feature used by Donet clients and AI/UD processes.
    // We still have our production rules defined to avoid a parser panic.
    python_import: ast::DCImport {
        py_module[(m, ms)] dclass_import[(c, cs)] => ast::DCImport {
            span: span!(),
            module: m,
            module_views: ms,
            class: c,
            class_views: cs,
        },
    }

    // e.g. "from views ..."
    // e.g. "from game.views.Donut/AI ..."
    py_module: (Vec<String>, Vec<String>) {
        // See note @ `import_with_suffix`
        From import_with_suffix[(m, ms)] => (vec![m], ms)
    }

    // e.g. "import DistributedDonut/AI/OV"
    dclass_import: (String, Vec<String>) {
        Import import_with_suffix[(c, cs)] => (c, cs)
    }

    import_with_suffix: (String, Vec<String>) {
        // e.g. "from views/AI/OV import DistributedDonut/AI/OV"
        // e.g. "from my-views/AI/OV import DistributedDonut/AI/OV"
        // e.g. "from views import *"
        Identifier(i) nested_py_modules[_] view_suffixes[is] => (i, is),
        Module(i) nested_py_modules[_] view_suffixes[is] => (i, is),
        Star => ("*".to_string(), vec![]),
        // NOTE: As you've noticed, we ignore nested_py_modules' value.
        // This is because I've spent 3 hours trying to capture its value
        // into a vector without having mutable reference issues. Since
        // the Donet server won't ever need this information, I'm going
        // to stop trying. We only require the parser to be able to match it.
    }

    // Bundles module names in 'from' statements, e.g. "myviews.Donut".
    nested_py_modules: Vec<String> {
        => vec![],
        nested_py_modules[mut nm] module[m] => {
            nm.push(m);
            nm
        }
    }

    // NOTE: Module names may be lexed as identifiers or module tokens.
    module: String {
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

    // ----- Field Declaration ----- //

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

    // ----- Molecular Field ----- //

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

    // ----- Atomic Field ----- //

    atomic_fields: Vec<ast::AtomicField> {
        => vec![],
        atomic_fields[mut afs] Comma atomic_field[af] => {
            afs.push(af);
            afs
        }
    }

    atomic_field: ast::AtomicField {
        Identifier(id) OpenParenthesis parameters[ps]
        CloseParenthesis dc_keyword_list[kl] Semicolon => ast::AtomicField {
            identifier: id,
            parameters: ps,
            keyword_list: kl,
        }
    }

    // ----- Parameter Fields ----- //

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
        struct_parameter[p] dc_keyword_list[kl] => ast::ParameterField {
            parameter: p,
            keyword_list: kl,
        }
    }

    // ----- Parameters ----- //

    parameters: Vec<ast::Parameter> {
        => vec![],
        parameters[mut ps] parameter[p] => {
            ps.push(p);
            ps
        }
    }

    struct_parameter: ast::Parameter {
        parameter[p] Semicolon => p
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

    size_constraint: Option<i64> {
        => None,
        OpenParenthesis DecimalLiteral(sc) CloseParenthesis => Some(sc)
    }

    int_range: Option<Range<i64>> {
        => None,
        OpenParenthesis DecimalLiteral(a) Hyphen DecimalLiteral(b) CloseParenthesis => Some(a .. b)
    }

    float_range: Option<Range<f64>> {
        => None,
        OpenParenthesis FloatLiteral(a) Hyphen FloatLiteral(b) CloseParenthesis => Some(a .. b)
    }

    array_range: Option<Vec<Range<i64>>> {
        => None,
        OpenBrackets array_range_opts[aros] CloseBrackets => Some(aros)
    }

    array_range_opts: Vec<Range<i64>> {
        => vec![],
        array_range_opts[mut aros] Hyphen DecimalLiteral(aro) => {
            aros.push(aro);
            aros
        },
        array_range_opts[mut aros] DecimalLiteral(aro) => {
            aros.push(aro);
            aros
        }
    }

    // ----- Char Parameter ----- //

    char_param: ast::CharParameter {
    }

    // ----- Integer Parameter ----- //

    int_param: ast::IntParameter {
    }

    // ----- Float Parameter ----- //

    float_param: ast::FloatParameter {
    }

    // ----- String Parameter ----- //

    string_param: ast::StringParameter {
    }

    // ----- Blob Parameter ----- //

    // FIXME: plex needs to fix optional grammar. cannot move on with the parser.
    // FIXME: https://github.com/goffrie/plex/issues/20
    //opt_id: Option<String> {
    //    => None,
    //    Identifier(id) => Some(id)
    //}

    blob_param: ast::BlobParameter {
        BlobType size_constraint[sc] => ast::BlobParameter {
            identifier: None,
            string_literal: None,
            size_constraint: sc,
        },
    }

    // ----- Struct Parameter ----- //

    struct_param: ast::StructParameter {
        Identifier(struct_type) Identifier(struct_id) => ast::StructParameter {
            struct_type: struct_type,
            identifier: Some(struct_id),
        },
    }

    // ----- Array Parameter ----- //

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

    // ----- DC Keywords ----- //

    // Bundle up all dc_keyword productions into one vector.
    dc_keyword_list: Vec<String> {
        => vec![],
        dc_keyword_list[mut kl] dc_keyword[k] => {
            kl.push(k);
            kl
        }
    }

    // Wrap hardcoded DC keyword tokens into ast::KeywordType node.
    dc_keyword: String {
        dc_keyword_[k] => k
    }

    // We use hardcoded DC keyword tokens, since using a plain
    // identifier token causes a shift-reduce conflict in parsing.
    dc_keyword_: String {
        RAM => "ram".to_string(),
        REQUIRED => "required".to_string(),
        DB => "db".to_string(),
        AIRECV => "airecv".to_string(),
        OWNRECV => "ownrecv".to_string(),
        CLRECV => "clrecv".to_string(),
        BROADCAST => "broadcast".to_string(),
        OWNSEND => "ownsend".to_string(),
        CLSEND => "clsend".to_string(),
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
        let dc_file: &str = "from example-views import DistributedDonut\n\
                             from views import DistributedDonut/AI/OV\n\
                             from views/AI/OV import DistributedDonut/AI/OV\n\
                             from game.views.Donut/AI import DistributedDonut/AI\n";
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
                        module: vec!["example-views".to_string()],
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
                // from game.views.Donut/AI import DistributedDonut/AI
                ast::TypeDecl {
                    span: Span {
                        min: 131,
                        max: 182,
                        line: 4,
                    },
                    node: ast::TypeDecl_::DCImport(ast::DCImport {
                        span: Span {
                            min: 131,
                            max: 182,
                            line: 4,
                        },
                        // NOTE: We can't actually record more than one module name. It sucks,
                        // I know- but we won't use it anyways. I've spent 3 hours on this.
                        module: vec!["game".to_string()],
                        //module: vec!["game".to_string(), "views".to_string(), "Donut".to_string()],
                        module_views: vec!["AI".to_string()],
                        class: "DistributedDonut".to_string(),
                        class_views: vec!["AI".to_string()],
                    }),
                },
            ],
        };
        parse_for_ast_target(dc_file, target_ast);
    }

    #[test]
    fn distributed_class_production() {
        let dc_file: &str = "dclass DistributedDonut {\n
                             };\n";
        let target_ast: ast::DCFile = ast::DCFile {
            type_decl: vec![ast::TypeDecl {
                span: Span {
                    min: 0,
                    max: 58,
                    line: 1,
                },
                node: ast::TypeDecl_::DistributedClassType(ast::DistributedClassType {
                    span: Span {
                        min: 0,
                        max: 58,
                        line: 1,
                    },
                    identifier: "DistributedDonut".to_string(),
                    field_declarations: vec![],
                }),
            }],
        };
        parse_for_ast_target(dc_file, target_ast);
    }
}
