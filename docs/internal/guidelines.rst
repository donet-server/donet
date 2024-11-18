..
   This file is part of the Donet reference manual.

   Copyright (c) 2024 Max Rodriguez <me@maxrdz.com>

   Permission is granted to copy, distribute and/or modify this document
   under the terms of the GNU Free Documentation License, Version 1.3
   or any later version published by the Free Software Foundation;
   with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
   A copy of the license is included in the section entitled "GNU
   Free Documentation License".

.. _guidelines:

Contributing Guidelines
=======================

**Thank you** for considering contributing to the Donet project! All
code contributions are made using **merge requests** on GitLab_.

The most important first step is to read the project
:ref:`code-of-conduct`.

Before starting to write your own contribution, please make sure to
read the project ``README.md`` file first. In addition to the project
readme file, please make sure to read over the following contributing
guidelines.

.. _GitLab: https://gitlab.com/donet-server/donet

Setting up the project locally
------------------------------

If you don't yet have an account with GitLab, please register. You
need to have a GitLab account to be able to submit your changes!

All the instructions for this step are available at
:ref:`gettingstarted`.

If you are only considering making contributions to the docs, you
might be able to skip some technical requirements that are
highlighted in the steps from the section referenced above.

Asserting Your Copyright
------------------------

As the writer of your own contribution, whether it is code or a
change in the Donet documentation, you own copyright over **your**
changes. You are highly encouraged to include your own copyright
disclaimer in any source/doc file you modify. You can do this by
adding a comment or notice at the top of the modified files, or in
documentation, indicating that you are the copyright holder.

Format:

.. code-block::

   Copyright (c) <YEAR> <FULL NAME (or username)> <EMAIL>

Example:

.. code-block::

   Copyright (c) 2024 Max Rodriguez <me@maxrdz.com>

These copyright notices should go below the first copyright notice
in a file header. If you are creating a new file, the header should
look like the following:

.. code-block:: rust

   /*
      This file is part of Donet.

      Copyright (c) 2024 Max Rodriguez <me@maxrdz.com>

      Donet is free software; you can redistribute it and/or modify
      it under the terms of the GNU Affero General Public License,
      as published by the Free Software Foundation, either version 3
      of the License, or (at your option) any later version.

      Donet is distributed in the hope that it will be useful,
      but WITHOUT ANY WARRANTY; without even the implied warranty of
      MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
      GNU Affero General Public License for more details.

      You should have received a copy of the GNU Affero General Public
      License along with Donet. If not, see <https://www.gnu.org/licenses/>.
   */

The following is another example from this documentation file:

.. code-block:: rst

   ..
      This file is part of the Donet reference manual.

      Copyright (c) 2024 Max Rodriguez <me@maxrdz.com>

      Permission is granted to copy, distribute and/or modify this document
      under the terms of the GNU Free Documentation License, Version 1.3
      or any later version published by the Free Software Foundation;
      with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
      A copy of the license is included in the section entitled "GNU
      Free Documentation License".

.. note::

   You do not have to include this copyright notice in order to own
   copyright over your changes. By U.S. copyright law, your published
   modifications are already automatically under your copyright.

Git Commit Naming Convention
----------------------------

The GitLab CI/CD pipeline for the Donet master branch includes a job
that verifies all commit messages in a push or MR meet the following
requirements. Any requirements enforced by this pipeline job are
derived from https://www.conventionalcommits.org/en/v1.0.0/.

A commit message should follow the format below:

.. code-block::

   <type>(<optional scope>): <description>

   <optional body>

   <optional footer>

Example commit messages are:

.. code-block::

   tests(donet-core): Integration test for DC language

.. code-block::

   docs: Updated understanding on message director behavior

.. code-block::

   docs(internal): Updated side note on IPv4/6 loopback

.. code-block::

   donet-message-director: Complete channel mapping logic

"As a general rule, it is always better to write too much in the
commit message body than too little." (`GNOME Shell`_)

.. _GNOME Shell: https://gitlab.gnome.org/GNOME/gnome-shell/-/blob/9f5a323e06d6b5b3818d934ba5b31c437c4c07b3/docs/commit-messages.md

Reporting Software Vulnerabilities
----------------------------------

For more information on reporting security issues, see
:ref:`security`.
