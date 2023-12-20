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

use crate::dcfile::*;
use crate::dclexer::DCToken::*;
use crate::dclexer::{DCToken, Span};
use plex::parser;
use std::ops::Range;

pub static mut DC_FILE: DCFile = DCFile::new();

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
    dc_file: () {
        type_declarations => {},
    }

    type_declarations: () {
        => {},
        type_declarations type_decl => {},
    }

    type_decl: () {
        keyword_type => {},
        struct_type => {},
        switch_type => {},
        distributed_class_type => {},
        python_import => {},
        type_definition => {},
    }

    keyword_type: () {
        Keyword Identifier(_) Semicolon => {}
    }

    struct_type: () {
        Struct Identifier(_) OpenBraces struct_fields CloseBraces Semicolon => {},
    }

    struct_fields: () {
        => {},
        struct_fields struct_field Semicolon => {},
    }

    struct_field: () {
        parameter => {},
        switch_type => {},
    }

    distributed_class_type: () {
        DClass Identifier(_) optional_inheritance[_] OpenBraces
        field_declarations[_] CloseBraces Semicolon => {}
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

    type_definition: () {
        Typedef CharT Identifier(_) opt_array_range[_] Semicolon => {},
        Typedef signed_integers[_] Identifier(_) opt_array_range[_] Semicolon => {},
        Typedef unsigned_integers[_] Identifier(_) opt_array_range[_] Semicolon => {},
        Typedef array_data_types[_] Identifier(_) opt_array_range[_] Semicolon => {},
        Typedef Float64T Identifier(_) opt_array_range[_] Semicolon => {},
        Typedef StringT Identifier(_) opt_array_range[_] Semicolon => {},
        Typedef BlobT Identifier(_) opt_array_range[_] Semicolon => {},
        Typedef Blob32T Identifier(_) opt_array_range[_] Semicolon => {},
    }

    python_import: () {
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

            let mut class_symbols: Vec<String> = vec![c.clone()];

            // Separates "Class/AI/OV" to ["Class", "ClassAI", "ClassOV"]
            if cvs_opt.is_some() {
                for class_suffix in &cvs_opt.unwrap() {
                    class_symbols.push(c.clone() + class_suffix);
                }
            }

            // Handles e.g. "from module/AI/OV/UD import DistributedThing/AI/OV/UD"
            if mvs_opt.is_some() {
                let mut c_symbol: String = class_symbols.get(0).unwrap().clone();

                unsafe {
                    DC_FILE.add_python_import(DCImport::new(m.clone(), vec![c_symbol]))
                }

                for (i, module_suffix) in mvs_opt.unwrap().into_iter().enumerate() {
                    let full_import: String = m.clone() + &module_suffix;

                    if (class_symbols.len() - 1) <= i {
                        c_symbol = class_symbols.last().unwrap().clone();
                    } else {
                        c_symbol = class_symbols.get(i + 1).unwrap().clone();
                    }

                    let dc_import: DCImport = DCImport::new(full_import, vec![c_symbol]);

                    unsafe {
                        DC_FILE.add_python_import(dc_import.clone());
                    }
                }
                return;
            }
            unsafe {
                DC_FILE.add_python_import(DCImport::new(m, class_symbols));
            }
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

    // Specifically used by the Python-style DC import grammar to accept
    // **legal** python module identifiers that may lexed as other tokens.
    legal_python_module_identifiers: String {
        Identifier(id) => id,
        DCKeyword(id) => id,
        ViewSuffix(id) => id,
        CharT => "char".to_string(),
        Int8T => "int8".to_string(),
        Int16T => "int16".to_string(),
        Int32T => "int32".to_string(),
        Int64T => "int64".to_string(),
        UInt8T => "uint8".to_string(),
        UInt16T => "uint16".to_string(),
        UInt32T => "uint32".to_string(),
        UInt64T => "uint64".to_string(),
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
        => vec![],
        slash_identifier[mut si] ForwardSlash ViewSuffix(id) => {
            si.push(id);
            si
        }
    }

    switch_type: () {
        Switch optional_name[_] OpenParenthesis parameter_or_atomic
        CloseParenthesis OpenBraces switch_fields CloseBraces => {
            // TODO: create new switch
        }
    }

    switch_fields: () {
        => {},
        switch_fields switch_case => {},
        switch_fields Default Colon => {},
        switch_fields Break Semicolon => {},
        switch_fields parameter Semicolon => {},
    }

    switch_case: () {
        Case parameter Colon => {},
        Case DecimalLiteral(_) Colon => {},
        Case OctalLiteral(_) Colon => {},
        Case HexLiteral(_) Colon => {},
        Case BinaryLiteral(_) Colon => {},
        Case FloatLiteral(_) Colon => {},
        Case CharacterLiteral(_) Colon => {},
        Case StringLiteral(_) Colon => {},
    }

    // ----- Field Declaration ----- //

    field_declarations: () {
        => {},
        field_declarations[_] field_declaration[_] => {},
    }

    field_declaration: () {
        molecular_field[_] => {},
        atomic_field[_] => {},
        parameter_field[_] => {},
    }

    // ----- Molecular Field ----- //

    molecular_field: () {
        Identifier(_) Colon atomic_field[_] atomic_fields[_] Semicolon => {},
        Identifier(_) Colon parameter_field[_] parameter_fields[_] Semicolon => {},
    }

    // ----- Atomic Field ----- //

    atomic_fields: () {
        => {},
        atomic_fields Comma atomic_field => {},
    }

    atomic_field: () {
        Identifier(_) OpenParenthesis parameters[_]
        CloseParenthesis dc_keyword_list[_] Semicolon => {},
    }

    dc_keyword_list: Vec<String> {
        => vec![],
        dc_keyword_list[mut kl] DCKeyword(k) => {
            kl.push(k);
            kl
        }
    }

    parameter_or_atomic: () {
        parameter => (),
        atomic_field => (),
    }

    // ----- Parameter Fields ----- //

    parameter_fields: () {
        => {},
        parameter_fields Comma parameter_field => {},
    }

    parameter_field: () {
        parameter[_] dc_keyword_list[_] => {},
    }

    // ----- Parameters ----- //

    parameters: () {
        => {},
        #[no_reduce(Comma)] // don't reduce if we're expecting more params
        parameters parameter => {},
        parameters parameter Comma => {},
    }

    parameter: () {
        char_param => {},
        int_param => {},
        float_param => {},
        string_param => {},
        blob_param => {},
        struct_param => {},
        array_param => {},
    }

    size_constraint: Option<i64> {
        => None,
        OpenParenthesis DecimalLiteral(sc) CloseParenthesis => Some(sc)
    }

    int_range: Option<Range<i64>> {
        => None,
        OpenParenthesis DecimalLiteral(a) Hyphen DecimalLiteral(b) CloseParenthesis => Some(a .. b),
        // NOTE: The rule below is a workaround to a small 'issue' that I cannot fix.
        //       The lexer may lex the hypen and literal as a negative literal if no
        //       spaces are used in between tokens. This mitigates this issue, so its ok.
        OpenParenthesis DecimalLiteral(a)
        DecimalLiteral(negative_b) CloseParenthesis => Some(a .. negative_b.abs()),
    }

    float_range: Option<Range<f64>> {
        => None,
        OpenParenthesis FloatLiteral(a) Hyphen FloatLiteral(b) CloseParenthesis => Some(a .. b),
        OpenParenthesis FloatLiteral(a)
        FloatLiteral(negative_b) CloseParenthesis => Some(a .. negative_b.abs()),
    }

    array_range: Range<i64> {
        OpenBrackets array_range_opt[ar] CloseBrackets => ar
    }

    opt_array_range: Option<Range<i64>> {
        => None,
        array_range[ar] => Some(ar),
    }

    array_range_opt: Range<i64> {
        => 0 .. 0,
        #[no_reduce(Hyphen)] // do not reduce if lookahead is the '-' token
        DecimalLiteral(a) => a .. a,
        DecimalLiteral(min) Hyphen DecimalLiteral(max) => min .. max,
        DecimalLiteral(min) DecimalLiteral(negative_max) => min .. negative_max.abs(),
    }

    int_transform: Option<()> {
        => None,
        Percent DecimalLiteral(_) => Some(()),
        ForwardSlash DecimalLiteral(_) => Some(()),
        Star DecimalLiteral(_) => Some(()),
        Hyphen DecimalLiteral(_) => Some(()),
        Plus DecimalLiteral(_) => Some(()),
    }

    float_transform: Option<()> {
        => None,
        Percent FloatLiteral(_) => Some(()),
        ForwardSlash FloatLiteral(_) => Some(()),
        Star FloatLiteral(_) => Some(()),
        Hyphen FloatLiteral(_) => Some(()),
        Plus FloatLiteral(_) => Some(()),
    }


    /* FIXME: this is stupid and i dont fully understand this area of DC syntax.
     *        Will fix once DC file classes are completed. */
    array_to_literal_hack: () {
        OpenBrackets array_literals[_] CloseBrackets => {},
    }

    optional_name: Option<String> {
        // if epsilon found AND lookahead is Identifier, don't reduce
        // this is what holds together the parser from shitting itself.
        #[no_reduce(Identifier)]
        => None,
        Identifier(id) => Some(id)
    }

    char_default: Option<char> {
        => None,
        Equals CharacterLiteral(cl) => Some(cl),
    }

    string_default: Option<String> {
        => None,
        Equals StringLiteral(sl) => Some(sl),
    }

    binary_default: Option<String> {
        => None,
        Equals BinaryLiteral(bl) => Some(bl),
        Equals array_to_literal_hack => None,
    }

    decimal_default: Option<i64> {
        => None,
        Equals DecimalLiteral(dc) => Some(dc),
    }

    float_default: Option<f64> {
        => None,
        Equals FloatLiteral(fl) => Some(fl),
    }

    array_default: Option<Vec<i64>> {
        => None,
        Equals OpenBrackets array_literals[_] CloseBrackets => None,
    }

    array_literals: Vec<i64> {
        => vec![],
        array_literals[mut al] DecimalLiteral(dl) int_transform[_] => {
            al.push(dl);
            al
        },
        array_literals[mut al] Comma DecimalLiteral(dl) int_transform[_] => {
            al.push(dl);
            al
        },
    }

    // ----- Char Parameter ----- //
    char_param: () {
        CharT optional_name[_] opt_array_range[_] char_default[_] => {}
    }

    // ----- Integer Parameter ----- //
    int_param: () {
        signed_integers[_] int_range[_] int_transform[_]
        optional_name[_] decimal_default[_] => {},

        unsigned_integers[_] int_range[_] int_transform[_]
        optional_name[_] decimal_default[_] => {},
    }

    integer_type_with_transform: () {
        data_type[tok] int_transform[it] => {},
        data_type[tok] float_transform[ft] => {},
    }

    integer_types: DCToken {
        signed_integers[tok] => tok,
        unsigned_integers[tok] => tok,
    }

    // ----- Float Parameter ----- //
    float_param: () {
        Float64T float_range[_] float_transform[_]
        optional_name[_] float_default[_] => {},
    }

    // ----- String Parameter ----- //
    string_param: () {
        StringT size_constraint[_] optional_name[_] string_default[_] => {}
    }

    // ----- Blob Parameter ----- //
    blob_param: () {
        BlobT size_constraint[_] optional_name[_] binary_default[_] => {},
    }

    // ----- Struct Parameter ----- //
    struct_param: () {
        #[no_reduce(OpenBrackets)] // avoids ambiguity between struct & array parameters
        Identifier(_) optional_name[_] => {},
    }

    // ----- Array Parameter ----- //
    array_param: () {
        Identifier(_) optional_name[_] int_transform[_] array_range[_] array_default[_] => {},
        signed_integers[_] array_range[_] int_transform[_] optional_name[_] array_default[_] => {},
        unsigned_integers[_] array_range[_] int_transform[_] optional_name[_] array_default[_] => {},
        array_data_types[_] array_range[_] int_transform[_] optional_name[_] array_default[_] => {},
    }

    // ----- Misc/Helper Productions ----- //
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
}

pub fn parse<I: Iterator<Item = (DCToken, Span)>>(
    i: I,
) -> Result<(), (Option<(DCToken, Span)>, &'static str)> {
    parse_(i)
}

#[cfg(test)]
mod unit_testing {
    use super::{parse, DC_FILE};
    use crate::dcfile::{DCFileInterface, DCImport};
    use crate::dclexer::Lexer;

    fn parse_dcfile_string(input: &str) {
        let lexer = Lexer::new(input).inspect(|tok| eprintln!("token: {:?}", tok));
        let _: () = parse(lexer).unwrap();
        unsafe {
            eprintln!("{:#?}", DC_FILE); // pretty print parser output to stderr
        }
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
        parse_dcfile_string(dc_file);

        unsafe {
            let expected_num_imports: usize = 10;
            let mut imports: Vec<DCImport> = vec![];
            assert_eq!(DC_FILE.get_num_imports(), expected_num_imports);

            for i in 0..expected_num_imports {
                imports.push(DC_FILE.get_python_import(i));
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
    }
}
