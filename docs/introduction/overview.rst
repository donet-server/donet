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
