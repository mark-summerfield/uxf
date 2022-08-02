#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

'''
For another example of this file's merge() function see include.py.
'''

import os
import sys

try:
    import uxf
except ImportError: # needed for development
    PATH = os.path.abspath(os.path.dirname(__file__))
    sys.path.append(PATH + '/..')
    import uxf


def main():
    if len(sys.argv) < 2 or sys.argv[1] in {'-h', '--help'}:
        raise SystemExit('''usage: merge.py \
[-l|--list] [-|[-o|--outfile] <outfile>] \
<infile1> <infile2> [... <infileN>]
If -l or --list is specified, the outfile will contain a list where each
value is the correspoding infile's data. The default is for the outfile to
contain a map where each key is the name of an infile and each value the the
corresponding infile's data.
The outfile will be in UXF format. If no outfile is specified, output is \
to stdout.
Regardless of suffix, all infiles are assumed to be UXF format.''')
    asmap, outfile, infiles = get_config()
    try:
        uxo = merge(*infiles, asmap=asmap)
        if outfile != '-':
            uxo.dump(outfile)
            print(f'saved {outfile}')
        else:
            print(uxo.dumps())
    except uxf.Error as err:
        print(f'failed to merge: {err}')


def get_config():
    outfile = '-'
    infiles = []
    want_outfile = False
    asmap = True
    for arg in sys.argv[1:]:
        if want_outfile:
            outfile = arg
            want_outfile = False
        elif arg in {'-l', '--list'}:
            asmap = False
        elif arg == '-':
            pass # this is the default
        elif arg in {'-o', '--outfile'}:
            want_outfile = True
        elif arg.startswith('-o'):
            outfile = arg[2:]
            if outfile.startswith('='):
                outfile = outfile[1:]
        elif arg.startswith('--outfile'):
            outfile = arg[9:]
            if outfile.startswith('='):
                outfile = outfile[1:]
        else:
            infiles.append(arg)
    return asmap, outfile, infiles


def merge(file1, file2, *files, asmap):
    uxo = uxf.Uxf({} if asmap else [])
    for filename in (file1, file2) + files:
        new_uxo = uxf.load(filename)
        if new_uxo.comment:
            if not uxo.comment:
                uxo.comment = new_uxo.comment
            elif new_uxo.comment not in uxo.comment.splitlines():
                uxo.comment += '\n' + new_uxo.comment
        merge_ttypes(uxo, new_uxo, filename)
        if asmap:
            uxo.value[filename] = new_uxo.value
        else:
            uxo.value.append(new_uxo.value) # value for map or list
    return uxo


def merge_ttypes(uxo, new_uxo, filename):
    _merge_imports(uxo, new_uxo, filename)
    _merge_tclasses(uxo, new_uxo, filename)


def _merge_imports(uxo, new_uxo, filename):
    for ttype, filename in new_uxo.imports.items():
        original_filename = uxo.imports.get(ttype)
        if original_filename is None:
            uxo.imports[ttype] = filename # add
        elif original_filename != filename:
            raise uxf.Error(f'cannot merge {filename} due to conflicting '
                            f'imported ttype {ttype} filename '
                            f'{original_filename!r} != {filename!r}')
        # else: same


def _merge_tclasses(uxo, new_uxo, filename):
    # *must* be called *after* merge_imports()
    for ttype, tclass in new_uxo.tclasses.items():
        if ttype not in uxo.tclasses:
            uxo.tclasses[ttype] = tclass
        elif uxo.tclasses[ttype].is_equivalent(
                tclass, uxf.Compare.IGNORE_COMMENTS):
            pass # same so safe to ignore
        else:
            raise uxf.Error(f'cannot merge {filename} due to conflicting '
                            f'ttype {ttype}')


if __name__ == '__main__':
    main()
