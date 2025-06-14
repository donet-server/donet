..
   This file is part of the Donet reference manual.

   Copyright (c) 2024-2025 Max Rodriguez <me@maxrdz.com>

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _clientagent:

Client Agent
============

.. _1000:

CLIENTAGENT_SET_STATE (1000)
----------------------------

.. _1001:

CLIENTAGENT_SET_CLIENT_ID (1001)
--------------------------------

.. _1002:

CLIENTAGENT_SEND_DATAGRAM (1002)
--------------------------------

.. _1004:

CLIENTAGENT_EJECT (1004)
------------------------

.. code-block:: rust

    args(eject_code: u16, reason: &str)

Manually disconnect the client.

Translates to a :ref:`CLIENT_EJECT <4>` message, which is sent from
the CA to the client.

Reserved Eject Reasons
^^^^^^^^^^^^^^^^^^^^^^

These codes are reserved for the convenience of game developers, as
they may be useful for certain games:

    - **100**: Another client logged in on the same account elsewhere.
    - **122**: Login issue; the login mechanism rejected the client's credentials.
    - **126**: Administrative access violation; the client attempted to issue an administrator command, but the gameserver did not authorize it.
    - **151**: Client logged out by administrator command, not necessarily for rules violation.
    - **152**: Client logged out (and possibly banned) by a moderator for rules violation.
    - **154**: Gameserver is going down for maintenance.

.. note::

    Excerpt taken from the Astron_ project, licensed under the
    BSD-3-Clause_ license.

    Copyright © 2013 Sam "CFSworks" Edwards

    Copyright © 2013 Kevin "Kestred" Stenerson

.. _1005:

CLIENTAGENT_DROP (1005)
-----------------------

.. _1006:

CLIENTAGENT_GET_NETWORK_ADDRESS (1006)
--------------------------------------

.. _1007:

CLIENTAGENT_GET_NETWORK_ADDRESS_RESP (1007)
-------------------------------------------

.. _1010:

CLIENTAGENT_DECLARE_OBJECT (1010)
---------------------------------

.. _1011:

CLIENTAGENT_UNDECLARE_OBJECT (1011)
-----------------------------------

.. _1012:

CLIENTAGENT_ADD_SESSION_OBJECT (1012)
-------------------------------------

.. _1013:

CLIENTAGENT_REMOVE_SESSION_OBJECT (1013)
----------------------------------------

.. _1014:

CLIENTAGENT_SET_FIELDS_SENDABLE (1014)
--------------------------------------

.. _1015:

CLIENTAGENT_GET_TLVS (1015)
---------------------------

.. _1016:

CLIENTAGENT_GET_TLVS_RESP (1016)
--------------------------------

.. _1100:

CLIENTAGENT_OPEN_CHANNEL (1100)
-------------------------------

.. _1101:

CLIENTAGENT_CLOSE_CHANNEL (1101)
--------------------------------

.. _1110:

CLIENTAGENT_ADD_POST_REMOVE (1110)
----------------------------------

.. _1111:

CLIENTAGENT_CLEAR_POST_REMOVES (1111)
-------------------------------------

.. _1200:

CLIENTAGENT_ADD_INTEREST (1200)
-------------------------------

.. _1201:

CLIENTAGENT_ADD_INTEREST_MULTIPLE (1201)
----------------------------------------

.. _1203:

CLIENTAGENT_REMOVE_INTEREST (1203)
----------------------------------

.. _Astron: https://github.com/Astron/Astron
.. _BSD-3-Clause: https://raw.githubusercontent.com/Astron/Astron/master/LICENSE.md
