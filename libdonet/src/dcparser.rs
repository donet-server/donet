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

/* The following suppress linting warnings, which are okay to ignore
 * as they go off in the parser grammar definitions, which we are writing
 * just as the plex crate readme says we should, so everything is okay.
 */
#![allow(
    clippy::type_complexity,
    clippy::redundant_field_names,
    clippy::ptr_arg,
    clippy::redundant_closure_call,
    clippy::enum_variant_names,
    clippy::let_unit_value
)]

use crate::dcfile::{DCFile, DCFileInterface, DCImport};
use crate::dckeyword;
use crate::dclass;
use crate::dclexer::DCToken::*;
use crate::dclexer::{DCToken, Span};
use crate::dcstruct;
use plex::parser;

/* To write the DC file elements to memory just as Panda and Astron do, I
 * initially stored the DCFile struct on static memory as mutable. This required
 * the unsafe block { } whenever it was accessed or modified, but it did not cause
 * undefined behavior ... until unit testing. Of course, it is ideal to not use
 * unsafe techniques from the beginning, so I decided to make use of Plex's features
 * by assigning types to the grammar's non-terminals and propogating the elements
 * bottom-up. (as LALR(1) parsers are 'bottom-up' parsers, where they start by
 * producing the 'edge' productions, until the parser reduces all non-terminals
 * to the root production of the language grammar.)
 *
 * Since we are propogating elements from the bottom of the parse tree and upwards,
 * the return types of non-terminals closer to the root production get bigger and bigger,
 * as they're carrying more and more of the total elements in the DC file until they are
 * all 'plugged in together' into the DC file struct once we reduce to the root production.
 *
 * Even though it may *appear* visually ugly, this is the safest, and cleanest,
 * approach to assembling the DC file structure in memory using Plex.
 */

enum TypeDeclaration {
    PythonImport(Vec<DCImport>),
    KeywordType(dckeyword::DCKeyword),
    StructType(dcstruct::DCStruct),
    SwitchType(Option<u8>),
    DClassType(dclass::DClass),
    TypedefType(Option<u8>),
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

    // root production of the grammar
    dc_file: DCFile {
        type_declarations[tds] => {
            let mut dc_file: DCFile = DCFile::new();

            for type_declaration in tds {
                match type_declaration {
                    TypeDeclaration::PythonImport(imports) => {
                        for import in imports {
                            dc_file.add_python_import(import);
                        }
                    },
                    TypeDeclaration::KeywordType(keyword) => {
                        dc_file.add_keyword(keyword);
                    },
                    TypeDeclaration::StructType(_) => {},
                    TypeDeclaration::SwitchType(_) => {},
                    TypeDeclaration::DClassType(mut dclass) => {
                        use dclass::DClassInterface;

                        let next_class_id: usize = dc_file.get_num_dclasses();
                        dclass.set_class_id(next_class_id.try_into().unwrap());

                        dc_file.add_dclass(dclass);
                    },
                    TypeDeclaration::TypedefType(_) => {},
                }
            }
            dc_file
        },
    }

    type_declarations: Vec<TypeDeclaration> {
        epsilon => vec![],
        type_declarations[tds] Semicolon => tds,
        type_declarations[mut tds] type_decl[td] => {
            tds.push(td);
            tds
        },
    }

    type_decl: TypeDeclaration {
        python_style_import[py_imports] => TypeDeclaration::PythonImport(py_imports),
        keyword_type[keyword] => TypeDeclaration::KeywordType(keyword),
        struct_type[strct] => TypeDeclaration::StructType(strct),
        switch_type => TypeDeclaration::SwitchType(None),
        distributed_class_type[dclass] => TypeDeclaration::DClassType(dclass),
        type_definition => TypeDeclaration::TypedefType(None),
    }

    // ---------- Python-style Imports ---------- //

    python_style_import: Vec<DCImport> {
        py_module[(m, ms)] dclass_import[(c, cs)] => {
            // Since more than one DCImport structure can be generated from
            // one python_style_import non-terminal, we return a vector type.
            let mut result_vec: Vec<DCImport> = vec![];

            /* NOTE: Workaround for not being able to pass Options through
             * the non-terminal parameters (due to moved values and borrow
             * checking issues (skill issues)), so we turn the Vectors
             * (which do implement the Copy trait) into Options here.
             */
            let mut mvs_opt: Option<Vec<String>> = None;
            let mut cvs_opt: Option<Vec<String>> = None;
            if !ms.is_empty() {
                mvs_opt = Some(ms);
            }
            if !cs.is_empty() {
                cvs_opt = Some(cs);
            }

            let mut class_symbols: Vec<String> = vec![c.clone()];

            // Separates "Class/AI/OV" to ["Class", "ClassAI", "ClassOV"]
            if cvs_opt.is_some() {
                for class_suffix in &cvs_opt.unwrap() {
                    class_symbols.push(c.clone() + class_suffix);
                }
            }

            // Handles e.g. "from module/AI/OV/UD import DistributedThing/AI/OV/UD"
            if mvs_opt.is_some() {
                let mut c_symbol: String = class_symbols.first().unwrap().clone();

                result_vec.push(DCImport::new(m.clone(), vec![c_symbol]));

                for (i, module_suffix) in mvs_opt.unwrap().into_iter().enumerate() {
                    let full_import: String = m.clone() + &module_suffix;

                    if (class_symbols.len() - 1) <= i {
                        c_symbol = class_symbols.last().unwrap().clone();
                    } else {
                        c_symbol = class_symbols.get(i + 1).unwrap().clone();
                    }

                    result_vec.push(DCImport::new(full_import, vec![c_symbol]));
                }
                return result_vec;
            }
            result_vec.push(DCImport::new(m, class_symbols));
            result_vec
        },
    }

    // e.g. "from views ..."
    // e.g. "from game.views.Donut/AI ..."
    py_module: (String, Vec<String>) {
        From modules[ms] slash_identifier[is] => {

            // We need to join all module identifiers into one string
            let mut modules_string: String = String::new();

            for (i, mod_) in ms.into_iter().enumerate() {
                if i != 0 {
                    modules_string.push('.');
                }
                modules_string.push_str(&mod_);
            }
            (modules_string, is)
        }
    }

    // Bundles module names in 'from' statements, e.g. "myviews.Donut".
    modules: Vec<String> {
        legal_python_module_identifiers[m] => vec![m],
        modules[mut nm] Period legal_python_module_identifiers[m] => {
            nm.push(m);
            nm
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
        DClass Identifier(id) optional_inheritance[_] OpenBraces
        optional_class_fields CloseBraces => {
            use dclass::DClassInterface;
            dclass::DClass::new(&id)
        }
    }

    optional_class_fields: () {
        epsilon => {},
        optional_class_fields Semicolon => {},
        optional_class_fields class_field Semicolon => {},
    }

    class_field: () {
        // e.g. "setPos(float64 x, float64 y, float64 z) ram broadcast"
        named_field dc_keyword_list => {},
        // e.g. "setStats : setAvatarCount, setNewAvatarCount"
        molecular_field => {},
    }

    dc_keyword_list: Vec<String> {
        epsilon => vec![],
        dc_keyword_list[mut kl] Identifier(k) => {
            kl.push(k);
            kl
        }
        dc_keyword_list[mut kl] DCKeyword(k) => {
            kl.push(k);
            kl
        }
    }

    optional_inheritance: Option<Vec<String>> {
        epsilon => None,
        Colon Identifier(parent) class_parents[mut cp] => {
            // TODO: Check if identifier is a defined class.
            cp.insert(0, parent);
            Some(cp)
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
        type_definition OpenBrackets array_range CloseBrackets => {},
    }

    // ---------- Panda DC Switch Statements ---------- //

    switch_type: () {
        Switch optional_name[_] OpenParenthesis parameter_or_atomic
        CloseParenthesis OpenBraces switch_fields CloseBraces => {
            // TODO: create new switch
        }
    }

    switch_fields: () {
        epsilon => {},
        switch_fields switch_case => {},
        switch_fields parameter Semicolon => {},
        switch_fields named_field Semicolon => {},
        switch_fields Break Semicolon => {},
    }

    switch_case: () {
        Case parameter Colon => {},
        Default Colon => {},
    }

    parameter_or_atomic: () {
        parameter => {},
        atomic_field => {},
    }

    // ---------- Molecular Field ---------- //

    molecular_field: () {
        Identifier(_) Colon atomic_field[_] atomic_fields[_] => {},
        Identifier(_) Colon parameter_field[_] parameter_fields[_] => {},
    }

    // ---------- Atomic Field ---------- //

    atomic_fields: () {
        epsilon => {},
        atomic_fields Comma atomic_field => {},
    }

    atomic_field: () {
        Identifier(_) OpenParenthesis parameters[_]
        CloseParenthesis dc_keyword_list[_] Semicolon => {},
    }

    // ---------- Method ---------- //

    method: () {
        OpenParenthesis parameters CloseParenthesis => {},
    }

    method_value: () {
        OpenParenthesis parameter_values CloseParenthesis => {},
    }

    method_as_field: () {
        Identifier(_) method => {},
    }

    nonmethod_type: () {
        nonmethod_type_no_array => {},
        #[no_reduce(OpenBrackets)] // avoids conflict with type_with_array rule
        type_with_array => {},
    }

    nonmethod_type_no_array: () {
        #[no_reduce(OpenBrackets)]
        Identifier(_) => {
            // TODO: check if it is a defined type.
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

    parameters: () {
        epsilon => {},
        #[no_reduce(Comma)] // don't reduce if we're expecting more params
        parameters parameter => {},
        parameters parameter Comma => {},
    }

    parameter: () {
        type_value => {},
        nonmethod_type => {},
        nonmethod_type Equals type_value => {},
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

    array_expansion: () {
        type_value => {},
        signed_integer[_] Star unsigned_16_bit_int[_] => {},
        DecimalLiteral(_) Star unsigned_16_bit_int[_] => {},
        HexLiteral(_) Star unsigned_16_bit_int[_] => {},
        StringT Star unsigned_16_bit_int[_] => {},
    }

    element_values: () {
        array_expansion => {},
        element_values Comma array_expansion => {},
    }

    array_value: () {
        OpenBrackets CloseBrackets => {},
        OpenBrackets element_values CloseBrackets => {},
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

    type_value: () {
        DecimalLiteral(_) => {},
        CharacterLiteral(_) => {},
        StringLiteral(_) => {},
        HexLiteral(_) => {},
        signed_integer[_] => {},
        array_value => {},
        struct_value => {},
    }

    numeric_type: () {
        numeric_type_token[_] => {},
        numeric_with_modulus[_] => {},
        numeric_with_divisor[_] => {},
        numeric_with_range[_] => {},
    }

    numeric_with_range: () {
        numeric_type_token[_] OpenParenthesis numeric_range CloseParenthesis => {},
        numeric_with_modulus[_] OpenParenthesis numeric_range CloseParenthesis => {},
        numeric_with_divisor[_] OpenParenthesis numeric_range CloseParenthesis => {},
    }

    numeric_with_divisor: () {
        numeric_type_token[_] ForwardSlash number[_] => {},
        numeric_with_modulus[_] ForwardSlash number[_] => {},
    }

    numeric_with_modulus: () {
        numeric_type_token[_] Percent number[_] => {},
    }

    numeric_range: () {
        epsilon => {},
        char_or_number => {},
        char_or_number Hyphen char_or_number => {},
    }

    array_range: () {
        epsilon => {},
        char_or_u16 => {},
        char_or_u16 Hyphen char_or_u16 => {},
    }

    // Both of these types represent a sized type (aka, array type)
    sized_type_token: DCToken {
        StringT => StringT,
        BlobT => BlobT,
        Blob32T => Blob32T,
        array_data_type[tok] => tok,
    }

    numeric_type_token: DCToken {
        CharT => CharT,
        signed_integer_type[tok] => tok,
        unsigned_integer_type[tok] => tok,
        Float32T => Float32T,
        Float64T => Float64T,
    }

    char_or_number: () {
        CharT => {},
        signed_integer[_] => {},
        number[_] => {},
    }

    signed_integer: i64 {
        Plus DecimalLiteral(dl) => dl,
        Hyphen DecimalLiteral(dl) => dl,
    }

    number: DCToken {
        DecimalLiteral(dl) => DecimalLiteral(dl),
        FloatLiteral(fl) => FloatLiteral(fl),
    }

    char_or_u16: () {
        CharT => {},
        unsigned_16_bit_int[_] => {},
    }

    /* In Panda's parser, this production is known as 'small_unsigned_integer'.
     * C++ standard for an 'unsigned int' size is at least 16 bits.
     * 16 bits for LP32 data model; ILP32, LLP64, & LP64 are 32 bits.
     */
    unsigned_16_bit_int: u16 {
        DecimalLiteral(v) => {
            match u16::try_from(v) {
                Ok(n) => { n },
                Err(err) => {
                    // Downcast failed, number must be out of range.
                    panic!("Number out of range.\n{}", err);
                },
            }
        }
    }

    signed_integer_type: DCToken {
        Int8T => Int8T,
        Int16T => Int16T,
        Int32T => Int32T,
        Int64T => Int64T,
    }

    unsigned_integer_type: DCToken {
        UInt8T => UInt8T,
        UInt16T => UInt16T,
        UInt32T => UInt32T,
        UInt64T => UInt64T,
    }

    array_data_type: DCToken {
        Int8ArrayT => Int8ArrayT,
        Int16ArrayT => Int16ArrayT,
        Int32ArrayT => Int32ArrayT,
        UInt8ArrayT => UInt8ArrayT,
        UInt16ArrayT => UInt16ArrayT,
        UInt32ArrayT => UInt32ArrayT,
        UInt32UInt8ArrayT => UInt32UInt8ArrayT,
    }

    optional_name: Option<String> {
        epsilon => None,
        Identifier(id) => Some(id)
    }

    epsilon: () {
        => {}, // alias for 'epsilon', or 'none', syntax
    }
}

pub fn parse<I: Iterator<Item = (DCToken, Span)>>(
    i: I,
) -> Result<DCFile, (Option<(DCToken, Span)>, &'static str)> {
    parse_(i)
}

#[cfg(test)]
mod unit_testing {
    use super::parse;
    use crate::dcfile::*;
    use crate::dclexer::Lexer;

    fn parse_dcfile_string(input: &str) -> DCFile {
        let lexer = Lexer::new(input).inspect(|tok| eprintln!("token: {:?}", tok));
        let dc_file: DCFile = parse(lexer).unwrap();
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
        let mut dc_file = parse_dcfile_string(dc_file);

        let expected_num_imports: usize = 10;
        let mut imports: Vec<DCImport> = vec![];
        assert_eq!(dc_file.get_num_imports(), expected_num_imports);

        for i in 0..expected_num_imports {
            imports.push(dc_file.get_python_import(i));
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
                                 int8(0-1) thisIsLiterallyABoolean;\n\
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
