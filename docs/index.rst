..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez <me@maxrdz.com>

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _main-page:

Donet Reference Manual
======================

.. admonition:: Copying Conditions

   The Donet documentation is released under the
   :ref:`GNU Free Documentation License <license>`.

.. warning::

   Donet is still under heavy development and **not** yet ready
   for use in a production environment.

   This documentation should give you a sense of what the final
   product will look like.

Welcome to the Donet network engine documentation. Donet is a
free and open source networking engine for massive multiplayer
online games.

The Donet project is a monorepo that houses two main
projects built with Rust_:

- **donet** - The Donet daemon source, which includes all Donet
  services.

- **donet-core** - The core utilities for the Donet daemon and
  clients, including datagram utilities and the DC language parser.
  See `docs.donet-server.org/donet_core <https://docs.donet-server.org/donet_core>`__.

Donet uses Git_ for version control and Meson_ as the build system.

.. _Rust: https://www.rust-lang.org/
.. _Git: https://git-scm.com/
.. _Meson: https://mesonbuild.com/

The manual is divided up into sections, which are listed below.
You can at any time use the sidebar on the left side to navigate
between the different sections and their contained pages.

.. toctree::
   :maxdepth: 2

   license
   conduct
   introduction/index
   gettingstarted/index
   protocol/index
   library
   dclanguage/index
   internal/index
   glossary

Get in Touch
------------
The official instant messaging channel for Donet development is
the ``#donet`` channel on the `matrix.org`_ homeserver. A link to
the channel can be found here_.

Feel free to also join our `community discord server`_!

.. _matrix.org: https://matrix.org/
.. _here: https://matrix.to/#/#donet:matrix.org
.. _community discord server: https://discord.gg/8WRwgve8uC
