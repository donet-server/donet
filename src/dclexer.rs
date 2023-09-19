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

    Identifier(String), // Letter { Letter | DecDigit }
    Keyword(String),    // "dclass" | "struct" | "keyword"

    // Operators
    Modulus,        // "%"
    Multiplication, // "*"
    Addition,       // "+"
    Subtraction,    // "-"
    Division,       // "/"

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

pub enum DCKeyword {
    RAM,       // ram
    Required,  // required
    DB,        // db
    AIRecv,    // airecv
    OwnRecv,   // ownrecv
    ClRecv,    // clrecv
    Broadcast, // broadcast
    OwnSend,   // ownsend
    ClSend,    // clsend
    Bypass,    // bypass
}

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

    r#"\'.\'"# => (
        #[allow(clippy::iter_nth_zero)]
        DCToken::CharacterLiteral(text.chars().nth(0).unwrap()),
        text
    ),
    r#"\".+\""# => (DCToken::StringLiteral(text.to_owned()), text),

    r#"char"# => (DCToken::CharType, text),
    r#"[u]?(int8|int16|int32|int64)"# => (DCToken::IntType(text.to_owned()), text),
    r#"float64"# => (DCToken::FloatType, text),
    r#"string"# => (DCToken::StringType, text),
    r#"blob"# => (DCToken::BlobType, text),

    r#"dclass|struct|keyword"# => (DCToken::Keyword(text.to_owned()), text),
    r#"[a-zA-Z_][a-zA-Z0-9_]*"# => (DCToken::Identifier(text.to_owned()), text),

    r#"\\(x[0-9a-fA-F]+|.)"# => (DCToken::EscapeCharacter(text.to_owned()), text),

    r#"%"# => (DCToken::Modulus, text),
    r#"\*"# => (DCToken::Multiplication, text),
    r#"\+"# => (DCToken::Addition, text),
    r#"-"# => (DCToken::Subtraction, text),
    r#"/"# => (DCToken::Division, text),

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
        error!("Found unexpected token: {}", text);
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

#[derive(Debug, Clone, Copy)]
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
            DCToken::Keyword(String::from("keyword")),
            DCToken::Identifier(String::from("test")),
            DCToken::Semicolon,
        ];
        lexer_test_for_target("keyword test;", target);
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
            // Binary literals
            DCToken::BinaryLiteral(String::from("0b1")),
            DCToken::BinaryLiteral(String::from("0B1")),
            DCToken::BinaryLiteral(String::from("0b0")),
            DCToken::BinaryLiteral(String::from("0b010")),
            DCToken::BinaryLiteral(String::from("0b101110")),
        ];
        lexer_test_for_target(
            "1 9 10 2010 \
            01 07 07472 \
            0xa 0xA 0Xa 0XA 0x123456789abcdef \
            0b1 0B1 0b0 0b010 0b101110",
            target,
        );
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
