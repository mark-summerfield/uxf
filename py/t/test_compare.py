#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import filecmp
import functools
import os
import sys
import tempfile

try:
    PATH = os.path.abspath(os.path.dirname(__file__))
    sys.path.append(os.path.abspath(os.path.join(PATH, '../')))
    import uxf
    sys.path.append(os.path.abspath(os.path.join(PATH, '../eg/')))
    import compare
    os.chdir(os.path.join(PATH, '../../testdata')) # move to test data
finally:
    pass


def main():
    regression = False
    if len(sys.argv) > 1 and sys.argv[1] in {'-r', '--regression'}:
        regression = True
    total = ok = 0
    on_error = functools.partial(uxf.on_error, verbose=False)

    # Two files with the equivalent UXF content; but different actual
    # content
    filename1 = 't63.uxf'
    filename2 = os.path.join(tempfile.gettempdir(), '63.uxf')
    uxo = uxf.load(filename1, drop_unused=True, replace_imports=True)
    uxo.dump(filename2, on_error=on_error)
    total, ok = test(total, ok, regression, 1, filename1, filename2,
                     different=False, equal=False, equivalent=True)

    filename1 = 't13.uxf'
    filename2 = 'expected/t13.uxf'
    total, ok = test(total, ok, regression, 2, filename1, filename2,
                     different=False, equal=True, equivalent=True)

    # Compare with self
    filename2 = 't13.uxf'
    total, ok = test(total, ok, regression, 3, filename1, filename2,
                     different=True, equal=True, equivalent=True)

    filename1 = filename2 = 't12.uxf'
    total, ok = test(total, ok, regression, 4, filename1, filename2,
                     different=True, equal=True, equivalent=True)

    # Compare with different
    filename2 = 't11.uxf'
    total, ok = test(total, ok, regression, 5, filename1, filename2,
                     different=False, equal=False, equivalent=False)

    # Compare with maps with completely different key orders
    filename1 = 't77.uxf'
    filename2 = 't78.uxf'
    total, ok = test(total, ok, regression, 6, filename1, filename2,
                     different=False, equal=True, equivalent=True)

    print(f'total={total} ok={ok}')


def test(total, ok, regression, n, filename1, filename2, *, different,
         equal, equivalent):
    on_error = functools.partial(uxf.on_error, verbose=False)

    total += 1
    if filecmp.cmp(filename1, filename2, shallow=False) == different:
        ok += 1
    elif not regression:
        print(f'{n}.1 filecmp.cmp() • FAIL files compared unexpectedly the '
              'same')

    total += 1
    if compare.compare(filename1, filename2, on_error=on_error) == equal:
        ok += 1
    elif not regression:
        print(f'{n}.2 compare() • FAIL files compared unexpectedly unequal')

    total += 1
    if compare.compare(filename1, filename2, equivalent=True,
                       on_error=on_error) == equivalent:
        ok += 1
    elif not regression:
        print(f'{n}.3 compare() • FAIL files compared unexpectedly '
              'nonequivalent')
    return total, ok


if __name__ == '__main__':
    main()
