#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

'''
Tests and shows how to convert to/from SQLite.

Such conversions are normally *lossy* in terms of datatypes and structure,
but _not_ in terms of data values of course.

In practice you'd always create your own custom database-specific code and
use the uxf module directly.
'''

import contextlib
import functools
import os
import sys
import tempfile

try:
    PATH = os.path.abspath(os.path.dirname(__file__))
    sys.path.append(os.path.abspath(os.path.join(PATH, '../')))
    import uxf
    import uxfconvert
    os.chdir(os.path.join(PATH, '../../testdata')) # move to test data
finally:
    pass


SUITABLE = ('t15.uxf', 't19.uxf', 't35.uxf', 't36.uxf', 't37.uxf', 't5.uxf')


def main():
    regression = False
    if len(sys.argv) > 1 and sys.argv[1] in {'-r', '--regression'}:
        regression = True
    total = ok = 0
    for name in SUITABLE:
        total, ok = check(total, ok, name, regression)
    print(f'total={total} ok={ok}')


def check(total, ok, name, regression):
    on_event = functools.partial(uxf.on_event, verbose=not regression)
    uxo1 = uxf.load(name, on_event=on_event)
    filename = os.path.join(tempfile.gettempdir(), name.replace('.uxf',
                                                                '.sqlite'))
    with contextlib.suppress(FileNotFoundError):
        os.remove(filename)
    if isinstance(uxo1.value, uxf.Table):
        uxo1.value = [uxo1.value]
    # Our simple SQLite converters can't handle Uxf custom strings or field
    # types. (All this could be done of course.)
    uxo1.custom = ''
    uxfconvert._uxf_to_sqlite(filename, uxo1.value)
    uxo2 = uxfconvert._sqlite_to_uxf(filename)
    total += 1
    if uxo1.is_equivalent(uxo2, uxf.Compare.UNTYPED_EQUIVALENT):
        ok += 1
        if not regression:
            print(f'test_sqlite • {name} OK')
    else:
        if not regression:
            print(f'test_sqlite • {name} FAIL')
    with contextlib.suppress(FileNotFoundError):
        os.remove(filename)
    return total, ok


if __name__ == '__main__':
    main()
