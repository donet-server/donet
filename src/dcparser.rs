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

use crate::dclexer::DCToken::*;
use crate::dclexer::{DCToken, Span};
use plex::parser;

#[derive(Debug)]
pub struct DCFile {
    pub statements: Vec<Expr>,
}

#[derive(Debug)]
pub struct Expr {
    pub span: Span,
    pub node: Expr_,
}

#[derive(Debug)]
pub enum Expr_ {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Var(String),
    Assign(String, Box<Expr>),
    Print(Box<Expr>),
    Literal(i64),
}

// DCFile ::= TypeDecl { TypeDecl }
// TypeDecl ::= KeywordType | StructType | ClassType
//
// Keywords
//
// KeywordType ::= "keyword" identifier
// KeywordList ::= identifier { "," identifier }
//
// StructType ::= "struct" identifier "{" Parameter ";" { Parameter ";" } "}" ";"
//
// ClassType ::= "dclass" identifier "{" { FieldDecl ";" } "}" ";"
//
// FieldDecl ::= MolecularField | AtomicField | ParameterField
//
// MolecularField ::= identifier ":" ( AtomicField | ParameterField )
//                                   { "," ( AtomicField | ParameterField ) }
// AtomicField    ::= identifier "(" Parameter { "," Parameter } ")" [ KeywordList ]
// ParameterField ::= Parameter [ KeywordList ]
//
// Parameter ::= CharParameter | IntParameter | FloatParameter | SizedParameter
//               | StructParameter | ArrayParameter
//
// CharParameter ::= charType [ identifier ] [ "=" charLiteral ]
//
// IntParameter ::= intType [ IntRange ] [ IntTransform ] [ identifier ] [ "=" IntConstant ]
// IntConstant  ::= intLiteral | "{" intLiteral IntTransform "}"
// IntTransform ::= operator intLiteral { IntTransform } | "(" IntTransform ")"
// IntRange     ::= "(" intLiteral "-" intLiteral ")"
//
// FloatParameter ::= floatType [ FloatRange ] [ FloatTransform ] [ identifier ] [ "=" FloatConstant ]
// FloatConstant  ::= numLiteral | "{" numLiteral FloatTransform "}"
// FloatTransform ::= operator numLiteral { FloatTransform } | "(" FloatTransform ")"
// FloatRange     ::= "(" floatLiteral "-" floatLiteral ")"
//
// SizedParameter ::= sizedType [ SizeConstraint ] [ identifier ] [ "=" stringLiteral ]
// SizeConstraint ::= "(" intLiteral ")"
//
// StructParameter ::= identifier [ identifier ]
//
// ArrayParameter ::= ( dataType | identifier ) [ identifier ] ArrayRange
// ArrayRange     ::= "[" [ intLiteral [ "-" intLiteral ] ] "]"

parser! {
    fn parse_(DCToken, Span);

    // combine two spans
    (a, b) {
        Span {
            min: a.min,
            max: b.max,
            line: a.line,
        }
    }

    program: DCFile {
        statements[s] => DCFile { statements: s }
    }

    statements: Vec<Expr> {
        => vec![],
        statements[mut st] assign[e] Semicolon => {
            st.push(e);
            st
        }
    }

    assign: Expr {
        Identifier(var) Equals assign[rhs] => Expr {
            span: span!(),
            node: Expr_::Assign(var, Box::new(rhs)),
        },
        term[t] => t,
    }

    term: Expr {
        term[lhs] Plus fact[rhs] => Expr {
            span: span!(),
            node: Expr_::Add(Box::new(lhs), Box::new(rhs)),
        },
        term[lhs] Hyphen fact[rhs] => Expr {
            span: span!(),
            node: Expr_::Sub(Box::new(lhs), Box::new(rhs)),
        },
        fact[x] => x
    }

    fact: Expr {
        fact[lhs] Star atom[rhs] => Expr {
            span: span!(),
            node: Expr_::Mul(Box::new(lhs), Box::new(rhs)),
        },
        fact[lhs] ForwardSlash atom[rhs] => Expr {
            span: span!(),
            node: Expr_::Div(Box::new(lhs), Box::new(rhs)),
        },
        atom[x] => x
    }

    atom: Expr {
        // round brackets to destructure tokens
        Identifier(i) => Expr {
            span: span!(),
            node: Expr_::Var(i),
        },
        DecimalLiteral(i) => Expr {
            span: span!(),
            node: Expr_::Literal(i),
        },
        OpenParenthesis assign[a] CloseParenthesis => a
    }
}

pub fn parse<I: Iterator<Item = (DCToken, Span)>>(
    i: I,
) -> Result<DCFile, (Option<(DCToken, Span)>, &'static str)> {
    parse_(i)
}
