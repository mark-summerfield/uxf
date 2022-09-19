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
    try:
        import uxf # prefer stable system version
    except ImportError:
        sys.path.append(str(ROOT / 'py'))
        import uxf
finally:
    pass

SERVER_PATH = ROOT / 'misc'
TEMP_PATH = tempfile.gettempdir()
EXE_FOR_LANG = {'py': ['python3', str(ROOT / 'py/uxf.py')],
                'rs': [str(ROOT / 'rs/target/release/uxf'), 'f']}
CMP_FOR_LANG = {'py': ['python3', str(ROOT / 'py/uxfcompare.py')],
                'rs': [str(ROOT / 'rs/target/release/uxf'), 'c']}


def main():
    os.chdir(ROOT / 'testdata')
    util.check_server(SERVER_PATH)
    tmin, tmax, langs, verbose = get_config()
    cleanup()
    all_total = all_ok = 0
    all_duration = 0.0
    print('=' * 30)
    tests = list(read_tests('regression.dat.gz'))
    for lang in langs:
        total, ok, duration = test_lang(tmin, tmax, lang, verbose, tests)
        all_total += total
        all_ok += ok
        all_duration += duration
        report(lang, total, ok, duration)
    report('All' if all_total == all_ok else 'Some', all_total, all_ok,
           all_duration, '=')
    if all_total == all_ok:
        cleanup()


def report(lang, total, ok, duration, sep='-'):
    if total == ok:
        print(f'{lang:4} {ok:,}/{total:,} OK ({duration:.3f} sec)')
    else:
        print(f'{lang:4} {ok:,}/{total:,} FAIL ({duration:.3f} sec)')
    print(sep * 30)


def get_config():
    tmin = 0
    tmax = sys.maxsize
    langs = set()
    verbose = False
    for arg in sys.argv[1:]:
        if arg in {'-h', '--help'}:
            raise SystemExit(USAGE.format(', '.join(sorted(EXE_FOR_LANG))))
        elif arg in {'-v', '--verbose'}:
            verbose = True
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
        langs = list(EXE_FOR_LANG)
    return tmin, tmax, tuple(langs), verbose


USAGE = '''\
usage: regression.py [-v|--verbose] [n|n-m] <lang1> [lang2 ... langN]

n      run test n
n-m    run tests n to m inclusive
       default: run all tests
langX  from: {}
       default: all
'''


def test_lang(tmin, tmax, lang, verbose, tests):
    total = ok = 0
    start = time.monotonic()
    print(f'{lang:3} tests ', end='')
    for i, t in enumerate(tests):
        if i < tmin:
            continue
        if i > tmax:
            break
        if t.langs is not None and lang not in t.langs:
            continue
        total += 1
        ok += test_one(lang, verbose, i, t)
    print()
    return total, ok, time.monotonic() - start


def test_one(lang, verbose, i, t):
    if verbose:
        print(i)
    else:
        print(f'{i} ', end='', flush=True)
    cmd = list(EXE_FOR_LANG[lang])
    if t.opts:
        cmd += t.opts
    cmd.append(t.ifile)
    if t.afile is not None:
        afile = 'actual/' + (t.ifile if t.afile is SAME else t.afile)
        cmd.append(afile)
    if verbose:
        print(' ', ' '.join(cmd))
    reply = subprocess.run(cmd, capture_output=True, text=True)
    if reply.returncode != t.returncode:
        print(f'\nexpected returncode {t.returncode}, got '
              f'{reply.returncode}')
        return 0
    if reply.stderr:
        if not check_stderr(lang, verbose, t, reply.stderr):
            return 0
    if t.efile is not None:
        if not check_expected(lang, verbose, t, afile):
            return 0
    return 1


def check_stderr(lang, verbose, t, rstderr):
    if t.stderr is None:
        print(f'\nunexpected stderr:\n{rstderr}')
        return False
    stderr = f'expected/{t.stderr}'
    lstderr = stderr.replace('.', f'-{lang}.')
    stderr = lstderr if os.path.exists(lstderr) else stderr
    with open(stderr, 'rt', encoding='utf-8') as file:
        stderr = file.read()
    if rstderr.strip() != stderr.strip():
        print(f'\nexpected stderr:\n{stderr}\n-got-\n{rstderr}')
        return False
    if verbose:
        print('  stderr matched')
    return True


def check_expected(lang, verbose, t, afile):
    efile = 'expected/' + (t.ifile if t.efile is SAME else t.efile)
    if not compare(lang, t.i_vs_e, t.ifile, efile):
        print(f'\nnot {t.i_vs_e.value}: {t.ifile!r} {efile!r}')
        return False
    elif verbose:
        print(f'  original vs expected {t.i_vs_e.value}')
    if not compare(lang, t.a_vs_e, afile, efile):
        print(f'\nnot {t.a_vs_e.value}: {afile!r} {efile!r}')
        return False
    elif verbose:
        print(f'  actual vs expected {t.i_vs_e.value}')
    return True


def compare(lang, compare, file1, file2):
    if compare is Compare.SKIP:
        return True
    if compare is Compare.IDENTICAL:
        return filecmp(file1, file2, shallow=False)
    cmd = list(CMP_FOR_LANG[lang])
    if compare is Compare.EQUIV:
        cmd.append('-e')
    cmd += [file1, file2]
    reply = subprocess.run(cmd, capture_output=True, text=True)
    return reply.stdout.startswith('EQU')


def cleanup():
    if os.path.exists('actual'):
        shutil.rmtree('actual')
    with contextlib.suppress(FileExistsError):
        os.mkdir('actual')


def read_tests(filename):
    def xfile(field):
        if field is not None:
            if isinstance(field, str):
                return field
            elif field.ttype == 'same':
                return SAME
        # else: return None

    def cmp(field):
        for compare in Compare:
            if compare.value == field.ttype:
                return compare

    uxo = uxf.load(filename)
    for record in uxo.value:
        langs = None if record.langs is None else set(record.langs)
        yield Test(record.ifile, tuple(record.opts), xfile(record.afile),
                   xfile(record.efile), cmp(record.i_vs_e),
                   cmp(record.a_vs_e), record.stderr, record.returncode,
                   langs)


@enum.unique
class Compare(enum.Enum):
    SKIP = 'skip'
    EQUIV = 'equiv'
    EQUAL = 'equal'
    IDENTICAL = 'same'


SAME = object()


Test = collections.namedtuple(
    'Test', ('ifile', # file to read
             'opts', # e.g., ['-l'] or ['-cl'] or ['-cl', '-i9', '-w40']
             'afile', # None or SAME or actual filename
             'efile', # None or SAME or expected filename
             'i_vs_e', # whether and if so how to compare
             'a_vs_e', # whether and if so how to compare
             'stderr', # None or expected stderr filename
             'returncode', # expected return code
             'langs', # set of langs for which this test is valid: None→all
             ))


if __name__ == '__main__':
    main()
