#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

'''
This test is to show a practical use case of saving and loading application
config data.
'''

import contextlib
import os
import random
import shutil
import sys
import tempfile

try:
    PATH = os.path.abspath(os.path.dirname(__file__))
    sys.path.append(os.path.abspath(os.path.join(PATH, '../')))
    import uxf
    old = '/home/mark/bin/sudoku.pyw'
    new = os.path.join(tempfile.gettempdir(), 't/sudoku.py')
    if os.path.isfile(old) and random.choice((0, 1)):
        os.makedirs(os.path.dirname(new), exist_ok=True)
        shutil.copyfile(old, new)
        sys.path.append(os.path.abspath(os.path.dirname(new)))
        from sudoku import Config, Symbols
        SUDOKU = True
    else:
        sys.path.append(os.path.abspath(os.path.join(PATH, '../eg/')))
        from Config import Config, Symbols
        SUDOKU = False
    os.chdir(os.path.join(PATH, '../../testdata')) # move to test data
finally:
    pass


def main():
    regression = False
    if len(sys.argv) > 1 and sys.argv[1] in {'-r', '--regression'}:
        regression = True
    total = ok = 0

    if not regression:
        print('using sudoku.Config' if SUDOKU else 'using eg/Config')

    original_file = os.path.join(tempfile.gettempdir(), 'sudoku.config')
    with open(original_file, 'wt', encoding='utf-8') as file:
        file.write(UXT)

    total += 1
    config1 = Config(original_file) # original and kept as-is
    config2 = Config(original_file) # original but edited
    ok += 1
    uxt1 = config2._uxo.dumps()
    if not regression:
        print(uxt1)

    total += 1
    if config2.width == 540:
        ok += 1
    elif not regression:
        print('fail #2')

    total += 1
    config2.width = 99
    if config2.width == 99:
        ok += 1
    elif not regression:
        print('fail #3')

    total += 1
    config2.symbols = Symbols.ROMAN
    config2.fontsize = 19
    config2.bgcolor2 = 'magenta'
    if (config2.symbols is Symbols.ROMAN and config2.fontsize == 19 and
            config2.bgcolor2 == 'magenta'):
        ok += 1
    elif not regression:
        print('fail #4')

    total += 1
    if not config1._uxo.is_equivalent(config2._uxo,
                                      uxf.Compare.IGNORE_COMMENTS):
        ok += 1
    elif not regression:
        print('fail #5')

    edited_file = os.path.join(tempfile.gettempdir(), 'sudoku-ed.config')
    config2.save(edited_file)

    config3 = Config(edited_file) # edited but edited back to original
    total += 1
    if config2._uxo.is_equivalent(config3._uxo,
                                  uxf.Compare.IGNORE_COMMENTS):
        ok += 1
    elif not regression:
        print('fail #6')

    # return to originals
    config3.bgcolor2 = '#FFE7FF'
    config3.fontsize = 22
    config3.symbols = Symbols.DECIMAL
    config3.width = 540
    total += 1
    if config1._uxo.is_equivalent(config3._uxo,
                                  uxf.Compare.IGNORE_COMMENTS):
        ok += 1
    elif not regression:
        print('fail #7')

    if total == ok:
        with contextlib.suppress(FileNotFoundError):
            os.remove(original_file)
        with contextlib.suppress(FileNotFoundError):
            os.remove(edited_file)

    print(f'total={total} ok={ok}')


UXT = '''uxf 1.0 Sudoku
=initiallyvisible min:int max:int
=SymbolsDecimal
=SymbolsRoman
=size width:int height:int
=numbers page:int game:int
{
  <fontsize> 22
  <general> [table
    (initiallyvisible 28 32)
    (SymbolsDecimal)
    (size 540 550)
    (numbers 372 743)
  ]
  <colors> {str str
    <bg1> <lightyellow>
    <bg2> <#FFE7FF>
    <annotation> <red>
    <confirmed> <blue>
    <number> <navy>
  }
}'''


if __name__ == '__main__':
    main()
