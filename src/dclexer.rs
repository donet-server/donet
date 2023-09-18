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
#[derive(Debug, Clone)]
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
    // NumberLiteral ::= IntegerLiteral | FloatLiteral

    // Floats
    FloatLiteral(f64), // decimals "." [ decimals ] | "." [ decimals ]

    // decimals ::= DecDigit { DecDigit }

    // Text Literals
    CharacterLiteral(char),
    StringLiteral(String),
    StringCharacter(String),
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

    // The following identifiers are reserved for datatypes
    // and may not be used as identifiers.
    //
    // dataType  ::= charType | intType | floatType | sizedType
    // charType  ::= "char"
    // intType   ::= "int8" | "int16" | "int32" | "int64"
    //               | "uint8" | "uint16" | "uint32" | "uint64"
    // floatType ::= "float64"
    // sizedType ::= "string" | "blob"

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

#[rustfmt::skip]
pub static RESERVED_IDENTIFIERS: [&str; 16] = [
    "charType", "intType", "floatType", "sizedType",
    "char",
    "int8", "int16", "int32", "int64",
    "uint8", "uint16", "uint32", "uint64",
    "float64",
    "string", "blob",
];

lexer! {
    fn next_token(text: 'a) -> (DCToken, &'a str);

    r#"[ \t\r\n]+"# => (DCToken::Whitespace, text),
    // C++-style comments '// ...'
    r#"//[^\n]*"# => (DCToken::Comment, text),
    // C-style comments '/* ... */'; cannot contain '*/'
    r#"/[*](~(.*[*]/.*))[*]/"# => (DCToken::Comment, text),
    r#"\n"# => (DCToken::Newline, text),

    r#"[1-9]+[0-9]"# => (DCToken::DecimalLiteral(match text.parse::<i64>() {
        Ok(n) => { n },
        Err(err) => {
            error!("Found DecimalLiteral token, but failed to parse as i64.\n\n{}", err);
            panic!("The DC lexer encountered an issue and could not continue.");
        },
    }), text),
    r#"0[0-7]+"# => (DCToken::OctalLiteral(text.to_owned()), text),
    r#"0[xX][0-9a-fA-F]+"# => (DCToken::HexLiteral(text.to_owned()), text),
    r#"0[bB][0-1]+"# => (DCToken::BinaryLiteral(text.to_owned()), text),

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
                (DCToken::Identifier(id_string), _) => {
                    // All identifier tokens cannot be equal to any reserved identifiers.
                    for reserved in RESERVED_IDENTIFIERS.into_iter() {
                        if id_string == reserved {
                            error!(
                                "Line {}: Invalid identifier: '{}' is a reserved identifier!",
                                self.line, reserved
                            );
                            panic!("The DC lexer encountered an issue and could not continue.");
                        }
                    }
                }
                (tok, span) => {
                    return Some((tok, span_in(span, self.original, self.line)));
                }
            }
        }
    }
}

// Unit Testing
//#[cfg(test)]
//mod tests {
//
//}
