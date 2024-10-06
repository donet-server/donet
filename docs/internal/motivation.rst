..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez.

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _motivation:

Project Motivation
==================

The Donet software project is practically a rewrite of Astron_.
Both Donet and Astron projects are inspired by Disney's Online
Theme Park server.

.. _Astron: https://github.com/Astron/Astron

About Astron
------------

Astron_ began development in August 29th, 2013, shortly after Disney
`announced the closure`_ of its legacy online MMOGs, which included
`Toontown Online`_, `Pirates of the Caribbean Online`_, and
`Pixie Hollow`_. These online virtual worlds would be closed down
on September 19th, 2013. Disney's `The World of Cars Online`_ was
also one of Disney's MMOGs, but was closed down earlier on February
of 2012. All four games shared one technology; An in-house networking
engine developed by Disney for developing massive virtual worlds, named
the Online Theme Park server, or 'OTP' for short.

Astron is written in C++, and uses libuv_ as its asynchronous runtime.
For the DC language parser, Astron uses a copy of Panda3D's DC parser,
which over time was also modified by the authors of Astron. Later in
the lifetime of the Astron project, the Astron DC parser and other
networking utilities were split into another project called Bamboo_,
which would also offer Python bindings for game clients written in
Python. Bamboo had initial work done, but never reached a release.

.. _announced the closure: https://web.archive.org/web/20130910102034/http://toontown.go.com/closing
.. _Toontown Online: https://en.wikipedia.org/wiki/Toontown_Online
.. _Pirates of the Caribbean Online: https://en.wikipedia.org/wiki/Pirates_of_the_Caribbean_Online
.. _Pixie Hollow: https://en.wikipedia.org/wiki/Pixie_Hollow_(video_game)
.. _The World of Cars Online: https://en.wikipedia.org/wiki/The_World_of_Cars_Online
.. _libuv: https://libuv.org/
.. _Bamboo: https://github.com/astron/bamboo

Astron's Decline in Development
-------------------------------

By 2014, Astron had matured enough for fan recreations to run the
networking for their virtual worlds. Over the span of a couple years,
the Astron project was under active development by multiple
contributors who were fans of Disney's MMOGs, and most were in teams
working towards rewriting these games from scratch for their fanbase
to play after the closure.

After 2014, the Astron authors had already discussed bigger goals
for the project, but many were never completed. As of the time of
writing this page, Astron's source code has not been updated for over
5 years, excluding README changes or trivial contributions made by
Donet authors.

Though Astron is used in production environments today for fan
rewrites of Disney games, Astron never reached a 1.0 release and
still has multiple open issues.

Donet's Mission
---------------

Donet was created as a solution to the following disadvantages of
Astron:

    - Lack of Development
    - Memory issues at runtime
    - Permissive Software License

Donet is written in Rust_, a general-purpose programming language
that is focused on performance, type safety, concurrency, and most
importantly memory safety. The asynchronous runtime used by Donet
is Tokio_, which includes multi-threaded capabilities.

"It enforces memory safety, meaning that all references point to
valid memory. It does so without a traditional garbage collector;
instead, both memory safety errors and data races are prevented by
the "borrow checker", which tracks the object lifetime of references
at compile time." (`"Rust (programming language)", Wikipedia`_)

Donet is also licensed under the `GNU Affero General Public License`_.
Unlike the Modified BSD License, also known as
`The 3-Clause BSD License`_, the AGPLv3 license is specifically
intended for software designed to be run over a network. The AGPL is
based off the `GNU General Public License version 3`_, which adds a
provision requiring that the corresponding source code of modified
versions of the software be prominently offered to all users who
interact with the software over a network.

A big part of the Donet project is libdonet, which is a rewrite of
Panda3D's DC parser in Rust. Donet's independence from Panda3D's
source code also gives it independence from Panda3D's licence, which
is also a permissive free software license.

.. _Rust: https://www.rust-lang.org/
.. _Tokio: https://tokio.rs/
.. _"Rust (programming language)", Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
.. _GNU Affero General Public License: https://en.wikipedia.org/wiki/GNU_Affero_General_Public_License
.. _The 3-Clause BSD License: https://opensource.org/license/BSD-3-Clause
.. _GNU General Public License version 3: https://www.gnu.org/licenses/gpl-3.0.en.html
