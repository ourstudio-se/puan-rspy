#!/usr/bin/env python

from distutils.core import setup

setup(
    name='puan-rspy',
    version='0.1.1',
    description='Puan Python Rust interface',
    author_email='rikard@ourstudio.se',
    packages=['distutils', 'distutils.command'],
    install_requires=[
        "maturin>=0.13,<0.14"
    ],
)