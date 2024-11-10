..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez <me@maxrdz.com>

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _eventlogger:

Event Logger
============

The Event Logger service purely handles receiving log messages over
the network and writing them to disk in a JSON_-like format, along
with a timestamp of when the log event was received by the event
logger.

This service is the only service that does not communicate over TCP_
and does not use the Donet protocol, but rather it simply sends a
datagram over UDP_ with a blob that contains the log message,
expected to be encoded in MessagePack_.

These UDP packets are expected to contain one `MsgPack map`_, like
the following, represented in JSON:

.. code-block:: json

   {
      "type": "event type",
      "sender": "identity of sender",
      "x": "lorem ipsum",
   }

Where map keys after 'sender' are simply payload from your program
logic. The ``type`` and ``sender`` keys are simply convention, but
you can send any kind of map with any kind of keys and the Event
Logger will simply log it to disk.

.. note::

   While the Donet protocol is little-endian, MessagePack encoding is
   in big-endian byte order. You should keep this in mind if you are
   writing your own MsgPack implementation or inspecting raw packets.

Historically, AI processes (which run the application/game logic)
make direct connections to the event logger and send log messages
directly to it, but by design there is a control message in the
Donet protocol for sending logs to the cluster's event logger service
by having a :ref:`messagedirector` receive this control message type
and route the log message to the event logger for you. See :ref:`9014`.

.. _JSON: https://www.json.org
.. _TCP: https://en.wikipedia.org/wiki/Transmission_Control_Protocol
.. _UDP: https://en.wikipedia.org/wiki/User_Datagram_Protocol
.. _MessagePack: https://msgpack.org
.. _MsgPack map: https://github.com/msgpack/msgpack/blob/master/spec.md#map-format-family
