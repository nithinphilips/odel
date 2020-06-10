Odel
====
Odel is a tool to automate TRIRIGA Data Upload using the Data Integrator tool.

Features
--------
* Detects object info from filename.
* Blocks until the uploaded file is completely processed.
* Automatically selects upload action.

Getting Started
---------------
Binaries are available for Windows and Linux.  Download it from the `Releases
<https://github.com/nithinphilips/odel/releases>`_ section.

Download the zip file, extract it and add the directory containing ``odel.exe``
to ``PATH``.

Shell auto-completion scrips are included for various shells. To install them:

Bash::

    cp completion/bash /etc/bash_completion.d/

PowerShell::

    Copy-Item completion\powershell $env:USERPROFILE\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1

Note: If you have existing startup commands, manually add the completion script
to the end, omitting the ``using namespace`` lines.


Basic Usage
-----------
Once added to ``PATH``, odel can be invoked using the ``odel`` command::

    odel --help

We will be working with a file called ``users.txt`` (the fields are TAB
delimited)::

    $ cat users.txt
    triFirstNameTX  triLastNameTX   triUserNameTX
    Homer   Simpson hsimpson
    Bender  Rodriguez       brodriguez

To upload ``users.txt`` to Tririga running on ``localhost``, you can run::

    odel --username=system --password=admin --module=triPeople \
         --businessobject=triPeople --form=triEmployee \
         --url=http://localhost:9080/ users.txt

You can save some typing if you name the DI files to include the module,
business object and form names. For example, if you rename ``users.txt``
to ``triPeople-triPeople-triEmployee.txt``, you can run::

    odel --username=system --password=admin \
         --url=http://localhost:9080/ triPeople-triPeople-triEmployee.txt

Odel will parse the file name to get the type information. See `File Naming
Conventions`_ below for more details.

The username and password default to ``system`` and ``admin``, so those can
also be omitted (also now is a good time to change that password!)::

    odel --url=http://localhost:9080 triPeople-triPeople-triEmployee.txt

Note that TRIRIGA processes uploads one at a time. So, if another user uploads
a file around the same time as you, your upload may appear to hang.

File Naming Conventions
-----------------------
If you name your DI files appropriately, Odel can detect the type information
required to upload the file to TRIRIGA. It is also a good practice to name your
DI files consistently.

Odel parses the file name like this:

1) Split the filename into parts, where each part is separated by a "-"
2) Take the last three parts, assume they are Module, Business Object and Form
   names, in that order.
2) If there are exactly two parts, they are treated as Module and Business Object.
   The default Form for the Business Object is selected.

So, you can have additional information in the file name, as long as the information
Odel is looking for is at the very end.

These are examples of files names for Employee Data. Odel parses all these as
Module = triPeople, Business Object = triPeople, Form = triEmployee

Simple:
 ``triPeople-triPeople-triEmployee.txt``
With spaces around the ``-``:
 ``triPeople - triPeople - triEmployee.txt``
With a prefix:
 ``IterationA - triPeople-triPeople-triEmployee.txt``
With two prefixes:
 ``001 - IterationA - triPeople-triPeople-triEmployee.txt``

If the regular parsing of three part file name fails, Odel will try a keyword
search to guess the type of the file. Only a single keyword is currently
searched. Keyword search is NOT case sensitive.

Patch Helpers:
 Searches for the word ``patchhelper``. For example,
 ``PatchHelper_UpgradeApplication.txt`` will parse to Module = triHelper,
 Business Object = triPatchHelper, Form = triPatchHelper.

Tririga has a limitation of 50 characters for all Data Integrator file names.
If the file name has more than 50 characters, Odel will truncate the file name
to fit the limits.

URL Naming Conventions
----------------------
Scheme, host and port:
 ``http://tririga.example.com:9080/``
Scheme, host, port and context path:
 ``http://tririga.example.com:9080/tririga``

Waiting for Processing
----------------------
By default Odel will wait until Tririga changes the data upload status to
*Rollup All Completed* or *Failed*, indicating the completion of the upload
process.

This only waits for creation of records. Tririga may still continue to process
*Associate* and other asynchronous tasks in the background (such as with patch
helpers.)

To disable this and quit as soon as the upload is complete, specify the
``--no-wait``` flag.

Building
--------
1. Install RustUp: https://www.rust-lang.org/tools/install or update to latest
   toolchain::

        rustup update
        rustup component add rustfmt
        rustup component add clippy

2. Use ``make`` to build::

        make all

To build releases::

    make dist

By default, the binary will be dynamically linked to C Runtime. To enable
static linkage, add to ``~/.cargo/config``::

    [target.x86_64-pc-windows-msvc]
    rustflags = ["-Ctarget-feature=+crt-static"]


License
-------
.. code::

    Odel. Tool to upload Data Integrator files to IBM Tririga.
    Copyright (C) 2014 Nithin Philips

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
