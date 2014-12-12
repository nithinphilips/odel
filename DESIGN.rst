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

