<img src="../logo/donet_banner.png" align="right" width="30%"/>

# 03 - The DC Language Specification

### Introduction

This document serves as a reference manual for the DistributedClass
protocol-specification language.

The Distributed Class language is a
[DSL](https://en.wikipedia.org/wiki/Domain-specific_language) which
defines the communication, or the
[network contract](https://en.wikipedia.org/wiki/Design_by_contract),
of your networked application based on object-oriented design principles.
In essence, a distributed class specifies an "interface" for an object
that is "shared" between multiple processes or "exposed" in a remote
process. (Kevin "Kestred" Stenerson, 2013) The instances of the Distributed
Classes defined in the DC file are known as Distributed Objects.

Unlike similar domain-specific languages for network protocols, such as
[Protocol Buffers](https://protobuf.dev/), the DC file inlines
security, routing, and data persistence all in one file. The DC file
itself gives you a topdown overview of the networking of your
application. (Kylie Smith, 2024)

### Technical Context

The Distributed Class language specification can be described using
[context-free grammar](https://en.wikipedia.org/wiki/Context-free_grammar)
and should be parsed by an
[LALR(1)](https://en.wikipedia.org/wiki/LALR_parser) parser.

The Distributed Class language was originally developed in Disney VR
Studios for the Online Theme Park server. Since Disney released the
source for the [Panda3D](https://www.panda3d.org/) game engine
under a free software license, the original source for the DC file
parser is available on
[GitHub](https://github.com/panda3d/panda3d/tree/master/direct/src/dcparser).

The original DC parser uses
[GNU Bison](https://en.wikipedia.org/wiki/GNU_Bison)
to generate its LALR(1) parser, and
[Flex](https://en.wikipedia.org/wiki/Flex_(lexical_analyser_generator))
to generate its lexical analyzer. Donet uses
[Plex](https://github.com/goffrie/plex/), which makes use of Rust
[procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html)
to generate the lexer and parser at compile time.

The grammar specification for the DC language is written in
[Extended Backus-Naur Form](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form)
(EBNF) notation. The EBNF specification used is
[ISO/IEC 14977](https://standards.iso.org/ittf/PubliclyAvailableStandards/s026153_ISO_IEC_14977_1996(E).zip).

### Lexical Tokens

```ebnf
(* Prelude *)
letter = 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z';

decimal digit = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
octal digit = decimal digit - ( '8' | '9' );
hexadecimal digit = decimal digit | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F';
binary digit = '0' | '1';
decimals = decimal digit, { decimal digit };

(* Literals *)
decimal literal = ( decimal digit - '0' ), { decimal digit };
octal literal = '0', { octal digit };
hex literal = '0', ( 'x' | 'X' ), hexadecimal digit, { hexadecimal digit };
binary literal = '0', ( 'b' | 'B' ), binary digit, { binary digit };
float literal = decimals, '.', [ decimals ] | '.', [ decimals ];
character literal = ? UTF-8 character ? ;
string literal = '"', { ? UTF-8 character ? - '"' }, '"';
escape character = '\', ( 'x', hexadecimal digit, { hexadecimal digit } | ? UTF-8 character ? );

(* Data Types *)
char type = 'char';
bool type = 'bool'; (* Unique to Donet; alias for uint8 *)
int8 type = 'int8';
int16 type = 'int16';
int32 type = 'int32';
int64 type = 'int64';
uint8 type = 'uint8';
uint16 type = 'uint16';
uint32 type = 'uint32';
uint64 type = 'uint64';
float32 type = 'float32'; (* Introduced by Astron *)
float64 type = 'float64';
int8 array type = 'int8array';
int16 array type = 'int16array';
int32 array type = 'int32array';
uint8 array type = 'uint8array';
uint16 array type = 'uint16array';
uint32 array type = 'uint32array';
uint32 uint8 array type = 'uint32uint8array';
string type = 'string';
blob type = 'blob';
blob32 type = 'blob32'; (* Used in Panda *)

(* Keywords *)
dclass = 'dclass';
struct = 'struct';
keyword = 'keyword';
typedef = 'typedef';
from = 'from';
import = 'import';
switch = 'switch'; (* Used in Panda *)
case = 'case';
default = 'default';
break = 'break';

(* Identifiers *)
identifier = ( letter | '_' ), { letter | decimal digit | '_' }
dc keyword = 'ram' | 'required' | 'db' | 'airecv' | 'ownrecv' | 'clrecv' | 'broadcast' | 'ownsend' | 'clsend';
view suffix = 'AI' | 'OV' | 'UD'; (* Used in imports *)
```

### Context-Free Grammar

```ebnf
(* Root production of the grammar *)
dc file = type declaration, { ';' | type declaration };

type declaration = dc import | keyword decl | struct decl | dclass decl | typedef decl;

(* DC Import *)
dc import = from, identifier, { '.', identifier }, view suffixes, import, ( '*' | ( identifier, view suffixes ) );
view suffixes = { '/', view suffix };

(* Keyword *)
keyword decl = keyword, ( identifier | dc keyword );

(* Struct *)
struct decl = struct, identifier, '{', { struct field, ';' }, '}';
struct field = switch decl | unnamed field | named field;

(* TODO! *)
```
