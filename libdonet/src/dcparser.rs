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

    #[derive(Debug, PartialEq, Clone)]
    pub struct DCFile {
        pub type_decl: Vec<TypeDecl>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct TypeDecl {
        pub span: Span,
        pub node: TypeDecl_,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum TypeDecl_ {
        KeywordType(IdentifierString),
        StructType(StructType),
        DistributedClassType(DistributedClassType),
        DCImport(DCImport),
        TypeDefinition(TypeDefinition),
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct StructType {
        pub span: Span,
        pub identifier: IdentifierString,
        pub parameters: Vec<Parameter>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct DistributedClassType {
        pub span: Span,
        pub identifier: IdentifierString,
        pub parent_classes: Option<Vec<IdentifierString>>,
        pub field_declarations: Vec<FieldDecl>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct DCImport {
        pub span: Span,
        pub module: Vec<String>, // python filename, or module(s)
        pub module_views: Option<Vec<String>>,
        pub class: IdentifierString,
        pub class_views: Option<Vec<String>>, // /AI, /UD, /OV ...
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct TypeDefinition {
        pub span: Span,
        pub dc_type: DCToken,
        pub alias: IdentifierString,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct FieldDecl {
        pub span: Span,
        pub node: FieldDecl_,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum FieldDecl_ {
        MolecularField(MolecularField),
        AtomicField(AtomicField),
        ParameterField(ParameterField),
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct MolecularField {
        pub identifier: IdentifierString,
        pub fields: Vec<FieldType>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum FieldType {
        Atomic(AtomicField),
        Parameter(ParameterField),
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct AtomicField {
        pub identifier: IdentifierString,
        pub parameters: Vec<Parameter>,
        pub keyword_list: Vec<IdentifierString>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ParameterField {
        pub parameter: Parameter,
        pub keyword_list: Vec<IdentifierString>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Parameter {
        Char(CharParameter),
        Int(IntParameter),
        Float(FloatParameter),
        String(StringParameter),
        Blob(BlobParameter),
        Struct(StructParameter),
        Array(ArrayParameter),
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CharParameter {
        pub identifier: Option<IdentifierString>,
        pub char_literal: Option<char>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct IntParameter {
        pub identifier: Option<IdentifierString>,
        pub int_type: DCToken,
        pub int_range: Option<Range<i64>>,
        pub int_transform: Option<IntTransform>,
        pub int_constant: Option<i64>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct FloatParameter {
        pub identifier: Option<IdentifierString>,
        pub float_range: Option<Range<f64>>,
        pub float_transform: Option<FloatTransform>,
        pub float_constant: Option<f64>,
    }

    // NOTE: StringParameter and BlobParameter hold the same information,
    // as by specification they are both 'SizedParameter' types. We use
    // separate structs for strings and blobs so Donet knows exactly
    // what data type they are and make optimizations.

    #[derive(Debug, PartialEq, Clone)]
    pub struct StringParameter {
        pub identifier: Option<IdentifierString>,
        pub string_literal: Option<String>,
        pub size_constraint: Option<i64>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct BlobParameter {
        pub identifier: Option<IdentifierString>,
        pub string_literal: Option<String>,
        pub size_constraint: Option<i64>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct StructParameter {
        pub struct_type: IdentifierString,
        pub identifier: Option<IdentifierString>,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ArrayParameter {
        pub data_type: DCToken,
        pub identifier: Option<IdentifierString>,
        pub array_range: Range<i64>,
    }

    #[rustfmt::skip]
    #[derive(Debug, PartialEq, Clone)]
    pub enum BaseType {
        CharType, IntType, FloatType,
        StringType, BlobType, StructType,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum IntTransform {
        OperatorIntLiteral { operator: DCToken, int_literal: i64 },
        ParenthesizedIntTransform(Box<IntTransform>),
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum FloatTransform {
        OperatorFloatLiteral { operator: DCToken, float_literal: f64 },
        ParenthesizedFloatTransform(Box<FloatTransform>),
    }
}

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
        Keyword Identifier(id) Semicolon => id
    }

    // ----- Struct Type ----- //

    struct_type: ast::StructType {
        Struct Identifier(id) OpenBraces struct_parameters[ps]
        CloseBraces Semicolon => ast::StructType {
            span: span!(),
            identifier: id,
            parameters: ps,
        }
    }

    // ----- Distributed Class Type ----- //

    distributed_class_type: ast::DistributedClassType {
        DClass Identifier(id) optional_inheritance[pc] OpenBraces
        field_declarations[fds] CloseBraces Semicolon => ast::DistributedClassType {
            span: span!(),
            identifier: id,
            parent_classes: pc,
            field_declarations: fds,
        }
    }

    optional_inheritance: Option<Vec<String>> {
        => None,
        Colon Identifier(parent) additional_parents[mut cp] => {
            cp.insert(0, parent);
            Some(cp)
        },
    }

    additional_parents: Vec<String> {
        => vec![],
        additional_parents[mut cp] Comma Identifier(class) => {
            cp.push(class);
            cp
        }
    }

    // ----- Type Definition Type ----- //

    type_definition: ast::TypeDefinition {
        Typedef CharT Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: CharT,
            alias: alias,
        },
        Typedef signed_integers[dt] Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: dt,
            alias: alias,
        },
        Typedef unsigned_integers[dt] Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: dt,
            alias: alias,
        },
        Typedef array_data_types[dt] Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: dt,
            alias: alias,
        },
        Typedef Float64T Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: Float64T,
            alias: alias,
        },
        Typedef StringT Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: StringT,
            alias: alias,
        },
        Typedef BlobT Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: BlobT,
            alias: alias,
        },
        Typedef Blob32T Identifier(alias) Semicolon => ast::TypeDefinition {
            span: span!(),
            dc_type: Blob32T,
            alias: alias,
        },
    }

    // ----- Python-style Import ----- //

    // Donet does not make use of python-style import statements,
    // as this is a feature used by Donet clients and AI/UD processes.
    // We still have our production rules defined to avoid a parser panic.
    python_import: ast::DCImport {
        py_module[(m, ms)] dclass_import[(c, cs)] => {
            // NOTE: This is an ugly fix for not being able to pass Options
            // through the production parameters (due to moved values and
            // borrow checking issues (skill issues)), so we turn the Vectors
            // (which do implement the Copy trait) into Options here.
            let mut mvs_opt: Option<Vec<String>> = None;
            let mut cvs_opt: Option<Vec<String>> = None;
            if !ms.is_empty() {
                mvs_opt = Some(ms);
            }
            if !cs.is_empty() {
                cvs_opt = Some(cs);
            }
            ast::DCImport {
                span: span!(),
                module: m,
                module_views: mvs_opt,
                class: c,
                class_views: cvs_opt,
            }
        },
    }

    // e.g. "from views ..."
    // e.g. "from game.views.Donut/AI ..."
    py_module: (Vec<String>, Vec<String>) {
        From modules[ms] slash_identifier[is] => (ms, is)
    }

    // Bundles module names in 'from' statements, e.g. "myviews.Donut".
    modules: Vec<String> {
        module_identifier[m] => vec![m],
        modules[mut nm] Period module_identifier[m] => {
            nm.push(m);
            nm
        }
    }

    // NOTE: Module names may be lexed as identifiers or module tokens.
    module_identifier: String {
        Identifier(m) => m,
        Module(m) => m,
    }

    // e.g. "... import DistributedDonut/AI/OV"
    // e.g. "... import *"
    dclass_import: (String, Vec<String>) {
        Import Identifier(c) slash_identifier[cs] => (c, cs),
        Import Star => ("*".to_string(), vec![]),
    }

    // Bundle up all views of a dclass/module to be imported, into a vector
    // of strings, each corresponding to a view suffix. (AI, UD, OV..)
    //
    //      slash_identifier -> ε
    //      slash_identifier -> slash_identifier '/' Identifier
    slash_identifier: Vec<String> {
        => vec![],
        slash_identifier[mut si] ForwardSlash Identifier(id) => {
            si.push(id);
            si
        }
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
        Identifier(id) Colon atomic_field[af] atomic_fields[mut afs] Semicolon => {
            afs.insert(0, af);
            let mut new_afs: Vec<ast::FieldType> = vec![];

            for atomic_field in &afs {
                new_afs.push(ast::FieldType::Atomic(atomic_field.clone()));
            }

            ast::MolecularField {
                identifier: id,
                fields: new_afs,
            }
        },
        Identifier(id) Colon parameter_field[pf] parameter_fields[mut pfs] Semicolon => {
            pfs.insert(0, pf);
            let mut new_pfs: Vec<ast::FieldType> = vec![];

            for parameter_field in &pfs {
                new_pfs.push(ast::FieldType::Parameter(parameter_field.clone()));
            }

            ast::MolecularField {
                identifier: id,
                fields: new_pfs,
            }
        },
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

    // ----- Parameters ----- //

    struct_parameters: Vec<ast::Parameter> {
        => vec![],
        struct_parameters[mut ps] struct_parameter[p] => {
            ps.push(p);
            ps
        }
    }

    struct_parameter: ast::Parameter {
        parameter[p] Semicolon => p
    }

    parameters: Vec<ast::Parameter> {
        => vec![],
        #[no_reduce(Comma)] // don't reduce if we're expecting more params
        parameters[mut ps] parameter[p] => {
            ps.push(p);
            ps
        },
        parameters[mut ps] parameter[p] Comma => {
            ps.push(p);
            ps
        },
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

    array_range: Range<i64> {
        OpenBrackets array_range_opt[array_range] CloseBrackets => array_range
    }

    array_range_opt: Range<i64> {
        => 0 .. 0,
        #[no_reduce(Hyphen)] // do not reduce if lookahead is the '-' token
        DecimalLiteral(a) => a .. a,
        DecimalLiteral(min) Hyphen DecimalLiteral(max) => min .. max,
    }

    int_transform: Option<ast::IntTransform> {
        => None,
        // FIXME: Accept spec's `IntegerLiteral`, not just DecimalLiteral.
        Percent DecimalLiteral(dl) => Some(ast::IntTransform::OperatorIntLiteral {
            operator: Percent,
            int_literal: dl,
        }),
        ForwardSlash DecimalLiteral(dl) => Some(ast::IntTransform::OperatorIntLiteral {
            operator: ForwardSlash,
            int_literal: dl,
        }),
        Star DecimalLiteral(dl) => Some(ast::IntTransform::OperatorIntLiteral {
            operator: Star,
            int_literal: dl,
        }),
        Hyphen DecimalLiteral(dl) => Some(ast::IntTransform::OperatorIntLiteral {
            operator: Hyphen,
            int_literal: dl,
        }),
        Plus DecimalLiteral(dl) => Some(ast::IntTransform::OperatorIntLiteral {
            operator: Plus,
            int_literal: dl,
        }),
    }

    float_transform: Option<ast::FloatTransform> {
        => None,
        // TODO: Implement
    }

    optional_name: Option<String> {
        // if epsilon found AND lookahead is Identifier, don't reduce
        // this is what holds together the parser from shitting itself.
        #[no_reduce(Identifier)]
        => None,
        Identifier(id) => Some(id)
    }

    param_char_init: Option<char> {
        => None,
        Equals CharacterLiteral(cl) => Some(cl),
    }

    param_str_init: Option<String> {
        => None,
        Equals StringLiteral(sl) => Some(sl),
    }

    param_bin_init: Option<String> {
        => None,
        Equals BinaryLiteral(bl) => Some(bl),
    }

    param_dec_const: Option<i64> {
        => None,
        Equals DecimalLiteral(dc) => Some(dc),
    }

    param_float_const: Option<f64> {
        => None,
        Equals FloatLiteral(fl) => Some(fl),
    }

    // ----- Char Parameter ----- //
    char_param: ast::CharParameter {
        CharT optional_name[id] param_char_init[cl] => ast::CharParameter {
            identifier: id,
            char_literal: cl,
        }
    }

    // ----- Integer Parameter ----- //
    int_param: ast::IntParameter {
        signed_integers[it] int_range[ir] int_transform[itr]
        optional_name[id] param_dec_const[dc] => ast::IntParameter {
            int_type: it,
            identifier: id,
            int_range: ir,
            int_transform: itr,
            int_constant: dc,
        },
        unsigned_integers[it] int_range[ir] int_transform[itr]
        optional_name[id] param_dec_const[dc] => ast::IntParameter {
            int_type: it,
            identifier: id,
            int_range: ir,
            int_transform: itr,
            int_constant: dc,
        },
    }

    signed_integers: DCToken {
        Int8T => Int8T,
        Int16T => Int16T,
        Int32T => Int32T,
        Int64T => Int64T,
    }

    unsigned_integers: DCToken {
        UInt8T => UInt8T,
        UInt16T => UInt16T,
        UInt32T => UInt32T,
        UInt64T => UInt64T,
    }

    array_data_types: DCToken {
        Int8ArrayT => Int8ArrayT,
        Int16ArrayT => Int16ArrayT,
        Int32ArrayT => Int32ArrayT,
        UInt8ArrayT => UInt8ArrayT,
        UInt16ArrayT => UInt16ArrayT,
        UInt32ArrayT => UInt32ArrayT,
        UInt32UInt8ArrayT => UInt32UInt8ArrayT,
    }

    // ----- Float Parameter ----- //
    float_param: ast::FloatParameter {
        Float64T float_range[fr] float_transform[ft]
        optional_name[id] param_float_const[fl] => ast::FloatParameter {
            identifier: id,
            float_range: fr,
            float_transform: ft,
            float_constant: fl,
        }
    }

    // ----- String Parameter ----- //
    string_param: ast::StringParameter {
        StringT size_constraint[sc] optional_name[id] param_str_init[sl] => ast::StringParameter {
            identifier: id,
            string_literal: sl,
            size_constraint: sc,
        }
    }

    // ----- Blob Parameter ----- //
    blob_param: ast::BlobParameter {
        BlobT size_constraint[sc] optional_name[id] param_bin_init[bl] => ast::BlobParameter {
            identifier: id,
            string_literal: bl,
            size_constraint: sc,
        },
    }

    // ----- Struct Parameter ----- //
    struct_param: ast::StructParameter {
        #[no_reduce(OpenBrackets)] // avoid ambiguity between struct & array parameters
        Identifier(st) optional_name[si] => ast::StructParameter {
            struct_type: st,
            identifier: si,
        }
    }

    // ----- Array Parameter ----- //
    array_param: ast::ArrayParameter {
        Identifier(_) optional_name[ai] array_range[ar] => ast::ArrayParameter {
            data_type: CharT, // fixme
            identifier: ai,
            array_range: ar,
        },
        signed_integers[dt] array_range[ar] optional_name[id] => ast::ArrayParameter {
            data_type: dt,
            identifier: id,
            array_range: ar,
        },
        unsigned_integers[dt] array_range[ar] optional_name[id] => ast::ArrayParameter {
            data_type: dt,
            identifier: id,
            array_range: ar,
        },
        array_data_types[dt] array_range[ar] optional_name[id] => ast::ArrayParameter {
            data_type: dt,
            identifier: id,
            array_range: ar,
        },
    }

    // ----- DC Keywords ----- //

    // Bundle up all (or none) DCKeyword tokens into one production.
    dc_keyword_list: Vec<String> {
        => vec![],
        dc_keyword_list[mut kl] DCKeyword(k) => {
            kl.push(k);
            kl
        }
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
    use super::{ast, parse, DCToken, Span};
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
                    dc_type: DCToken::CharT,
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
                             from game.views.Donut/AI import DistributedDonut/AI\n\
                             from views import *\n";
        let target_ast: ast::DCFile = ast::DCFile {
            type_decl: vec![
                // "from example_views import DistributedDonut"
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
                        module_views: None,
                        class: "DistributedDonut".to_string(),
                        class_views: None,
                    }),
                },
                // "from views import DistributedDonut/AI/OV"
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
                        module_views: None,
                        class: "DistributedDonut".to_string(),
                        class_views: Some(vec!["AI".to_string(), "OV".to_string()]),
                    }),
                },
                // "from views/AI/OV import DistributedDonut/AI/OV"
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
                        module_views: Some(vec!["AI".to_string(), "OV".to_string()]),
                        class: "DistributedDonut".to_string(),
                        class_views: Some(vec!["AI".to_string(), "OV".to_string()]),
                    }),
                },
                // "from game.views.Donut/AI import DistributedDonut/AI"
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
                        module: vec!["game".to_string(), "views".to_string(), "Donut".to_string()],
                        module_views: Some(vec!["AI".to_string()]),
                        class: "DistributedDonut".to_string(),
                        class_views: Some(vec!["AI".to_string()]),
                    }),
                },
                // "from views import *"
                ast::TypeDecl {
                    span: Span {
                        min: 183,
                        max: 202,
                        line: 5,
                    },
                    node: ast::TypeDecl_::DCImport(ast::DCImport {
                        span: Span {
                            min: 183,
                            max: 202,
                            line: 5,
                        },
                        module: vec!["views".to_string()],
                        module_views: None,
                        class: "*".to_string(),
                        class_views: None,
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
                    parent_classes: None,
                    field_declarations: vec![],
                }),
            }],
        };
        parse_for_ast_target(dc_file, target_ast);
    }
}
