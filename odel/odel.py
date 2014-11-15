# -*- coding: utf-8 -*-
# PYTHON_ARGCOMPLETE_OK

"""odel.odel: provides entry point main()."""


__version__ = "0.2.0"


import sys
import logging
from argh import *
import argcomplete
from .stuff import Stuff

def main():
    parser = ArghParser()
    parser.add_commands([ting, pang, parse_filename])
    #logging.basicConfig(level=logging.DEBUG, format='%(asctime)s %(levelname)s: %(message)s')
    completion.autocomplete(parser)
    parser.dispatch()

from suds.plugin import MessagePlugin
from suds.client import Client

import cookielib
import requests

import os
import sys

site_url="http://192.168.0.107:9080"

def parse_filename(filename):
    """
    Extract Module, BO, and Form name from filename.
    """

    basename = os.path.basename(filename)
    basename = os.path.splitext(basename)[0]

    return basename.split('-')


def pang(filename, module=None, businessobject=None, form=None):
    s = requests.Session()

    if not module or not businessobject or not form:
        module, businessobject, form = parse_filename(filename)

    if not module or not businessobject or not form:
        sys.stderr.write("Unable to detect module, business object and form name from filename. Please provide them.\n")
        return

    moduleid, businessobjectid, formid, transitions = ting(module, businessobject, form)

    transition = transitions[0]

    authpayload = {
                    'USERNAME': 'xphilin',
                    'PASSWORD': 'password',
                    'objectId': '1000',
                    'actionId': '1006' # 1005 for regular login. 1006 to force
    }

    url = '{}/WebProcess.srv'.format(site_url)
    r = s.post(url, data=authpayload, allow_redirects=False)

    #print r.headers
    #print r.text

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

    url = '{}/html/en/default/common/dataUploadFile.jsp'.format(site_url)
    r = s.post(url, params=diparams, files=files)


    url = '{}/html/en/default/common/dataSmartUpload.jsp'.format(site_url)
    r = s.post(url, params=diparams)
    #print(r.url)
    #print r.headers

def ting(module, businessobject, form):
    WSDL_URL = '{}/ws/TririgaWS?wsdl'.format(site_url)
    HEADERS = {"Username": "system", "Password" : "admin"}

    class MultipartMimeFilter(MessagePlugin):
        def received(self, context):
            context.reply = context.reply.split('\n')[6]

    client = Client(WSDL_URL, headers=HEADERS, plugins=[MultipartMimeFilter()])

    moduleid = client.service.getModuleId(module)
    objectid = client.service.getObjectTypeId(module, businessobject)
    guiid = None
    transitions = []

    guis = client.service.getGUIs(objectid)[0]

    for gui in guis:
        if gui.name == form:
            guiid = gui.id
            break

    #states = client.service.getGUIStateTransitions(objectid, guiid)[0]
    #for state in states:
    #    if state.state == "null":
    #        for transition in state.trans[0]:
    #            transitions.append(transition.action)

    actions = client.service.getObjectTypeActions(moduleid, objectid, -1, -1)[0]

    for action in actions:
        for aaction in action[1]:
            transitions.append(aaction.action)


    return moduleid, objectid, guiid, transitions

