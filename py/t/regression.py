#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import contextlib
import filecmp
import functools
import gzip
import os
import random
import re
import shutil
import subprocess
import sys
import tempfile
import time

try:
    PATH = os.path.abspath(os.path.dirname(__file__))
    sys.path.append(os.path.abspath(os.path.join(PATH, '../')))
    import uxf
    UXF_EXE = os.path.join(PATH, '../uxf.py')
    UXFCONVERT_EXE = os.path.join(PATH, '../uxfconvert.py')
    SLIDES = os.path.join(PATH, '../eg/slides.py')
    SLIDES_SLD = os.path.join(PATH, '../eg/slides.sld')
    TEST_TABLE = os.path.join(PATH, '../t/test_table.py')
    TEST_SQLITE = os.path.join(PATH, '../t/test_sqlite.py')
    TEST_ERRORS = os.path.join(PATH, '../t/test_errors.py')
    TEST_LINTS = os.path.join(PATH, '../t/test_lints.py')
    TEST_IMPORTS = os.path.join(PATH, '../t/test_imports.py')
    TEST_MERGE = os.path.join(PATH, '../t/test_merge.py')
    TEST_INCLUDE = os.path.join(PATH, '../t/test_include.py')
    TEST_EDITABLETUPLE = os.path.join(PATH, '../t/test_editabletuple.py')
    TEST_TLM = os.path.join(PATH, '../t/test_tlm.py')
    TEST_COMPARE = os.path.join(PATH, '../t/test_compare.py')
    TEST_VISIT = os.path.join(PATH, '../t/test_visit.py')
    BENCHMARK = os.path.join(PATH, '../t/benchmark.py')
    os.chdir(os.path.join(PATH, '../../testdata')) # move to test data
finally:
    pass


def main():
    print(f'testing uxf.py {uxf.__version__} (UXF {uxf.VERSION})')
    check_server()
    max_total, verbose = get_config()
    cleanup()
    t = time.monotonic()
    uxffiles = sorted((name for name in next(os.walk('.'))[-1]
                      if name.endswith(('.uxf', '.uxf.gz'))), key=by_number)
    print('0', end='', flush=True)
    total = ok = 0
    total, ok = test_uxf_files(uxffiles, verbose=verbose,
                               max_total=max_total)
    print('1', end='', flush=True)
    total, ok = test_uxf_loads_dumps(uxffiles, total, ok, verbose=verbose,
                                     max_total=max_total)
    print('2', end='', flush=True)
    total, ok = test_uxf_equal(uxffiles, total, ok, verbose=verbose,
                               max_total=max_total)
    print('3', end='', flush=True)
    total, ok = test_uxfconvert(uxffiles, total, ok, verbose=verbose,
                                max_total=max_total)
    print('4', end='', flush=True)
    total, ok = test_table_is_scalar(total, ok, verbose=verbose)
    print('5', end='', flush=True)
    if total < max_total:
        total, ok = test_slides(SLIDES, total, ok, verbose=verbose)
        print('6', end='', flush=True)
    if total < max_total:
        total, ok = test_format(total, ok, verbose=verbose)
        print('7', end='', flush=True)
    if total < max_total:
        total, ok = test_externals(
            (('A', TEST_TABLE), ('B', TEST_SQLITE), ('C', TEST_LINTS),
             ('D', TEST_IMPORTS), ('E', TEST_MERGE), ('F', TEST_INCLUDE),
             ('G', TEST_EDITABLETUPLE), ('H', TEST_TLM),
             ('I', TEST_COMPARE), ('J', TEST_VISIT), ('Z', TEST_ERRORS)),
            total, ok, verbose=verbose, max_total=max_total)
        time.sleep(0.2) # allow Z to be visible
    if ok == total and os.isatty(sys.stdout.fileno()):
        span = min(1000, total // 10)
        for c in ('\b', ' ', '\b'):
            print(c * span, end='', flush=True)
    t = time.monotonic() - t
    if total == ok:
        print(f'{ok:,}/{total:,} All OK ({t:.3f} sec)')
        cmd = prep_cmd([BENCHMARK, '--quiet', '1'])
        subprocess.run(cmd)
        cleanup()
    else:
        print(f': {ok:,}/{total:,} • FAIL ({t:.3f} sec)')


def get_config():
    verbose = False
    max_total = 999999
    for arg in sys.argv[1:]:
        if arg in {'-h', '--help'}:
            raise SystemExit('usage: regression.py [-v|--verbose] [max]')
        elif arg in {'-v', '--verbose'}:
            verbose = True
        elif isasciidigit(arg):
            max_total = int(arg)
    return max_total, verbose


def test_uxf_files(uxffiles, *, verbose, max_total):
    total = ok = 0
    for name in uxffiles:
        total += 1
        if total > max_total:
            return total - 1, ok
        actual = f'actual/{name}'
        expected = f'expected/{name}'
        if expected.endswith('.gz'):
            expected = expected[:-3]
        cmd = prep_cmd([UXF_EXE, name, actual])
        reply = subprocess.run(cmd, capture_output=True, text=True)
        cmd = ' '.join(cmd)
        if reply.returncode != 0:
            stderr = f': {reply.stderr.strip()}' if reply.stderr else ''
            print(f'{cmd} • FAIL (execute){stderr}')
        else:
            ok += compare(cmd, name, actual, expected, verbose=verbose)
            if not verbose and not ok % 10:
                print('.', end='', flush=True)
    return total, ok


def test_uxf_loads_dumps(uxffiles, total, ok, *, verbose, max_total):
    temp_uxo = uxf.Uxf()
    for name in uxffiles:
        if name.startswith('l'):
            continue # skip linted files that may have changed
        total += 1
        if total > max_total:
            return total - 1, ok
        try:
            with open(name, 'rt', encoding='utf-8') as file:
                uxt = file.read()
        except UnicodeDecodeError:
            with gzip.open(name, 'rt', encoding='utf-8') as file:
                uxt = file.read()
        on_event = functools.partial(uxf.on_event, verbose=verbose)
        try:
            if random.choice((0, 1)):
                uxo = uxf.loads(uxt, on_event=on_event)
            else:
                temp_uxo.loads(uxt, on_event=on_event)
                uxo = temp_uxo
        except uxf.Error as err:
            print(f'loads()/dumps() • {name} FAIL #1: {err}')
        if random.choice((0, 1)):
            new_uxt = uxo.dumps(on_event=on_event)
        else:
            new_uxt = uxf.dumps(uxo, on_event=on_event)
        try:
            new_uxo = uxf.loads(new_uxt, on_event=on_event)
        except uxf.Error as err:
            print(f'{name} • FAIL (loads()/dumps()) #2: {err}')
            continue
        try:
            if uxo == new_uxo:
                ok += 1
                if verbose:
                    print(f'loads()/dumps() • {name} OK')
                elif not ok % 10:
                    print('.', end='', flush=True)
            else:
                print(f'{name} • FAIL #3 (loads()/dumps())')
        except uxf.Error as err:
            print(f'{name} • FAIL (loads()/dumps()) #4: {err}')
    return total, ok


def test_uxf_equal(uxffiles, total, ok, *, verbose, max_total):
    for name in uxffiles:
        if name.startswith('l'):
            continue # skip linted files that may have changed
        total += 1
        if total > max_total:
            return total - 1, ok
        try:
            with open(name, 'rt', encoding='utf-8') as file:
                uxt = file.read()
        except UnicodeDecodeError:
            with gzip.open(name, 'rt', encoding='utf-8') as file:
                uxt = file.read()
        on_event = functools.partial(uxf.on_event, verbose=verbose)
        try:
            uxo1 = uxf.loads(uxt, on_event=on_event)
        except uxf.Error as err:
            print(f'== 1 • {name} FAIL: {err}')
        expected = f'expected/{name}'
        try:
            with open(expected, 'rt', encoding='utf-8') as file:
                uxt = file.read()
        except UnicodeDecodeError:
            with gzip.open(name, 'rt', encoding='utf-8') as file:
                uxt = file.read()
        try:
            uxo2 = uxf.loads(uxt, on_event=on_event)
        except uxf.Error as err:
            print(f'== 2 • {expected} FAIL: {err}')
        try:
            if uxo1 == uxo2:
                ok += 1
                if verbose:
                    print(f'== • {name} OK')
                elif not ok % 10:
                    print('.', end='', flush=True)
            else:
                print(f'{name} • FAIL (==)')
        except uxf.Error as err:
            print(f'{name} • FAIL (loads()/dumps()): {err}')
    return total, ok


def test_uxfconvert(uxffiles, total, ok, *, verbose, max_total):
    N, Y, NF, YR = (0, 1, 2, 3) # No, Yes, No with -f, Yes with -f
    files = [(name, name.replace('.uxf', '.json'), Y) for name in uxffiles]
    files += [(name, name.replace('.uxf', '.xml'), Y) for name in uxffiles]
    files += [('t1.uxf', 't1.csv', N), ('t2.uxf', 't2.csv', N),
              ('0.csv', '0.uxf', N), ('1.csv', '1.uxf', NF),
              ('2.csv', '2.uxf', NF), ('ini.ini', 'ini.uxf', N)]
    for infile, outfile, roundtrip in files:
        if infile.startswith('l'):
            continue # skip linted files that may have changed
        total += 1
        if total > max_total:
            return total - 1, ok
        actual = f'actual/{outfile}'
        cmd = prep_cmd([UXFCONVERT_EXE, '-f', infile, actual]
                       if roundtrip == NF else
                       [UXFCONVERT_EXE, infile, actual])
        reply = subprocess.run(cmd, capture_output=True, text=True)
        cmd = ' '.join(cmd)
        if reply.returncode != 0:
            stderr = f': {reply.stderr.strip()}' if reply.stderr else ''
            print(f'{cmd} • FAIL (execute){stderr}')
        else:
            expected = f'expected/{outfile}'
            n = compare(cmd, infile, actual, expected, verbose=verbose)
            ok += n
            if not verbose and not ok % 10:
                print('.', end='', flush=True)
            if n:
                if roundtrip in (Y, YR):
                    total += 1
                    new_actual = tempfile.gettempdir() + f'/{infile}'
                    cmd = prep_cmd(
                        [UXFCONVERT_EXE, '-f', expected, new_actual]
                        if roundtrip == YR else
                        [UXFCONVERT_EXE, expected, new_actual])
                    reply = subprocess.run(cmd, capture_output=True,
                                           text=True)
                    cmd = ' '.join(cmd)
                    if reply.returncode != 0:
                        stderr = (f': {reply.stderr.strip()}' if
                                  reply.stderr else '')
                        print(f'{cmd} • FAIL (execute roundtrip){stderr}')
                    else:
                        compare_with = expected
                        i = compare_with.rfind('.')
                        if i > -1:
                            compare_with = compare_with[:i] + '.uxf'
                        if compare(cmd, expected, new_actual, compare_with,
                                   verbose=verbose, roundtrip=True):
                            ok += 1
                            if not verbose and not ok % 10:
                                print('.', end='', flush=True)
                            with contextlib.suppress(FileNotFoundError):
                                os.remove(new_actual)
    total += 1
    actual = 'actual/1-2-csv.uxf'
    infile = '1.csv 2.csv'
    cmd = prep_cmd([UXFCONVERT_EXE, '-f', '1.csv', '2.csv', actual])
    reply = subprocess.run(cmd, capture_output=True, text=True)
    cmd = ' '.join(cmd)
    if reply.returncode != 0:
        stderr = f': {reply.stderr.strip()}' if reply.stderr else ''
        print(f'{cmd} • FAIL (execute){stderr}')
    else:
        expected = 'expected/1-2-csv.uxf'
        ok += compare(cmd, infile, actual, expected, verbose=verbose)
        if not verbose and not ok % 10:
            print('.', end='', flush=True)
    return total, ok


def test_table_is_scalar(total, ok, *, verbose):
    for (filename, is_scalar) in (('t40.uxf', True), ('t41.uxf', False),
                                  ('t42.uxf', True), ('t43.uxf', False)):
        total += 1
        on_event = functools.partial(uxf.on_event, verbose=verbose)
        uxo = uxf.load(filename, on_event=on_event)
        if is_scalar == uxo.value.is_scalar:
            ok += 1
            if verbose:
                print(f'{filename} • (Table.is_scalar) OK')
        else:
            print(f'{filename} • FAIL (Table.is_scalar)')
    return total, ok


def test_slides(slides_py, total, ok, *, verbose):
    cmd = prep_cmd([slides_py, SLIDES_SLD, 'actual/slides'])
    total += 1
    reply = subprocess.run(cmd, capture_output=True, text=True)
    cmd = ' '.join(cmd)
    if reply.returncode != 0:
        stderr = f': {reply.stderr.strip()}' if reply.stderr else ''
        print(f'{cmd} • FAIL (execute slides){stderr}')
    else:
        ok += 1
        for name in sorted(
                name for name in os.listdir('actual/slides')
                if name.endswith('.html')):
            total += 1
            ok += compare(cmd, 'slides.sld', f'actual/slides/{name}',
                          f'expected/slides/{name}', verbose=verbose)
    return total, ok


def test_format(total, ok, *, verbose):
    uxt_original = '''uxf 1.0
=Test one:int two:bool three:datetime four:real five:date \
six:bool seven:int eight:str nine ten eleven twelve
(Test 1 yes 1980-01-17T23:59:07 98.654321 2022-07-29 no 2 \
<A short string of text> 9 <ten> <eleven> 12.0)
'''
    uxt_default_format = '''uxf 1.0
=Test one:int two:bool three:datetime four:real five:date \
six:bool seven:int eight:str nine ten
   eleven twelve
(Test 1 yes 1980-01-17T23:59:07 98.654321 2022-07-29 no 2 \
<A short string of text> 9 <ten>
<eleven> 12.0)
'''
    uxt_custom_format = '''uxf 1.0
=Test one:int two:bool three:datetime four:real five:date
   six:bool seven:int eight:str nine ten eleven twelve
(Test 1 yes 1980-01-17T23:59:07 98.654321 2022-07-29 no 2
<A short string of text> 9 <ten> <eleven> 12.0)
'''
    total += 1
    uxo = uxf.loads(uxt_original)
    uxt1 = uxo.dumps()
    if uxt1 == uxt_default_format:
        ok += 1
        if verbose:
            print('default format OK')
    elif verbose:
        print('default format FAIL')
    total += 1
    uxt2 = uxo.dumps(format=uxf.Format(wrap_width=None))
    if uxt2 == uxt_original:
        ok += 1
        if verbose:
            print('original format #1 OK')
    elif verbose:
        print('original format #1 FAIL')
    total += 1
    uxt2 = uxo.dumps(format=uxf.Format(wrap_width=0))
    if uxt2 == uxt_original:
        ok += 1
        if verbose:
            print('original format #2 OK')
    elif verbose:
        print('original format #2 FAIL')
    total += 1
    uxt2 = uxo.dumps(format=uxf.Format(wrap_width=60))
    if uxt2 == uxt_custom_format:
        ok += 1
        if verbose:
            print('custom format #3 OK')
    elif verbose:
        print('custom format #3 FAIL')
    return total, ok


def test_externals(cmds, total, ok, *, verbose, max_total):
    for letter, cmd in cmds:
        if total >= max_total:
            return total - 1, ok
        total += 1
        diff = total - ok
        total, ok = test_external([cmd, '--regression'], total, ok,
                                  verbose=verbose)
        if total - ok > diff:
            print(f'{cmd} • FAIL')
        print(letter, end='', flush=True)
    return total, ok


def test_external(cmd, total, ok, *, verbose):
    reply = subprocess.run(prep_cmd(cmd), capture_output=True, text=True)
    cmd = ' '.join(cmd)
    if reply.returncode != 0:
        total += 1 # whole cmd failed
        stderr = f': {reply.stderr.strip()}' if reply.stderr else ''
        print(f'{cmd} • FAIL{stderr}')
    else:
        total -= 1 # whole cmd succeeded
        parts = reply.stdout.split()
        try:
            total += int(parts[0].split('=')[1])
            ok += int(parts[1].split('=')[1])
        except (IndexError, ValueError):
            print(f'failed to read total/ok from {cmd}, got {parts!r}')
        if verbose:
            print(f'{cmd} • OK')
    return total, ok


def compare(cmd, infile, actual, expected, *, verbose,
            roundtrip=False):
    try:
        if filecmp.cmp(actual, expected, False):
            if verbose:
                print(f'{cmd} • {infile} → {actual} OK')
            return 1
        elif roundtrip:
            with open(actual, 'rb') as af, open(expected, 'rb') as ef:
                a = af.read()
                i = a.find(b'\n')
                if i > -1:
                    a = a[i:]
                b = ef.read()
                i = b.find(b'\n')
                if i > -1:
                    b = b[i:]
                if a == b:
                    if verbose:
                        print(f'{cmd} • {infile} → {actual} (roundtrip) OK')
                    return 1
        with open(actual, 'rb') as file:
            adata = file.read()
        with open(expected, 'rb') as file:
            edata = file.read()
        adata = adata.replace(b'\r', b'')
        edata = edata.replace(b'\r', b'')
        if adata == edata:
            return 1 # UXF ↔ UXF may have \r\n vs \n differences Win vs Unix
        flags = re.DOTALL | re.MULTILINE
        adata = re.sub(rb'\s+', b'', adata, flags=flags)
        edata = re.sub(rb'\s+', b'', edata, flags=flags)
        if adata == edata:
            if infile.endswith('.xml'): # UXF ↔ XML doesn't round-trip
                return 1                # due to ws normalization
            print(
                f'{cmd} • FAIL (compare whitespace) {actual} != {expected}')
        else:
            print(f'{cmd} • FAIL (compare) {actual} != {expected}')
    except FileNotFoundError:
        print(f'{cmd} • FAIL (missing {expected!r})')
    return 0


def cleanup():
    if os.path.exists('actual'):
        shutil.rmtree('actual')
    with contextlib.suppress(FileExistsError):
        os.mkdir('actual')
    with contextlib.suppress(FileExistsError):
        os.mkdir('actual/slides1')
    with contextlib.suppress(FileExistsError):
        os.mkdir('actual/slides2')


def by_number(s):
    match = re.match(r'(?P<name>\D+)(?P<max_total>\d+)', s)
    if match is not None:
        return match['name'], int(match['max_total'])
    return s, 0


def isasciidigit(s):
    '''Returns True if s matches /^[0-9]+$/.'''
    return s.isascii() and s.isdigit()


def prep_cmd(cmd):
    if sys.platform.startswith('win'):
        cmd = ['py.bat'] + cmd
    return cmd


def check_server():
    reply = subprocess.run(['nc', '-vz', 'localhost', '5558'],
                           stdout=subprocess.DEVNULL,
                           stderr=subprocess.DEVNULL)
    if reply.returncode != 0:
        raise SystemExit('run: ./misc/test_server.py &')


if __name__ == '__main__':
    main()
