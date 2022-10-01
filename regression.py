#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import collections
import contextlib
import enum
import filecmp
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile
import time

try:
    from colorama import Fore, init
    OK = Fore.BLUE
    FAIL = Fore.RED
    SKIP = Fore.GREEN
except ImportError:
    OK = FAIL = SKIP = ''

try:
    WIN = sys.platform == 'win32'
    ROOT = pathlib.Path(__file__).parent.resolve()
    if WIN:
        root = str(ROOT)
        if (root.startswith(r'\\VBOXSVR\app') or
                root.startswith('//VBOXSVR/app')):
            ROOT = pathlib.Path(root[13:])
    sys.path.append(str(ROOT / 'py/t'))
    import util
    try:
        import uxf # prefer stable system version
    except ImportError:
        sys.path.append(str(ROOT / 'py'))
        import uxf
    if OK:
        init(autoreset=True)
finally:
    pass

SERVER_PATH = ROOT / 'misc'
TEMP_PATH = tempfile.gettempdir()
if WIN:
    EXE_FOR_LANG = {'py': ['C:\\bin\\py38.bat', str(ROOT / 'py/uxf.py')],
                    'rs': [str(ROOT / 'rs/target/release/uxf.exe'), 'f']}
    CMP_FOR_LANG = {'py': ['C:\\bin\\py38.bat',
                           str(ROOT / 'py/uxfcompare.py')],
                    'rs': [str(ROOT / 'rs/target/release/uxf.exe'), 'c']}
else:
    EXE_FOR_LANG = {'py': ['python3', str(ROOT / 'py/uxf.py')],
                    'rs': [str(ROOT / 'rs/target/release/uxf'), 'f']}
    CMP_FOR_LANG = {'py': ['python3', str(ROOT / 'py/uxfcompare.py')],
                    'rs': [str(ROOT / 'rs/target/release/uxf'), 'c']}


def main():
    os.chdir(ROOT / 'testdata')
    if WIN:
        print('have you manually run (in another console): '
              'py38.bat misc/test_server.py')
    else:
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
        report(lang, total, ok, duration, good=total == ok)
    report('All' if all_total == all_ok else 'Some', all_total, all_ok,
           all_duration, '=', all_total == all_ok)
    if all_total == all_ok:
        cleanup()


def report(lang, total, ok, duration, sep='-', good=False):
    color = OK if good else FAIL
    if total == ok:
        print(color + f'{lang:4} {ok:,}/{total:,} OK ({duration:.3f} sec)')
    else:
        print(color +
              f'{lang:4} {ok:,}/{total:,} FAIL ({duration:.3f} sec)')
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
            if not right:
                right = sys.maxsize
            elif not right.isdecimal():
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
    print(f'{lang:3} tests ', end='\n' if verbose else '')
    for i, t in enumerate(tests):
        if i < tmin:
            continue
        if i > tmax:
            break
        if t.langs is not None and lang not in t.langs:
            if verbose:
                print(f'{i: 4}: ' + SKIP + 'skipped')
            else:
                print(f'{i} ' + SKIP + 'skipped')
            continue
        total += 1
        n = test_one(lang, verbose, i, t)
        ok += n
        if verbose or not n:
            print()
    print()
    return total, ok, time.monotonic() - start


def test_one(lang, verbose, i, t):
    cmd = list(EXE_FOR_LANG[lang])
    if not os.path.exists(cmd[0]) and lang == 'rs':
        build_rs(verbose)
    if verbose:
        print(f'{i: 4}: ', end='', flush=True)
    else:
        print(f'{i} ', end='', flush=True)
    if t.opts:
        cmd += t.opts
    cmd.append(t.ifile)
    if t.afile is not None:
        afile = 'actual/' + (t.ifile if t.afile is SAME else t.afile)
        cmd.append(afile)
    if verbose:
        print(' '.join(cmd), end='', flush=True)
    reply = subprocess.run(cmd, capture_output=True, text=True,
                           encoding='utf-8')
    if reply.returncode != t.returncode:
        print(FAIL + f'expected returncode {t.returncode}, got '
              f'{reply.returncode}', end='', flush=True)
        return 0
    if reply.stderr:
        if not check_stderr(lang, verbose, t, reply.stderr):
            return 0
    elif t.stderr is not None:
        print(FAIL + ' • missing stderr')
        return 0
    if t.efile is not None:
        if not check_expected(lang, verbose, t, afile):
            return 0
    return 1


def build_rs(verbose):
    if verbose:
        print('building rs uxf')
    subprocess.run(['cargo', 'build', '--release'], cwd=str(ROOT / 'rs'),
                   encoding='utf-8')


def check_stderr(lang, verbose, t, rstderr):
    if t.stderr is None:
        print(FAIL + f'\nunexpected stderr:\n{rstderr}')
        return False
    stderr = get_stderr_file(lang, t.stderr)
    with open(stderr, 'rt', encoding='utf-8') as file:
        stderr = file.read()
    rstderr = rstderr.strip()
    stderr = stderr.strip()
    if rstderr != stderr:
        rstderr = normalize(rstderr)
        stderr = normalize(stderr)
        if rstderr != stderr:
            print(FAIL + f'\nexpected stderr:\n{stderr}\n-got-\n{rstderr}')
            return False
    if verbose:
        print(OK + ' • stderr ok', end='', flush=True)
    return True


def get_stderr_file(lang, stderr):
    stderr = f'expected/{stderr}'
    lstderr = stderr.replace('.', f'-{lang}.')
    if WIN:
        wstderr = stderr.replace('.', '-win.')
        wlstderr = lstderr.replace('.', '-win.')
        if os.path.exists(wlstderr):
            stderr = wlstderr
        elif os.path.exists(wstderr):
            stderr = wstderr
        elif os.path.exists(lstderr):
            stderr = lstderr
    elif os.path.exists(lstderr):
        stderr = lstderr
    return stderr


def normalize(s):
    match = re.search(r'(?:^|:)[EFRW](?P<code>\d\d\d):', s)
    code = int(match.group('code')) if match else None
    s = re.sub(r'(:?(?:[A-Za-z]:)?[/\\]+[^:]+)?testdata[/\\]+', '', s)
    match = re.search(r':\d+:', s)
    text = re.sub(r'''['"]''', '', s[match.end():]) if match else s
    return code, text


def check_expected(lang, verbose, t, afile):
    efile = 'expected/' + (t.ifile if t.efile is SAME else t.efile)
    if not compare(lang, t.o_vs_e, t.ifile, efile):
        print(FAIL + f'{t.ifile!r} !{t.o_vs_e.symbol} {efile!r}', end='',
              flush=True)
        return False
    elif verbose and t.o_vs_e is not Compare.SKIP:
        print(OK + f' • o_vs_e {t.o_vs_e.symbol}', end='', flush=True)
    if not compare(lang, t.a_vs_e, afile, efile):
        print(FAIL + f'{afile!r} !{t.a_vs_e.symbol} {efile!r}', end='',
              flush=True)
        return False
    elif verbose and t.a_vs_e is not Compare.SKIP:
        print(OK + f' • a_vs_e {t.a_vs_e.symbol}', end='', flush=True)
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
    reply = subprocess.run(cmd, capture_output=True, text=True,
                           encoding='utf-8')
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
            elif field.ttype == 'dummy':
                return 'dummy.uxf'
        # else: return None

    def cmp(field):
        for compare in Compare:
            if compare.value == field.ttype:
                return compare

    uxo = uxf.load(filename)
    for record in uxo.value:
        langs = None if record.langs is None else set(record.langs)
        yield Test(record.ifile, tuple(record.opts), xfile(record.afile),
                   xfile(record.efile), cmp(record.o_vs_e),
                   cmp(record.a_vs_e), record.stderr, record.returncode,
                   langs)


@enum.unique
class Compare(enum.Enum):
    SKIP = 'skip'
    EQUIV = 'equiv'
    EQUAL = 'equal'
    IDENTICAL = 'same'

    @property
    def symbol(self):
        if self == Compare.SKIP:
            return '?'
        elif self == Compare.EQUIV:
            return '~'
        elif self == Compare.EQUAL:
            return '='
        elif self == Compare.IDENTICAL:
            return '=='


SAME = object()


Test = collections.namedtuple(
    'Test', ('ifile', # file to read
             'opts', # e.g., ['-l'] or ['-cl'] or ['-cl', '-i9', '-w40']
             'afile', # None or SAME or actual filename
             'efile', # None or SAME or expected filename
             'o_vs_e', # whether and if so how to compare
             'a_vs_e', # whether and if so how to compare
             'stderr', # None or expected stderr filename
             'returncode', # expected return code
             'langs', # set of langs for which this test is valid: None→all
             ))


if __name__ == '__main__':
    main()
