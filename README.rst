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

    cat users.txt
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

The url portion can be shortend to just the server name::

    odel --username=system --password=admin \
         localhost:9080 triPeople-triPeople-triEmployee.txt

The port can be removed as well. The username and password default to 
``system`` and ``admin``, so those can also be omitted (also now is a good
time to change that password lest you get pwned!)::

    odel localhost triPeople-triPeople-triEmployee.txt

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
Odel uses the Python Requests module to upload files through Data Integrator.
To check the status of the Data Upload, Odel uses BusinessConnect.


