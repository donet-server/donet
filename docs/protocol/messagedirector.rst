..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez <me@maxrdz.com>

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _messagedirector:

Message Director
================

The Message Director service is based on the `Publish-subscribe pattern`_
(also known as 'PubSub' for short.) This service is at the core of
every Donet cluster, as it is required for enabling communication
across all services in the cluster. It receives Donet protocol
messages, reads the sender/recipient channels in the packet header,
and routes that message accordingly to all subscribers of the given
channel(s).

.. _Publish-subscribe pattern: https://en.wikipedia.org/wiki/Publish%E2%80%93subscribe_pattern

For a Message Director service node to subscribe to a channel or
channel range, it **must** send a control message to its **upstream**
message director requesting to be subscribed. This only does not
apply to the **master** message director. A **master** message
director is an MD service that does not have an upstream connection,
and only has one or more downstream MDs connected to it as
subscribers. There can only be **one** master message director in a
Donet cluster.

As this service is based on the `Publish-subscribe pattern`_, all
messages are only routed **downlink**, to the subscribers which
explicitly requested to be subscribed to a channel or channel range.
However, **uplink** messages are sent unsolicited.

**Control messages** have the following properties:

   - They must have only one recipient channel: Channel **1**.
   - They must **omit** the sender field. This is because the
     sender is speculated to be the subscriber on the other end
     of the TCP connection.

The remainder of this page is the documentation of all control
messages in the Donet protocol:

.. _9000:

CONTROL_ADD_CHANNEL (9000)
--------------------------

.. code-block:: rust

   args(channel: u64)

.. _9001:

CONTROL_REMOVE_CHANNEL (9001)
-----------------------------

.. code-block:: rust

   args(channel: u64)

These messages allow a downstream Message Director to subscribe or
unsubscribe from a channel. The argument is the channel to be added
or removed from the subscriptions.

.. _9002:

CONTROL_ADD_RANGE (9002)
------------------------

.. code-block:: rust

   args(min_channel: u64, max_channel: u64)

Adds a range of channels. The given range is inclusive.

.. _9003:

CONTROL_REMOVE_RANGE (9003)
---------------------------

.. code-block:: rust

   args(min_channel: u64, max_channel: u64)

Removes a range of channels. The given range is inclusive.

.. _9010:

CONTROL_ADD_POST_REMOVE (9010)
------------------------------

.. code-block:: rust

   args(sender: u64, datagram: blob)

.. _9011:

CONTROL_CLEAR_POST_REMOVES (9011)
---------------------------------

.. code-block:: rust

   args(sender: u64)

.. note::

   The following is an excerpt taken from the Astron_ project,
   licensed under the BSD-3-Clause_ license.

      Copyright © 2013 Sam "CFSworks" Edwards

      Copyright © 2013 Kevin "Kestred" Stenerson

Often, Message Directors may be unexpectedly disconnected from one
another, or a Message Director may crash while under normal operation
without the chance to clean up. These control messages allow a
downstream MD to schedule messages on the upstream MD to be sent in
the event of an unexpected disconnect.

The sender is the channel (typically representing the participant who
sends the message) that the post removes should be tied to. This
field is only used to be able to clear a bundle of post removes for a
particular sender. Unlike other messages, post removes MUST NOT be
sent by Roles or AIs with a feigned sender -- the post remove is only
sent when the participant that sent it disconnects.

The second argument to CONTROL_ADD_POST_REMOVE is a blob; the blob
contains a message, minus the length tag (since the blob already
includes a length tag of its own, this would be redundant
information). CONTROL_CLEAR_POST_REMOVE is used to reset all of the
on-disconnect messages. This may be used prior to a MD's clean
shutdown, if it doesn't wish the unexpected-disconnect messages to
be processed.

.. _9012:

CONTROL_SET_CON_NAME (9012)
---------------------------

.. code-block:: rust

   args(name: &str)

.. _9013:

CONTROL_SET_CON_URL (9013)
--------------------------

.. code-block:: rust

   args(url: &str)

.. note::

   The following is an excerpt taken from the Astron_ project,
   licensed under the BSD-3-Clause_ license.

      Copyright © 2013 Sam "CFSworks" Edwards

      Copyright © 2013 Kevin "Kestred" Stenerson

As every Astron daemon may include a webserver with debug
information, it is often helpful to understand the purpose of
incoming MD connections. A downstream MD may be configured with a
specific name, and it may wish to inform the upstream MD what its
name and webserver URL are. These control messages allow the
downstream MD to communicate this information.

.. _9014:

CONTROL_LOG_MESSAGE (9014)
---------------------------

.. code-block:: rust

   args(msgpack_datagram: blob)

.. note::

   The following is an excerpt taken from the Astron_ project,
   licensed under the BSD-3-Clause_ license.

      Copyright © 2013 Sam "CFSworks" Edwards

      Copyright © 2013 Kevin "Kestred" Stenerson

In production layouts, it may be useful for AIs to log messages to
the eventlogger infrastructure (preferably a fluentd instance)
without needing to have redundant configuration on the AI servers,
which could come out of sync. Using this message, the MD will simply
route the message argument to the configured eventlogger.

.. _Astron: https://github.com/Astron/Astron
.. _BSD-3-Clause: https://raw.githubusercontent.com/Astron/Astron/master/LICENSE.md
