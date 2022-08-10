#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import contextlib
import filecmp
import io
import os
import sys

try:
    PATH = os.path.abspath(os.path.dirname(__file__))
    sys.path.append(os.path.abspath(os.path.join(PATH, '../eg/')))
    import visit
    os.chdir(os.path.join(PATH, '../../testdata')) # move to test data
finally:
    pass


def main():
    total = ok = 0

    out = 't5tree.txt'
    err = 't5err.txt'
    actual_err = io.StringIO()
    with contextlib.redirect_stderr(actual_err):
        visit.visit('t5.uxf', f'actual/{out}')
    total = 2
    if filecmp.cmp(f'expected/{out}', f'actual/{out}', shallow=False):
        ok += 1
    with open(f'expected/{err}', 'rt', encoding='utf-8') as file:
        expected_err = file.read()
    if expected_err == actual_err.getvalue():
        ok += 1

    print(f'total={total} ok={ok}')


if __name__ == '__main__':
    main()
