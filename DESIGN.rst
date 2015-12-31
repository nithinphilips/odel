How it Works
------------
Tririga does not provide a public API to upload files through Data Integrator.
To make Odel work we simulate an interactive user session. Odel uses a
combination of BusinessConnect (SOAP API) and web scraping to do its job.

The first use for the SOAP API is to turn the module, business object and form
names into IDs. The Data Integrator feature expects IDs rather than string
values. In the Tririga web interface, these fields are populated using
Javascript. Odel does not execute any Javascript, so the SOAP API fills in the
gap. The ``getModuleId``, ``getObjectTypeId`` and ``getGUIs`` API calls are
used.

Odel also retrieves the list possible *State Transitions* using the SOAP API.
These are state transitions that originate from a *null* record. If the user
did not specify a state transition using the ``--action`` argument the first
possible state transition is selected. The ``getObjectTypeActions`` API call is
used to get this.

Next, Odel simulates a Tririga user login. It uses the *force* login option
(same as clicking the "remove active session" link in the login page) to make
sure the login always succeeds. This has the effect of ending any other active
sessions for the Tririga user account::

    POST /WebProcess.srv HTTP/1.1
    <...http-headers...>

    USERNAME=<username>&PASSWORD=<password>&objectId=1000&actionId=1006

Once logged in the next step is to upload the file. Data Integrator works in
two steps. Step one sends the file data to tririga::

    POST /html/en/default/common/dataUploadFile.jsp HTTP/1.1
    <...http-headers...>

    <...filedata...>

Step two sends the commands necessary to start the file processing. Tririga
takes care of matching up the file contents and the processing commands::

    POST /html/en/default/common/dataSmartUpload.jsp HTTP/1.1
    <...http-headers...>

    <...parameters...>


If the user specified the ``--no-wait`` option, Odel will then quit.

Otherwise, Odel needs to find out if the records that were just uploaded have
been processed. Tririga processes Data Integrator uploads asynchronously. In
the web interface the user will get a notification when the processing is
complete. Checking the user's notifications is one possible method to see if
the upload completed. It is however more reliable to check the Data Upload
records themselves. So Odel invokes the ``runNamedQuery`` SOAP operation to run
the ``Data Upload - System - Manager Query`` query. Odel passes in the file
name as a dynamic filter to limit the number of records returned.

Odel performs some heuristics on all the returned records (there could be more
than one if another file with the same name was uploaded previously) to see
which one was just uploaded. Once the record is identified, Odel repeatedly
check the record status to see if the upload is completed. The delay between
the checks are increased exponentially to avoid overloading TRIRIGA. It will
perform 23 checks (which is roughly equal to 24 hours) before giving up. If
the upload status changes to completed or failed, Odel will quit.

The upload's final status is communicated via Odel's exit codes. A *Rollup All
Completed* status will cause Odel to exit with ``0``. A *Failed* status will
return the exit code ``10``. Other less common exit codes may also be returned
if the maximum wait period has passed. ``20`` for *New*, ``30`` for *Done* and
``40`` for *Uploading...*.
