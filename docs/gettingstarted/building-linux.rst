..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez.

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _building-linux:

Building Donet on Linux
=======================

Getting a copy of the code
--------------------------

The Donet software repository is hosted on GitLab, and mirrored to
GitHub. You can get a copy of the repository on your local machine
by using Git_ to clone the repo.

On your computer, clone with:

.. code-block:: shell

    git clone https://gitlab.com/donet-server/donet.git

Building from source
--------------------

Donet uses Meson_ as the build system, which in turn calls Cargo_.

The quickest way to **build for debug** is to do the following:

Run the following Meson commands under the project directory:

.. code-block:: shell

    meson setup build -Dprofile=debug
    meson compile -C build

.. note::

    The instructions above will compile the Donet project and its
    crates without optimizations and includes more verbose logging.
    To build Donet for **release**, simply remove
    ``-Dprofile=debug``.

If you are working on a contribution to either the Donet daemon or
libdonet, please run code linting and unit testing before pushing:

.. code-block:: shell

    meson compile linting -C build
    meson compile tests -C build

These checks should go over all source files in the ``donet/`` and
``libdonet/`` source directories.

.. tip::

    If you would like to build only certain Donet services into the
    output binary, you can use the available Meson options to trigger
    feature flags in the Crate:

    .. code-block:: shell

        meson setup build -Dbuild_state_server=true

    If any ``build_*`` Meson options are passed,
    ``--no-default-features`` is passed to cargo build, which disables
    building all services. Only the service(s) that you explicitly
    request to be built will be activated via `feature flags`_.

.. _Git: https://git-scm.com/
.. _Meson: https://mesonbuild.com/
.. _Cargo: https://doc.rust-lang.org/cargo/
.. _feature flags: https://doc.rust-lang.org/cargo/reference/features.html
