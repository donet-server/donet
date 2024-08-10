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

use super::lexer::DCToken::*;
use super::lexer::{DCToken, Span};
use crate::dcatomic::*;
use crate::dcfield::*;
use crate::dcfile::*;
use crate::dckeyword; // Avoid wildcard import due to conflict with DCToken variant.
use crate::dclass; // Same reason as comment above.
use crate::dcmolecular::*;
use crate::dcnumeric::*;
use crate::dcstruct;
use crate::dctype::*;
use crate::parser::ast;

use plex::parser;
use std::mem::discriminant;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

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
    dc_file: Arc<Mutex<DCFile>> {
        type_declarations[tds] => {

            // Allocates a DC File struct on the heap; Wrapped in a Mutex for mutability.
            let dc_file: Arc<Mutex<DCFile>> = Arc::new(Mutex::new(DCFile::new()));

            for type_declaration in tds {
                match type_declaration {
                    ast::TypeDeclaration::PythonImport(imports) => {
                        for import in imports {
                            dc_file.lock().unwrap().add_python_import(import);
                        }
                    },
                    ast::TypeDeclaration::KeywordType(keyword) => {
                        dc_file.lock().unwrap().add_keyword(keyword);
                    },
                    ast::TypeDeclaration::StructType(_) => {},
                    ast::TypeDeclaration::SwitchType(_) => {},
                    ast::TypeDeclaration::DClassType(mut dclass) => {
                        use dclass::DClassInterface;

                        dclass.set_dcfile(dc_file.clone());

                        let next_class_id: usize = dc_file.lock().unwrap().get_num_dclasses();
                        dclass.set_dclass_id(next_class_id.try_into().unwrap());

                        dc_file.lock().unwrap().add_dclass(dclass);
                    },
                    ast::TypeDeclaration::TypedefType(_) => {},
                }
            }
            // TODO: maybe properly handle semantic errors in the future
            assert!(dc_file.lock().unwrap().semantic_analysis().is_ok());

            dc_file
        },
    }

    type_declarations: ast::Root {
        epsilon => vec![],
        type_declarations[tds] Semicolon => tds,
        type_declarations[mut tds] type_decl[td] => {
            tds.push(td);
            tds
        },
    }

    type_decl: ast::TypeDeclaration {
        python_style_import[py_imports] => ast::TypeDeclaration::PythonImport(py_imports),
        keyword_type[keyword] => ast::TypeDeclaration::KeywordType(keyword),
        struct_type[strct] => ast::TypeDeclaration::StructType(strct),
        switch_type => ast::TypeDeclaration::SwitchType(None),
        distributed_class_type[dclass] => ast::TypeDeclaration::DClassType(dclass),
        type_definition => ast::TypeDeclaration::TypedefType(DCTypeDefinition::new()),
    }

    // ---------- Python-style Imports ---------- //

    python_style_import: Vec<ast::PythonImport> {
        py_module[(module, module_views)] dclass_import[(class, class_views)] => {
            // Since more than one `PythonImport` structure can be generated from
            // one python_style_import non-terminal, we return a vector type.
            let mut result_vec: Vec<ast::PythonImport> = vec![];

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

                result_vec.push(ast::PythonImport {
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

                    result_vec.push(ast::PythonImport {
                        python_module: full_import,
                        symbols: vec![c_symbol]
                    });
                }
                return result_vec;
            }

            result_vec.push(ast::PythonImport {
                python_module: module,
                symbols: class_symbols
            });

            result_vec
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

    /* Mandatory fix for resolving issue #16.
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

    keyword_type: dckeyword::DCKeyword {
        Keyword Identifier(id) => {
            use dckeyword::DCKeywordInterface;

            // TODO: register keyword identifier in DC file
            dckeyword::DCKeyword::new(id, None)
        },
        Keyword DCKeyword(historic) => {
            // This is already a legacy keyword.
            use dckeyword::DCKeywordInterface;

            dckeyword::DCKeyword::new(historic, None)
        }
    }

    // ---------- DC Struct ---------- //

    struct_type: dcstruct::DCStruct {
        Struct Identifier(_) OpenBraces struct_fields CloseBraces => {
            // TODO: DC Struct missing implementation
            dcstruct::DCStruct::new()
        },
    }

    struct_fields: () {
        epsilon => {},
        struct_fields struct_field Semicolon => {},
    }

    struct_field: () {
        switch_type => {},
        unnamed_field => {},
        named_field => {},
    }

    // ---------- Distributed Class ---------- //

    distributed_class_type: dclass::DClass {
        DClass Identifier(id) optional_inheritance[oi] OpenBraces
        optional_class_fields[ocf] CloseBraces => {
            use dclass::DClassInterface;

            let mut dclss: dclass::DClass = dclass::DClass::new(&id);

            // TODO: dclass parents

            // .add_class_field() sets `is_bogus_class` to false,
            // so if this for loop doesn't have 1 iteration, bogus stays true.
            for cf in ocf {
                match cf {
                    ClassField::Molecular(mut mf) => {
                        // All molecular fields in the `ocf` vector are storing
                        // their atomic field names in memory, waiting to receive
                        // their **real** atomic field smart pointers. Now that we
                        // have them in the dclass, we can give them the references.
                        for id in mf._get_atomic_names() {
                            if let Some(f_ptr) = dclss.get_field_by_name(&id) {

                                let new_ptr: Arc<Mutex<ClassField>> = f_ptr.clone();
                                let mutex_ref: &Mutex<ClassField> = new_ptr.deref();
                                let cfield: MutexGuard<'_, ClassField> = mutex_ref.lock().unwrap();

                                match *cfield {
                                    ClassField::Atomic(_) => {},
                                    // FIXME: The span! macro will return the span at the point
                                    // of the dclass declaration, and not the molecular field.
                                    // We could fix this by storing spans for each DC element,
                                    // but I don't want more unnecessary memory usage.
                                    _ => panic!("{:?}\n\nA molecular field can only \
                                    contain atomic field names.", span!()),
                                }
                            } else {
                                // TODO: uncomment panic below once fields are done.
                                // TODO: AND write unit tests for these checks
                                //panic!("{:?}\n\nMolecular field atom '{}' is \
                                //not declared.", span!(), id);
                            }
                        }
                        mf._drop_atomic_names();

                        dclss.add_class_field(ClassField::Molecular(mf))
                    },
                    _ => dclss.add_class_field(cf),
                }
            }
            dclss
        }
    }

    optional_class_fields: Vec<ClassField> {
        epsilon => vec![],
        optional_class_fields[ocf] Semicolon => ocf,
        optional_class_fields[mut ocf] class_field[cf] Semicolon => {
            ocf.push(cf);
            ocf
        },
    }

    class_field: ClassField {
        // e.g. "setPos(float64 x, float64 y, float64 z) ram broadcast" (atomic)
        // e.g. "string DcObjectType db" (plain field)
        named_field[nf] dc_keyword_list[kl] => {
            let mut f: DCAtomicField = DCAtomicField::new("", true); // TODO!
            f.set_keyword_list(kl);
            ClassField::Atomic(f)
        },
        // e.g. "setStats : setAvatarCount, setNewAvatarCount"
        molecular_field[mf] => {
            ClassField::Molecular(mf)
        },
    }

    dc_keyword_list: dckeyword::DCKeywordList {
        epsilon => dckeyword::DCKeywordList::default(),

        dc_keyword_list[mut kl] Identifier(k) => {
            use dckeyword::{DCKeywordInterface, DCKeywordListInterface};

            let _ = kl.add_keyword(dckeyword::DCKeyword::new(k, None));
            kl
        }
        dc_keyword_list[mut kl] DCKeyword(k) => {
            use dckeyword::{DCKeywordInterface, DCKeywordListInterface};

            let _ = kl.add_keyword(dckeyword::DCKeyword::new(k, None));
            kl
        }
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
        Typedef UInt8T BoolT => println!("{:?}\n\n\"typedef uint8 bool;\" is deprecated!\n\n\
        Cannot declare type alias for uint8 as 'bool', as it is a reserved identifier \
        in the DC language.\nDonet introduces the 'bool' data type, which is an alias \
        for uint8 under the hood.\n", span!()),
        type_definition OpenBrackets array_range CloseBrackets => {},
    }

    // ---------- Panda DC Switch Statements ---------- //

    switch_type: () {
        Switch optional_name[_] OpenParenthesis parameter
        CloseParenthesis OpenBraces switch_fields CloseBraces => {
            // TODO: create new switch
        }
    }

    switch_fields: () {
        epsilon => {},
        switch_fields switch_case => {},
        switch_fields type_value Semicolon => {},
        switch_fields named_field Semicolon => {},
        switch_fields Break Semicolon => {},
    }

    switch_case: () {
        Case type_value Colon => {},
        Default Colon => {},
    }

    // ---------- Molecular Field ---------- //

    molecular_field: DCMolecularField {
        Identifier(id) Colon Identifier(atom1) molecular_atom_list[mut mal] => {
            // Molecular fields require at least one atomic name.
            // They **should** require a minimum of two as suggested in Astron
            // docs and Panda source comments, but one atomic name is historically legal.
            //
            // Insert the required atomic name into the beginning of the array.
            mal.insert(0, atom1);

            // We need to pass the array of atomic names up the parse tree
            // until we have access to assembled atomic field structures.
            //
            // We do so by giving the array to the constructor, and it stores it for us,
            // so we don't have to pollute the return types of the productions here.
            DCMolecularField::new(&id, mal)
        },
    }

    molecular_atom_list: Vec<String> {
        epsilon => vec![],
        molecular_atom_list[mut mal] Comma Identifier(atomic_name) => {
            mal.push(atomic_name);
            mal
        },
    }

    // ---------- Method ---------- //

    method: ast::Method {
        OpenParenthesis parameters[ps] CloseParenthesis => ps,
    }

    method_value: () {
        OpenParenthesis parameter_values[_] CloseParenthesis => {},
    }

    method_as_field: DCAtomicField {
        Identifier(id) method[m] => DCAtomicField::new(&id, m.is_empty()),
    }

    nonmethod_type: () {
        nonmethod_type_no_array => {},
        #[no_reduce(OpenBrackets)] // avoids conflict with type_with_array rule
        type_with_array => {},
    }

    nonmethod_type_no_array: () {
        #[no_reduce(OpenBrackets)]
        Identifier(_) => {
            // TODO: check if it is a defined type, such as an alias / struct.
        },
        #[no_reduce(OpenBrackets)]
        numeric_type => {},
        #[no_reduce(OpenBrackets)]
        builtin_array_type => {},
    }

    nonmethod_type_with_name: () {
        nonmethod_type Identifier(_) => {},
    }

    // ---------- DC Fields ---------- //

    field_with_name_as_array: () {
        nonmethod_type_with_name OpenBrackets array_range CloseBrackets => {},
        field_with_name_as_array OpenBrackets array_range CloseBrackets => {},
    }

    field_with_name_and_default: () {
        nonmethod_type_with_name Equals type_value => {},
        field_with_name_as_array Equals type_value => {},
        method_as_field Equals method_value type_value => {},
    }

    named_field: () {
        method_as_field => {},
        nonmethod_type_with_name => {},
        field_with_name_as_array => {},
        field_with_name_and_default => {},
    }

    unnamed_field: () {
        nonmethod_type => {},
        nonmethod_type Equals type_value => {},
    }

    // ---------- Parameter Fields ---------- //

    parameter_fields: () {
        epsilon => {},
        parameter_fields Comma parameter_field => {},
    }

    parameter_field: () {
        parameter[_] dc_keyword_list[_] => {},
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
        nonmethod_type => ast::Parameter {
            data_type: crate::dctype::DCTypeEnum::TChar,
            identifier: String::default(),
            default_value: None,
        },
        nonmethod_type Equals type_value[value] => ast::Parameter {
            data_type: crate::dctype::DCTypeEnum::TChar,
            identifier: String::default(),
            default_value: Some(value),
        },
    }

    // ---------- DC Data Types ---------- //

    type_with_array: () {
        numeric_type OpenBrackets array_range CloseBrackets => {},
        Identifier(_) OpenBrackets array_range CloseBrackets => {
            // TODO: Check if identifier is a defined type.
        },
        builtin_array_type OpenBrackets array_range CloseBrackets => {},
        type_with_array OpenBrackets array_range CloseBrackets => {},
    }

    builtin_array_type: () {
        sized_type_token[_] => {},
        sized_type_token[_] OpenParenthesis array_range CloseParenthesis => {},
    }

    // e.g. "blob = [0 * 14]"
    array_expansion: ast::ArrayExpansion {
        type_value[tv] => (tv, 1_u32), // factor of 1 by default
        signed_integer[i] Star unsigned_32_bit_int[f] => (ast::TypeValue::I64(i), f),
        DecimalLiteral(i) Star unsigned_32_bit_int[f] => (ast::TypeValue::I64(i), f),
        HexLiteral(hs) Star unsigned_32_bit_int[f] => (ast::TypeValue::String(hs), f),
        StringLiteral(s) Star unsigned_32_bit_int[f] => (ast::TypeValue::String(s), f),
    }

    element_values: Vec<ast::ArrayExpansion> {
        array_expansion[ae] => vec![ae],
        element_values[mut ev] Comma array_expansion[ae] => {
            ev.push(ae);
            ev
        },
    }

    array_value: Vec<ast::ArrayExpansion> {
        OpenBrackets CloseBrackets => vec![],
        OpenBrackets element_values[ev] CloseBrackets => ev,
    }

    struct_value: () {
        OpenBraces field_values CloseBraces => {},
    }

    field_values: () {
        type_value => {},
        field_values Comma type_value => {},
        method_value => {},
        field_values Comma method_value => {},
    }

    parameter_values: () {
        type_value => {},
        parameter_values Comma type_value => {},
    }

    type_or_sized_value: () {
        type_value => {},
        sized_type_token[_] => {},
    }

    type_value: ast::TypeValue {
        DecimalLiteral(i) => ast::TypeValue::I64(i),
        CharacterLiteral(c) => ast::TypeValue::Char(c),
        StringLiteral(s) => ast::TypeValue::String(s),
        HexLiteral(hs) => ast::TypeValue::String(hs),
        signed_integer[i] => ast::TypeValue::I64(i),
        array_value[av] => ast::TypeValue::ArrayValue(av),
        struct_value[_] => todo!(), // TODO
    }

    numeric_type: DCNumericType {
        numeric_type_token[nt] => nt,
        numeric_with_explicit_cast[nt] => nt,
        numeric_with_modulus[nt] => nt,
        numeric_with_divisor[nt] => nt,
        numeric_with_range[nt] => nt,
    }

    // TODO: Apply range to DCNumericType struct
    numeric_with_range: DCNumericType {
        numeric_type_token[nt] OpenParenthesis numeric_range[_] CloseParenthesis => nt,
        numeric_with_explicit_cast[nt] OpenParenthesis numeric_range[_] CloseParenthesis => nt,
        numeric_with_modulus[nt] OpenParenthesis numeric_range[_] CloseParenthesis => nt,
        numeric_with_divisor[nt] OpenParenthesis numeric_range[_] CloseParenthesis => nt,
    }

    // TODO: Apply divisor to DCNumericType struct
    numeric_with_divisor: DCNumericType {
        numeric_type_token[nt] ForwardSlash number[_] => nt,
        numeric_with_explicit_cast[nt] ForwardSlash number[_] => nt,
        numeric_with_modulus[nt] ForwardSlash number[_] => nt,
    }

    numeric_with_modulus: DCNumericType {
        numeric_type_token[mut nt] Percent number[n] => {
            match n {
                DCToken::DecimalLiteral(m) => {
                    if let Err(msg) = nt.set_modulus(m as f64) {
                        panic!("{:?}\n{}", span!(), msg);
                    }
                    nt
                },
                DCToken::FloatLiteral(m) => {
                    if let Err(msg) = nt.set_modulus(m) {
                        panic!("{:?}\n{}", span!(), msg);
                    }
                    nt
                },
                _ => panic!("{:?}\nThis shouldn't be possible.", span!()),
            }
        },
        // FIXME: See if plex has a feature to use **one** block
        // for multiple production rules.
        numeric_with_explicit_cast[mut nt] Percent number[n] => {
            match n {
                DCToken::DecimalLiteral(m) => {
                    if let Err(msg) = nt.set_modulus(m as f64) {
                        panic!("{:?}\n{}", span!(), msg);
                    }
                    nt
                },
                DCToken::FloatLiteral(m) => {
                    if let Err(msg) = nt.set_modulus(m) {
                        panic!("{:?}\n{}", span!(), msg);
                    }
                    nt
                },
                _ => panic!("{:?}\nThis shouldn't be possible.", span!()),
            }
        },
    }

    // This is unique to Donet, and a new addition to the historic DC language.
    // Originally, the DC system was used with Python clients, which do not need
    // strict type annotations as Python is a dynamically typed language.
    //
    // Since we are not expecting the client to use a dynamically typed language, we need
    // to explicitly tell the client what data type to cast to when we perform these
    // operations on numeric types after they are received from the network.
    numeric_with_explicit_cast: DCNumericType {
        // Explicit casts do not use the `numeric_type_token` non-terminal, because
        // there is zero need to cast any numeric data type to a Char or Bool, since
        // this is used for types that have arithmetic operations applied, such as division.
        //
        // Also because it is 2:27 AM and its giving me a shift-reduce conflict again.
        numeric_type_token[mut nt]
        OpenParenthesis signed_integer_type[(_, dct)] CloseParenthesis => {
            if let Err(msg) =  nt.set_explicit_cast(DCTypeDefinition::new_with_type(dct)) {
                panic!("{:?}\n{}", span!(), msg);
            }
            nt
        },
        numeric_type_token[mut nt]
        OpenParenthesis unsigned_integer_type[(_, dct)] CloseParenthesis => {
            if let Err(msg) =  nt.set_explicit_cast(DCTypeDefinition::new_with_type(dct)) {
                panic!("{:?}\n{}", span!(), msg);
            }
            nt
        },
        numeric_type_token[mut nt]
        OpenParenthesis floating_point_type[(_, dct)] CloseParenthesis => {
            if let Err(msg) =  nt.set_explicit_cast(DCTypeDefinition::new_with_type(dct)) {
                panic!("{:?}\n{}", span!(), msg);
            }
            nt
        },
    }

    numeric_range: Option<DCNumericRange> {
        epsilon => None,

        char_or_number[v] => match v {
            ast::CharOrNumber::Char(c) => {
                let min_max: u64 = u64::from(c);
                Some(DCNumericRange::new_unsigned_integer_range(min_max, min_max))
            },
            ast::CharOrNumber::I64(i) => Some(DCNumericRange::new_integer_range(i, i)),
            ast::CharOrNumber::F64(f) => Some(DCNumericRange::new_floating_point_range(f, f)),
        },

        char_or_number[min] Hyphen char_or_number[max] => {
            assert!(
                discriminant(&min) == discriminant(&max),
                "{:?}\nCannot define a numeric range with a min and max of different data types!",
                span!()
            );

            match min {
                ast::CharOrNumber::Char(min_c) => {
                    let min_u64: u64 = u64::from(min_c);
                    let max_u64: u64 = match max {
                        ast::CharOrNumber::Char(max_c) => u64::from(max_c),
                        _ => panic!("This isn't possible."),
                    };
                    Some(DCNumericRange::new_unsigned_integer_range(min_u64, max_u64))
                },
                ast::CharOrNumber::I64(min_i) => Some(DCNumericRange::new_integer_range(min_i, match max {
                    ast::CharOrNumber::I64(max_i) => max_i,
                    _ => panic!("This isn't possible."),
                })),
                ast::CharOrNumber::F64(min_f) => Some(DCNumericRange::new_floating_point_range(min_f, match max {
                    ast::CharOrNumber::F64(max_f) => max_f,
                    _ => panic!("This isn't possible."),
                })),
            }
        },
    }

    array_range: Option<DCNumericRange> {
        epsilon => None,
        char_or_u16[v] => match v {
            ast::CharOrU16::Char(c) => Some(DCNumericRange::new_unsigned_integer_range(u64::from(c), u64::from(c))),
            ast::CharOrU16::U16(u) => Some(DCNumericRange::new_unsigned_integer_range(u64::from(u), u64::from(u))),
        },
        char_or_u16[min] Hyphen char_or_u16[max] => {
            let min_uint: u64;
            let max_uint: u64;

            match min {
                ast::CharOrU16::Char(c) => min_uint = u64::from(c),
                ast::CharOrU16::U16(u) => min_uint = u64::from(u),
            }
            match max {
                ast::CharOrU16::Char(c) => max_uint = u64::from(c),
                ast::CharOrU16::U16(u) => max_uint = u64::from(u),
            }
            Some(DCNumericRange::new_unsigned_integer_range(min_uint, max_uint))
        },
    }

    // Both of these types represent a sized type (aka, array type)
    sized_type_token: DCToken {
        StringT => StringT,
        BlobT => BlobT,
        Blob32T => Blob32T,
        array_data_type[(tok, _)] => tok,
    }

    numeric_type_token: DCNumericType {
        CharT => DCNumericType::new(DCTypeEnum::TChar),
        BoolT => DCNumericType::new(DCTypeEnum::TUInt8), // 'bool' is an alias for uint8
        signed_integer_type[(_, dct)] => DCNumericType::new(dct),
        unsigned_integer_type[(_, dct)] => DCNumericType::new(dct),
        floating_point_type[(_, dct)] => DCNumericType::new(dct),
    }

    char_or_number: ast::CharOrNumber {
        CharacterLiteral(c) => ast::CharOrNumber::Char(c),
        signed_integer[v] => ast::CharOrNumber::I64(v),

        number[tok] => match tok {
            DecimalLiteral(dl) => ast::CharOrNumber::I64(dl),
            FloatLiteral(fl) => ast::CharOrNumber::F64(fl),
            _ => panic!("'number' non-terminal returned an unexpected DC token!"),
        },
    }

    signed_integer: i64 {
        Plus DecimalLiteral(dl) => dl,
        Hyphen DecimalLiteral(dl) => dl * -1, // hyphen consumed by lexer, so its parsed as positive
    }

    number: DCToken {
        DecimalLiteral(dl) => DecimalLiteral(dl),
        FloatLiteral(fl) => FloatLiteral(fl),
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
                    panic!("{:?}\nNumber out of range for u32.\n{}", span!(), err);
                },
            }
        }
    }

    floating_point_type: (DCToken, DCTypeEnum) {
        Float32T => (Float32T, DCTypeEnum::TFloat32),
        Float64T => (Float64T, DCTypeEnum::TFloat64),
    }

    signed_integer_type: (DCToken, DCTypeEnum) {
        Int8T => (Int8T, DCTypeEnum::TInt8),
        Int16T => (Int16T, DCTypeEnum::TInt16),
        Int32T => (Int32T, DCTypeEnum::TInt32),
        Int64T => (Int64T, DCTypeEnum::TInt64),
    }

    unsigned_integer_type: (DCToken, DCTypeEnum) {
        UInt8T => (UInt8T, DCTypeEnum::TUInt8),
        UInt16T => (UInt16T, DCTypeEnum::TUInt16),
        UInt32T => (UInt32T, DCTypeEnum::TUInt32),
        UInt64T => (UInt64T, DCTypeEnum::TUInt64),
    }

    array_data_type: (DCToken, DCTypeEnum) {
        Int8ArrayT => (Int8ArrayT, DCTypeEnum::TArray),
        Int16ArrayT => (Int16ArrayT, DCTypeEnum::TArray),
        Int32ArrayT => (Int32ArrayT, DCTypeEnum::TArray),
        UInt8ArrayT => (UInt8ArrayT, DCTypeEnum::TArray),
        UInt16ArrayT => (UInt16ArrayT, DCTypeEnum::TArray),
        UInt32ArrayT => (UInt32ArrayT, DCTypeEnum::TArray),
        UInt32UInt8ArrayT => (UInt32UInt8ArrayT, DCTypeEnum::TArray),
    }

    optional_name: Option<String> {
        epsilon => None,
        Identifier(id) => Some(id)
    }

    epsilon: () {
        => {}, // alias for 'epsilon', a.k.a 'none' in GNU Bison
    }
}

pub fn parse<I: Iterator<Item = (DCToken, Span)>>(
    i: I,
) -> Result<Arc<Mutex<DCFile>>, (Option<(DCToken, Span)>, &'static str)> {
    parse_(i)
}

#[cfg(test)]
mod unit_testing {
    use super::parse;
    use super::{Arc, Mutex};
    use crate::dcfile::*;
    use crate::parser::ast;
    use crate::parser::lexer::Lexer;

    fn parse_dcfile_string(input: &str) -> Arc<Mutex<DCFile>> {
        let lexer = Lexer::new(input).inspect(|tok| eprintln!("token: {:?}", tok));
        let dc_file: Arc<Mutex<DCFile>> = parse(lexer).unwrap();

        eprintln!("{:#?}", dc_file); // pretty print DC element tree to stderr
        dc_file
    }

    #[test]
    fn python_module_imports() {
        let dc_file: &str = "from example_views import DistributedDonut\n\
                             from views import DistributedDonut/AI/OV\n\
                             from views/AI/OV/UD import DistributedDonut/AI/OV/UD\n\
                             from game.views.Donut/AI import DistributedDonut/AI\n\
                             from views import *\n
                             /* The next one tests handling legal python identifiers\n\
                              * that may be lexed as tokens other than Id/Module.
                              */
                             from db.char import DistributedDonut\n";

        let dc_file = parse_dcfile_string(dc_file);

        let expected_num_imports: usize = 10;
        let mut imports: Vec<ast::PythonImport> = vec![];

        assert_eq!(dc_file.lock().unwrap().get_num_imports(), expected_num_imports);

        for i in 0..expected_num_imports {
            imports.push(dc_file.lock().unwrap().get_python_import(i));
        }

        assert_eq!(imports[0].python_module, "example_views");
        assert_eq!(imports[0].symbols, vec!["DistributedDonut"]);
        assert_eq!(imports[1].python_module, "views");
        assert_eq!(
            imports[1].symbols,
            vec!["DistributedDonut", "DistributedDonutAI", "DistributedDonutOV"]
        );
        assert_eq!(imports[2].python_module, "views");
        assert_eq!(imports[2].symbols, vec!["DistributedDonut"]);
        assert_eq!(imports[3].python_module, "viewsAI");
        assert_eq!(imports[3].symbols, vec!["DistributedDonutAI"]);
        assert_eq!(imports[4].python_module, "viewsOV");
        assert_eq!(imports[4].symbols, vec!["DistributedDonutOV"]);
        assert_eq!(imports[5].python_module, "viewsUD");
        assert_eq!(imports[5].symbols, vec!["DistributedDonutUD"]);
        assert_eq!(imports[6].python_module, "game.views.Donut");
        assert_eq!(imports[6].symbols, vec!["DistributedDonut"]);
        assert_eq!(imports[7].python_module, "game.views.DonutAI");
        assert_eq!(imports[7].symbols, vec!["DistributedDonutAI"]);
        assert_eq!(imports[8].python_module, "views");
        assert_eq!(imports[8].symbols, vec!["*"]);
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
    fn sample_keyword_definitions() {
        let dc_file: &str = "keyword p2p;\n\
                             keyword monkey;\n\
                             // rUDP to donet one day?
                             keyword unreliable;\n";
        parse_dcfile_string(dc_file);
    }

    #[test]
    fn sample_struct_declarations() {
        let dc_file: &str = "struct GiftItem {\n\
                                 blob Item;\n\
                                 string giftTag;\n\
                             };\n\
                             struct activity {\n\
                                string activityName;\n\
                                uint8 activityId;\n\
                             };\n\
                             struct party {\n\
                                activity activities[];\n\
                                uint8 status;\n\
                             };\n";
        parse_dcfile_string(dc_file);
    }

    #[test]
    fn sample_distributed_class() {
        let dc_file: &str = "dclass WelcomeValleyManager : DistributedObject {\n\
                                 clientSetZone(uint32) airecv clsend;\n\
                                 requestZoneIdMessage(uint32, uint16) airecv clsend;\n\
                                 requestZoneIdResponse(uint32, uint16);\n\
                             };\n\
                             dclass ToontownDistrictStats {\n\
                                 settoontownDistrictId(uint32) broadcast required ram;\n\
                                 setAvatarCount(uint32) broadcast required ram;\n\
                                 setNewAvatarCount(uint32) broadcast required ram;\n\
                                 setStats : setAvatarCount, setNewAvatarCount;\n\
                             };\n\
                             dclass DistributedChild : DistributedParent, DistributedP2 {\n\
                             };\n";
        parse_dcfile_string(dc_file);
    }

    #[test]
    fn sample_switch_type() {
        let dc_file: &str = "struct BuffData {\n\
                               switch (uint16) {\n\
                                 case 0:\n\
                                   break;\n\
                                 case 1:\n\
                                   uint8 val1;\n\
                                   break;\n\
                                 case 2:\n\
                                   uint8 val1;\n\
                                   uint8 val2;\n\
                                   break;\n\
                                 case 3:\n\
                                   uint8 val1;\n\
                                   break;\n\
                                 case 4:\n\
                                   int16/100 val1;\n\
                                   break;\n\
                               };\n\
                             };\n";
        parse_dcfile_string(dc_file);
    }

    #[test]
    fn test_method_data_types() {
        let dc_file: &str = "struct MethodDataTypesTest {\n\
                                 Char character;\n\
                                 blob Item;\n\
                                 blob32 pandaOnlyToken;\n\
                                 float32 astronOnlyToken;\n\
                                 string giftTag;\n\
                                 int32(0-990999) testMethodValue;\n\
                                 int8(-1-1) testNegativeValues;\n\
                                 int8(-5--99) testNegativeValuesPartTwo;\n\
                                 int8(+0-+9) plusForPositiveForSomeReason;\n\
                                 int8array arrayDataTypeTest;
                                 int16array anotherArray;
                                 int32array evenMoreComplexArray;
                                 uint8array byteArray;
                                 uint16array unsignedIntegerArray;
                                 uint32array unsignedLongArray;
                                 uint32uint8array thisWeirdPandaArrayType;
                             };\n";
        parse_dcfile_string(dc_file);
    }

    #[test]
    fn test_value_transforms() {
        let dc_file: &str = "struct TransformedTypesTest {\n\
                                 int32%360 angle;\n\
                                 int32%360/1000 floatingPointAngle;\n\
                                 int32/1000 efficientFloatIn32Bits;\n\
                                 float32 waitIsntAstronsFloat32TheSame;\n\
                                 int16(float32)%360/10 forTheStaticallyTypedLanguages;\n\
                                 int8(0-1) thisIsLiterallyABoolean;\n\
                                 bool thisIsLiterallyJustAn8BitInt;\n\
                                 uint16/1000(0-1) youCanStackThemToo;\n\
                                 int64/10000(+50-+999) [] thisIsValid;\n\
                                 int8%10(0-10) anotherOne;\n\
                                 int16%100/10(-80-+100) lastTest;\n\
                             };\n";
        parse_dcfile_string(dc_file);
    }

    #[test]
    fn developer_defined_keywords() {
        let dc_file: &str = "keyword f6f7;\n\
                             dclass DistributedDonut {\n\
                                 testingField() f6f7;\n\
                             };\n";
        parse_dcfile_string(dc_file);
    }

    #[test]
    fn type_declaration_optional_delimiter() {
        let dc_file: &str = "typedef int16 test1[2];\n\
                             typedef int32 test2[2]\n\
                             typedef int64 test3;";
        parse_dcfile_string(dc_file);
    }
}
