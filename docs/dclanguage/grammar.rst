..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez.

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _dclanguage_grammar:

Context-Free Grammar
====================

DC File
^^^^^^^

The ``DC File`` is the root production of the grammar.
The root production is made up of **one or more** type
declarations.

In Panda, historically, each type declaration could
optionally be terminated with a semicolon (``;``) token.
In libdonet, every type declaration (excluding Python-style
imports) requires a semicolon delimiter.

.. code-block:: ebnf

    (* Root production of the grammar *)
    dc file = type declaration, { type declaration };

    type declaration = python import | (keyword decl, ';') | (typedef decl, ';') | (dclass decl, ';') | (struct decl, ';');

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
