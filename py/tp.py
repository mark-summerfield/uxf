#!/usr/bin/env py310
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import os

import pretty
import uxf


def main():
    filename = os.path.dirname(__file__) + '/../testdata/t13.uxf'
    uxo = uxf.load(filename)


if __name__ == '__main__':
    main()
