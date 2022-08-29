#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

'''
See eg/merge.py for more about the merge() function.

Example 'include' file:

uxf 1 UXF Include
#<e.g., include.uxf>
=include filename:str
(include
  <file1.uxf>
  <file2.uxf>
  <file3.uxf>
)
'''

import os
import sys

import merge

try:
    import uxf
except ImportError: # needed for development
    sys.path.append(os.path.abspath(os.path.dirname(__file__) + '/..'))
    import uxf


def main():
    if len(sys.argv) < 2 or sys.argv[1] in {'-h', '--help'}:
        raise SystemExit('usage: include.py <include.uxf> [<outfile.uxf>]')
    include_uxo = uxf.load(sys.argv[1])
    filenames = [record.filename for record in include_uxo.value]
    uxo = merge.merge(*filenames, asmap=False)
    if len(sys.argv) > 2:
        uxo.dump(sys.argv[2])
    else:
        print(uxo.dumps())


if __name__ == '__main__':
    main()
