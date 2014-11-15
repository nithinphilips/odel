# -*- coding: utf-8 -*-
# PYTHON_ARGCOMPLETE_OK

"""odel.odel: provides entry point main()."""


__version__ = "0.2.0"


#import argcomplete
import argparse
#import logging
#import sys

from argh import ArghParser, completion, set_default_command

from .diuploader import upload

# These arguments are used by this global dispatcher and each individual
# stand-alone commands.
COMMON_PARSER = argparse.ArgumentParser(add_help=False)
COMMON_PARSER.add_argument('--debug',
                           action='store_true',
                           default=False,
                           help="Enable debug logging.")

def main():
    """Main application entrypoint"""
    parser = ArghParser()
    set_default_command(parser, upload)
    completion.autocomplete(parser)

    # Parse ahead
    #args = parser.parse_args()
    #if args.debug:
    #    logging.basicConfig(
    #        level=logging.DEBUG,
    #        format='%(asctime)s %(levelname)s: %(message)s'
    #    )

    parser.dispatch()
