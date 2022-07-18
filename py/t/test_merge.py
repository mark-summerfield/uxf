#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import os
import subprocess
import sys

try:
    PATH = os.path.abspath(os.path.dirname(__file__))
    sys.path.append(os.path.abspath(os.path.join(PATH, '../')))
    import uxf
    sys.path.append(os.path.abspath(os.path.join(PATH, '../eg/')))
    import eq
    MERGE_EXE = os.path.abspath(os.path.join(PATH, '../eg/merge.py'))
    os.chdir(os.path.join(PATH, '../../testdata')) # move to test data
finally:
    pass


def main():
    regression = False
    if len(sys.argv) > 1 and sys.argv[1] in {'-r', '--regression'}:
        regression = True
    total = ok = 0

    filename = 'm1.uxf.gz'
    cmd = [MERGE_EXE, '--list', '--outfile', f'actual/{filename}',
           't15.uxf', 't16.uxf', 't19.uxf', 't47.uxf', 't63.uxf']
    total += 2
    done = check(cmd, regression, filename,
                 "uxf.py:t63.uxf:14:#422:unused ttype: 'dob'")
    if done:
        ok += 1
        ok += compare(filename, regression)

    filename = 'm2.uxf.gz'
    cmd = [MERGE_EXE, '--outfile', f'actual/{filename}', 't15.uxf',
           't16.uxf', 't19.uxf', 't47.uxf', 't63.uxf']
    total += 2
    done = check(cmd, regression, filename,
                 "uxf.py:t63.uxf:14:#422:unused ttype: 'dob'")
    if done:
        ok += 1
        ok += compare(filename, regression)

    print(f'total={total} ok={ok}')


def check(cmd, regression, filename, expected):
    if sys.platform.startswith('win'):
        cmd = ['py.bat'] + cmd
    reply = subprocess.run(cmd, capture_output=True, text=True)
    cmd = ' '.join(cmd)
    if reply.returncode != 0:
        if not regression:
            print(f'{cmd} • FAIL command failed to run')
        return 0
    saved = f'saved actual/{filename}'
    stdout = reply.stdout.strip()
    if stdout != saved:
        if not regression:
            print(f'{cmd} • FAIL expected {saved!r}, got {stdout!r}')
            return 0
    stderr = reply.stderr.strip()
    if stderr != expected:
        if not regression:
            print(f'{cmd} • FAIL expected {expected!r}, got {stderr!r}')
            return 0
    return 1


def compare(filename, regression):
    def on_event(*_args, **_kwargs):
        pass

    try:
        name = f'actual/{filename}'
        actual_uxo = uxf.load(name, on_event=on_event)
    except uxf.Error as err:
        if not regression:
            print(f'compare • FAIL {name!r}: {err}')
        return 0
    try:
        name = f'expected/{filename}'
        expected_uxo = uxf.load(name, on_event=on_event)
    except uxf.Error as err:
        if not regression:
            print(f'compare • FAIL {name!r}: {err}')
        return 0
    try:
        if eq.eq(actual_uxo, expected_uxo):
            return 1
        if not regression:
            print(f'compare • FAIL actual != expected {filename!r}')
        return 0
    except uxf.Error as err:
        if not regression:
            print(f'compare • FAIL {filename!r}: {err}')
        return 0


if __name__ == '__main__':
    main()
