#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import os
import subprocess

try:
    EXE = os.path.expanduser('~/app/uxf/rs/target/release/uxf')
    TESTDATA = os.path.expanduser('~/app/uxf/testdata')
    os.chdir(TESTDATA)
finally:
    pass


BASENAMES = (
    't0', 't1', 't2', 't3', 't4', 't5', 't6', 't7', 't8', 't9', 't10',
    't11', 't12', 't13', 't14', 't15', 't16', 't17', 't18', 't19', 't20',
    't21', 't22', 't23', 't24', 't25', 't26', 't27', 't28', 't29', 't30',
    't31', 't32', 't33', 't34', 't35', 't36', 't37', 't38', 't39', 't40',
    't41', 't42', 't43', 't44', 't45', 't46', 't47', 't48', 't49', 't50',
    't51', 't52', 't53', 't54', 't55', 't61', 't62', 't63r', 't63', 't70',
    't71', 't73', 't74', 't75', 't76', 't77', 't78', 't79', 't80', 't81',
    't82', 't83', 't84', 't85')


def main():
    total = ok = 0
    for basename in BASENAMES:
        total += 1
        with open(f'expected/{basename}.tok', 'rt',
                  encoding='utf-8') as file:
            expected = file.read().rstrip()
        cmd = [EXE, f'{basename}.uxf']
        reply = subprocess.run(cmd, capture_output=True, text=True)
        cmd = ' '.join(cmd)
        # We ignore the return code because we expect it to be failure
        actual = reply.stdout.rstrip()
        if expected == actual:
            ok += 1
        else:
            filename = f'/tmp/{basename}.tok'
            with open(filename, 'wt', encoding='utf-8') as file:
                file.write(f'{actual}\n')
            print(f'{cmd} • DIFFERENT: compare with {filename}')
    if total == ok:
        print(f'All {total:,} OK')
    else:
        print(f'{ok:,}/{total:,}')


if __name__ == '__main__':
    main()
