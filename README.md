# donet

![Pipeline Status](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fgit.maxrdz.com%2Fapi%2Fv1%2Frepos%2Fdonet%2Fdonet%2Fstatuses%2Fmaster%3Fsort%3Dleastindex%26limit%3D1&query=%24[%3A1].status&style=flat&label=pipeline&color=white) ![Code Coverage](https://codecov.io/gh/donet-server/donet/branch/master/graph/badge.svg) ![Matrix Channel](https://img.shields.io/matrix/donet:maxrdz.com?color=blue&label=%23donet%3Amaxrdz.com&logo=matrix)

<img src="logo/donet_banner.png" alt="Donet logo artwork by honeymatsu." align="right" width="40%"/>

Donet is a free and open source network engine designed after the Distributed
Networking protocol,  as defined in the high-level networking API of the
[Panda3D](https://panda3d.org) game engine, which was originally developed by
Disney Interactive (*formerly known as Disney VR Studios*) to connect with
their in-house server technology, the OTP (*Online Theme Park*) server, which
was used to power their massive multiplayer online games, such as Toontown
Online and Pirates of the Caribbean Online, from 2001 to 2013.

## Getting Started

The Donet repository is a monorepo that houses three main Rust projects:
- **donet** - The Donet daemon source, which includes all the Donet services.
See [donet-server.org](https://www.donet-server.org).
- **donet-core** - The core utilities for Donet services, including datagram
utilities and the DC language parser. See
[docs.donet-server.org](https://docs.donet-server.org/master/library).
- **donet-wcp** - A cluster-internal client with a graphical user interface
for interacting with Distributed Objects at runtime, or updating fields of
Distributed Objects that are in the DBSS.

Please read the
[introduction to Donet](https://docs.donet-server.org/master/introduction)
for an overview of the project and how the engine works.

Before starting your own contribution to Donet, please read over the
[Contributing Guidelines](https://docs.donet-server.org/master/internal/guidelines).

## Building

The build instructions are available at
[docs.donet-server.org](https://docs.donet-server.org/master/gettingstarted/building-linux).

## Documentation
The Donet project documentation is available at
[docs.donet-server.org](https://docs.donet-server.org).

## Communication

The address of the official Matrix channel for Donet development is
[#donet:maxrdz.com](https://matrix.to/#/#donet:maxrdz.com).

We also have a Discord community server, which you can join [here!](https://discord.gg/8WRwgve8uC)

## Copyright and License

Copyright © 2023-2026 Donet Authors

"Donet" can be found at https://git.maxrdz.com/donet/donet

"Donet" is distributed under the terms of the GNU Affero General Public
License, either version 3.0 or, at your option, any later
version WITHOUT ANY WARRANTY. You can read the full copy of
the software license in the [COPYING](./COPYING) file.

Donet logo artwork created by [honeymatsu](https://honeymatsu.carrd.co/). 🍩

Older revisions of the Donet logo created and designed by
[Karla Valeria Rodriguez](https://valerierdz.com/). 🍩

## Legal

"we", "us", and "our" refer to the authors of the Donet engine.

“Disney” may refer to The Walt Disney Company, or any of its subsidiaries.

The Donet project and organization is **not** in any way endorsed by, or
affiliated with, the Toontown: Corporate Clash non-profit game studio registered
in the United States. We are not affiliated with Disney.
