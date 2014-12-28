Odel
====
Odel uploads Data Integrator files to Tririga from the command-line. You can
import the files quickly and easily with Odel. It can also be called from
script as part of a batch process.


Getting Started
---------------
Windows Users
~~~~~~~~~~~~~
An installer is available for Windows. Download it from the `Releases
<https://github.com/nithinphilips/odel/releases>`_ section.  Once installed,
you will have the ``odel`` command available in your Command Prompt.

All Other Platforms
~~~~~~~~~~~~~~~~~~~
Install Python 2.7. Download the source and install Odel and its
dependencies::

    python setup.py install

Basic Usage
~~~~~~~~~~~
Once installed, odel can be invoked using the ``odel`` command::

    odel --help

We will be working with a file called ``users.txt`` (the fields are TAB delimited)::

    $ cat users.txt
    triFirstNameTX  triLastNameTX   triUserNameTX
    Homer   Simpson hsimpson
    Bender  Rodriguez       brodriguez

To upload ``users.txt`` to Tririga running on ``localhost``, you can run::

    odel --username=system --password=admin --module=triPeople \
         --businessobject=triPeople --form=triEmployee \
         http://localhost:9080/ users.txt

You can save some typing if you name you DI files to include the module,
business object and form names. For example, if you rename ``users.txt``
to ``triPeople-triPeople-triEmployee.txt``, you can run::

    odel --username=system --password=admin \
         http://localhost:9080/ triPeople-triPeople-triEmployee.txt

Odel will parse the file name to get the type information. See `File Naming
Conventions`_ below for more details.

The url portion may be shortened to just the server name::

    odel --username=system --password=admin \
         localhost:9080 triPeople-triPeople-triEmployee.txt

The port can be removed as well. 9080 will be used as the deafult. See `URL
Naming Conventions`_ below for more.

The username and password default to ``system`` and ``admin``, so those can
also be omitted (also now is a good time to change that password!)::

    odel localhost triPeople-triPeople-triEmployee.txt

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
Odel is fairly flexible in handling URL values. Here's what you can enter:

Scheme, host and port:
 ``http://tririga.example.com:9080/``
Host and port:
 ``tririga.example.com:9080``. This resolves to
 ``http://tririga.example.com:9080/``
Host only:
 ``tririga.example.com``. This resolves to 
 ``http://tririga.example.com:9080/``.

Odel uses port 9080 as the default. This is the default WebSphere application
virtual host port.

There are two special cases. If you enter ``tririga.example.com:443``, it
resolves to ``https://tririga.example.com/``. Note the HTTPS scheme. If you
enter ``tririga.example.com:80``, Odel resolves this to
``http://tririga.example.com/``. This is plain HTTP and the port is omitted.

Waiting for Processing
----------------------
By default Odel will wait until Tririga changes the data upload status to
*Rollup All Completed* or *Failed*, indicating the completion of the upload
process.

This only waits for creation of records. Tririga may still continue to process
*Associate* and other asynchronous tasks in the background.

To disable this and quit as soon as the upload is complete, specify the
``-no-wait``` flag.

Building Windows Installer
--------------------------
Windows installer can be built on Windows machines. You will need Python 2.7 (Windows version)
and the ``pyinstaller`` package (version 2.0).


1. From the project root run::

    make

   This will build the binaries.

2. Change directory to the ``windows`` folder and run make again::

    cd windows/
    make

   This will build the ``.msi`` installer.

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
