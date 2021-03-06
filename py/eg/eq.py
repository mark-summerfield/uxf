#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import collections
import math
import pprint

try:
    import os
    import sys
    sys.path.append(os.path.abspath(os.path.dirname(__file__) + '/..'))
    import uxf
finally:
    pass


def eq(a, b, *, ignore_comments=False, ignore_custom=False,
       ignore_types=False, debug=False):
    '''Returns True if a and b are eq Uxf's, Tables, Lists, or Maps;
    otherwise returns False. This function compares all the values (and
    comments) recursively, so is potentially expensive.
    Maps are compared by ktype and vtype and then by their dict data with
    items sorted by str(key). For example, although Python dicts are
    insertion-ordered, Maps are compared regardless of order:

        >>> import datetime
        >>> when = datetime.date(2022, 9, 19)
        >>> later = when + datetime.timedelta(days=27)
        >>> ma = uxf.Map(dict(x=1, y=2, z=4), ktype='str', vtype='int')
        >>> lst1 = [later, 349.85, 'Pear Tree', ma]
        >>> mb = uxf.Map(dict(z=4), ktype='str', vtype='int')
        >>> mb['y'] = 2
        >>> mb['x'] = 1
        >>> lst2 = [later, 349.850, 'Pear ' + 'Tree', mb]
        >>> m1 = uxf.Map({1: 'a', 2: when, 'three': lst1})
        >>> m1
        {1: 'a', 2: datetime.date(2022, 9, 19), 'three': \
[datetime.date(2022, 10, 16), 349.85, 'Pear Tree', \
{'x': 1, 'y': 2, 'z': 4}]}
        >>> m2 = uxf.Map({'three': lst2, 1: 'a', 2: when})
        >>> m2
        {'three': [datetime.date(2022, 10, 16), 349.85, 'Pear Tree', \
{'z': 4, 'y': 2, 'x': 1}], 1: 'a', 2: datetime.date(2022, 9, 19)}
        >>> m1 == m2
        True
        >>> m1.data == m2.data
        True

    This function requires that values in sets are sortable.
    '''
    def by_key(item):
        return str(item[0])

    def eq_text(a, b):
        '''Returns True if a and b are both either empty or None or have the
        same nonempty text; otherwise False.'''
        return (not bool(a) and not bool(b)) or a == b

    kwargs = dict(ignore_comments=ignore_comments,
                  ignore_custom=ignore_custom, ignore_types=ignore_types,
                  debug=debug)

    if a.__class__.__name__.startswith('UXF_'):
        a = tuple(a)
    if b.__class__.__name__.startswith('UXF_'):
        b = tuple(b)

    if isinstance(a, uxf.Uxf):
        if not isinstance(b, uxf.Uxf):
            if debug:
                _fail(f'Uxf can\'t be compared with {b.__class__.__name__}')
            return False
        if not ignore_custom and not eq_text(a.custom, b.custom):
            if debug:
                _fail('custom', a.custom, b.custom)
            return False
        if not ignore_comments and not eq_text(a.comment, b.comment):
            if debug:
                _fail('Uxf.comment', a.comment, b.comment)
            return False
        if not ignore_types:
            if not eq(a.tclasses, b.tclasses, **kwargs):
                if debug:
                    _fail('Uxf.tclasses', a.tclasses, b.tclasses)
                return False
            if a.imports != b.imports:
                if debug:
                    _fail('Uxf.imports', a.imports, b.imports)
                return False
        if not eq(a.value, b.value, **kwargs):
            if debug:
                _fail('Uxf.value', a.value, b.value)
            return False
        return True
    if isinstance(a, uxf.List):
        if not isinstance(b, uxf.List):
            if debug:
                _fail(
                    f'List can\'t be compared with {b.__class__.__name__}')
            return False
        if not ignore_comments and not eq_text(a.comment, b.comment):
            if debug:
                _fail('List.comment', a.comment, b.comment)
            return False
        if not ignore_types and a.vtype != b.vtype:
            if debug:
                _fail('List.vtype', a.vtype, b.vtype)
            return False
        if not eq(a.data, b.data, **kwargs):
            if debug:
                _fail('List.data', a.data, b.data)
            return False
        return True
    if isinstance(a, uxf.Map):
        if not isinstance(b, uxf.Map):
            if debug:
                _fail(f'Map can\'t be compared with {b.__class__.__name__}')
            return False
        if not ignore_comments and not eq_text(a.comment, b.comment):
            if debug:
                _fail('Map.comment', a.comment, b.comment)
            return False
        if not ignore_types:
            if a.ktype != b.ktype:
                if debug:
                    _fail('Map.ktype', a.ktype, b.ktype)
                return False
            if a.vtype != b.vtype:
                if debug:
                    _fail('Map.vtype', a.vtype, b.vtype)
                return False
        if not eq(a.data, b.data, **kwargs): # Compare's the dict data
            if debug:
                _fail('Map.data', a.data, b.data)
            return False
        return True
    if isinstance(a, uxf.TClass):
        if not isinstance(b, uxf.TClass):
            if debug:
                _fail('TClass can\'t be compared with '
                      f'{b.__class__.__name__}')
            return False
        if not ignore_comments and not eq_text(a.comment, b.comment):
            if debug:
                _fail('TClass.comment', a.comment, b.comment)
            return False
        if a.ttype != b.ttype:
            if debug:
                _fail('TClass.ttype', a.ttype, b.ttype)
            return False
        if len(a.fields) != len(b.fields):
            if debug:
                _fail('TClass.fields (len)', a.fields, b.fields)
            return False
        for afield, bfield in zip(a.fields, b.fields):
            if afield.name != bfield.name:
                if debug:
                    _fail('TClass.fields (name)', afield, bfield)
                return False
            if not ignore_types and afield.vtype != bfield.vtype:
                if debug:
                    _fail('TClass.fields (vtype)', afield, bfield)
                return False
        return True
    if isinstance(a, uxf.Table):
        if not isinstance(b, uxf.Table):
            if debug:
                _fail(
                    f'Table can\'t be compared with {b.__class__.__name__}')
            return False
        if not ignore_comments and not eq_text(a.comment, b.comment):
            if debug:
                _fail('Table.comment', a.comment, b.comment)
            return False
        if a.ttype != b.ttype:
            if debug:
                _fail('Table.ttype', a.ttype, b.ttype)
            return False
        if not ignore_types and not eq(a.tclass, b.tclass, **kwargs):
            if debug:
                _fail('Table.tclass', a.tclass, b.tclass)
            return False
        for i, (arec, brec) in enumerate(zip(iter(a), iter(b))):
            if not eq(arec, brec, **kwargs):
                if debug:
                    _fail(f'Table[{i}]', arec, brec)
                return False
        return True
    if isinstance(a, (list, tuple, collections.deque)):
        if len(a) != len(b):
            if debug:
                _fail('List (len)', a, b)
            return False
        for i, (avalue, bvalue) in enumerate(zip(a, b)):
            if not eq(avalue, bvalue, **kwargs):
                if debug:
                    _fail(f'List[{i}]', avalue, bvalue)
                return False
        return True
    if isinstance(a, (set, frozenset)):
        if len(a) != len(b):
            if debug:
                _fail('set (len)', a, b)
            return False
        for i, (avalue, bvalue) in enumerate(zip(sorted(a), sorted(b))):
            if not eq(avalue, bvalue, **kwargs):
                if debug:
                    _fail('sorted(set)[i]', avalue, bvalue)
                return False
        return True
    if isinstance(a, dict):
        if len(a) != len(b):
            if debug:
                _fail('dict (len)', a, b)
            return False
        for (akey, avalue), (bkey, bvalue) in zip( # Compares irrespective
                sorted(a.items(), key=by_key),     # of original order
                sorted(b.items(), key=by_key)):
            if akey != bkey:
                if debug:
                    _fail('dict (key)', akey, bkey)
                return False
            if not eq(avalue, bvalue, **kwargs):
                if debug:
                    _fail('dict (value)', avalue, bvalue)
                return False
        return True
    if isinstance(a, float):
        if not math.isclose(a, b):
            if debug:
                _fail('float', a, b)
            return False
        return True
    if a != b:
        if debug:
            _fail('generic', a, b)
        return False
    return True


def _fail(where, a, b):
    print(f'{where} a != b {a == b}')
    print()
    print('type(a)', type(a))
    print()
    _pprint(a)
    print()
    print('type(b)', type(b))
    print()
    _pprint(b)
    raise SystemExit()


def _pprint(x):
    if isinstance(x, uxf.Table):
        _print_table(x)
    elif isinstance(x, uxf.List):
        _print_list(x)
    else:
        pprint.pprint(x)


def _print_table(t):
    print('name', t.ttype)
    print('comment', t.comment)
    print('fields:')
    pprint.pprint(t.fields)
    print('records:')
    for record in t:
        _pprint(record)


def _print_list(t):
    print('comment', t.comment)
    print('values:')
    for value in t:
        _pprint(value)


if __name__ == '__main__':
    import doctest
    doctest.testmod()
