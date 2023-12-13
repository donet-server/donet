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

use crate::globals::{DC_KEYWORDS, DC_VIEW_SUFFIXES};
use plex::lexer;

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub enum DCToken {
    Whitespace,       // Not a DC token; Ignores: " " | tab | carriage-return
    Comment,          // Not a DC token; Ignored. Satisfies lexer match.
    Newline,          // Not a DC token; Used by lexer iterator to keep track of line #.
    // Letter   ::= "A" ... "z"
    // DecDigit ::= "0" ... "9"
    // OctDigit ::= "0" ... "7"
    // HexDigit ::= "0" ... "9" | "A" ... "F" | "a" ... "f"
    // BinDigit ::= "0" | "1"

    // Integers
    DecimalLiteral(i64),   // ( "1" … "9" ) { DecDigit }
    OctalLiteral(String),  // "0" { OctDigit }
    HexLiteral(String),    // "0" ( "x" | "X" ) HexDigit { HexDigit }
    BinaryLiteral(String), // "0" ( "b" | "B" ) BinDigit { BinDigit }

    // IntegerLiteral ::= DecimalLiteral | OctalLiteral | HexLiteral | BinaryLiteral
    // NumberLiteral  ::= IntegerLiteral | FloatLiteral
    // decimals       ::= DecDigit { DecDigit }

    // Floats
    FloatLiteral(f64), // decimals "." [ decimals ] | "." [ decimals ]

    // Text Literals
    CharacterLiteral(char),
    StringLiteral(String),
    // nonSingleQuote  ::= <any printable character except "'" or newline>
    // nonDoubleQuote  ::= <any printable character except `"` or newline>
    EscapeCharacter(String), // "\" ( <any character> | "x" hexDigit { hexDigit } )

    // Data Types
    CharT,             // "char"
    Int8T,             // "int8"
    Int16T,            // "int16"
    Int32T,            // "int32"
    Int64T,            // "int64"
    UInt8T,            // "uint8"
    UInt16T,           // "uint16"
    UInt32T,           // "uint32"
    UInt64T,           // "uint64"
    Float64T,          // "float64"
    Int8ArrayT,        // "int8array"
    Int16ArrayT,       // "int16array"
    Int32ArrayT,       // "int32array"
    UInt8ArrayT,       // "uint8array"
    UInt16ArrayT,      // "uint16array"
    UInt32ArrayT,      // "uint32array"
    UInt32UInt8ArrayT, // "uint32uint8array"
    StringT,           // "string"
    BlobT,             // "blob"
    Blob32T,           // "blob32"

    // Keywords
    DClass,  // "dclass"
    Struct,  // "struct"
    Keyword, // "keyword"
    Typedef, // "typedef"
    From,    // "from"
    Import,  // "import"
    Switch,  // "switch"
    Case,    // "case"
    Default, // "default"
    Break,   // "break"

    // NOTE: Module names in import statements may be matched as Indentifiers, and
    // not Module tokens. We have the module token to accept Python module names with
    // a '-' hyphen character. The parser will check for either token in import statements.
    Identifier(String), // ( Letter | "_" ) { Letter | DecDigit | "_" }
    Module(String),     // ( Letter | "_" ) { Letter | DecDigit | "_" | "-" }
    DCKeyword(String),  // ( "ram" | "required" | "db" | "airecv" | "ownrecv" |
                        //   "clrecv" | "broadcast" | "ownsend" | "clsend" )
    ViewSuffix(String), // ( "AI", "OV", "UD" )

    // Operators
    Percent,      // "%"
    Star,         // "*"
    Plus,         // "+"
    Hyphen,       // "-"
    ForwardSlash, // "/"
    Period,       // "."

    // Delimiters
    OpenParenthesis,  // "("
    CloseParenthesis, // ")"
    OpenBraces,       // "{"
    CloseBraces,      // "}"
    OpenBrackets,     // "["
    CloseBrackets,    // "]"
    Comma,            // ","
    Semicolon,        // ";"
    Equals,           // "="
    Colon,            // ":"
}

lexer! {
    fn next_token(text: 'a) -> (DCToken, &'a str);

    r#"[ \t\r]+"# => (DCToken::Whitespace, text),
    // C++-style comments '// ...'
    r#"//[^\n]*"# => (DCToken::Comment, text),
    // C-style comments '/* ... */'; cannot contain '*/'
    r#"/[*](~(.*[*]/.*))[*]/"# => (DCToken::Comment, text),
    r#"\n"# => (DCToken::Newline, text),

    r#"0|([1-9][0-9]*)"# => (DCToken::DecimalLiteral(match text.parse::<i64>() {
        Ok(n) => { n },
        Err(err) => {
            panic!("dclexer: Found DecimalLiteral token, but failed to parse as i64.\n\n{}", err);
        },
    }), text),

    r#"0[0-7]+"# => (DCToken::OctalLiteral(text.to_owned()), text),
    r#"0[xX][0-9a-fA-F]+"# => (DCToken::HexLiteral(text.to_owned()), text),
    r#"0[bB][0-1]+"# => (DCToken::BinaryLiteral(text.to_owned()), text),

    r#"([0-9]?)+\.[0-9]+"# => (DCToken::FloatLiteral(match text.parse::<f64>() {
        Ok(f) => { f },
        Err(err) => {
            panic!("dclexer: Found FloatLiteral token, but failed to parse as f64.\n\n{}", err);
        }
    }), text),

    // Rust doesn't support lookahead/lookbehind regex, so for character literals
    // we match the entire ''x'' and extract the second (nth(1)) character.
    r#"'.'"# => (DCToken::CharacterLiteral(text.chars().nth(1).unwrap()), text),
    r#"\"[^\"]+\""# => (DCToken::StringLiteral(text.to_owned().replace('\"', "")), text),

    // Signed/unsigned integer data types *could* be a single token,
    // but parsing is easier if they are all individual lexical tokens.
    r#"char"# => (DCToken::CharT, text),
    r#"int8"# => (DCToken::Int8T, text),
    r#"int16"# => (DCToken::Int16T, text),
    r#"int32"# => (DCToken::Int32T, text),
    r#"int64"# => (DCToken::Int64T, text),
    r#"uint8"# => (DCToken::UInt8T, text),
    r#"uint16"# => (DCToken::UInt16T, text),
    r#"uint32"# => (DCToken::UInt32T, text),
    r#"uint64"# => (DCToken::UInt64T, text),
    r#"float64"# => (DCToken::Float64T, text),
    r#"int8array"# => (DCToken::Int8ArrayT, text),
    r#"int16array"# => (DCToken::Int16ArrayT, text),
    r#"int32array"# => (DCToken::Int32ArrayT, text),
    r#"uint8array"# => (DCToken::UInt8ArrayT, text),
    r#"uint16array"# => (DCToken::UInt16ArrayT, text),
    r#"uint32array"# => (DCToken::UInt32ArrayT, text),
    r#"uint32uint8array"# => (DCToken::UInt32UInt8ArrayT, text),
    r#"string"# => (DCToken::StringT, text),
    r#"blob"# => (DCToken::BlobT, text),
    r#"blob32"# => (DCToken::Blob32T, text),

    r#"dclass"# => (DCToken::DClass, text),
    r#"struct"# => (DCToken::Struct, text),
    r#"keyword"# => (DCToken::Keyword, text),
    r#"from"# => (DCToken::From, text),
    r#"import"# => (DCToken::Import, text),
    r#"typedef"# => (DCToken::Typedef, text),
    r#"switch"# => (DCToken::Switch, text),
    r#"case"# => (DCToken::Case, text),
    r#"default"# => (DCToken::Default, text),
    r#"break"# => (DCToken::Break, text),

    r#"[a-zA-Z_][a-zA-Z0-9_]*"# => {
        // Decide whether this is an identifier, keyword, or view suffix.
        if DC_KEYWORDS.contains(&text) {
            (DCToken::DCKeyword(text.to_owned()), text)
        } else if DC_VIEW_SUFFIXES.contains(&text) {
            (DCToken::ViewSuffix(text.to_owned()), text)
        } else {
            (DCToken::Identifier(text.to_owned()), text)
        }
    },
    // Yes, Python modules can legally have hyphens and start with a number!
    r#"[a-zA-Z0-9_][a-zA-Z0-9_\-]*"# => (DCToken::Module(text.to_owned()), text),
    r#"\\(x[0-9a-fA-F]+|.)"# => (DCToken::EscapeCharacter(text.to_owned()), text),

    r#"%"# => (DCToken::Percent, text),
    r#"\*"# => (DCToken::Star, text),
    r#"\+"# => (DCToken::Plus, text),
    r#"-"# => (DCToken::Hyphen, text),
    r#"/"# => (DCToken::ForwardSlash, text),
    r#"\."# => (DCToken::Period, text),

    r#"\("# => (DCToken::OpenParenthesis, text),
    r#"\)"# => (DCToken::CloseParenthesis, text),
    r#"\{"# => (DCToken::OpenBraces, text),
    r#"\}"# => (DCToken::CloseBraces, text),
    r#"\["# => (DCToken::OpenBrackets, text),
    r#"\]"# => (DCToken::CloseBrackets, text),
    r#"\,"# => (DCToken::Comma, text),
    r#"\;"# => (DCToken::Semicolon, text),
    r#"\="# => (DCToken::Equals, text),
    r#"\:"# => (DCToken::Colon, text),
    r#"."# => {
        panic!("dclexer: Found an unexpected character: {}", text);
    }
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
            line: 1,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    pub min: usize,
    pub max: usize,
    pub line: usize,
}

fn span_in(s: &str, t: &str, l: usize) -> Span {
    let min = s.as_ptr() as usize - t.as_ptr() as usize;
    Span {
        min,
        max: min + s.len(),
        line: l,
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (DCToken, Span);
    fn next(&mut self) -> Option<(DCToken, Span)> {
        loop {
            let tok: (DCToken, &str) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                self.remaining = new_remaining;
                tok
            } else {
                return None;
            };
            match tok {
                (DCToken::Whitespace, _) | (DCToken::Comment, _) => {
                    // These tokens are ignored by the lexer.
                    continue;
                }
                (DCToken::Newline, _) => {
                    self.line += 1;
                    continue;
                }
                (tok, span) => {
                    return Some((tok, span_in(span, self.original, self.line)));
                }
            }
        }
    }
}

#[cfg(test)]
mod unit_testing {
    use super::{DCToken, Lexer};

    // Utility for unit testing lexer. Gives the test_string to the lexer
    // and compares the lexer results with the target_tokens vector given.
    fn lexer_test_for_target(test_string: &str, target_tokens: Vec<DCToken>) {
        let lexer = Lexer::new(&test_string).inspect(|tok| eprintln!("token: {:?}", tok));
        let mut token_quota_reached: bool = false;

        for (i, (token, _span)) in lexer.enumerate() {
            if i >= (target_tokens.len() - 1) {
                token_quota_reached = true;
            }
            if i > (target_tokens.len() - 1) {
                panic!("Lexer returned more tokens than expected!");
            }
            assert_eq!(token, *target_tokens.get(i).unwrap());
        }
        if !token_quota_reached {
            panic!("Did not receive all the expected tokens!");
        }
    }

    #[test]
    fn ignored_tokens_test() {
        // Covers Whitespace, Comment (C and C++ style), and Newline
        let test_string: String = String::from(
            "// Single line comment\n\
            /* multiline comment*/\n\
            \n    \n",
        );
        let lexer = Lexer::new(&test_string).inspect(|tok| eprintln!("token: {:?}", tok));

        for (_token, _span) in lexer {
            panic!("No tokens should have been returned by the lexer!");
        }
    }

    #[test]
    fn keyword_definition_test() {
        let target: Vec<DCToken> = vec![
            DCToken::Keyword,
            DCToken::Identifier(String::from("test")),
            DCToken::Semicolon,
            // Just need to cover these two tokens for coverage.
            DCToken::DClass,
            DCToken::Struct,
        ];
        lexer_test_for_target("keyword test;\n dclass struct", target);
    }

    #[test]
    fn switch_tokens_test() {
        let target: Vec<DCToken> = vec![DCToken::Switch, DCToken::Case, DCToken::Default, DCToken::Break];
        lexer_test_for_target("switch case default break", target);
    }

    #[test]
    fn dclass_import_statement() {
        // We will not be making use of import statements, as it is
        // a client thing (both client and AI do this), but we still want
        // their DC files to pass our lexer / parser without issues.
        let target: Vec<DCToken> = vec![
            DCToken::From,
            DCToken::Module(String::from("my-views")),
            DCToken::Period,
            DCToken::Identifier(String::from("Donut")),
            DCToken::Import,
            DCToken::Identifier(String::from("DistributedDonut")),
            DCToken::ForwardSlash,
            DCToken::ViewSuffix(String::from("AI")),
            DCToken::ForwardSlash,
            DCToken::ViewSuffix(String::from("OV")),
        ];
        lexer_test_for_target("from my-views.Donut import DistributedDonut/AI/OV", target);
    }

    #[test]
    fn number_literals() {
        let target: Vec<DCToken> = vec![
            // Decimal Literals
            DCToken::DecimalLiteral(1),
            DCToken::DecimalLiteral(9),
            DCToken::DecimalLiteral(10),
            DCToken::DecimalLiteral(2010),
            // Octal Literals
            DCToken::OctalLiteral(String::from("01")),
            DCToken::OctalLiteral(String::from("07")),
            DCToken::OctalLiteral(String::from("07472")),
            // Hex Literals
            DCToken::HexLiteral(String::from("0xa")),
            DCToken::HexLiteral(String::from("0xA")),
            DCToken::HexLiteral(String::from("0Xa")),
            DCToken::HexLiteral(String::from("0XA")),
            DCToken::HexLiteral(String::from("0x123456789abcdef")),
            // Binary Literals
            DCToken::BinaryLiteral(String::from("0b1")),
            DCToken::BinaryLiteral(String::from("0B1")),
            DCToken::BinaryLiteral(String::from("0b0")),
            DCToken::BinaryLiteral(String::from("0b010")),
            DCToken::BinaryLiteral(String::from("0b101110")),
            // Float Literal
            DCToken::FloatLiteral(0.0),
            DCToken::FloatLiteral(9.0),
            DCToken::FloatLiteral(0.0),
            DCToken::FloatLiteral(0.9),
            DCToken::FloatLiteral(1.23456789),
        ];
        lexer_test_for_target(
            "1 9 10 2010 \
            01 07 07472 \
            0xa 0xA 0Xa 0XA 0x123456789abcdef \
            0b1 0B1 0b0 0b010 0b101110 \
            0.0 9.0 .0 .9 1.23456789",
            target,
        );
    }

    #[test]
    fn text_literals() {
        let target: Vec<DCToken> = vec![
            // Character Literals
            DCToken::CharacterLiteral('a'),
            DCToken::CharacterLiteral('1'),
            DCToken::CharacterLiteral('*'),
            // String Literals
            DCToken::StringLiteral(String::from("x")),
            DCToken::StringLiteral(String::from("foo")),
            DCToken::StringLiteral(String::from("*")),
            // Escape Characters
            DCToken::EscapeCharacter(String::from("\\n")),
            DCToken::EscapeCharacter(String::from("\\t")),
            DCToken::EscapeCharacter(String::from("\\xa19")),
        ];
        lexer_test_for_target(
            "'a' '1' '*' \
            \"x\" \"foo\" \"*\" \
            \\n \\t \\xa19",
            target,
        );
    }

    #[test]
    fn data_types() {
        #[rustfmt::skip]
        let target: Vec<DCToken> = vec![
            DCToken::CharT,
            // Signed / Unsigned Integers
            DCToken::Int8T, DCToken::Int16T, DCToken::Int32T, DCToken::Int64T,
            DCToken::UInt8T, DCToken::UInt16T, DCToken::UInt32T, DCToken::UInt64T,
            // Array Data Types
            DCToken::Int8ArrayT, DCToken::Int16ArrayT, DCToken::Int32ArrayT,
            DCToken::UInt8ArrayT, DCToken::UInt16ArrayT, DCToken::UInt32ArrayT,
            DCToken::UInt32UInt8ArrayT,
            // Floating Point (float64)
            DCToken::Float64T,
            // Sized Types (string / blob)
            DCToken::StringT,
            DCToken::BlobT,
            DCToken::Blob32T,
        ];
        lexer_test_for_target(
            "char \
            int8 int16 int32 int64 \
            uint8 uint16 uint32 uint64 \
            int8array int16array int32array \
            uint8array uint16array uint32array uint32uint8array \
            float64 string blob blob32",
            target,
        );
    }

    #[test]
    fn operators_and_delimiters() {
        let target: Vec<DCToken> = vec![
            // Operators
            DCToken::Percent,
            DCToken::Star,
            DCToken::Plus,
            DCToken::Hyphen,
            DCToken::ForwardSlash,
            // Delimiters
            DCToken::OpenParenthesis,
            DCToken::CloseParenthesis,
            DCToken::OpenBraces,
            DCToken::CloseBraces,
            DCToken::OpenBrackets,
            DCToken::CloseBrackets,
            DCToken::Comma,
            DCToken::Colon,
        ];
        lexer_test_for_target("%*+-/(){}[],:", target);
    }

    #[test]
    fn dc_keywords_tokens() {
        let target: Vec<DCToken> = vec![
            DCToken::DCKeyword("ram".to_string()),
            DCToken::DCKeyword("required".to_string()),
            DCToken::DCKeyword("db".to_string()),
            DCToken::DCKeyword("airecv".to_string()),
            DCToken::DCKeyword("ownrecv".to_string()),
            DCToken::DCKeyword("clrecv".to_string()),
            DCToken::DCKeyword("broadcast".to_string()),
            DCToken::DCKeyword("ownsend".to_string()),
            DCToken::DCKeyword("clsend".to_string()),
        ];
        lexer_test_for_target(
            "ram required db airecv ownrecv \
            clrecv broadcast ownsend clsend",
            target,
        );
    }

    #[test]
    #[should_panic]
    fn unexpected_token_test() {
        let test_string: String = String::from("uint8 invalid_token = \\");
        let lexer = Lexer::new(&test_string).inspect(|tok| eprintln!("token: {:?}", tok));

        for (_, (_token, _span)) in lexer.enumerate() {
            // iterate through lexer tokens until we get a panic
        }
    }

    #[test]
    fn register_newline() {
        let test_string: String = String::from("keyword\nkeyword\nkeyword");
        let lexer = Lexer::new(&test_string).inspect(|tok| eprintln!("token: {:?}", tok));

        for (i, (_, span)) in lexer.enumerate() {
            // We use one token every line, so our line # should match our index.
            if span.line != i + 1 {
                panic!("Lexer failed to register a new line!");
            }
        }
    }
}
