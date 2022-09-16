#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import collections
import contextlib
import enum
import filecmp
import os
import pathlib
import shutil
import subprocess
import sys
import tempfile
import time

try:
    ROOT = pathlib.Path(__file__).parent.resolve()
    sys.path.append(str(ROOT / 'py/t'))
    import util
finally:
    pass

SERVER_PATH = ROOT / 'misc'
TEMP_PATH = tempfile.gettempdir()
EXE_FOR_LANG = {
    'py': ['python3', str(ROOT / 'py/uxf.py')],
    'rs': [str(ROOT / 'rs/target/release/uxf')],
    }


def main():
    os.chdir(ROOT / 'testdata')
    util.check_server(SERVER_PATH)
    tmin, tmax, langs = get_config()
    cleanup()
    all_total = all_ok = 0
    all_duration = 0.0
    print('=' * 30)
    for lang in langs:
        total, ok, duration = test_lang(tmin, tmax, lang)
        all_total += total
        all_ok += ok
        all_duration += duration
        report(lang, total, ok, duration)
    report('All', all_total, all_ok, all_duration, '=')
    cleanup()


def report(lang, total, ok, duration, sep='-'):
    if total == ok:
        print(f'{lang:3} {ok:,}/{total:,} OK ({duration:.3f} sec)')
    else:
        print(f'{lang:3} {ok:,}/{total:,} FAIL ({duration:.3f} sec)')
    print(sep * 30)


def get_config():
    tmin = 0
    tmax = sys.maxsize
    langs = set()
    for arg in sys.argv[1:]:
        if arg in {'-h', '--help'}:
            raise SystemExit(USAGE.format(', '.join(sorted(EXE_FOR_LANG))))
        elif arg in EXE_FOR_LANG:
            langs.add(arg)
        elif '-' in arg:
            left, right = arg.split('-', 1)
            if not left.isdecimal():
                raise SystemExit('error: n must be an integer')
            if not right.isdecimal():
                raise SystemExit('error: m must be an integer')
            tmin = int(left)
            tmax = int(right)
            if tmin > tmax:
                tmin, tmax = tmax, tmin
        elif arg.isdecimal():
            tmin = tmax = int(arg)
        else:
            raise SystemExit(f'error: unrecognized argument: {arg!r}')
    if not langs:
        raise SystemExit('error: must specify at least one lang from '
                         f'{sorted(EXE_FOR_LANG)}')
    return tmin, tmax, tuple(langs)


USAGE = '''\
usage: regression.py [n|n-m] <lang1> [lang2 ... langN]

n      run test n
n-m    run tests n to m inclusive
       default: run all tests
langX  from: {}
'''


def test_lang(tmin, tmax, lang):
    total = ok = 0
    start = time.monotonic()
    print(f'{lang:3} tests... ')
    for i, t in enumerate(TESTS):
        if i < tmin:
            continue
        if i > tmax:
            break
        total += 1
        ok += test(lang, i, t)
    print()
    return total, ok, time.monotonic() - start


def test(lang, i, t):
    print(f'{i} ', end='', flush=True)
    cmd = list(EXE_FOR_LANG[lang])
    if t.opts:
        cmd += t.opts
    cmd.append(t.ifile)
    if t.afile is not None:
        afile = 'actual/' + (t.ifile if t.afile is SAME else t.afile)
        cmd.append(afile)
    reply = subprocess.run(cmd, capture_output=True, text=True)
    if reply.returncode != t.returncode:
        print(f'\nexpected returncode {t.returncode}, got '
              f'{reply.returncode}')
        return 0
    if reply.stderr:
        if t.stderr is None:
            print(f'\nunexpected stderr:\n{reply.stderr}')
            return 0
        with open(t.stderr, 'rt', encoding='utf-8') as file:
            stderr = file.read()
        if reply.stderr.strip() != stderr.strip():
            print(f'\nexpected stderr:\n{stderr}\n-got-\n{reply.stderr}')
            return 0
    if t.efile is not None:
        efile = 'expected/' + (t.ifile if t.efile is SAME else t.efile)
        if not compare(lang, t.i_vs_e, t.ifile, efile):
            print(f'\nnot {t.i_vs_e.value}: {t.ifile!r} {efile!r}')
            return 0
        if not compare(lang, t.a_vs_e, afile, efile):
            print(f'\nnot {t.a_vs_e.value}: {afile!r} {efile!r}')
            return 0
    return 1


def compare(lang, compare, file1, file2):
    if compare is Compare.SKIP:
        return True
    if compare is Compare.IDENTICAL:
        return filecmp(file1, file2, shallow=False)
    cmd = list(EXE_FOR_LANG[lang])
    cmd.append('c')
    if compare is Compare.EQUIV:
        cmd.append('-e')
    cmd += [file1, file2]
    reply = subprocess.run(cmd, capture_output=True, text=True)
    return reply.stdout.startswith('Un')


def cleanup():
    if os.path.exists('actual'):
        shutil.rmtree('actual')
    with contextlib.suppress(FileExistsError):
        os.mkdir('actual')


@enum.unique
class Compare(enum.Enum):
    SKIP = 'skip'
    EQUIV = 'equiv'
    EQUAL = 'equal'
    IDENTICAL = 'same'


SAME = object()


T = collections.namedtuple(
    'T', ('ifile', # file to read
          'opts', # e.g., ['-l'] or ['-cl'] or ['-cl', '-i9', '-w40']
          'afile', # None or SAME or actual filename
          'efile', # None or SAME, expected filename
          'i_vs_e', # whether and if so how to compare
          'a_vs_e', # whether and if so how to compare
          'stderr', # None expected stderr filename
          'returncode', # expected return code
          ),
    defaults=(SAME, SAME, Compare.EQUAL, Compare.EQUAL, None, 0))


L = ('-l',)
CL = ('-cl',)

TESTS = (
    T('t0.uxf', L),
    T('t0.uxf', CL, 't0c.uxf', 't0c.uxf'),
    )


if __name__ == '__main__':
    main()
