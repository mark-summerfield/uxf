#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

'''
Tests and shows how to do one-way conversions of sets, frozensets, tuples,
and deques into a Uxf object, and how to handle round-trippable custom data
including enums, complex numbers, and a custom type.
'''

import sys
import os


try:
    PATH = os.path.abspath(os.path.dirname(__file__))
    sys.path.append(os.path.abspath(os.path.join(PATH, '../')))
    import uxf
    os.chdir(os.path.join(PATH, '../../testdata')) # move to test data
finally:
    pass


def main():
    regression = False
    if len(sys.argv) > 1 and sys.argv[1] in {'-r', '--regression'}:
        regression = True
    total = ok = 0

    tclass = uxf.TClass('point', ('x', 'y'))
    t = uxf.Table(tclass)
    # append
    t.append((1, -6))
    t.append((3, 21))
    t.append((-4, 8))
    t.append((5, 17))
    # insert
    t.insert(1, (-2, 19))
    total += 1
    p = t[3]
    if p.x == -4 and p.y == 8:
        ok += 1
    elif not regression:
        print('fail #1')
    # __setitem__
    t[1] = (-20, 191)
    total += 1
    # __getitem__
    p = t[1]
    if p.x == -20 and p.y == 191:
        ok += 1
    elif not regression:
        print('fail #2')
    # len()
    total += 1
    if len(t) == 5:
        ok += 1
    elif not regression:
        print('fail #3')
    # __delitem__
    del t[3]
    total += 1
    if len(t) == 4:
        ok += 1
    elif not regression:
        print('fail #4')
    # __iter__
    total += 1
    expected = [t.RecordClass(*p)
                for p in [(1, -6), (-20, 191), (3, 21), (5, 17)]]
    if list(t) == expected:
        ok += 1
    elif not regression:
        print('fail #5')
    # properties
    total += 1
    if t.ttype == 'point':
        ok += 1
    elif not regression:
        print('fail #6')
    total += 1
    if t.fields == [uxf.Field('x'), uxf.Field('y')]:
        ok += 1
    elif not regression:
        print('fail #7')

    total += 1
    if tuple(t.first) == (1, -6):
        ok += 1
    elif not regression:
        print('fail #8')

    total += 1
    if tuple(t.second) == (-20, 191):
        ok += 1
    elif not regression:
        print('fail #9')

    total += 1
    if list(t.third) == [3, 21]:
        ok += 1
    elif not regression:
        print('fail #10')

    total += 1
    if list(t.fourth) == [5, 17]:
        ok += 1
    elif not regression:
        print('fail #11')

    total += 1
    if tuple(t.last) == (5, 17):
        ok += 1
    elif not regression:
        print('fail #12')

    total += 1
    if t.second.x == -20 and t.second.y == 191:
        ok += 1
    elif not regression:
        print('fail #13')

    # editing row refs
    total += 1
    t.second.x = abs(t.second.x)
    t[1].y -= 100
    if t.second.x == 20 and t.second.y == 91:
        ok += 1
    elif not regression:
        print('fail #14', t.second)

    total += 1
    t[3].x *= 2
    t[3].y -= 5
    if tuple(t[3]) == (10, 12):
        ok += 1
    elif not regression:
        print('fail #15')

    # errors (see test_errors.py for 320 340
    try:
        total += 1
        t.append((-7, -8, -9))
        fail('test_table • expected TypeError FAIL', regression)
    except TypeError:
        ok += 1

    print(f'total={total} ok={ok}')


def got_error(code, err, regression):
    err = str(err)
    code = f'#{code}:'
    if code not in err:
        fail(f'test_errors • expected {code} got, {err!r} FAIL',
             regression)
        return 0
    return 1


def fail(message, regression):
    if not regression:
        print(message)


if __name__ == '__main__':
    main()
