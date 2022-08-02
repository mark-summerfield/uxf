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
    INCLUDE_EXE = os.path.abspath(os.path.join(PATH, '../eg/include.py'))
    os.chdir(os.path.join(PATH, '../../testdata')) # move to test data
finally:
    pass


def main():
    regression = False
    if len(sys.argv) > 1 and sys.argv[1] in {'-r', '--regression'}:
        regression = True
    total = ok = 0

    total += 1
    filename = 'i68inc.uxf.gz'
    cmd = [INCLUDE_EXE, 'i68.uxi', f'actual/{filename}']
    ok += check(cmd, regression, "uxf:W422:t63.uxf:14:unused ttype: 'dob'")
    total += 1
    ok += compare(filename, regression)

    print(f'total={total} ok={ok}')


def check(cmd, regression, expected):
    if sys.platform.startswith('win'):
        cmd = ['py.bat'] + cmd
    reply = subprocess.run(cmd, capture_output=True, text=True)
    cmd = ' '.join(cmd)
    if reply.returncode != 0:
        if not regression:
            print(f'{cmd} • FAIL command failed to run')
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
        actual_uxo = uxf.load(f'actual/{filename}', on_event=on_event)
        expected_uxo = uxf.load(f'expected/{filename}', on_event=on_event)
        if actual_uxo == expected_uxo:
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
