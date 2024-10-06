..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez.

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _overview:

Overview
========

Distributed Networking is the high-level network API of the Panda3D engine. When
a distributed object is created, all interested clients will automatically
create a copy of that object. Updates to the object will automatically propagate
to the copies. Field updates can be culled according to the rules applied to
object fields via keywords [1]_.

The distributed network is composed of several layers: The DC file (\*.dc),
is a :term:`DSL` which defines the communication, or the
:term:`network contract`, of your networked application, the Donet cluster
which handles communication between clients, Client/AI Repositories which
interact and manage the Distributed Objects, and the Distributed Objects
themselves.

Refer to the DC file documentation for more information.

.. [1] Modifies behavior of object fields, such as permissions or culling.
