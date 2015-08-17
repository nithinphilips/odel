# -*- coding: utf-8 -*-
# PYTHON_ARGCOMPLETE_OK

"""odel.odel: provides entry point main()."""


__version__ = "0.4.0"


import argparse
import logging
import sys

from argh import ArghParser, completion, set_default_command

from .diuploader import upload

# These arguments are used by this global dispatcher and each individual
# stand-alone commands.
COMMON_PARSER = argparse.ArgumentParser(add_help=False)
COMMON_PARSER.add_argument('--debug',
                           action='store_true',
                           default=False,
                           help="Enable debug logging.")
COMMON_PARSER.add_argument('--version',
                           action='store_true',
                           default=False,
                           help="Print version information and quit.")

def main():
    """Main application entrypoint"""
    parser = ArghParser(parents=[COMMON_PARSER])
    set_default_command(parser, upload)
    completion.autocomplete(parser)

    # Peek CLI arguments and globally enable debugging
    args = parser.parse_args()
    if args.debug:
        logging.basicConfig(
            level=logging.DEBUG,
            format='%(asctime)s %(levelname)s: %(message)s'
        )

        # Cut down suds logs
        logging.getLogger('suds').setLevel(logging.WARNING)

    if args.version:
        print_version()
        return

    parser.dispatch()


def print_version():
    sys.stdout.write(
            "Odel. Tool to upload Data Integrator files to IBM Tririga.\n"
            "Version {}\n"
            "\n"
            "Copyright (C) 2014 Nithin Philips\n"
            "\n"
            "This program is free software: you can redistribute it and/or modify\n"
            "it under the terms of the GNU General Public License as published by\n"
            "the Free Software Foundation, either version 3 of the License, or\n"
            "(at your option) any later version.\n"
            "\n"
            "This program is distributed in the hope that it will be useful,\n"
            "but WITHOUT ANY WARRANTY; without even the implied warranty of\n"
            "MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n"
            "GNU General Public License for more details.\n"
            "\n"
            "You should have received a copy of the GNU General Public License\n"
            "along with this program.  If not, see <http://www.gnu.org/licenses/>.\n".format(__version__)
    )
