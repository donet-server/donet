.. _dclanguage:

The DC Language Specification
=============================

Introduction
------------

This document serves as a reference manual for the DistributedClass
protocol-specification language.

The Distributed Class language is a :term:`DSL` which defines the
communication, or the :term:`network contract`, of your networked
application based on object-oriented design principles. In essence,
a distributed class specifies an "interface" for an object that is
"shared" between multiple processes or "exposed" in a remote process.
(Kevin "Kestred" Stenerson, 2013) The instances of the Distributed
Classes defined in the DC file are known as Distributed Objects.

Unlike similar domain-specific languages for network protocols, such as
`Protocol Buffers`_, the DC file inlines
security, routing, and data persistence all in one file. The DC file
itself gives you a topdown overview of the networking of your
application. (Kylie Smith, 2024)

.. _Protocol Buffers: https://protobuf.dev/

Technical Context
-----------------

The Distributed Class language specification can be described using
:term:`context-free grammar` and should be parsed by an `LALR(1)`_
parser. The file extension for DC files should be ``.dc``.

The Distributed Class language was originally developed in Disney VR
Studios for the Online Theme Park server. Since Disney released the
source for the Panda3D_ game engine under a free software license, the
original source for the DC file parser is available on GitHub_.

The original DC parser uses `GNU Bison`_ to generate its LALR(1) parser,
and Flex_ to generate its lexical analyzer. Donet uses Plex_, which
makes use of Rust `procedural macros`_ to generate the lexer and parser
at compile time.

The grammar specification for the DC language is written in
`Extended Backus-Naur Form`_ (EBNF) notation. The EBNF specification
followed in this document is `ISO/IEC 14977`_.

.. _LALR(1): https://en.wikipedia.org/wiki/LALR_parser
.. _Panda3D: https://www.panda3d.org/
.. _GitHub: https://github.com/panda3d/panda3d/tree/master/direct/src/dcparser
.. _GNU Bison: https://en.wikipedia.org/wiki/GNU_Bison
.. _Flex: https://en.wikipedia.org/wiki/Flex_(lexical_analyser_generator)
.. _Plex: https://github.com/goffrie/plex/
.. _procedural macros: https://doc.rust-lang.org/reference/procedural-macros.html
.. _Extended Backus-Naur Form: https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form
.. _ISO/IEC 14977: https://standards.iso.org/ittf/PubliclyAvailableStandards/s026153_ISO_IEC_14977_1996(E).zip

Lexical Tokens
--------------

In a *.dc* file, spacing characters such as ``0x20`` and ``\t`` as
well as newline characters like ``\n`` and ``\r`` that exist
between lexical elements are ignored.

The DC lexer also accepts comments as tokens, which it then ignores.
C++-style comments (single line) and C-style comments (multi-line)
are both recognized by the lexer.

.. code-block:: cpp

    // This is a C++-style comment.
    /* This is a C-style comment. */

These ignored tokens should be used to make the file easier
to read for the developer.

Prelude
^^^^^^^

.. code-block:: ebnf

    letter = 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z';

    decimal digit = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
    octal digit = decimal digit - ( '8' | '9' );
    hexadecimal digit = decimal digit | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F';
    binary digit = '0' | '1';
    decimals = decimal digit, { decimal digit };

Literals
^^^^^^^^

.. code-block:: ebnf

    decimal literal = ( decimal digit - '0' ), { decimal digit };
    octal literal = '0', { octal digit };
    hex literal = '0', ( 'x' | 'X' ), hexadecimal digit, { hexadecimal digit };
    binary literal = '0', ( 'b' | 'B' ), binary digit, { binary digit };
    float literal = decimals, '.', [ decimals ] | '.', [ decimals ];
    character literal = ? UTF-8 character ? ;
    string literal = '"', { ? UTF-8 character ? - '"' }, '"';
    escape character = '\', ( 'x', hexadecimal digit, { hexadecimal digit } | ? UTF-8 character ? );

Data Types
^^^^^^^^^^

.. code-block:: ebnf

    char type = 'char';
    bool type = 'bool'; (* Unique to Donet; alias for uint8 *)

    (* Signed integer types *)
    int8 type = 'int8';
    int16 type = 'int16';
    int32 type = 'int32';
    int64 type = 'int64';

    (* Unsigned integer types *)
    uint8 type = 'uint8';
    uint16 type = 'uint16';
    uint32 type = 'uint32';
    uint64 type = 'uint64';

    (* Floating point types *)
    float32 type = 'float32'; (* Introduced by Astron *)
    float64 type = 'float64';

    (* Array types *)
    int8 array type = 'int8array';
    int16 array type = 'int16array';
    int32 array type = 'int32array';
    uint8 array type = 'uint8array';
    uint16 array type = 'uint16array';
    uint32 array type = 'uint32array';
    uint32 uint8 array type = 'uint32uint8array';

    (* Sized types *)
    string type = 'string';
    blob type = 'blob';
    blob32 type = 'blob32'; (* Used in Panda *)

Keywords
^^^^^^^^

The following identifiers are reserved as keywords and may
not be used as identifiers.

.. code-block:: ebnf

    (* Keyword tokens will be referred to by their literal string
       (e.g. 'dclass') in the context-free grammar for readability. *)

    dclass = 'dclass';
    struct = 'struct';
    keyword = 'keyword';
    typedef = 'typedef';

    (* Python-style imports *)
    from = 'from';
    import = 'import';

    (* Panda switch statements *)
    switch = 'switch';
    case = 'case';
    default = 'default';
    break = 'break';

Identifiers
^^^^^^^^^^^

.. code-block:: ebnf

    identifier = ( letter | '_' ), { letter | decimal digit | '_' };
    dc keyword = 'ram' | 'required' | 'db' | 'airecv' | 'ownrecv' | 'clrecv' | 'broadcast' | 'ownsend' | 'clsend';
    view suffix = 'AI' | 'OV' | 'UD'; (* Used in python-style imports *)

Operators
^^^^^^^^^

.. code-block:: ebnf

    (* Operators will be referred to by their literal character
       (e.g. '%') in the context-free grammar for readability. *)

    percent = '%';
    star = '*';
    plus = '+';
    hyphen = '-';
    forward slash = '/';
    period = '.';

Delimiters
^^^^^^^^^^

Delimiters are used to separate other lexical tokens. Some delimiter
tokens may have additional special meaning in certain productions
in the :term:`Context-Free Grammar`.

.. code-block:: ebnf

    (* Delimiters will be referred to by their literal character
       (e.g. ';') in the context-free grammar for readability. *)

    open parenthesis = '(';
    close parenthesis = ')';
    open braces = '{';
    close braces = '}';
    open brackets = '[';
    close brackets = ']';
    comma = ',';
    semicolon = ';';
    equals = '=';
    colon = ':';

Context-Free Grammar
--------------------

DC File
^^^^^^^

The ``DC File`` is the root production of the grammar.
The root production is made up of **one or more** type
declarations. Each type declaration can optionally be
terminated with a semicolon (``;``) character.

.. code-block:: ebnf

    (* Root production of the grammar *)
    dc file = type declaration, { ';' | type declaration };

    type declaration = python import | keyword decl | typedef decl | dclass decl | struct decl;

Python-style Import
^^^^^^^^^^^^^^^^^^^

.. code-block:: ebnf

    python import = py modules, py symbols;
    py modules = 'from', identifier, { '.', identifier }, view suffixes;
    py symbols = 'import', ( '*' | ( identifier, view suffixes ) );
    view suffixes = { '/', view suffix };

Keyword
^^^^^^^

.. code-block:: ebnf

    (* Can be a historical DC keyword or a defined one. *)
    keyword decl = 'keyword', ( identifier | dc keyword );
    keyword list = { identifier | dc keyword };

Type Definition
^^^^^^^^^^^^^^^

.. code-block:: ebnf

    typedef decl = 'typedef', nonmethod type with name, [ '[', array range, ']' ];

Struct
^^^^^^

.. code-block:: ebnf

    struct decl = 'struct', identifier, '{', struct fields, '}';
    struct fields = { struct field, ';' };
    struct field = switch decl | unnamed field | named field;

Distributed Class
^^^^^^^^^^^^^^^^^

.. code-block:: ebnf

    dclass decl = 'dclass', identifier, parents, '{', class fields, '}';
    parents = ':', identifier, { ',', identifier };

    class fields = { class field, [ ';' ] };
    class field = atomic field | molecular field;

Class Fields
^^^^^^^^^^^^

.. code-block:: ebnf

    atomic field = named field, keyword list;
    molecular field = identifier, ':', identifier, { ',', identifier };

Switch
^^^^^^

.. code-block:: ebnf

    switch decl = 'switch', '(', parameter, ')', '{', switch fields, '}';
    switch fields = { switch case | ( type value, ';' ) | ( named field, ';' ) | ( 'break', ';' ) };
    switch case = ( ( 'case', type value ) | 'default' ), ':';
