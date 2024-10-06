..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez.

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _intro:

Introduction to Donet
=====================

**Donet** [1]_ ( `/ˈdoʊ.net/`_ )
is a free and open source server software, designed for powering massive
multiplayer online virtual worlds. The design of Donet focuses on solving
five critical problems: Network **culling**, data **persistence**,
**security**, **reliability**, and **scalability**.

The architecture of this project is inspired by the OTP (Online Theme Park)
server, which was developed by Disney Interactive (formerly known as Disney VR
Studios) and used from 2001 to 2013 to power massive multiplayer online games
such as Toontown Online, Pirates of the Caribbean Online, and others. Donet is
licensed under the `GNU Affero General Public License`_.

Donet is designed to distribute the workload of operating a virtual world (or
online application) by separating it's fundamental functions into different
services. In a production environment, many instances of Donet can be running in
different machines, each serving a specific role in the cluster while
communicating with each other over the Donet protocol.

.. _/ˈdoʊ.net/: https://en.wikipedia.org/wiki/Help:IPA/English
.. _GNU Affero General Public License: https://www.gnu.org/licenses/agpl-3.0.html

.. [1] An acronym for 'Distributed Object Networking'.

.. toctree::
   :maxdepth: 2
   :caption: Table of Contents

   overview
   fundamentals
   services
   resources
