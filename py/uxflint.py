#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import os
import sys

import uxf


def main():
    if len(sys.argv) == 1 or sys.argv[1] in {'-h', '--help'}:
        raise SystemExit('usage: uxflint.py <file1> [file2 [file3 ...]]')
    for filename in sys.argv[1:]:
        if os.path.isfile(filename):
            try:
                uxf.load(filename, on_event=on_event)
            except (uxf.Error, OSError) as err:
                print(f'uxflint.py:{filename}:{err}')


def on_event(*args, **kwargs):
    kwargs['prefix'] = 'uxflint'
    print(uxf.Error(*args, **kwargs))


if __name__ == '__main__':
    main()
