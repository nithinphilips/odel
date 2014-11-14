# -*- coding: utf-8 -*-
# PYTHON_ARGCOMPLETE_OK

"""bootstrap.bootstrap: provides entry point main()."""


__version__ = "0.2.0"


import sys
import logging
from argh import *
import argcomplete
from .stuff import Stuff

def main():
    parser = ArghParser()
    parser.add_commands([ting])
    logging.basicConfig(level=logging.DEBUG, format='%(asctime)s %(levelname)s: %(message)s')
    completion.autocomplete(parser)
    parser.dispatch()

# adding help to `foo` which is in the function signature:
@arg('foo', help='blah')
# these are not in the signature so they go to **kwargs:
@arg('baz')
@arg('-q', '--quux')
# the function itself:
def ting(foo, bar=1, *args, **kwargs):
    yield foo
    yield bar
    yield ', '.join(args)
    yield kwargs['baz']
    yield kwargs['quux']

class Boo(Stuff):
    pass
