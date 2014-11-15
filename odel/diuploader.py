# -*- coding: utf-8 -*-

"""DI Uploader"""

import cookielib
import requests
import time
import os
import sys

from argh import arg
from suds.plugin import MessagePlugin
from suds.client import Client

TRIRIGA_AUTH_OBJECTID = '1000'
TRIRIGA_AUTH_LOGIN_ACTIONID = '1005'
TRIRIGA_AUTH_FORCELOGIN_ACTIONID = '1006'

def parse_filename(filename):
    """
    Extract Module, BO, and Form name from filename.

    The filename should be in the format ``Module-BusinessObject-Form.txt``

    >>> parse_filename("triPeople-triPeople-triEmployee.txt")
    ['triPeople', 'triPeople', 'triEmployee']
    >>> parse_filename("triPeople-triPeople-triEmployee.xlsx")
    ['triPeople', 'triPeople', 'triEmployee']
    >>> parse_filename("/home/odel/triPeople-triPeople-triEmployee.xlsx")
    ['triPeople', 'triPeople', 'triEmployee']
    """
    basename = os.path.basename(filename)
    basename = os.path.splitext(basename)[0]
    return basename.split('-')

def normalize_url(url):
    """
    Normalizes a URL.

    >>> normalize_url("http://www.google.com/")
    'http://www.google.com'
    >>> normalize_url("http://www.google.com")
    'http://www.google.com'
    """

    if url.endswith("/"):
        return url[:-1]

    return url


@arg(
    '--wait', '-w',
    help="Wait and keep running until the file is processed by Tririga. Odel "
         "will poll Tririga to check the record status. Useful for batch "
         "processing"
)
@arg('--module', '-m', help="The name of the module to which to upload")
@arg('--businessobject', '-b', help="The name of the business object to which to upload")
@arg('--form', '-f', help="The name of the form to which to upload")
@arg('--username', '-u', help="Your tririga username.")
@arg('--password', '-p', help="Your tririga password.")
@arg('filename', help="The file to upload.")
@arg('url', help="The URL to the TRIRIGA environment. Include any context paths")
def upload(url, filename, username="system", password="admin",
           module=None, businessobject=None, form=None, wait=False):
    """
Uploads a file to Data Integrator.

The module, businessobject and form arguments are optional if the file is named
in the following pattern "<module>-<businessObject>-<form>.txt". Otherwise,
you must specify them as arguments.


    """

    s = requests.Session()
    site_url = normalize_url(url)

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

    transition = transitions[0]

    authpayload = {
        'USERNAME': username,
        'PASSWORD': password,
        'objectId': TRIRIGA_AUTH_OBJECTID,
        'actionId': TRIRIGA_AUTH_FORCELOGIN_ACTIONID,
    }

    url = '{}/WebProcess.srv'.format(site_url)
    r = s.post(url, data=authpayload, allow_redirects=False)

    files = {'theFile': open(filename, 'rb')}
    filenameonly = os.path.basename(filename)

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

    # 1. Upload the file contents
    url = '{}/html/en/default/common/dataUploadFile.jsp'.format(site_url)
    r = s.post(url, params=diparams, files=files)

    # 2. Create the Data Upload job
    url = '{}/html/en/default/common/dataSmartUpload.jsp'.format(site_url)
    r = s.post(url, params=diparams)

    if wait:
        wait_for_upload(filenameonly, site_url, username, password)

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

def wait_for_upload(filename, site_url, username, password):
    """
    Wait until the upload of a given file completes.

    The method will give up after 10 minutes
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

    projectName = ""
    moduleName = "System"
    objectTypeName = "Data Upload"
    queryName = "Data Upload - System - Manager Query"
    start = 1
    maximumResultCount = 999

    ok_status = ("Rollup All Completed", "Failed")
    processing_status = ("NEW", "DONE", "UPLOADING...")

    for x in xrange(1, 60):
        # We will not run a continuation query.
        # If you uploaded more than 999 files with the same name, screw you.
        results = client.service.runNamedQuery(
            projectName, moduleName, objectTypeName, queryName, filters, start,
            maximumResultCount
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
                    if column.value in processing_status:
                        found_processing = True

        if not found_processing:
            break

        time.sleep(10)

