# donet

<img src="logo/donet_banner.png" alt="Donet logo artwork by honeymatsu." align="right" width="40%"/>

Donet is a free and open source network engine designed after the Distributed
Networking protocol,  as defined in the high-level networking API of the
[Panda3D](https://panda3d.org) game engine, which was originally developed by
Disney Interactive (*formerly known as Disney VR Studios*) to connect with
their in-house server technology, the OTP (*Online Theme Park*) server, which
was used to power their massive multiplayer online games, such as Toontown
Online and Pirates of the Caribbean Online, from 2001 to 2013.

## Getting Started

The Donet repository is a monorepo that houses two different Rust projects:
- **donet** - The Donet daemon source, which includes all the Donet services.
See [donet-server.org](https://www.donet-server.org).
- **libdonet** - The core utilities for Donet services, including datagram
utilities and the DC language parser. See [libdonet.rs](https://libdonet.rs).

Please read the [introduction to Donet](./docs/01-Introduction.md) for an
overview of the project and how the engine works.

Before starting your own contribution to Donet, please read over the
[Contributing Guidelines](./CONTRIBUTING.md).

We use Git for version control and Meson as the build system.

The quickest way to build for release is to do the following:

To build Donet, run the following under the project directory:
```sh
meson setup build -Dprofile=debug
meson compile -C build
```

If you are working on a contribution to either the Donet daemon or libdonet,
please run code linting and unit testing before pushing:
```sh
meson compile linting -C build
meson compile tests -C build
```
These checks should go over all source files in the `donet/` and `libdonet/`
source directories.

If you would like to build only certain Donet services into the output binary,
you can use the available Meson options to trigger feature flags in the Crate:
```sh
meson setup build -Dbuild_state_server=true
```
If any `build_*` Meson options are passed, `--no-default-features` is passed
to cargo build, which disables building all services. Only the service(s) that
you explicitly request to be built will be activated via feature flags.

## Communication

The address of the official Matrix channel for Donet development is
[#donet:matrix.org](https://matrix.to/#/#donet:matrix.org).

## Documentation
Currently the Donet engine is still under heavy development.

For the libdonet rust library documentation, visit
[libdonet.rs](https://libdonet.rs).

### Distributed Networking architecture resources

Resources for more info on Panda's Distributed Networking
(Sources listed in chronological order):

- [October 2003: Building a MMOG for the Million - Disney's Toontown Online](https://dl.acm.org/doi/10.1145/950566.950589)
- [Apr 16, 2008: The DistributedObject System, client side](https://www.youtube.com/watch?v=JsgCFVpXQtQ)
- [Apr 23, 2008: DistributedObjects and the OTP server](https://www.youtube.com/watch?v=r_ZP9SInPcs)
- [Apr 30, 2008: OTP Server Internals](https://www.youtube.com/watch?v=SzybRdxjYoA)
- [October 2010: (GDC Online) MMO 101 - Building Disney's Server System](https://www.gdcvault.com/play/1013776/MMO-101-Building-Disney-s)
- [(PDF Slideshow) MMO 101 - Building Disney's Server System](https://ubm-twvideo01.s3.amazonaws.com/o1/vault/gdconline10/slides/11516-MMO_101_Building_Disneys_Sever.pdf)

<br>

## Copyright and License

Copyright © 2023-2024 Donet Authors

"Donet" can be found at https://gitlab.com/donet-server/donet

"Donet" is distributed under the terms of the GNU Affero General Public
License, either version 3.0 or, at your option, any later
version WITHOUT ANY WARRANTY. You can read the full copy of
the software license in the [COPYING](./COPYING) file.

Donet logo artwork created by [honeymatsu](https://honeymatsu.carrd.co/). 🍩

Older revisions of the Donet logo created and designed by
[Karla Valeria Rodriguez](https://github.com/karla-valeria). 🍩

## Legal

"we", "us", and "our" refer to the authors of the Donet engine.

“Disney” may refer to The Walt Disney Company, or any of its subsidiaries.

The Donet project and organization is **not** in any way endorsed by, or
affiliated with, the Toontown: Corporate Clash non-profit game studio registered
in the United States. We are not affiliated with Disney.
