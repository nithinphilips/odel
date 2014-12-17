# -*- coding: utf-8 -*-

"""DI Uploader"""

import requests
import time
import os
import sys
import logging
import re

from argh import arg
from suds.plugin import MessagePlugin
from suds.client import Client

# Maximum length of the file name allowed by TRIRIGA Data Integrator.
FILE_NAME_MAX_LEN = 50

TRIRIGA_AUTH_OBJECTID = '1000'
TRIRIGA_AUTH_LOGIN_ACTIONID = '1005'
TRIRIGA_AUTH_FORCELOGIN_ACTIONID = '1006'

def trim_filename(filename, maxlength=FILE_NAME_MAX_LEN):
    """
    Trim a file name to a maximum length while trying to preserve the
    extension.

    If the extension alone is longer than the maxlength, the extension
    may be shortened or removed altogether.

    >>> trim_filename("There-are-fifty-four-characters-in-this-file-name.txt",
    ...               maxlength=10)
    'There-.txt'
    >>> trim_filename("There-are.txt", maxlength=10)
    'There-.txt'

    No trimming is necessary if the file length is less than or equal to the
    max length:

    >>> trim_filename("There-.txt", maxlength=10)
    'There-.txt'
    >>> trim_filename("There.txt", maxlength=10)
    'There.txt'

    If the extension is very long it may be removed or trimmed:

    >>> trim_filename("a.a-very-long-file-ext", maxlength=10)
    'a.a-very-l'
    >>> trim_filename("a-very-long-filename.a-very-long-file-ext", maxlength=10)
    'a-very-lon'
    """
    filename, extension = os.path.splitext(filename)

    if len(extension) >= maxlength:
        # The extension alone is longer than the maxlength we combine them to
        # get the first maxlength characters
        filename = filename + extension
        extension = ""
    else:
        maxlength = maxlength - len(extension)

    if len(filename) > maxlength:
        filename = filename[:maxlength]

    return filename + extension

def parse_filename(filename, separator='-'):
    """
    Extract Module, BO, and Form name from filename.

    The filename should be in the format ``Module-BusinessObject-Form.txt``

    File name only, no extension:
    >>> parse_filename("triPeople-triPeople-triEmployee")
    ['triPeople', 'triPeople', 'triEmployee']

    File name only with extension:
    >>> parse_filename("triPeople-triPeople-triEmployee.txt")
    ['triPeople', 'triPeople', 'triEmployee']

    File name only with another extension:
    >>> parse_filename("triPeople-triPeople-triEmployee.xlsx")
    ['triPeople', 'triPeople', 'triEmployee']

    File name with full path:
    >>> parse_filename("/home/odel/triPeople-triPeople-triEmployee.txt")
    ['triPeople', 'triPeople', 'triEmployee']

    File name with full path and a sequence prefix:
    >>> parse_filename("/home/odel/001-triPeople-triPeople-triEmployee.txt")
    ['triPeople', 'triPeople', 'triEmployee']

    File name with full path and a sequence prefix and spaces around the
    separators:
    >>> parse_filename(
    ...  "/home/odel/001 - triPeople - triPeople - triEmployee.txt"
    ... )
    ['triPeople', 'triPeople', 'triEmployee']


    This method handles a special case. If the file does not have a three
    part identifier, but contains the work PatchHelper, it will return the
    info for the TRIRIGA Patch Helper module:
    >>> parse_filename("/home/odel/PatchHelper_UpgradeApp.txt")
    ['triHelper', 'triPatchHelper', 'triPatchHelper']

    It is not case sensitive or position dependent:
    >>> parse_filename("/home/odel/UpgradeApp_patchhelper.txt")
    ['triHelper', 'triPatchHelper', 'triPatchHelper']

    If the file name does not have three parts a ValueError is raised:
    >>> parse_filename("/home/odel/badname.xlsx")
    Traceback (most recent call last):
      File "odel/diuploader.py", line 60, in parse_filename
        raise ValueError("The filename must have at least three components separated by '-'.")
    ValueError: The filename must have at least three components separated by '-'.
    """
    basename = os.path.basename(filename)
    basename = os.path.splitext(basename)[0]
    results = basename.split(separator)[-3:]
    if len(results) == 3:
        results = map(str.strip, results)
        return results

    logging.debug("No type information in the file name")

    if "patchhelper" in basename.lower():
        logging.debug("File name matches keyword: patchhelper")
        return ['triHelper', 'triPatchHelper', 'triPatchHelper']

    raise ValueError(
        "The filename must have at least three components separated by '-'."
    )

def normalize_url(url):
    """
    Normalizes a URL by stripping the trailing slash.

    >>> normalize_url("http://www.google.com/")
    'http://www.google.com'
    >>> normalize_url("http://www.google.com")
    'http://www.google.com'
    """
    return url[:-1] if url.endswith("/") else url


def parse_url(url, port='9080'):
    """
    Parses a flexible user provided URL

    If port is set, the port will be appended as :port, unless
    it is 80 or 443. If 443 the returned URL will have https
    scheme

    Examples:

    Host name only:
    >>> parse_url("localhost")
    'http://localhost:9080'

    >>> parse_url("localhost:9080")
    'http://localhost:9080'

    >>> parse_url("localhost:8001")
    'http://localhost:8001'

    Host name, port and context path
    >>> parse_url("localhost:9080/context_path")
    'http://localhost:9080/context_path'

    Host name and context path
    >>> parse_url("localhost/context_path")
    'http://localhost:9080/context_path'

    Alternate default port:
    >>> parse_url("localhost", port=8001)
    'http://localhost:8001'

    Fully qualified URLs are not modified:
    >>> parse_url("http://localhost")
    'http://localhost'

    >>> parse_url("https://localhost")
    'https://localhost'

    >>> parse_url("http://localhost:9080")
    'http://localhost:9080'

    >>> parse_url("http://localhost:9080/context_path")
    'http://localhost:9080/context_path'

    >>> parse_url("HTTP://localhost:9080/context_path")
    'HTTP://localhost:9080/context_path'

    Host name and port 80:
    >>> parse_url("localhost", port=80)
    'http://localhost'

    Host name and port 443:
    >>> parse_url("localhost", port=443)
    'https://localhost'

    >>> parse_url("localhost:443", port=9080)
    'https://localhost'
    """

    # Port argument may be given as a string or int
    port = int(port)

    if re.match(r'^https?://', url, re.IGNORECASE):
        return url

    # Any ports in the input override the default port.
    match = re.search(r'(?P<host>[^:/]+)(:(?P<port>\d+))?(?P<path>/.*)?', url)
    if not match:
        raise ValueError(
            "The give URL value {} is not fully qualified and cannot be "
            "parsed".format(url)
        )

    hostname = match.group('host')
    userport = match.group('port')
    path = match.group('path')

    if userport:
        port = int(userport)

    scheme = "https" if port == 443 else "http"

    url = scheme + "://" + hostname

    if port not in (443, 80):
        url = url + ":" + str(port)

    if path:
        url = url + path

    return url


@arg(
    '--no-wait', '-w',
    help="Do not Wait until the file is processed by Tririga. By default Odel "
         "will wait until Tririga has fully processed the file.",
)
@arg('--module', '-m', help="The name of the module to which to upload")
@arg('--businessobject', '-b',
     help="The name of the business object to which to upload")
@arg('--action', '-a',
     help="The action to apply to the newly created records. "
          "By default the first possible action is applied. "
          "If an invalid action is specified, Odel will warn "
          "you and attempt to continue.")
@arg('--form', '-f', help="The name of the form to which to upload")
@arg('--username', '-u', help="Your tririga username.")
@arg('--password', '-p', help="Your tririga password.")
@arg('filename', help="The file to upload.")
@arg('url',
     help="The URL to the TRIRIGA environment. Include any context paths. "
          "This could be just the hostname. In that case port 9080 will be "
          "appended")
def upload(url, filename, username="system", password="admin",
           module=None, businessobject=None, form=None, action=None,
           no_wait=False):
    """
    Uploads a file to Tririga Data Integrator.

    The module, businessobject and form arguments are optional if the file is
    named in the following pattern "<module>-<businessObject>-<form>.txt".
    Otherwise, you must specify them as arguments.

    Action is also optional. If it is omitted, Odel will retrive a list of
    applicable actions and use the first one.
    """

    session = requests.Session()
    site_url = normalize_url(parse_url(url, port=9080))

    logging.debug("Normalized URL: {}".format(site_url))

    # Try to get Module/Bo/Form from the file name
    if not module or not businessobject or not form:
        module, businessobject, form = parse_filename(filename)

    if not module or not businessobject or not form:
        sys.stderr.write("Unable to detect module, business object and "
                         "form name from filename. Please provide them.\n")
        return

    # Convert names to ids so we can POST
    moduleid, businessobjectid, formid, transitions = get_object_info(
        module, businessobject, form, site_url, username, password
    )

    logging.debug("Uploading to {}({})::{}({})::{}({})".format(
        module, moduleid, businessobject, businessobjectid, form, formid
    ))

    if action:
        transition = action
    else:
        transition = transitions[0]

    if transition not in transitions:
        sys.stderr.write(
            "WARNING: The state transition {} does not appear to be valid for "
            "the selected record type. The upload may fail.".format(transition)
        )

    logging.debug(
        "The {} state transitions will be "
        "triggered on the new records.".format(transition)
    )

    authpayload = {
        'USERNAME': username,
        'PASSWORD': password,
        'objectId': TRIRIGA_AUTH_OBJECTID,
        'actionId': TRIRIGA_AUTH_FORCELOGIN_ACTIONID,
    }

    logging.debug("Logging in")
    url = '{}/WebProcess.srv'.format(site_url)
    response = session.post(url, data=authpayload, allow_redirects=False)

    filenameonly = os.path.basename(filename)

    trimmed_filename = trim_filename(filenameonly)
    if trimmed_filename != filenameonly:
        sys.stdout.write(
            "File name is too long. It has been shortened to '{}'.\n".format
            (
                trimmed_filename
            )
        )

    filenameonly = trimmed_filename

    diparams = {
        'updateAct': "createSpec",
        'filenames': filenameonly,
        'classTypeN': moduleid,
        'objectTypeN': businessobjectid,
        'guiIdN': formid,
        'delimiterN': '.TAB',
        'charSet': 'UTF-8',
        'transactionTypeN': 'Insert/New',
        'batch': 'NO',
        'actionName': transition,
        'stateName': 'triDraft',
    }

    files = {'theFile': (filenameonly, open(filename, 'rb'))}

    logging.debug("Uploading the file contents.")
    url = '{}/html/en/default/common/dataUploadFile.jsp'.format(site_url)
    response = session.post(url, params=diparams, files=files)

    logging.debug("Creating Data Upload record.")
    url = '{}/html/en/default/common/dataSmartUpload.jsp'.format(site_url)
    response = session.post(url, params=diparams)

    ecode = 0

    if not no_wait:
        logging.debug("Waiting for the processing to complete.")
        ecode = wait_for_upload(filenameonly, site_url, username, password)

    sys.exit(ecode)

class MultipartMimeFilter(MessagePlugin):
    """
    Filter used by suds to convert MultiPartMime response from TRIRIGA to a
    process that can be processed.
    """
    def received(self, context):
        """
        Method called when data is received
        """
        context.reply = context.reply.split('\n')[6]

def get_object_info(module, businessobject, form, site_url, username, password):
    """
    Converts the module, business object and form names to IDs using
    the Tririga BusinessConnect WebService. Also retrieves the list
    of possible state transitions that apply to a 'null' object.

    Arguments
    * module: (str) Module name. eg: triPeople
    * businessconnect: (str) Business object name, eg: triPeople
    * form: (str) Form name, eg: triEmployee
    * username: The username to login to the webservice.
    * password: Password to login to the webservice.

    Return a tuple containing 4 items.

    (moduleid, objectid, guiid, transitions)

    transitions is a list of possible state transitions that apply to a 'null'
    object.

    >>> get_object_info('triPeople', 'triPeople', 'triEmployee',
    ...                 'http://192.168.0.107:9080', 'system', 'admin')
    (7, 106402L, 10002361L, [triCreateDraft, triCreateTemplate, triActivate])

    """

    wsurl = '{}/ws/TririgaWS?wsdl'.format(site_url)
    headers = {"Username": username, "Password" : password}

    client = Client(wsurl, headers=headers, plugins=[MultipartMimeFilter()])

    moduleid = client.service.getModuleId(module)
    objectid = client.service.getObjectTypeId(module, businessobject)

    guiid = 0
    guis = client.service.getGUIs(objectid)[0]
    for gui in guis:
        if gui.name == form:
            guiid = gui.id
            break

    if guiid <= 0:
        raise ValueError("The form '{}' does not exist.".format(form))

    transitions = []
    actions = client.service.getObjectTypeActions(moduleid, objectid, -1, -1)[0]
    for action in actions:
        for aaction in action[1]:
            transitions.append(aaction.action)

    return moduleid, objectid, guiid, transitions

FILTER_EQUALS = 10
FILTER_NOT_EQUALS = 11
FILTER_LESS_THAN = 12
FILTER_LESS_THAN_OR_EQUALS = 13
FILTER_GREATER_THAN = 14
FILTER_GREATER_THAN_OR_EQUALS = 15
FILTER_CONTAINS = 16
FILTER_STARTS_WITH = 17
FILTER_ENDS_WITH = 18
FILTER_BEFORE = 20
FILTER_AFTER = 21
FILTER_IN = 22
FILTER_NOT_IN = 23

DATATYPE_STRING = 320
DATATYPE_NUMBER = 310
DATATYPE_DATE = 330
DATATYPE_DATETIME = 335

MAX_RETRIES = 23


def wait_for_upload(filename, site_url, username, password):
    """
    Wait until the upload of a given file completes.

    Implements exponential backoff algorithm described at
    http://docs.aws.amazon.com/general/latest/gr/api-retries.html

    MAX_RETRIES defines the number of attempts. 23 tries is about 24
    hours of wait.
    """

    wsurl = '{}/ws/TririgaWS?wsdl'.format(site_url)
    headers = {"Username": username, "Password" : password}

    client = Client(wsurl, headers=headers, plugins=[MultipartMimeFilter()])

    # Create a filter to look for only the file we uploaded.
    filters = client.factory.create('ns2:ArrayOfFilter')
    namefilter = client.factory.create('ns2:Filter')
    namefilter.fieldName = "File Name"
    namefilter.value = filename
    namefilter.dataType = DATATYPE_STRING
    namefilter.operator = FILTER_EQUALS
    namefilter.sectionName = "General"
    filters.Filter = [namefilter]

    projectname = ""
    modulename = "System"
    objecttypename = "Data Upload"
    queryname = "Data Upload - System - Manager Query"
    start = 1
    maximumresultcount = 999

    status_codes = {
        "Rollup All Completed": 0,
        "Failed": 10,
        "NEW": 20,
        "DONE": 30,
        "UPLOADING...": 40
    }

    processing_status = ("NEW", "DONE", "UPLOADING...")

    total_sleep_time = 0
    retry = True
    retries = 0
    last_status = None

    while retry and retries < MAX_RETRIES:
        sleep_time = 2**retries / 100.0
        total_sleep_time = total_sleep_time + sleep_time

        logging.debug(
            "Upload status check #{}: Wait {} seconds".format(retries, sleep_time)
        )

        time.sleep(sleep_time)

        # We will not run a continuation query.
        # If you uploaded more than 999 files with the same name, screw you!
        results = client.service.runNamedQuery(
            projectname, modulename, objecttypename, queryname, filters, start,
            maximumresultcount
        )

        # It is possible that there is more than one file with the same name.
        # And we do not have a way to reliably tell which one is ours.
        #
        # So, we will look at all the files and check if ANY of them are in
        # one of the processing status.
        found_processing = False
        for result in results.queryResponseHelpers[0]:
            for column in result.queryResponseColumns[0]:
                if column.name == "Status":
                    logging.debug("Uploaded record status: {}".format(column.value))
                    if column.value in processing_status:
                        found_processing = True
                    last_status = column.value

        if not found_processing:
            logging.debug("File appears to be fully processed")
            retry = False

        retries = retries + 1

    logging.debug("File processed in about {} seconds".format(total_sleep_time))

    return status_codes[last_status]

