..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez.

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _dclanguage_abstract:

Abstract
========

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
