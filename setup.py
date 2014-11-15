# -*- coding: utf-8 -*-


"""setup.py: setuptools control."""


import re
from setuptools import setup


version = re.search(
    '^__version__\s*=\s*"(.*)"',
    open('odel/odel.py').read(),
    re.M
    ).group(1)


with open("README.rst", "rb") as f:
    long_descr = f.read().decode("utf-8")


setup(
    name = "cmdline-odel",
    packages = ["odel"],
    entry_points = {
        "console_scripts": ['odel = odel.odel:main']
        },
    install_requires = [
        "argh",
        "argcomplete",
        "bunch",
        "suds",
        "mechanize"
    ],
    version = version,
    description = "Python command line application bare bones template.",
    long_description = long_descr,
    author = "Jan-Philip Gehrcke",
    author_email = "jgehrcke@googlemail.com",
    url = "http://gehrcke.de/2014/02/distributing-a-python-command-line-application",
    )
