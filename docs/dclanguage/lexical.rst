..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez.

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _dclanguage_lexical:

Lexical Tokens
==============

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

    boolean literal = 'true' | 'false';
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
