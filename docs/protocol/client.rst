..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez.

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _client:

Client Protocol
===============

.. _1:

CLIENT_HELLO (1)
^^^^^^^^^^^^^^^^

.. code-block:: rust

    args(dc_hash: u32, version: &str)

This is the first message a client may send. The ``dc_hash`` is a
**32-bit** hash value calculated from all fields/classes listed in
the client's DC file. The version is an app/game-specific string
that developers should change whenever they release a new client
build. Both values are compared to the Client Agent's DC file
hash and configured version string to ensure that the client is
fully up-to-date. If the client is not up-to-date, it will be
disconnected with a ``ClientEject``. If the client is up-to-date,
the gameserver will send a ``ClientHelloResp`` to inform the
client that it may proceed with its normal logic flow.

.. note::

    Excerpt taken from the Astron_ project, licensed under the
    BSD-3-Clause_ license.

    Copyright © 2013 Sam "CFSworks" Edwards

    Copyright © 2013 Kevin "Kestred" Stenerson

.. _2:

CLIENT_HELLO_RESP (2)
^^^^^^^^^^^^^^^^^^^^^

This is sent by the Client Agent to the client when the client's
``ClientHello`` is accepted. This message contains no arguments.

.. note::

    Excerpt taken from the Astron_ project, licensed under the
    BSD-3-Clause_ license.

    Copyright © 2013 Sam "CFSworks" Edwards

    Copyright © 2013 Kevin "Kestred" Stenerson

.. _3:

CLIENT_DISCONNECT (3)
^^^^^^^^^^^^^^^^^^^^^

Sent by the client when it's closing the connection.
This message contains no arguments.

.. _4:

CLIENT_EJECT (4)
^^^^^^^^^^^^^^^^

.. code-block:: rust

    args(error_code: u16, reason: &str)

This is sent by the Client Agent to the client when the client is
being disconnected. The ``error_code`` and ``reason`` arguments
provide some explanation as to why the client is being dropped
from the game.

.. note::

    Excerpt taken from the Astron_ project, licensed under the
    BSD-3-Clause_ license.

    Copyright © 2013 Sam "CFSworks" Edwards

    Copyright © 2013 Kevin "Kestred" Stenerson

.. _5:

CLIENT_HEARTBEAT (5)
^^^^^^^^^^^^^^^^^^^^

The client should send this message on a regular interval.
If the Client Agent does not receive a ``ClientHeartbeat`` for a
certain (configurable) amount of time, it will assume that the
client has crashed and disconnect the client.
This message contains no arguments.

.. note::

    Excerpt taken from the Astron_ project, licensed under the
    BSD-3-Clause_ license.

    Copyright © 2013 Sam "CFSworks" Edwards

    Copyright © 2013 Kevin "Kestred" Stenerson

.. _142:

CLIENT_ENTER_OBJECT_REQUIRED (142)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _143:

CLIENT_ENTER_OBJECT_REQUIRED_OTHER (143)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _172:

CLIENT_ENTER_OBJECT_REQUIRED_OWNER (172)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _173:

CLIENT_ENTER_OBJECT_REQUIRED_OTHER_OWNER (173)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _120:

CLIENT_OBJECT_SET_FIELD (120)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _121:

CLIENT_OBJECT_SET_FIELDS (121)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _132:

CLIENT_OBJECT_LEAVING (132)
^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _140:

CLIENT_OBJECT_LOCATION (140)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _200:

CLIENT_ADD_INTEREST (200)
^^^^^^^^^^^^^^^^^^^^^^^^^

.. _201:

CLIENT_ADD_INTEREST_MULTIPLE (201)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _203:

CLIENT_REMOVE_INTEREST (203)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _204:

CLIENT_DONE_INTEREST_RESP (204)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. _Astron: https://github.com/Astron/Astron
.. _BSD-3-Clause: https://raw.githubusercontent.com/Astron/Astron/master/LICENSE.md
