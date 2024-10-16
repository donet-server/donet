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

//! Definition of the DC language [`Context Free Grammar`] for the
//! LALR(1) parser processing the stream of lexical tokens.
//!
//! [`Context Free Grammar`]: https://en.wikipedia.org/wiki/Context-free_grammar

// Please see plex issue #45. https://github.com/goffrie/plex/issues/45
#![allow(
    clippy::type_complexity,
    clippy::redundant_field_names,
    clippy::ptr_arg,
    clippy::redundant_closure_call,
    clippy::enum_variant_names,
    clippy::let_unit_value
)]

use super::ast;
use super::lexer::DCToken::*;
use super::lexer::{DCToken, Span};
use crate::dctype::DCTypeEnum;

use plex::parser;
use std::mem::discriminant;

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

    // The 'dc_file' production is the root production of the grammar.
    // Plex knows this is the start symbol of our grammar as it is declared first.
    dc_file: ast::Root {
        epsilon => ast::Root {
            type_declarations: vec![],
        },
        dc_file[root] Semicolon => root,
        dc_file[mut root] type_decl[type_decl] => {
            root.type_declarations.push(type_decl);
            root
        },
    }

    type_decl: ast::TypeDeclaration {
        python_style_import[py_imports] => ast::TypeDeclaration::PythonImport(py_imports),
        keyword_type[keyword] => ast::TypeDeclaration::KeywordType(keyword),
        struct_type[strct] => ast::TypeDeclaration::StructType(strct),
        distributed_class_type[dclass] => ast::TypeDeclaration::DClassType(dclass),
        type_definition => ast::TypeDeclaration::TypedefType,
    }

    // ---------- Python-style Imports ---------- //

    python_style_import: ast::PythonImport {
        py_module[(module, module_views)] dclass_import[(class, class_views)] => {
            // Since we can store multiple module imports with many
            // symbols each, (via view suffixes on module identifiers)
            // we store a vector of `PyModuleImport` structs in our result.
            let mut result = ast::PythonImport {
                span: span!(),
                imports: vec![],
            };

            /* NOTE: Workaround for not being able to pass Options through
             * the non-terminal parameters (due to moved values and borrow
             * checking issues (skill issues)), so we turn the Vectors
             * (which do implement the Copy trait) into Options here.
             */
            let mut optional_module_views: Option<Vec<String>> = None;
            let mut optional_class_views: Option<Vec<String>> = None;

            if !module_views.is_empty() {
                optional_module_views = Some(module_views);
            }
            if !class_views.is_empty() {
                optional_class_views = Some(class_views);
            }

            let mut class_symbols: Vec<String> = vec![class.clone()];

            // Separates "Class/AI/OV" to ["Class", "ClassAI", "ClassOV"]
            if optional_class_views.is_some() {
                for class_suffix in &optional_class_views.unwrap() {
                    class_symbols.push(class.clone() + class_suffix);
                }
            }

            // Handles e.g. "from module/AI/OV/UD import DistributedThing/AI/OV/UD"
            if optional_module_views.is_some() {
                let mut c_symbol: String = class_symbols.first().unwrap().clone();

                result.imports.push(ast::PyModuleImport {
                    python_module: module.clone(),
                    symbols: vec![c_symbol],
                });

                for (i, module_suffix) in optional_module_views.unwrap().into_iter().enumerate() {
                    let full_import: String = module.clone() + &module_suffix;

                    if (class_symbols.len() - 1) <= i {
                        c_symbol = class_symbols.last().unwrap().clone();
                    } else {
                        c_symbol = class_symbols.get(i + 1).unwrap().clone();
                    }

                    result.imports.push(ast::PyModuleImport {
                        python_module: full_import,
                        symbols: vec![c_symbol]
                    });
                }
                return result;
            }

            result.imports.push(ast::PyModuleImport {
                python_module: module,
                symbols: class_symbols
            });

            result
        },
    }

    // e.g. "from views ..."
    // e.g. "from game.views.Donut/AI ..."
    py_module: (String, Vec<String>) {
        From modules[modules] slash_identifier[views] => {

            // We need to join all module identifiers into one string
            let mut modules_string: String = String::new();

            for (i, mod_) in modules.into_iter().enumerate() {
                if i != 0 {
                    modules_string.push('.');
                }
                modules_string.push_str(&mod_);
            }
            (modules_string, views)
        }
    }

    // Bundles module names in 'from' statements, e.g. "myviews.Donut".
    modules: Vec<String> {
        legal_python_module_identifiers[module] => vec![module],
        modules[mut vector] Period legal_python_module_identifiers[module] => {
            vector.push(module);
            vector
        }
    }

    /* Mandatory fix for resolving issue #12.
     *
     * Specifically used by the Python-style DC import grammar to accept
     * **LEGAL** python module identifiers that may lexed as other tokens.
     */
    legal_python_module_identifiers: String {
        Identifier(id) => id,
        DCKeyword(id) => id,
        CharT => "char".to_string(),
        Int8T => "int8".to_string(),
        Int16T => "int16".to_string(),
        Int32T => "int32".to_string(),
        Int64T => "int64".to_string(),
        UInt8T => "uint8".to_string(),
        UInt16T => "uint16".to_string(),
        UInt32T => "uint32".to_string(),
        UInt64T => "uint64".to_string(),
        Float32T => "float32".to_string(),
        Float64T => "float64".to_string(),
        Int8ArrayT => "int8array".to_string(),
        Int16ArrayT => "int16array".to_string(),
        Int32ArrayT => "int32array".to_string(),
        UInt8ArrayT => "uint8array".to_string(),
        UInt16ArrayT => "uint16array".to_string(),
        UInt32ArrayT => "uint32array".to_string(),
        UInt32UInt8ArrayT => "uint32uint8array".to_string(),
        StringT => "string".to_string(),
        BlobT => "blob".to_string(),
        Blob32T => "blob32".to_string(),
        DClass => "dclass".to_string(),
        Struct => "struct".to_string(),
        Keyword => "keyword".to_string(),
        Typedef => "typedef".to_string(),
        Switch => "switch".to_string(),
        Default => "default".to_string(),
        Break => "break".to_string(),
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
        epsilon => vec![],
        slash_identifier[mut si] ForwardSlash ViewSuffix(id) => {
            si.push(id);
            si
        }
    }

    // ---------- DC Keyword ---------- //

    keyword_type: ast::KeywordDefinition {
        Keyword Identifier(id) => {
            ast::KeywordDefinition {
                span: span!(),
                identifier: id,
            }
        },
        Keyword DCKeyword(historic) => {
            ast::KeywordDefinition {
                span: span!(),
                identifier: historic,
            }
        }
    }

    // ---------- Distributed Class ---------- //

    distributed_class_type: ast::DClass {
        DClass Identifier(id) optional_inheritance[parents] OpenBraces
        optional_class_fields[fields] CloseBraces => {
            ast::DClass {
                span: span!(),
                identifier: id,
                parents,
                fields,
            }
        }
    }

    optional_class_fields: ast::ClassFields {
        epsilon => vec![],
        optional_class_fields[mut vector] class_field[field] Semicolon => {
            vector.push(field);
            vector
        },
    }

    class_field: ast::AtomicOrMolecular {
        // e.g. "setPos(float64 x, float64 y, float64 z) ram broadcast" (atomic)
        // e.g. "string DcObjectType db" (plain field)
        named_field[nf] dc_keyword_list[keywords] => {
            ast::AtomicOrMolecular::Atomic(
                ast::AtomicField::from_named_field(nf, keywords, span!())
            )
        },
        // e.g. "setStats : setAvatarCount, setNewAvatarCount"
        molecular_field[molecular] => {
            ast::AtomicOrMolecular::Molecular(molecular)
        },
    }

    optional_inheritance: Vec<String> {
        epsilon => vec![],
        Colon Identifier(parent) class_parents[mut cp] => {
            cp.insert(0, parent);
            cp
        },
    }

    class_parents: Vec<String> {
        epsilon => vec![],
        class_parents[mut cp] Comma Identifier(class) => {
            cp.push(class);
            cp
        }
    }

    // ---------- Type Definitions ---------- //

    type_definition: () {
        Typedef nonmethod_type_with_name => {},
        // This rule handles a specific piece of illegal grammar that is legal in Panda.
        // The parser will print a useful message to stdout describing the issue,
        // and will ignore this grammar and continue without a panic.
        Typedef UInt8T BoolT => println!("{}\n\n\"typedef uint8 bool;\" is deprecated!\n\n\
        Cannot declare type alias for uint8 as 'bool', as it is a reserved identifier \
        in the DC language.\nDonet introduces the 'bool' data type, which is an alias \
        for uint8 under the hood.\n", span!()),
        type_definition OpenBrackets array_range CloseBrackets => {},
    }

    // ---------- Molecular Field ---------- //

    // e.g. "setStats : setAvatarCount, setNewAvatarCount"
    molecular_field: ast::MolecularField {
        // Molecular fields require at least one atomic name.
        // They **should** require a minimum of two as suggested by Astron
        // docs and Panda source comments, but one atomic name is historically legal.
        Identifier(id) Colon Identifier(first_atomic) molecular_atom_list[mut atomics] => {
            ast::MolecularField {
                span: span!(),
                identifier: id,
                atomic_field_identifiers: {
                    let mut vec: Vec<String> = vec![first_atomic];

                    vec.append(&mut atomics);
                    vec
                }
            }
        },
    }

    molecular_atom_list: Vec<String> {
        epsilon => vec![],
        molecular_atom_list[mut atomics] Comma Identifier(atomic_name) => {
            atomics.push(atomic_name);
            atomics
        },
    }

    // ---------- DC Struct ---------- //

    struct_type: ast::Struct {
        Struct Identifier(id) OpenBraces struct_fields[fields] CloseBraces => {
            ast::Struct {
                span: span!(),
                identifier: id,
                fields,
            }
        },
    }

    struct_fields: Vec<ast::StructField> {
        epsilon => vec![],
        struct_fields[mut vec] struct_field[field] Semicolon => {
            vec.push(field);
            vec
        },
    }

    struct_field: ast::StructField {
        switch_type[sw] => ast::StructField::Switch(sw),
        unnamed_field[pf] => ast::StructField::ParameterField(pf),
        named_field[nf] => nf.into(),
    }

    // ---------- DC Switch Statements ---------- //

    switch_type: ast::Switch {
        Switch optional_name[id] OpenParenthesis parameter_field[field] CloseParenthesis
        OpenBraces switch_cases[cases] CloseBraces => {
            ast::Switch {
                span: span!(),
                identifier: id,
                key_parameter: field,
                cases: cases,
            }
        }
    }

    switch_cases: Vec<ast::Case> {
        epsilon => vec![],
        switch_cases[mut vec] switch_case[case] => {
            vec.push(case);
            vec
        },
    }

    switch_case: ast::Case {
        switch_case_body[mut case] optional_break[breaks] => {
            case.span = span!(); // update span
            case.breaks = breaks;
            case
        }
    }

    switch_case_body: ast::Case {
        Default Colon => ast::Case {
            span: span!(),
            condition: None, // `None` means default
            fields: vec![],
            breaks: false,
        },
        Case type_value[condition] Colon => ast::Case {
            span: span!(),
            condition: Some(condition),
            fields: vec![],
            breaks: false,
        },
        switch_case[mut case] parameter_field[field] Semicolon => {
            case.fields.push(field);
            case
        },
    }

    optional_break: bool {
        epsilon => false,
        Break Semicolon => true,
    }

    // ---------- DC Fields ---------- //

    named_field: ast::NamedField {
        method_as_field[mf] => ast::NamedField::MethodAsField(mf),
        nonmethod_type_with_name[nmt] => {
            let param: ast::Parameter = nmt.into();

            ast::NamedField::ParameterField(param.into())
        },
        field_with_name_as_array[field] => ast::NamedField::ParameterField(field),
        field_with_name_and_default[field] => ast::NamedField::ParameterField(field),
    }

    field_with_name_as_array: ast::ParameterField {
        nonmethod_type_with_name[nmt]
        OpenBrackets array_range[_] CloseBrackets => {
            let param: ast::Parameter = nmt.into();

            // FIXME: apply array range
            param.into()
        },
        field_with_name_as_array[pf]
        OpenBrackets array_range[_] CloseBrackets => {
            // FIXME: apply array range
            pf
        },
    }

    field_with_name_and_default: ast::ParameterField {
        nonmethod_type_with_name[nmt] Equals type_value[value] => {
            let mut param: ast::Parameter = nmt.into();

            param.default_value = Some(value);
            param.into()
        },
        field_with_name_as_array[mut field] Equals type_value[value] => {
            field.parameter.default_value = Some(value);
            field
        },
    }

    unnamed_field: ast::ParameterField {
        nonmethod_type[nmt] => {
            let param: ast::Parameter = nmt.into();
            param.into()
        },
        nonmethod_type[nmt] Equals type_value[value] => {
            let mut param: ast::Parameter = nmt.into();
            param.default_value = Some(value);

            param.into()
        },
    }

    // e.g. "setName(string)"
    method_as_field: ast::MethodAsField {
        Identifier(id) method_body[parameters] => {
            ast::MethodAsField {
                span: span!(),
                identifier: id,
                parameters,
            }
        },
    }

    // e.g. "(int8, int16, string, blob)"
    method_body: ast::MethodBody {
        OpenParenthesis parameters[params] CloseParenthesis => params,
    }

    // ---------- Parameter Fields ---------- //

    parameter_fields: Vec<ast::ParameterField> {
        epsilon => vec![],
        parameter_fields[mut vec] Comma parameter_field[pf] => {
            vec.push(pf);
            vec
        },
    }

    parameter_field: ast::ParameterField {
        parameter[param] dc_keyword_list[kl] => {
            let mut pf: ast::ParameterField = param.into();

            pf.keywords = kl;
            pf
        },
    }

    dc_keyword_list: ast::KeywordList {
        epsilon => vec![],
        dc_keyword_list[mut vec] Identifier(keyword) => {
            vec.push(keyword);
            vec
        }
        dc_keyword_list[mut vec] DCKeyword(keyword) => {
            vec.push(keyword);
            vec
        }
    }

    // ---------- Parameter ---------- //

    parameters: Vec<ast::Parameter> {
        epsilon => vec![],
        #[no_reduce(Comma)] // don't reduce if we're expecting more params
        parameters[mut vector] parameter[param] => {
            vector.push(param);
            vector
        },
        parameters[mut vector] parameter[param] Comma => {
            vector.push(param);
            vector
        },
    }

    parameter: ast::Parameter {
        nonmethod_type[nmt] => nmt.into(),
        nonmethod_type[nmt] Equals type_value[value] => {
            let mut param: ast::Parameter = nmt.into();

            param.default_value = Some(value);
            param
        },
    }

    // ---------- DC Data Types ---------- //

    nonmethod_type_with_name: ast::NonMethodType {
        nonmethod_type[mut nmt] Identifier(id) => {
            nmt.identifier = Some(id);
            nmt
        },
    }

    nonmethod_type: ast::NonMethodType {
        nonmethod_type_no_array[nmt] => nmt,
        #[no_reduce(OpenBrackets)] // avoids conflict with `type_with_array`
        type_with_array[twa] => ast::NonMethodType {
            span: span!(),
            identifier: None,
            data_type: ast::NonMethodDataType::TypeWithArray(twa),
        },
    }

    nonmethod_type_no_array: ast::NonMethodType {
        #[no_reduce(OpenBrackets)]
        Identifier(id) => ast::NonMethodType {
            span: span!(),
            identifier: None,
            data_type: ast::NonMethodDataType::StructType(id),
        },
        #[no_reduce(OpenBrackets)]
        numeric_type[nt] => ast::NonMethodType {
            span: span!(),
            identifier: None,
            data_type: ast::NonMethodDataType::NumericType(nt),
        },
        #[no_reduce(OpenBrackets)]
        builtin_array_type[twa] => ast::NonMethodType {
            span: span!(),
            identifier: None,
            data_type: ast::NonMethodDataType::TypeWithArray(twa),
        },
    }

    type_with_array: ast::TypeWithArray {
        numeric_type[nt] OpenBrackets array_range[ar] CloseBrackets => {
            ast::TypeWithArray {
                span: span!(),
                data_type: ast::ArrayableType::NumericType(nt),
                array_ranges: match ar {
                    Some(range) => vec![range],
                    None => vec![],
                },
            }
        },
        Identifier(id) OpenBrackets array_range[ar] CloseBrackets => {
            ast::TypeWithArray {
                span: span!(),
                data_type: ast::ArrayableType::StructType(id),
                array_ranges: match ar {
                    Some(range) => vec![range],
                    None => vec![],
                },
            }
        },
        builtin_array_type[mut twa] OpenBrackets array_range[ar] CloseBrackets => {
            if let Some(range) = ar {
                twa.array_ranges.push(range);
            }
            twa
        },
        type_with_array[mut twa] OpenBrackets array_range[ar] CloseBrackets => {
            if let Some(range) = ar {
                twa.array_ranges.push(range);
            }
            twa
        },
    }

    builtin_array_type: ast::TypeWithArray {
        sized_type_token[st] => ast::TypeWithArray {
            span: span!(),
            data_type: ast::ArrayableType::SizedType(st),
            array_ranges: vec![],
        },
        sized_type_token[st] OpenParenthesis array_range[ar] CloseParenthesis => {
            ast::TypeWithArray {
                span: span!(),
                data_type: ast::ArrayableType::SizedType(st),
                array_ranges: match ar {
                    Some(range) => vec![range],
                    None => vec![],
                },
            }
        },
    }

    // e.g. "[0 * 14]"
    array_expansion: ast::ArrayExpansion {
        type_value[tv] => (tv, 1_u32), // factor of 1 by default
        signed_integer[i] Star unsigned_32_bit_int[f] => (ast::TypeValue::I64(i), f),
        DecimalLiteral(i) Star unsigned_32_bit_int[f] => (ast::TypeValue::I64(i), f),
        HexLiteral(hs) Star unsigned_32_bit_int[f] => (ast::TypeValue::String(hs), f),
        StringLiteral(s) Star unsigned_32_bit_int[f] => (ast::TypeValue::String(s), f),
    }

    array_value: Vec<ast::ArrayExpansion> {
        OpenBrackets CloseBrackets => vec![],
        OpenBrackets element_values[ev] CloseBrackets => ev,
    }

    element_values: Vec<ast::ArrayExpansion> {
        array_expansion[ae] => vec![ae],
        element_values[mut ev] Comma array_expansion[ae] => {
            ev.push(ae);
            ev
        },
    }

    parameter_values: ast::ParameterValues {
        type_value[tv] => vec![tv],
        parameter_values[mut vec] Comma type_value[tv] => {
            vec.push(tv);
            vec
        },
    }

    type_or_sized_value: ast::TypeOrSizedValue {
        type_value[tv] => ast::TypeOrSizedValue::TypeValue(tv),
        sized_type_token[st] => ast::TypeOrSizedValue::SizedValue(st),
    }

    type_value: ast::TypeValue {
        BooleanLiteral(b) => ast::TypeValue::I64(match b {
            true => 1,
            false => 0,
        }),
        DecimalLiteral(i) => ast::TypeValue::I64(i),
        CharacterLiteral(c) => ast::TypeValue::Char(c),
        StringLiteral(s) => ast::TypeValue::String(s),
        HexLiteral(hs) => ast::TypeValue::String(hs),
        signed_integer[i] => ast::TypeValue::I64(i),
        array_value[av] => ast::TypeValue::ArrayValue(av),
    }

    numeric_type: ast::NumericType {
        numeric_type_token[nt] => nt,
        numeric_with_explicit_cast[nt] => nt,
        numeric_with_modulus[nt] => nt,
        numeric_with_divisor[nt] => nt,
        numeric_with_range[nt] => nt,
    }

    numeric_with_range: ast::NumericType {
        numeric_type_token[mut nt] OpenParenthesis numeric_range[nr] CloseParenthesis => {
            nt.range = nr;
            nt
        },
        numeric_with_explicit_cast[mut nt] OpenParenthesis numeric_range[nr] CloseParenthesis => {
            nt.range = nr;
            nt
        },
        numeric_with_modulus[mut nt] OpenParenthesis numeric_range[nr] CloseParenthesis => {
            nt.range = nr;
            nt
        },
        numeric_with_divisor[mut nt] OpenParenthesis numeric_range[nr] CloseParenthesis => {
            nt.range = nr;
            nt
        },
    }

    numeric_with_divisor: ast::NumericType {
        numeric_type_token[mut nt] ForwardSlash number[num] => {
            nt.add_divisor(num);
            nt
        },
        numeric_with_explicit_cast[mut nt] ForwardSlash number[num] => {
            nt.add_divisor(num);
            nt
        },
        numeric_with_modulus[mut nt] ForwardSlash number[num] => {
            nt.add_divisor(num);
            nt
        },
    }

    numeric_with_modulus: ast::NumericType {
        numeric_type_token[mut nt] Percent number[num] => {
            nt.add_modulus(num);
            nt
        },
        numeric_with_explicit_cast[mut nt] Percent number[num] => {
            nt.add_modulus(num);
            nt
        },
    }

    // This is unique to Donet, and a new addition to the historic DC language.
    // Originally, the DC system was used with Python clients, which do not need
    // strict type annotations as Python is a dynamically typed language.
    //
    // Since we are not expecting the client to use a dynamically typed language, we need
    // to explicitly tell the client what data type to cast to when we perform these
    // operations on numeric types after they are received from the network.
    numeric_with_explicit_cast: ast::NumericType {
        // Explicit casts do not use the `numeric_type_token` non-terminal, because
        // there is zero need to cast any numeric data type to a Char or Bool, since
        // this is used for types that have arithmetic operations applied, such as division.
        //
        // Also because it is 2:27 AM and its giving me a shift-reduce conflict again.
        numeric_type_token[mut nt]
        OpenParenthesis signed_integer_type[dt] CloseParenthesis => {
            nt.cast = Some(dt);
            nt
        },
        numeric_type_token[mut nt]
        OpenParenthesis unsigned_integer_type[dt] CloseParenthesis => {
            nt.cast = Some(dt);
            nt
        },
        numeric_type_token[mut nt]
        OpenParenthesis floating_point_type[dt] CloseParenthesis => {
            nt.cast = Some(dt);
            nt
        },
    }

    numeric_range: Option<ast::NumericRange> {
        epsilon => None,

        char_or_number[v] => match v {
            ast::CharOrNumber::Char(c) => {
                let min_max: f64 = f64::from(u32::from(c));
                Some(min_max .. min_max)
            },
            ast::CharOrNumber::I64(i) => {
                let min_max: f64 = i as f64;
                Some(min_max .. min_max)
            },
            ast::CharOrNumber::F64(f) => Some(f .. f),
        },

        char_or_number[min] Hyphen char_or_number[max] => {
            assert!(
                discriminant(&min) == discriminant(&max),
                "{}\nCannot define a numeric range with a min and max of different data types!",
                span!()
            );

            match min {
                ast::CharOrNumber::Char(min_c) => {
                    let min: f64 = f64::from(u32::from(min_c));
                    let max: f64 = match max {
                        ast::CharOrNumber::Char(max_c) => f64::from(u32::from(max_c)),
                        _ => panic!("Assertion makes this panic impossible."),
                    };
                    Some(min .. max)
                },
                ast::CharOrNumber::I64(min_i) => {
                    Some(min_i as f64 .. match max {
                        ast::CharOrNumber::I64(max_i) => max_i as f64,
                        _ => panic!("Assertion makes this panic impossible."),
                    })
                },
                ast::CharOrNumber::F64(min_f) => Some(min_f .. match max {
                    ast::CharOrNumber::F64(max_f) => max_f,
                    _ => panic!("Assertion makes this panic impossible."),
                }),
            }
        },
    }

    array_range: Option<ast::NumericRange> {
        epsilon => None,
        char_or_u16[v] => match v {
            ast::CharOrU16::Char(c) => {
                let min_max: f64 = f64::from(u32::from(c));
                Some(min_max .. min_max)
            },
            ast::CharOrU16::U16(u) => {
                let min_max: f64 = f64::from(u);
                Some(min_max .. min_max)
            },
        },
        char_or_u16[min] Hyphen char_or_u16[max] => {
            let min_float: f64 = match min {
                ast::CharOrU16::Char(c) => f64::from(u32::from(c)),
                ast::CharOrU16::U16(u) => f64::from(u),
            };
            let max_float: f64 = match max {
                ast::CharOrU16::Char(c) => f64::from(u32::from(c)),
                ast::CharOrU16::U16(u) => f64::from(u),
            };
            Some(min_float .. max_float)
        },
    }

    // Both of these types represent a sized type (aka, array type)
    // Strings and blobs are another form of array types.
    sized_type_token: ast::SizedTypeToken {
        StringT => ast::SizedTypeToken::String,
        BlobT => ast::SizedTypeToken::Blob,
        Blob32T => ast::SizedTypeToken::Blob32,
        array_data_type[dt] => match dt.token {
            Int8ArrayT => ast::SizedTypeToken::Int8Array,
            Int16ArrayT => ast::SizedTypeToken::Int16Array,
            Int32ArrayT => ast::SizedTypeToken::Int32Array,
            UInt8ArrayT => ast::SizedTypeToken::UInt8Array,
            UInt16ArrayT => ast::SizedTypeToken::UInt16Array,
            UInt32ArrayT => ast::SizedTypeToken::UInt32Array,
            UInt32UInt8ArrayT => ast::SizedTypeToken::UInt32UInt8Array,
            _ => panic!("Not possible due to production rules."),
        },
    }

    numeric_type_token: ast::NumericType {
        CharT => ast::NumericType::from_type(DCTypeEnum::TChar, span!()),
        // 'bool' is an alias for uint8
        BoolT => ast::NumericType::from_type(DCTypeEnum::TUInt8, span!()),
        signed_integer_type[dt] => ast::NumericType::from_type(dt.dctype, span!()),
        unsigned_integer_type[dt] => ast::NumericType::from_type(dt.dctype, span!()),
        floating_point_type[dt] => ast::NumericType::from_type(dt.dctype, span!()),
    }

    char_or_number: ast::CharOrNumber {
        CharacterLiteral(c) => ast::CharOrNumber::Char(c),
        signed_integer[v] => ast::CharOrNumber::I64(v),

        number[num] => match num {
            ast::Number::Decimal(dl) => ast::CharOrNumber::I64(dl),
            ast::Number::Float(fl) => ast::CharOrNumber::F64(fl),
        },
    }

    signed_integer: i64 {
        Plus DecimalLiteral(dl) => dl,
        Hyphen DecimalLiteral(dl) => -dl, // hyphen consumed by lexer, so its parsed as positive
    }

    number: ast::Number {
        DecimalLiteral(dl) => ast::Number::Decimal(dl),
        FloatLiteral(fl) => ast::Number::Float(fl),
    }

    char_or_u16: ast::CharOrU16 {
        CharacterLiteral(cl) => ast::CharOrU16::Char(cl),
        unsigned_32_bit_int[u] => ast::CharOrU16::U16(u as u16),
    }

    // In Panda's parser, this production is known as 'small_unsigned_integer'.
    // C++ standard for an 'unsigned int' size is at least 16 bits.
    // 16 bits for LP32 data model; ILP32, LLP64, & LP64 are 32 bits.
    // Most C/C++ compilers store 'unsigned int' types with 32 bits.
    unsigned_32_bit_int: u32 {
        DecimalLiteral(v) => {
            match u32::try_from(v) {
                Ok(n) => { n },
                Err(err) => {
                    // Downcast failed, number must be out of range.
                    panic!("{}\nNumber out of range for u32.\n{}", span!(), err);
                },
            }
        }
    }

    floating_point_type: ast::DataType {
        Float32T => ast::DataType::from_token(Float32T, span!()),
        Float64T => ast::DataType::from_token(Float64T, span!()),
    }

    signed_integer_type: ast::DataType {
        Int8T => ast::DataType::from_token(Int8T, span!()),
        Int16T => ast::DataType::from_token(Int16T, span!()),
        Int32T => ast::DataType::from_token(Int32T, span!()),
        Int64T => ast::DataType::from_token(Int64T, span!()),
    }

    unsigned_integer_type: ast::DataType {
        UInt8T => ast::DataType::from_token(UInt8T, span!()),
        UInt16T => ast::DataType::from_token(UInt16T, span!()),
        UInt32T => ast::DataType::from_token(UInt32T, span!()),
        UInt64T => ast::DataType::from_token(UInt64T, span!()),
    }

    array_data_type: ast::DataType {
        Int8ArrayT => ast::DataType::from_token(Int8ArrayT, span!()),
        Int16ArrayT => ast::DataType::from_token(Int16ArrayT, span!()),
        Int32ArrayT => ast::DataType::from_token(Int32ArrayT, span!()),
        UInt8ArrayT => ast::DataType::from_token(UInt8ArrayT, span!()),
        UInt16ArrayT => ast::DataType::from_token(UInt16ArrayT, span!()),
        UInt32ArrayT => ast::DataType::from_token(UInt32ArrayT, span!()),
        UInt32UInt8ArrayT => ast::DataType::from_token(UInt32UInt8ArrayT, span!()),
    }

    optional_name: Option<String> {
        epsilon => None,
        Identifier(id) => Some(id)
    }

    epsilon: () {
        => {}, // alias for 'epsilon' (ε), a.k.a 'none' in GNU Bison
    }
}

/// Public function for the DC parser, takes in a stream of lexical tokens.
pub fn parse<I: Iterator<Item = (DCToken, Span)>>(
    i: I,
) -> Result<ast::Root, (Option<(DCToken, Span)>, &'static str)> {
    parse_(i)
}

#[cfg(test)]
mod unit_testing {
    use super::ast;
    use super::parse;
    use crate::parser::lexer::Lexer;

    fn parse_dcfile_string(input: &str) -> ast::Root {
        let lexer = Lexer::new(input).inspect(|tok| eprintln!("token: {:?}", tok));
        let dc_file_ast: ast::Root = parse(lexer).unwrap();

        dc_file_ast
    }

    #[test]
    fn python_module_imports() {
        let dc_file: ast::Root = parse_dcfile_string(
            "
            from example_views import DistributedDonut
            from views import DistributedDonut/AI/OV
            from views/AI/OV/UD import DistributedDonut/AI/OV/UD
            from views/AI import DistributedDonut
            from game.views.Donut/AI import DistributedDonut/AI
            from views import *

            /* The next one tests handling legal python identifiers
            * that may be lexed as tokens other than Id/Module.
            */
            from db.char import DistributedDonut
            ",
        );

        assert_eq!(dc_file.type_declarations.len(), 7);
    }

    #[test]
    fn legal_python_module_identifiers() {
        // See comment at 'legal_python_module_identifiers' non-terminal.
        #[rustfmt::skip]
        let legal_identifiers: Vec<&str> = vec![
            "char", "int8", "int16", "int32", "int64",
            "uint8", "uint16", "uint32", "uint64", "float32", "float64",
            "int8array", "int16array", "int32array",
            "uint8array", "uint16array", "uint32array", "uint32uint8array",
            "string", "blob", "blob32", "dclass", "struct", "keyword",
            "typedef", "switch", "default", "break",
        ];
        let mut dc_file: String = String::new();

        for module_name in &legal_identifiers {
            let code: String = format!("from {} import DistributedClass\n", *module_name);
            dc_file.push_str(code.as_str());
        }
        parse_dcfile_string(dc_file.as_str());
    }

    #[test]
    fn keyword_definitions() {
        parse_dcfile_string(
            "
            keyword p2p;
            keyword monkey;
            keyword unreliable;
            keyword db;
            ",
        );
    }

    #[test]
    fn struct_declarations() {
        parse_dcfile_string(
            "
            struct GiftItem {
                blob Item;
                string giftTag;
            };

            struct Activity {
                string activityName;
                uint8 activityId;
            };

            struct Party {
                activity activities[];
                uint8 status;
            };

            struct Fixture {
                bool;
                int32/10 x;
                int32/10 y;
                int32/10 z;
                int16/10 h;
                int16/10 p;
                int16/10 r;
                string state;
            };
            ",
        );
    }

    #[test]
    fn distributed_class() {
        parse_dcfile_string(
            "
            dclass OfflineShardManager : DistributedObject {
                clientSetZone(uint32) airecv clsend;
                requestZoneIdMessage(uint32, uint16) airecv clsend;
                requestZoneIdResponse(uint32, uint16);
            };

            dclass ShardStats {
                setShardId(uint32) broadcast required ram;
                setAvatarCount(uint32) broadcast required ram;
                setNewAvatarCount(uint32) broadcast required ram;
                setStats : setAvatarCount, setNewAvatarCount;
            };

            dclass DistributedChild : Parent, Parent2 {
            };
            ",
        );
    }

    #[test]
    fn switch_fields() {
        parse_dcfile_string(
            "
            struct BuffData {
                switch (uint16) {
                    case 0:
                        break;
                    case 1:
                        uint8 val1;
                        break;
                    case 2:
                        uint8 val1;
                        uint8 val2;
                        break;
                    case 3:
                        uint8 val1;
                        break;
                    case 4:
                        int16/100 val1;
                        break;
                };
                switch OptionalName (uint8) {
                    case 0:
                        break;
                    default:
                        uint8 value;
                        break;
                };
                switch WithDefault (char) {
                    case 'a':
                        break;
                    case 'b':
                    case 'c':
                    case 'd':
                    default:
                        string val1;
                        break;
                };
            };
            ",
        );
    }

    #[test]
    #[should_panic]
    fn switch_redundant_break() {
        parse_dcfile_string(
            "
            struct BuffData {
                switch (uint16) {
                    case 0:
                        break;
                        break;
                };
            };
            ",
        );
    }

    #[test]
    fn atomic_fields() {
        parse_dcfile_string(
            "
            dclass AtomicFields {
                simple();
                keyw0rd() ram;
                keywords() db ownsend airecv;
                parameter(string);
                params(bool, char, float64);
                named_params(bool flag = true, string text);
            };
            ",
        );
    }

    #[test]
    fn molecular_fields() {
        parse_dcfile_string(
            "
            dclass MolecularFields {
                setXYZ : setX, setY, setZ;
                setPos : setXYZ;
                setXY : setX, setY;
                setHPR : setH, setP, setR;
            };
            ",
        );
    }

    #[test]
    fn field_data_types() {
        parse_dcfile_string(
            "
            struct MethodDataTypesTest {
                Char character;
                blob Item;
                blob32 pandaOnlyToken;
                float32 astronOnlyToken;
                string giftTag;
                int32(0-990999) testMethodValue;
                int8(-1-1) testNegativeValues;
                int8(-5--99) testNegativeValuesPartTwo;
                int8(+0-+9) plusForPositiveForSomeReason;
                int8array arrayDataTypeTest;
                int16array anotherArray;
                int32array evenMoreComplexArray;
                uint8array byteArray;
                uint16array unsignedIntegerArray;
                uint32array unsignedLongArray;
                uint32uint8array thisWeirdPandaArrayType;
            };
            ",
        );
    }

    #[test]
    fn value_transforms() {
        parse_dcfile_string(
            "
            struct TransformedTypesTest {
                int32%360 angle;
                int32%360/1000 floatingPointAngle;
                int32/1000 efficientFloatIn32Bits;
                float32 waitIsntAstronsFloat32TheSame;
                int16(int32) forTheStaticallyTypedLanguages;
                int16(float64)(0.0-1.0) withRangeTest;
                int16(float32)%360/10.0 anotherTest;
                int16(uint32)/10 moreTests;
                bool thisIsLiterallyJustAn8BitInt;
                uint16/1000(0-1) youCanStackThemToo;
                int64/10000(+50-+999) [] thisIsValid;
                int8%10(0-10) anotherOne;
                int32('a'-'b') numericRangeWithChar;
                float32(0.1-0.99) floatingRange;
                float32%10.0 modulusWithFloat;
                float32(float64)%10.0 coverage;
                int16%100/10(-80-+100) lastTest;
            };
            ",
        );
    }

    #[test]
    fn numeric_ranges() {
        parse_dcfile_string(
            "
            struct NumericRanges {
                int8(0-1) thisIsLiterallyABoolean;
                int64(-5) signedRange;
                int64(+50-+999) thisIsValid;
                int32('a') numericRangeWithChar;
                int32('a'-'z') rangeMinMaxWithChar;
                float32(0.1-0.99) floatingRange;
                float32(0.1) anotherFloatRange;
                int32() pandaSaysThisIsLegal;
            };
            ",
        );
    }

    #[test]
    #[should_panic]
    fn invalid_numeric_ranges() {
        parse_dcfile_string(
            "
            struct InvalidNumericRange {
                uint64('a'-10);
            };
            ",
        );
    }

    #[test]
    fn parameters_with_default() {
        parse_dcfile_string(
            "
            struct ParamsWithDefaultTest {
                string = \"\";
                MyStruct[] = [];
                MyStruct strukt[] = [];
                int32 = -99;
                string = \"VALUE\";
                string = 0xabcdef;
                uint16 accessLevel = 0;
                bool = false;
            };
            ",
        );
    }

    #[test]
    fn array_ranges() {
        parse_dcfile_string(
            "
            struct ArrayRangesTest {
                uint8 test['a'];
                uint8 test2[9];
                uint32uint8array[0-1] test3;
                uint32uint8array[0-1][9-99] test4;
                uint8 test5['a'-'b'] [ ];
                string(5) test6; // builtin array type
            };
            ",
        );
    }

    #[test]
    fn array_expansions() {
        parse_dcfile_string(
            "
            struct ArrayExpansionsTest {
                uint8array test = [0];
                uint8array test2 = [0 * 10];
                int8array test3 = [-1 * 10];
                int8array test4 = [5 * 5, 10 * 10, -2 * 4];
                uint8array test5 = [0xf * 10];
                uint8array test6 = [\"TEST\" * 2];
            };
            ",
        );
    }

    #[test]
    #[should_panic]
    fn integer_literal_overflow() {
        parse_dcfile_string(
            "
            struct OverflowTest {
                test(uint8array = [0 * 4294967296]);
            };
            ",
        );
    }

    #[test]
    fn developer_defined_keywords() {
        parse_dcfile_string(
            "
            keyword f6f7;

            dclass DistributedDonut {
                testingField() f6f7;
            };
            ",
        );
    }

    #[test]
    fn type_declaration_optional_delimiter() {
        parse_dcfile_string(
            "
            typedef int16 test1[2]
            typedef int32 test2[2]
            typedef uint64 test3

            dclass Bogus {}
            ",
        );
    }

    #[test]
    fn handle_deprecated_bool_alias() {
        // The lexer picks up 'bool' as a data type token,
        // not an identifier, so it would be illegal grammar.
        // This test ensures we handle this as a deprecation warning.
        parse_dcfile_string(
            "
            typedef uint8 bool;
            ",
        );
    }
}
