# -*- coding: utf-8 -*-


"""setup.py: setuptools control."""


import re
from setuptools import setup, find_packages


version = re.search(
    '^__version__\s*=\s*"(.*)"',
    open('odel/odel.py').read(),
    re.M
    ).group(1)


with open("README.rst", "rb") as f:
    long_descr = f.read().decode("utf-8")


setup(
    name = "cmdline-odel",
    packages = find_packages(exclude=['contrib', 'docs', 'tests*']),
    entry_points = {
        "console_scripts": ['odel = odel.odel:main']
        },
    install_requires = [
        "argh",
        "argcomplete",
        "bunch",
        "suds",
        "requests",
    ],
    version = version,
    description = "Odel uploads Data Integrator files to Tririga",
    long_description = long_descr,
    author = "Nithin Philips",
    author_email = "nithin@nithinphilips.com",
    url = "",
    test_suite="odel.tests",
    )
