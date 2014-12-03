Odel
====
Odel uploads Data Integrator files to Tririga from the command-line.

You can import the files quickly and easily with Odel. It can also be called
from script as part of a batch process.

Get started by installing Odel::

    python setup.py install

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

Odel will parse the file name to get the necessary information.

The url portion may be shortend to just the server name::

    odel --username=system --password=admin \
         localhost:9080 triPeople-triPeople-triEmployee.txt

The port can be removed as well. The username and password default to 
``system`` and ``admin``, so those can also be omitted (also now is a good
time to change that password lest you get pwned!)::

    odel localhost triPeople-triPeople-triEmployee.txt

Tririga has a limitation of 150 characters for all Data Integrator file names.
If the file name has more than 150 characters, Odel will truncate the file
name.

Waiting for Processing
----------------------
Normally Odel will terminate as soon as the file is transmitted to Tririga.  It
will still take a few minutes for Tririga to process the file and create the
records. Often, when running as part of a batch process you will want to wait
until the file is processed before performing the next task. 

If the ``-w`` flag is set, Odel will wait until Tririga changes the data upload
status to *Rollup All Completed* or *Failed*, indicating the completion of the
upload process.

How it Works
------------
Tririga does not provide a public API to upload files through Data Integrator.
To make Odel work we simulate an interactive user session. Odel uses a
combination of BusinessConnect (SOAP API) and web scraping to do its job.

The first use for the SOAP API is to turn the module, business object and form
names into IDs. The Data Integrator feature expects IDs rather than string
values. In the Tririga web interface, these fields are populated using
Javascript. Odel does not execute any Javascript, so the SOAP API fills in the
gap.

Odel also retrieves the list possible *State Transitions* using the SOAP API.
These are state transitions that originate from a *null* record. If the user
did not specify a state transition using the ``--action`` argument the first
possible state transition is selected.

Next, Odel simulates a Tririga user login. It uses the *force* login option
(same as clicking the "remove active session" link in the login page) to make
sure the login always succeeds. This has the effect of ending any other active
sessions for the Tririga user account. Once logged in the next step is to
upload the file. Data Integrator works in two steps. Step one sends the file
data to tririga. Step two sends the commands necessary to start the file
processing. Tririga takes care of matching up the file contents and the
processing commands.

If the user did not specify the ``--wait`` option, Odel will then quit.

If the ``--wait`` option is on, Odel needs to find out if the records that were
just uploaded have been processed. Tririga processes Data Integrator uploads
asynchronously. In the web interface the user will get a notification when the
processing is complete. Checking the user's notifications is one possible
method to see if the upload completed. It is however more reliable to check the
Data Upload records themselves. So Odel invokes the ``runNamedQuery`` SOAP
operation to run the ``Data Upload - System - Manager Query`` query. Odel
passes in the file name as a dynamic filter to limit the number of records
returned.

Odel checks all the returned records (there could be more than one if another
file with the same name was uploaded previously) to see if any of them are in
one of the processing statuses (``NEW``, ``DONE`` or ``UPLOADING...``.) If they
are, Odel waits a few seconds and checks again. It repeats the process until
the status changes. When the status changes Odel will quit. If the file is not
processed in about 10 minutes Odel will simply give up. This can happen with
very large files or more commonly if the ``Data Integrator`` agent is not
running on the host where the file was uploaded.

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
