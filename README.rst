Odel
====
Odel is a tool to automate TRIRIGA Data Upload using the Data Integrator tool.

Features
--------
* Detects object info from filename.
* Waits until the uploaded files are completely processed.
* Concurrently upload multiple files.
* Synchronously upload hierarchical files to multiple business objects.
* Automatically selects upload action.

Getting Started
---------------
Binaries are available for Windows and Linux.  Download it from the `Releases
<https://github.com/nithinphilips/odel/releases>`_ section.

Download the zip file, extract it and add the directory containing ``odel.exe``
to ``PATH``.

Shell auto-completion scripts are included for various shells. To install them:

Bash:
 .. code:: bash

    cp completion/bash /etc/bash_completion.d/odel
    chmod ugo+x /etc/bash_completion.d/odel
    source ~/.bashrc

PowerShell:
 .. code:: powershell

    Copy-Item completion\powershell $env:USERPROFILE\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1

Note: If you have existing startup commands in
``Microsoft.PowerShell_profile.ps1``, do not copy. Instead manually add the
completion script to the end, omitting the ``using namespace`` lines.


Basic Usage
-----------
Once added to ``PATH``, Odel can be invoked using the ``odel`` command::

    odel --help

We will be working with a file called ``users.txt`` (the fields are TAB
delimited)::

    $ cat users.txt
    triFirstNameTX  triLastNameTX   triUserNameTX
    Homer   Simpson hsimpson
    Bender  Rodriguez       brodriguez

To upload ``users.txt`` to TRIRIGA running on ``localhost``, you can run::

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

You can optionally put the connection information in a JSON file named
``TRIRIGA.json`` in the current directory.

The file should look like this:

.. code:: json

    {
        "name" : "Example-dev",
        "webHost" : "http://10.10.0.100:9080/",
        "webUsername" : "system",
        "webPassword" : "admin"
    }

If you have a ``TRIRIGA.json`` file and you set the command-line connection
options, the values set in command-line take precedence. You can also override
individual settings in the ``TRIRIGA.json`` file by setting the matching
command-line options.

Note that TRIRIGA processes uploads one at a time. So, if another user uploads
a file around the same time as you, your upload may appear to hang.

Concurrent File Upload
----------------------
If you provide multiple files as input to Odel, they will be uploaded and
processed concurrently. For example::

    odel Location-triBuilding-triBuilding.txt triPeople-triPeople-triEmployee.txt

Odel will upload both of these files at once and wait until both files are
processed.

This is useful to upload multiple unrelated files as quickly as possible.

Synchronous File Upload
-----------------------
You can upload files one after another by chaining multiple Odel invocations.
For example::

    odel Location-triBuilding-triBuilding.txt && odel triPeople-triPeople-triEmployee.txt

Odel will upload the first file and wait until it is processed by TRIRIGA. Then
the shell will execute the second upload.

This is useful if the second upload requires the records in the first file to exist
in order to create associations.

File Naming Conventions
-----------------------
If you name your DI files appropriately, Odel can detect the type information
required to upload the file to TRIRIGA. It is also a good practice to name your
DI files consistently.

Odel parses the file name like this:

1) Split the filename into parts, where each part is separated by a "-"
2) Take the last three parts, assume they are Module, Business Object and Form
   names, in that order.
3) If there are exactly two parts, they are treated as Module and Business Object.
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
Module and Business Object Only (no prefixes are allowed):
 ``triPeople-triPeople.txt``


If the regular parsing of three part file name fails, Odel will try a keyword
search to guess the type of the file. Only a single keyword is currently
searched. Keyword search is NOT case sensitive.

Patch Helpers:
 Searches for the word ``patchhelper``. For example,
 ``PatchHelper_UpgradeApplication.txt`` will parse to Module = triHelper,
 Business Object = triPatchHelper, Form = triPatchHelper.

TRIRIGA has a limitation of 50 characters for all Data Integrator file names.
If the file name has more than 50 characters, Odel will truncate the file name
to fit the limits. If you try to upload multiple files with the same name at
once Odel will instead send a randomly generated file name to TRIRIGA. Run with
the ``-v`` flag to see the name changes.

URL Naming Conventions
----------------------
Scheme, host and port:
 ``http://TRIRIGA.example.com:9080/``
Scheme, host, port and context path:
 ``http://TRIRIGA.example.com:9080/TRIRIGA``

Waiting for Processing
----------------------
By default Odel will wait until TRIRIGA changes the data upload status to
*Rollup All Completed* or *Failed*, indicating the completion of the upload
process.

This only waits for creation of records. TRIRIGA may still continue to process
*Associate* and other asynchronous tasks in the background (such as with patch
helpers.)

To disable this and quit as soon as the upload is complete, specify the
``--no-wait`` flag.

Uploading CSV and other files
-----------------------------
You can use a tool like XSV_ to convert CSVs and other delimited files
to tab delimited format suitable for use with Odel and TRIRIGA.

Run XSV like this::

    xsv fmt -t '\t' 'input.csv' > 'output.txt'

.. _XSV: https://github.com/BurntSushi/xsv

Building
--------
1. Install RustUp: https://www.rust-lang.org/tools/install or update to latest
   toolchain::

        rustup update
        rustup component add rustfmt
        rustup component add clippy

   On Ubuntu, install these packages::

        apt install build-essential pkg-config libssl-dev zip

2. Use ``make`` to build::

        make all

To build releases::

    make dist

By default, the Windows binary will be dynamically linked to C Runtime and
requires that the MSVC Runtime is installed to run the executable. To enable
static linkage, add to ``~/.cargo/config``::

    [target.x86_64-pc-windows-msvc]
    rustflags = ["-Ctarget-feature=+crt-static"]


License
-------
.. code::

    Odel. Tool to upload Data Integrator files to IBM TRIRIGA.
    Copyright (C) 2021 Nithin Philips

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
