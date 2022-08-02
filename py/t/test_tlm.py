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
    total = ok = 0

    total += 1
    expected = 'tlm-eg.uxx.gz'
    tlm1 = Tlm.Model(expected)
    ok += 1

    total += 1
    actual_tlm = 'actual/1.tlm'
    tlm1.save(filename=actual_tlm)
    ok += 1

    total += 1
    tlm2 = Tlm.Model(actual_tlm)
    actual_uxf = 'actual/1.uxf.gz'
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
        print('unequal')

    total += 1
    with gzip.open('expected/tlm-eg.uxx.gz', 'rt',
                   encoding='utf-8') as file:
        uxt3 = file.read().rstrip()
    if uxo1.dumps().rstrip() == uxt3:
        ok += 1
    elif not regression:
        print('unequal text formats')

    print(f'total={total} ok={ok}')


if __name__ == '__main__':
    main()
