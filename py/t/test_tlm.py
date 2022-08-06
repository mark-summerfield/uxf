#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import gzip
import os
import sys

try:
    PATH = os.path.abspath(os.path.dirname(__file__))
    sys.path.append(os.path.abspath(os.path.join(PATH, '../')))
    import uxf
    sys.path.append(os.path.abspath(os.path.join(PATH, '../eg/')))
    import Tlm
    os.chdir(os.path.join(PATH, '../../testdata')) # move to test data
finally:
    pass


def main():
    regression = False
    if len(sys.argv) > 1 and sys.argv[1] in {'-r', '--regression'}:
        regression = True
    total, ok = test(1, 0, 0, regression) 
    total, ok = test(2, total, ok, regression) 
    print(f'total={total} ok={ok}')


def test(n, total, ok, regression):
    total += 1
    expected = f'tlm-eg{n}.uxx.gz'
    tlm1 = Tlm.Model(expected)
    ok += 1

    total += 1
    actual_tlm = f'actual/{n}.tlm'
    tlm1.save(filename=actual_tlm)
    ok += 1

    total += 1
    tlm2 = Tlm.Model(actual_tlm)
    actual_uxf = f'actual/{n}.uxf.gz'
    tlm2.save(filename=actual_uxf)
    ok += 1

    total += 1
    uxo1 = uxf.load(expected)
    ok += 1

    total += 1
    uxo2 = uxf.load(actual_uxf)
    ok += 1

    total += 1
    if uxo1 == uxo2:
        ok += 1
    elif not regression:
        print('unequal #1')

    total += 1
    uxo3 = uxf.load(f'expected/tlm-eg{n}.uxx.gz')
    if uxo1 == uxo3:
        ok += 1
    elif not regression:
        print('unequal #2')

    total += 1
    with gzip.open(f'expected/tlm-eg{n}.uxx.gz', 'rt',
                   encoding='utf-8') as file:
        uxt4 = file.read().rstrip()
    if uxo1.dumps().rstrip() == uxt4:
        ok += 1
    elif not regression:
        print('unequal text #3')

    return total, ok


if __name__ == '__main__':
    main()
