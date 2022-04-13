#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import contextlib
import filecmp
import gzip
import importlib.util
import os
import re
import subprocess
import sys
import tempfile
import textwrap
import time

os.chdir(os.path.dirname(__file__))
module_name = 'uxf'
spec = importlib.util.spec_from_file_location(module_name, '../py/uxf.py')
uxf = importlib.util.module_from_spec(spec)
sys.modules[module_name] = uxf
spec.loader.exec_module(uxf)


def main():
    uxf, uxfconvert, verbose = get_config()
    cleanup()
    t = time.monotonic()
    uxffiles = sorted((name for name in os.listdir('.')
                       if os.path.isfile(name) and name.endswith('.uxf')),
                      key=by_number)
    total, ok = test_uxf_files(uxf, uxffiles, verbose=verbose)
    total, ok = test_uxf_loads_dumps(uxffiles, total, ok, verbose=verbose)
    total, ok = test_uxfconvert(uxfconvert, uxffiles, total, ok,
                                verbose=verbose)
    if total == ok:
        t = time.monotonic() - t
        print(f'{ok}/{total} All OK ({t:.3f} sec)')
        cleanup()
    else:
        print(f'{ok}/{total} FAIL')


def get_config():
    uxf_default = '../py/uxf.py'
    uxfconvert_default = '../py/uxfconvert.py'
    uxf = uxfconvert = None
    verbose = False
    for arg in sys.argv[1:]:
        if arg in {'-h', '--help'}:
            raise SystemExit(f'''\
usage: regression.py [-v|--verbose] [uxf-exe] [uxfconvert-exe]
uxf-exe default is {uxf_default}
uxfconvert-exe default is {uxfconvert_default}''')
        elif arg in {'-v', '--verbose'}:
            verbose = True
        elif uxf is None:
            uxf = arg
        elif uxfconvert is None:
            uxfconvert = arg
    if uxf is None:
        uxf = uxf_default
    if uxfconvert is None:
        uxfconvert = uxfconvert_default
    return uxf, uxfconvert, verbose


def test_uxf_files(uxf, uxffiles, *, verbose):
    total = ok = 0
    for name in uxffiles:
        total += 1
        actual = f'actual/{name}'
        expected = f'expected/{name}'
        cmd = [uxf, name, actual]
        reply = subprocess.call(cmd)
        cmd = ' '.join(cmd)
        if reply != 0:
            print(f'{cmd} • FAIL (execute)')
        else:
            ok += compare(cmd, name, actual, expected, verbose=verbose)
            if not verbose and not ok % 10:
                print('.', end='', flush=True)
    return total, ok


def test_uxf_loads_dumps(uxffiles, total, ok, *, verbose):
    for name in uxffiles:
        total += 1
        try:
            with open(name, 'rt', encoding='utf-8') as file:
                uxf_text = file.read()
        except UnicodeDecodeError:
            with gzip.open(name, 'rt', encoding='utf-8') as file:
                uxf_text = file.read()
        use_true_false = 'true' in uxf_text or 'false' in uxf_text
        data, custom = uxf.loads(uxf_text)
        new_uxf_text = uxf.dumps(data, custom, one_way_conversion=True,
                                 use_true_false=use_true_false)
        nws_uxf_text = normalize_uxf_text(uxf_text)
        nws_new_uxf_text = normalize_uxf_text(new_uxf_text)
        if nws_uxf_text == nws_new_uxf_text:
            ok += 1
            if verbose:
                print(f'loads()/dumps() • {name} OK')
            elif not ok % 10:
                print('.', end='', flush=True)
        else:
            print(f'{name} • FAIL (loads()/dumps())')
            if verbose:
                print(f'LOADS = {nws_uxf_text}\nDUMPS = {nws_new_uxf_text}')
    return total, ok


def normalize_uxf_text(text):
    i = text.find('\n') + 1
    header, body = text[:i], text[i:]
    body = ''.join(body.split()) # eliminate whitespace
    return header + '\n'.join(textwrap.wrap(body, 40)) # easier to compare


def test_uxfconvert(uxfconvert, uxffiles, total, ok, *, verbose):
    N, Y, NF, YR = (0, 1, 2, 3) # No, Yes, No with -f, Yes with -f
    files = [(name, name.replace('.uxf', '.json'), Y) for name in uxffiles]
    files += [('t1.uxf', 't1.csv', N), ('t2.uxf', 't2.csv', N),
              ('0.csv', '0.uxf', N), ('1.csv', '1.uxf', NF),
              ('2.csv', '2.uxf', NF)]
    # TODO add tests for ini, sqlite, and xml
    for infile, outfile, roundtrip in files:
        total += 1
        actual = f'actual/{outfile}'
        cmd = ([uxfconvert, '-f', infile, actual] if roundtrip == NF else
               [uxfconvert, infile, actual])
        reply = subprocess.call(cmd)
        cmd = ' '.join(cmd)
        if reply != 0:
            print(f'{cmd} • FAIL (execute)')
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
                    cmd = ([uxfconvert, '-f', expected, new_actual]
                           if roundtrip == YR else
                           [uxfconvert, expected, new_actual])
                    reply = subprocess.call(cmd)
                    cmd = ' '.join(cmd)
                    if reply != 0:
                        print(f'{cmd} • FAIL (execute roundtrip)')
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
    cmd = [uxfconvert, '-f', '1.csv', '2.csv', actual]
    reply = subprocess.call(cmd)
    cmd = ' '.join(cmd)
    if reply != 0:
        print(f'{cmd} • FAIL (execute)')
    else:
        expected = 'expected/1-2-csv.uxf'
        ok += compare(cmd, infile, actual, expected, verbose=verbose)
        if not verbose and not ok % 10:
            print('.', end='', flush=True)
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
        print(f'{cmd} • FAIL (compare) {actual} != {expected}')
    except FileNotFoundError:
        print(f'{cmd} • FAIL (missing {expected!r})')
    return 0


def cleanup():
    if os.path.exists('actual'):
        for name in os.listdir('actual'):
            name = f'actual/{name}'
            if os.path.isfile(name):
                os.remove(name)
    else:
        os.mkdir('actual')


def by_number(s):
    match = re.match(r'(?P<name>\D+)(?P<number>\d+)', s)
    if match is not None:
        return match['name'], int(match['number'])
    return s, 0


if __name__ == '__main__':
    main()