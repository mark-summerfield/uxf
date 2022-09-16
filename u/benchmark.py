#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import pathlib
import sys

import util

try:
    PATH = pathlib.Path(__file__).parent.parent.resolve()
finally:
    pass


EXE_FOR_LANG = {
    'py': ['python3', PATH / 'py/uxf.py'],
    'rs': [PATH / 'rs/target/release/uxf'],
    }


def main():
    scale, langs = get_config()

    print(PATH)
    print(EXE_FOR_LANG)
    print(scale, langs)


def get_config():
    tmin = 0
    tmax = sys.maxsize
    langs = set()
    for arg in sys.argv[1:]:
        if arg in {'-h', '--help'}:
            raise SystemExit(USAGE.format(', '.join(sorted(EXE_FOR_LANG))))
        elif arg in EXE_FOR_LANG:
            langs.add(arg)
        elif arg.isdecimal():
            scale = int(arg)
        else:
            raise SystemExit(f'error: unrecognized argument: {arg!r}')
    if not langs:
        raise SystemExit('error: must specify at least one lang from '
                         f'{sorted(EXE_FOR_LANG)}')
    return scale, tuple(langs)


USAGE = '''\
usage: benchmark.py [scale] <lang1> [lang2 ... langN]

scale  generate test UXF of this scale: 1 ~= 60K, 7 ~= 1MB
langX  from: {}
'''

if __name__ == '__main__':
    main()
