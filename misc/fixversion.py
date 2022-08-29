#!/usr/bin/env python3
# Copyright © 2022 Qtrac Ltd. All rights reserved.

import gzip
import os
import sys


def main():
    if len(sys.argv) > 1 and sys.argv[1] in {'-h', '--help'}:
        raise SystemExit('''usage: {}
Fixes the version of all UXF files in the current directory and its
subdirectories.'''.format(os.path.basename(sys.argv[0])))
    count = 0
    for root, _, files in os.walk('.'):
        for filename in files:
            filename = os.path.join(root, filename)
            count == maybe_fix_version(filename)
    print(f'fixed {count:,} files')


def maybe_fix_version(filename):
    opener = open
    try:
        try:
            with gzip.open(filename, 'rt', encoding='utf-8') as file:
                text = file.read()
                opener = gzip.open
        except gzip.BadGzipFile:
            with open(filename, 'rt', encoding='utf-8') as file:
                text = file.read()
        if text.startswith('uxf 1.0'):
            text = text[:5] + text[7:]
            with opener(filename, 'wt', encoding='utf-8') as file:
                file.write(text)
            print(f'fixed {filename}')
            return 1
    except OSError as err:
        print(err)
        return 0


if __name__ == '__main__':
    main()
