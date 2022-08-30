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


BASENAMES = ('t0', 't1', 't2', 't3', 't4', 't5', 't6', 't7', 't8', 't9',
             't10', 't11', 't12', 't13', 't14', 't15', 't16', 't17', 't18',
             't19', 't20', 't21', 't22', 't23', 't24', 't25')


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
            print(f'{cmd} • DIFFERENT:\n{actual}')
    if total == ok:
        print(f'All {total:,} OK')
    else:
        print(f'{ok:,}/{total:,}')


if __name__ == '__main__':
    main()
