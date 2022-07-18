#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import functools
import os
import sys

import eq

try:
    import uxf
except ImportError: # needed for development
    sys.path.append(os.path.abspath(os.path.dirname(__file__) + '/..'))
    import uxf


def main():
    equivalent = False
    filename1 = filename2 = None
    for arg in sys.argv[1:]:
        if arg in {'-e', '--equiv', '--equivalent'}:
            equivalent = True
        elif filename1 is None:
            filename1 = arg
        elif filename2 is None:
            filename2 = arg
    if (filename1 is not None and filename2 is not None and
            os.path.exists(filename1) and os.path.exists(filename2)):
        on_event = functools.partial(uxf.on_event, verbose=False)
        eq = compare(filename1, filename2, equivalent=equivalent,
                     on_event=on_event)
        print(f'{filename1} {"==" if eq else "!="} {filename2}')
    else:
        raise SystemExit(
            'usage: compare.py [-e|--equiv[alent]] file1.uxf file2.uxf')


def compare(filename1: str, filename2: str, *, equivalent=False,
            on_event=uxf.on_event):
    '''If equivalent=False, returns True if filename1 is the same as
    filename2 (ignoring insignificant whitespace); otherwise returns False.
    If equivalent=True, returns True if filename1 is equivalent to filename2
    (i.e., the same ignoring insignificant whitespace, ignoring any unused
    ttypes, and, in effect replacing any imports with the ttypes the
    define—if they are used); otherwise returns False.'''
    drop_unused = replace_imports = equivalent
    d = dict(drop_unused=drop_unused, replace_imports=replace_imports,
             on_event=on_event)
    try:
        uxo1 = uxf.load(filename1, **d)
    except uxf.Error as err:
        print(f'compare.py failed on {filename1}: {err}')
        return False
    try:
        uxo2 = uxf.load(filename2, **d)
    except uxf.Error as err:
        print(f'compare.py failed on {filename2}: {err}')
        return False
    return eq.eq(uxo1, uxo2)


if __name__ == '__main__':
    main()
