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

use log::error;
use plex::lexer;

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub enum DCToken {
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
    CharType,           // "char"
    IntType(String),    // "int8" | "int16" | "int32" | "int64"
                        // | "uint8" | "uint16" | "uint32" | "uint64"
    FloatType,          // "float64"
    StringType,         // "string"
    BlobType,           // "blob"
    // NOTE: Astron DC specification defines both string and blob type under
    // one 'SizedType' lexical token. We match them as separate tokens so that
    // when DB tables are created for objects they can use the corresponding SQL types.

    Identifier(String), // ( Letter | "_" ) { Letter | DecDigit | "_" }
    Module(String),   // ( Letter | "_" ) { Letter | DecDigit | "_" | "-" }
    // NOTE: Module names in import statements may be matched as Indentifiers, and
    // not Module tokens. We have the module token to accept Python module names with
    // a '-' hyphen character. The parser will check for either token in import statements.

    // Keywords
    DClassType,     // "dclass"
    StructType,     // "struct"
    KeywordType,    // "keyword"
    From,           // "from"
    Import,         // "import"
    TypeDefinition, // "typedef"

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
    Whitespace,       // " " | tab | carriage-return | newline

    Comment,          // Not a DC token; Ignored. Satisfies lexer match.
    Newline,          // Not a DC token; Used by lexer iterator to keep track of line #.
}

// Plex macro to start defining our lexer regex rules.
lexer! {
    fn next_token(text: 'a) -> (DCToken, &'a str);

    r#"[ \t\r]+"# => (DCToken::Whitespace, text),
    // C++-style comments '// ...'
    r#"//[^\n]*"# => (DCToken::Comment, text),
    // C-style comments '/* ... */'; cannot contain '*/'
    r#"/[*](~(.*[*]/.*))[*]/"# => (DCToken::Comment, text),
    r#"\n"# => (DCToken::Newline, text),

    r#"[1-9]+[0-9]?+"# => (DCToken::DecimalLiteral(match text.parse::<i64>() {
        Ok(n) => { n },
        Err(err) => {
            error!("Found DecimalLiteral token, but failed to parse as i64.\n\n{}", err);
            panic!("The DC lexer encountered an issue and could not continue.");
        },
    }), text),
    r#"0[0-7]+"# => (DCToken::OctalLiteral(text.to_owned()), text),
    r#"0[xX][0-9a-fA-F]+"# => (DCToken::HexLiteral(text.to_owned()), text),
    r#"0[bB][0-1]+"# => (DCToken::BinaryLiteral(text.to_owned()), text),

    r#"([0-9]?)+\.[0-9]+"# => (DCToken::FloatLiteral(match text.parse::<f64>() {
        Ok(f) => { f },
        Err(err) => {
            error!("Found FloatLiteral token, but failed to parse as f64.\n\n{}", err);
            panic!("The DC lexer encountered an issue and could not continue.");
        }
    }), text),

    // Rust doesn't support lookahead/lookbehind regex, so for character literals
    // we match the entire ''x'' and extract the second (nth(1)) character.
    r#"'.'"# => (DCToken::CharacterLiteral(text.chars().nth(1).unwrap()), text),
    r#"\"[^\"]+\""# => (DCToken::StringLiteral(text.to_owned().replace('\"', "")), text),

    r#"char"# => (DCToken::CharType, text),
    r#"[u]?(int8|int16|int32|int64)"# => (DCToken::IntType(text.to_owned()), text),
    r#"float64"# => (DCToken::FloatType, text),
    r#"string"# => (DCToken::StringType, text),
    r#"blob"# => (DCToken::BlobType, text),

    r#"dclass"# => (DCToken::DClassType, text),
    r#"struct"# => (DCToken::StructType, text),
    r#"keyword"# => (DCToken::KeywordType, text),
    r#"from"# => (DCToken::From, text),
    r#"import"# => (DCToken::Import, text),
    r#"typedef"# => (DCToken::TypeDefinition, text),

    r#"[a-zA-Z_][a-zA-Z0-9_]*"# => (DCToken::Identifier(text.to_owned()), text),
    r#"[a-zA-Z_][a-zA-Z0-9_\-]*"# => (DCToken::Module(text.to_owned()), text),

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
        error!("Found unexpected character: {}", text);
        panic!("The DC lexer encountered an issue and could not continue.");
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
            DCToken::KeywordType,
            DCToken::Identifier(String::from("test")),
            DCToken::Semicolon,
        ];
        lexer_test_for_target("keyword test;", target);
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
            DCToken::Identifier(String::from("AI")),
            DCToken::ForwardSlash,
            DCToken::Identifier(String::from("OV")),
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
        let target: Vec<DCToken> = vec![
            DCToken::CharType,
            DCToken::IntType(String::from("int8")),
            DCToken::IntType(String::from("int16")),
            DCToken::IntType(String::from("int32")),
            DCToken::IntType(String::from("int64")),
            DCToken::IntType(String::from("uint8")),
            DCToken::IntType(String::from("uint16")),
            DCToken::IntType(String::from("uint32")),
            DCToken::IntType(String::from("uint64")),
            DCToken::FloatType,
            DCToken::StringType,
            DCToken::BlobType,
        ];
        lexer_test_for_target(
            "char \
            int8 int16 int32 int64 \
            uint8 uint16 uint32 uint64 \
            float64 string blob",
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
    #[should_panic]
    fn unexpected_token_test() {
        let test_string: String = String::from("uint8 invalid_literal = 09;");
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
