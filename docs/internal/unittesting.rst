.. _unittesting:

Unit Testing
============

Both donet and libdonet crates make use of :term:`unit testing` with
Rust. If you are new to unit testing Rust code, see `Unit Testing`_
from the Rust by Example book.

.. _Unit Testing: https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html

Code Coverage
-------------

.. image:: https://codecov.io/gl/donet-server/donet/graph/badge.svg?token=XCESKI8ISS
    :alt: Codecov Dashboard for Donet
    :target: https://codecov.io/gl/donet-server/donet

The Donet project uses Codecov_ as its online dashboard for viewing
unit test coverage statistics. The CI/CD pipeline generates the
Cobertura XML and uploads it to Codecov if it is on the default
branch. You can use the badge above or the badge in the GitLab
repository to go to the dashboard for Donet.

.. _Codecov: https://codecov.io

.. figure:: https://codecov.io/gl/donet-server/donet/graphs/sunburst.svg?token=XCESKI8ISS
    :alt: Latest coverage graph
    :align: center

    Donet's latest coverage graph from Codecov.io

Debugging Unit Tests
--------------------

To debug unit tests with a debugger such as GDB_, you need to have
the unit tests binary. You can build this with the following
Meson build command:

.. _GDB: https://sourceware.org/gdb/

.. code-block:: shell

    meson compile build-tests -C build

This Meson run target will build unit tests for each crate in the
workspace. The unit test binaries should be written to the following
path:

    build/target/debug/deps/donetd-<hash>

    build/target/debug/deps/libdonet-<hash>

Viewing Local Coverage Reports
------------------------------

The latest commit's unit test code coverage report can be viewed
online at `codecov.io`_. The dashboard displays the code coverage
percentage for the entire project and allows you to view covered and
missing lines per source file if you are logged in.

During development, you may need to inspect the code coverage report
directly from your latest local changes before you can see it on the
online dashboard after pushing a new commit.

Donet uses Tarpaulin_ to generate code coverage reports. To build the
coverage report locally, run the following run target using Meson:

.. _codecov.io: https://codecov.io/gl/donet-server/donet
.. _Tarpaulin: https://github.com/xd009642/tarpaulin

.. code-block:: shell

    meson compile code-coverage -C build

The output of this run target should be 2 coverage report files:

    build/target/tarpaulin/cobertura.xml

    build/target/tarpaulin/coverage.json

These are large XML/JSON files, so you will need a tool to view the
report. You can use pycobertura_ to view the code coverage report
from your terminal. To do this, run:

.. _pycobertura: https://github.com/aconrad/pycobertura

.. code-block:: shell

    python -m pip install pycobertura
    python -m pycobertura show build/target/tarpaulin/cobertura.xml
