..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez.

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _configuration:

The Configuration File
======================

Configuring the Donet daemon
----------------------------

The very first thing the Donet daemon process does on boot is read
the daemon TOML configuration file. This file controls all server
options provided to the developer using Donet for their application
\- what services will this daemon launch, what ports will those
services bind and listen to, will the database backend for
disk-persisted distributed object fields be SQL or MongoDB, and so
forth.

By default, it will look for a ``daemon.toml`` file in the present
working directory. You can also give it a different path to the TOML
configuration file as a CLI argument.

.. code-block:: bash

    $ donetd ./config/donet.toml

If the configuration file cannot be read, the donetd process will
exit with an IO error.

Example TOML configuration
--------------------------

The following is an example configuration file for Donet:

.. code-block:: toml

    # The 'daemon' section describes identification
    # information which can be queried from an MD instance.
    #
    # For this example, this single daemon will perform the
    # role of all services in a cluster.
    [daemon]
    name = "Donet Cluster"
    #id = 3 # default: automatically assigned
    log_level = "info" # default: "info"

    # The 'global' section contains configuration that
    # is shared among all daemons in the cluster.
    [global]
    eventlogger = "127.0.0.1:7197"
    dc_files = ["main.dc", "game.dc"]
    # The following global settings are for the DC parser, and are optional.
    # These are used for support with legacy code.
    # They are under the global section as these settings must be the same on all clients.
    dc_multiple_inheritance = true # default: true
    dc_sort_inheritance_by_file = true # default: true
    dc_virtual_inheritance = true # default: true

    # The 'services' section describes the service(s) that
    # this daemon should perform as. (e.g. Client Agent, State Server, etc.)
    #
    #    Valid Services:
    #        - 'services.client_agent'
    #        - 'services.message_director'
    #        - 'services.state_server'
    #        - 'services.dbss'
    #        - 'services.database_server'
    #        - 'services.event_logger'

    [services.client_agent]
    bind = "127.0.0.1:7198"
    # 'dc_file_hash' tells the daemon what DC hash (32-bit) to expect from the client.
    # This setting may be used if the AI / clients don't have the same DC parser as Donet.
    #dc_file_hash = 0xABCDEF12
    version_string = "v1.0.0"

    [services.message_director]
    # The 'bind' value specifies the port and address to
    # bind its listening socket to receive messages.
    bind = "127.0.0.1:7199"
    # The 'upstream' value specifies the upstream MD to
    # connect to, if this MD instance should not act as
    # the master message director of the cluster.
    #upstream = "127.0.0.1:5555"

    [services.state_server]
    control_channel = 102000

    [services.database_server]
    control_channel = 103000
    # Valid Database Backends:
    #    - 'mysql'
    db_backend = "mysql"
    [services.database_server.sql]
    host = "192.168.1.252:3306"
    user = "root"
    pass = ""
    database = "test"

    # The DBSS service does not have a control channel, so
    # it cannot generate or activate new Distributed Objects.
    [services.dbss]
    db_channel = 103000
    # The DBSS manages a range of Distributed Objects by DoIDs.
    range_min = 100000000
    range_max = 200000000

    [services.event_logger]
    bind = "127.0.0.1:7197" # NOTE: UDP protocol
    output = "/var/log/donet/" # Logs output directory
    log_format = "el-%Y-%m-%d-%H-%M-%S.log" # Log file name format
    rotate_interval = "1d"
