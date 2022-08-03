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
    tclasses = {}
    for ttype, tclass in uxo1.tclasses.items():
        fields = [field.name for field in tclass.fields]
        tclass = uxf.TClass(ttype, fields=fields, comment=tclass.comment)
        tclasses[ttype] = tclass
    uxo1.tclasses = tclasses
    # NOTE I don't really know why normalizing is necessary.
    # The only difference dumps() shows is in the comments & these are all
    # correctly ignored.
    #uxo1 = uxf.loads(uxo1.dumps()) # normalize
    uxfconvert._uxf_to_sqlite(filename, uxo1.value)
    uxo2 = uxfconvert._sqlite_to_uxf(filename)
    #uxo2 = uxf.loads(uxo2.dumps()) # normalize
    total += 1
    if uxo1.is_equivalent(uxo2, uxf.Compare.EQUIVALENT):
        ok += 1
        if not regression:
            print(f'test_sqlite • {name} OK')
    else:
        if not regression:
            print(f'test_sqlite • {name} FAIL')
            #debug(1, uxo1);debug(2, uxo2); raise SystemExit
    with contextlib.suppress(FileNotFoundError):
        os.remove(filename)
    return total, ok


def debug(i, uxo):
    with open(f'/tmp/{i}.txt', 'wt', encoding='utf-8') as file:
        file.write(f'custom={uxo.custom!r}\n')
        file.write(f'comment={uxo.comment!r}\n')
        for ttype, tclass in sorted(uxo.tclasses.items()):
            file.write(f'TClass={ttype!r} comment={tclass.comment!r}\n')
            for field in sorted(tclass.fields):
                file.write(f'  Field({field.name!r}, {field.vtype!r})\n')
        debug_value(file, 0, uxo.value)
        print('wrote', file.name)


def debug_value(file, i, value):
    if uxf.is_scalar(value):
        file.write(f'#{i}:{type(value)}={value!r}\n')
    elif isinstance(value, uxf.List):
        file.write(f'#{i}:{type(value)} vtype={value.vtype!r} '
                   f'comment={value.comment!r}\n')
        for j, item in enumerate(value):
            debug_value(file, f'{i}[{j}]', item)
    elif isinstance(value, uxf.Table):
        file.write(f'#{i}:{type(value)} ttype={value.ttype!r} '
                   f'comment={value.comment!r}\n')
        for j, record in enumerate(value):
            for k, field in enumerate(record):
                debug_value(file, f'{i}[{j}][{k}]', field)


if __name__ == '__main__':
    main()
