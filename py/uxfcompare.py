#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import collections
import functools
import math
import os
import sys

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
    return eq(uxo1, uxo2)


def eq(a, b, *, ignore_comments=False, ignore_custom=False,
       ignore_types=False):
    '''Returns True if a and b are equal Uxf's, Tables, Lists, Maps, or
    scalars; otherwise returns False. This function compares all the values
    (and comments) recursively, so is potentially expensive. Maps are
    compared by ktype and vtype and then by their dict data with items
    sorted by str(key). For example, although Python dicts are
    insertion-ordered, UXF Maps are unordered, so they are compared
    regardless of order. This function requires that values in sets are
    sortable.
    '''
    def by_key(item):
        return str(item[0])

    def eq_text(a, b):
        '''Returns True if a and b are both either empty or None or have the
        same nonempty text; otherwise False.'''
        return (not bool(a) and not bool(b)) or a == b

    kwargs = dict(ignore_comments=ignore_comments,
                  ignore_custom=ignore_custom, ignore_types=ignore_types)

    if a.__class__.__name__.startswith('UXF_'):
        a = tuple(a)
    if b.__class__.__name__.startswith('UXF_'):
        b = tuple(b)

    if isinstance(a, uxf.Uxf):
        if not isinstance(b, uxf.Uxf):
            return False
        if not ignore_custom and not eq_text(a.custom, b.custom):
            return False
        if not ignore_comments and not eq_text(a.comment, b.comment):
            return False
        if not ignore_types:
            if not eq(a.tclasses, b.tclasses, **kwargs):
                return False
            if a.imports != b.imports:
                return False
        if not eq(a.value, b.value, **kwargs):
            return False
        return True
    if isinstance(a, uxf.List):
        if not isinstance(b, uxf.List):
            return False
        if not ignore_comments and not eq_text(a.comment, b.comment):
            return False
        if not ignore_types and a.vtype != b.vtype:
            return False
        if not eq(a.data, b.data, **kwargs):
            return False
        return True
    if isinstance(a, uxf.Map):
        if not isinstance(b, uxf.Map):
            return False
        if not ignore_comments and not eq_text(a.comment, b.comment):
            return False
        if not ignore_types:
            if a.ktype != b.ktype:
                return False
            if a.vtype != b.vtype:
                return False
        if not eq(a.data, b.data, **kwargs): # Compare's the dict data
            return False
        return True
    if isinstance(a, uxf.TClass):
        if not isinstance(b, uxf.TClass):
            return False
        if not ignore_comments and not eq_text(a.comment, b.comment):
            return False
        if a.ttype != b.ttype:
            return False
        if len(a.fields) != len(b.fields):
            return False
        for afield, bfield in zip(a.fields, b.fields):
            if afield.name != bfield.name:
                return False
            if not ignore_types and afield.vtype != bfield.vtype:
                return False
        return True
    if isinstance(a, uxf.Table):
        if not isinstance(b, uxf.Table):
            return False
        if not ignore_comments and not eq_text(a.comment, b.comment):
            return False
        if a.ttype != b.ttype:
            return False
        if not ignore_types and not eq(a.tclass, b.tclass, **kwargs):
            return False
        for i, (arec, brec) in enumerate(zip(iter(a), iter(b))):
            if not eq(arec, brec, **kwargs):
                return False
        return True
    if isinstance(a, (list, tuple, collections.deque)):
        if len(a) != len(b):
            return False
        for i, (avalue, bvalue) in enumerate(zip(a, b)):
            if not eq(avalue, bvalue, **kwargs):
                return False
        return True
    if isinstance(a, (set, frozenset)):
        if len(a) != len(b):
            return False
        for i, (avalue, bvalue) in enumerate(zip(sorted(a), sorted(b))):
            if not eq(avalue, bvalue, **kwargs):
                return False
        return True
    if isinstance(a, dict):
        if len(a) != len(b):
            return False
        for (akey, avalue), (bkey, bvalue) in zip( # Compares irrespective
                sorted(a.items(), key=by_key),     # of original order
                sorted(b.items(), key=by_key)):
            if akey != bkey:
                return False
            if not eq(avalue, bvalue, **kwargs):
                return False
        return True
    if isinstance(a, float):
        if not math.isclose(a, b):
            return False
        return True
    return a == b


if __name__ == '__main__':
    main()
