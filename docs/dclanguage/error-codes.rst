..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez.

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _dcparser_error_index:

DC Error Codes Index
====================

This page lists all the error codes emitted by the DC parser.

E0100
^^^^^

.. warning::

    Due to a limitation in Plex, this is the only error code that
    is returned by the LALR parser if there is a syntax error.
    See issue #19 for more details on the upstream limitation.

``SyntaxError``

This error is emitted if there is a syntax error in the DC file.

The error message will contain the error message from the LALR
parser, and the lexical token the parser found that it could not
match to a grammar production rule. The parser error message will
list the list of lexical tokens that it was expecting.

E0200
^^^^^

``AlreadyDefined``

A type declaration with the same identifier as a previously
defined type declaration was found by the semantic analyzer.

Erroneous code example:

.. code-block:: cpp

    dclass DistributedDonut {
    };

    dclass DistributedDonut {
    };
    // error[E0200]: `DistributedDonut` is already defined

E0201
^^^^^

``NotDefined``

Erroneous code example:

.. code-block:: cpp

    struct Pantry {
        Donut donuts[]; // error[E0201]: `Donut` is not defined
    };

E0210
^^^^^

``MultipleInheritanceDisabled``

E0211
^^^^^

``DClassOverflow``

This error message should never be encountered, unless you've
been working on the largest virtual world in history.

Emitted when there are no more IDs to assign to dclass definitions.

E0212
^^^^^

``FieldOverflow``

This error message should never be encountered, unless you've
been working on the largest virtual world in history.

Emitted when there are no more IDs to assign to field definitions.

E0220
^^^^^

``RedundantViewSuffix``

Erroneous code example:

.. code-block:: cpp

    from donet import DistributedDonut/AI/AI
    // error[E0220]: redundant view suffix `AI`

E0230
^^^^^

``RedundantKeyword``

Erroneous code example:

.. code-block:: cpp

    dclass DistributedDonut {
        setPos(float32 x, float32 y) ownsend ram ram;
        // error[E0230]: redundant keyword `ram`
    };

E0240
^^^^^

``KeywordsInStructField``

Erroneous code example:

.. code-block:: cpp

    struct Donut {
        string name db;
        // error[E0240]: dc keywords are not allowed in struct fields
    };

E0250
^^^^^

``RedundantCase``

Erroneous code example:

.. code-block:: cpp

    struct Donut {
        switch (uint16) {
            case 0:
                break;
            case 0: // error[E0250]: duplicate case value
                break;
        };
    };

E0251
^^^^^

``RedundantDefault``

Erroneous code example:

.. code-block:: cpp

    struct Donut {
        switch (uint16) {
            case 0:
                break;
            case 1: // error[E0250]: duplicate case value
                break;
            default:

        };
    };

E0260
^^^^^

``MismatchedKeywords``

Erroneous code example:

.. code-block:: cpp

    dclass DistributedDonut {
        setX(uint32 x) ownsend broadcast;
        setY(uint32 y) ownsend;
        setXY : setX, setY;
        // error[E0260]: mismatched dc keywords in molecule between `setX` and `setY`
    };

E0261
^^^^^

``ExpectedAtomic``

Erroneous code example:

.. code-block:: cpp

    dclass DistributedDonut {
        uint32 setX;
        setY(uint32 y) ownsend;
        setXY : setX, setY;
        // error[E0261]: `setX` is not an atomic field
    };

E0270
^^^^^

``InvalidRange``

E0271
^^^^^

``OverlappingRange``

E0272
^^^^^

``ValueOutOfRange``

E0280
^^^^^

``InvalidDivisor``

E0281
^^^^^

``InvalidModulus``

E0290
^^^^^

``InvalidDefault``

Erroneous code example:

.. code-block:: cpp

    struct Donut {
        string name = 32; // error[E0290]: invalid default value for type
    };

E0300
^^^^^

``ExpectedStruct``

Erroneous code example:

.. code-block:: cpp

    dclass Donut {
        uint32 x;
        uint32 y;
    };

    struct Pantry {
        Donut donuts[]; // error[E0300]: `Donut` is not a struct
    };
