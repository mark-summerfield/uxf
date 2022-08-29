#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

'''
This shows a practical use case of saving and loading application
configuration data, preserving comments, providing defaults, and validating.

The configuration file looks like this:

    uxf 1 Sudoku
    #<Sudoku Configuration>
    =initiallyvisible min:int max:int
    =#<internal use> numbers page:int game:int
    =size width:int height:int
    =#<Or SymbolsRoman> SymbolsDecimal
    =#<Or SymbolsDecimal> SymbolsRoman
    {#<fontsize range 8-36>
    <fontsize> 22
    <general>
        [table
        (#<range 9-72> initiallyvisible 28 32)
        (#<Or SymbolsRoman> SymbolsDecimal)
        (#<width and height &gt;= -1> size 540 550)
        (#<internal use: don't edit> numbers 385 769)
        ]
    <colors> {#<colors HTML-style #HHHHHH or names> str str
              <bg1> <lightyellow>
              <bg2> <#FFE7FF>
              <annotation> <red>
              <confirmed> <blue>
              <number> <navy>}
    }

But this Config module lets its users ignore the structure entirely (while
preserving it) to provide simple attribute-based access:

    size = config.fontsize * 0.5
    config.gamenumber += 1
    config.pagenumber += 1
    color = config.bgcolor1
    colors = dict(bgcolor1=config.bgcolor1,
                  bgcolor2=config.bgcolor2,
                  annotationcolor=config.annotationcolor,
                  confirmedcolor=config.confirmedcolor,
                  numbercolor=config.numbercolor)

And for colors name access is also supported:

    for name, color in colors.items():
        config[name] = color

It would be perfectly possible (and easiest) to operate directly on the Uxf
object loaded from file, e.g.,

    uxo['colors']['bg1'] = 'red'
    uxo['general']['numbers'].first.game = 19
    # -or-
    # uxo['general']['numbers'][0].game = 19

Providing a wrapper class (like Config), allows for a well-structured UXF
file and at the same time easy access to the loaded data.
'''

import enum
import os
import sys

try:
    import uxf
except ImportError: # needed for development
    sys.path.append(os.path.abspath(os.path.dirname(__file__) + '/..'))
    import uxf


class Config:

    def __init__(self, filename=None):
        '''All the data is held in self._uxo in UXF format. However, the
        class' properties get and set using the appropriate format (e.g.,
        .symbols as a Symbols enum, etc.)'''
        self.filename = filename
        self._prepare_uxo()
        self._set_defaults()
        if self.filename is not None:
            self.load()


    def _prepare_uxo(self):
        initiallyvisible = uxf.TClass(
            INITIALLYVISIBLE, (uxf.Field('min', 'int'),
                               uxf.Field('max', 'int')))
        decimal = uxf.TClass(DECIMAL, comment='Or SymbolsRoman')
        roman = uxf.TClass(ROMAN, comment='Or SymbolsDecimal')
        size = uxf.TClass('size', (uxf.Field('width', 'int'),
                                   uxf.Field('height', 'int')))
        numbers = uxf.TClass('numbers', (uxf.Field('page', 'int'),
                                         uxf.Field('game', 'int')),
                             comment='internal use')
        self._uxo = uxf.Uxf({}, custom='Sudoku')
        self._uxo.comment = 'Sudoku Configuration'
        self._uxo.add_tclasses(initiallyvisible, decimal, roman, size,
                               numbers)


    def _set_defaults(self):
        general = uxf.List((
            uxf.Table(self._uxo.tclasses[INITIALLYVISIBLE],
                      records=(28, 32), comment='range 9-72'),
            uxf.Table(self._uxo.tclasses[DECIMAL],
                      comment='Or SymbolsRoman'),
            uxf.Table(self._uxo.tclasses['size'], records=(-1, -1),
                      comment='width and height >= -1'),
            uxf.Table(self._uxo.tclasses['numbers'], records=(1, 1),
                      comment='internal use: don\'t edit')), vtype='table')
        colors = uxf.Map(dict(bg1='lightyellow', bg2='#FFE7FF',
                              annotation='red', confirmed='blue',
                              number='navy'), ktype='str', vtype='str',
                         comment='colors HTML-style #HHHHHH or names')
        self._uxo.value = uxf.Map(dict(fontsize=18, general=general,
                                       colors=colors),
                                  comment='fontsize range 8-36')


    def load(self, filename=None):
        if filename is not None:
            self.filename = filename
        try:
            uxo = uxf.load(self.filename)
            self._maybe_merge_comment(self._uxo, uxo)
            self._maybe_merge_comment(self._uxo.value, uxo.value)
            for name, value in uxo.value.items():
                if name == COLORS:
                    self._load_colors(value)
                elif name == GENERAL:
                    self._load_general(value)
                elif name == FONTSIZE:
                    self.fontsize = value
        except (uxf.Error, OSError) as err:
            print(f'Failed to load configuration file: {err}. '
                  'Will try to create a new one on exit.')


    def _load_colors(self, value):
        for name, color in value.items():
            if name in COLORNAMES:
                self[name] = color


    def _load_general(self, value):
        for table in value:
            if table.ttype == INITIALLYVISIBLE:
                record = table.first
                self.mininitiallyvisible = record.min
                self.maxinitiallyvisible = record.max
            elif table.ttype == SIZE:
                record = table.first
                self.width = record.width
                self.height = record.height
            elif table.ttype == DECIMAL:
                self.symbols = Symbols.DECIMAL
            elif table.ttype == ROMAN:
                self.symbols = Symbols.ROMAN
            elif table.ttype == NUMBERS:
                record = table.first
                self.pagenumber = record.page
                self.gamenumber = record.game
            if table.comment:
                self._maybe_merge_comment(self._table_of(table.ttype),
                                          table)


    def _maybe_merge_comment(self, old, new):
        if new.comment:
            if not old.comment:
                old.comment = new.comment
            elif new.comment not in old.comment.splitlines():
                old.comment += '\n' + new.comment


    def save(self, filename=None):
        if filename is not None:
            self.filename = filename
        self._uxo.dump(self.filename)


    @property
    def symbols(self):
        symbol, _ = self._symbol_and_index()
        if symbol is not None:
            return symbol
        self._uxo.value[GENERAL].append(
            uxf.Table(self._uxo.tclasses[DECIMAL]))
        return Symbols.DECIMAL


    def _symbol_and_index(self):
        self._symbol_for_name = {symbol.name.upper(): symbol
                                 for symbol in Symbols}
        for i, value in enumerate(self._uxo.value[GENERAL]):
            ttype = value.ttype.replace('Symbols', '').upper()
            symbol = self._symbol_for_name.get(ttype)
            if symbol is not None:
                return symbol, i
        return None, -1


    @symbols.setter
    def symbols(self, value):
        symbol, i = self._symbol_and_index()
        if symbol is not None:
            if symbol is value:
                return # unchanged
        if i == -1:
            i = len(self._uxo.value[GENERAL])
            self._uxo.value[GENERAL].append(None)
        ttype = f'Symbols{value.name.capitalize()}'
        self._uxo.value[GENERAL][i] = uxf.Table(self._uxo.tclasses[ttype])


    @property
    def fontsize(self):
        return self._uxo.value[FONTSIZE]


    @fontsize.setter
    def fontsize(self, value):
        if isinstance(value, int) and 8 <= value <= 36:
            self._uxo.value[FONTSIZE] = value


    @property
    def width(self):
        return self._table_of(SIZE).first.width


    @width.setter
    def width(self, value):
        if isinstance(value, int) and value >= -1:
            self._table_of(SIZE).first.width = value


    def _table_of(self, what):
        for i, table in enumerate(self._uxo.value[GENERAL]):
            if table.ttype == what:
                return self._uxo.value[GENERAL][i]


    @property
    def height(self):
        return self._table_of(SIZE).first.height


    @height.setter
    def height(self, value):
        if isinstance(value, int) and value >= -1:
            self._table_of(SIZE).first.height = value


    @property
    def mininitiallyvisible(self):
        return self._table_of(INITIALLYVISIBLE).first.min


    @mininitiallyvisible.setter
    def mininitiallyvisible(self, value):
        if isinstance(value, int) and 9 <= value <= 72:
            self._table_of(INITIALLYVISIBLE).first.min = value


    @property
    def maxinitiallyvisible(self):
        return self._table_of(INITIALLYVISIBLE).first.max


    @maxinitiallyvisible.setter
    def maxinitiallyvisible(self, value):
        if isinstance(value, int) and 9 <= value <= 72:
            self._table_of(INITIALLYVISIBLE).first.max = value


    @property
    def pagenumber(self):
        return self._table_of(NUMBERS).first.page


    @pagenumber.setter
    def pagenumber(self, value):
        if isinstance(value, int):
            self._table_of(NUMBERS).first.page = value


    @property
    def gamenumber(self):
        return self._table_of(NUMBERS).first.game


    @gamenumber.setter
    def gamenumber(self, value):
        if isinstance(value, int):
            self._table_of(NUMBERS).first.game = value


    @property
    def bgcolor1(self):
        return self._uxo.value[COLORS][BGCOLOR1]


    @bgcolor1.setter
    def bgcolor1(self, value):
        if value != '':
            self._uxo.value[COLORS][BGCOLOR1] = value


    @property
    def bgcolor2(self):
        return self._uxo.value[COLORS][BGCOLOR2]


    @bgcolor2.setter
    def bgcolor2(self, value):
        if value != '':
            self._uxo.value[COLORS][BGCOLOR2] = value


    @property
    def annotationcolor(self):
        return self._uxo.value[COLORS][ANNOTATIONCOLOR]


    @annotationcolor.setter
    def annotationcolor(self, value):
        if value != '':
            self._uxo.value[COLORS][ANNOTATIONCOLOR] = value


    @property
    def confirmedcolor(self):
        return self._uxo.value[COLORS][CONFIRMEDCOLOR]


    @confirmedcolor.setter
    def confirmedcolor(self, value):
        if value != '':
            self._uxo.value[COLORS][CONFIRMEDCOLOR] = value


    @property
    def numbercolor(self):
        return self._uxo.value[COLORS][NUMBERCOLOR]


    @numbercolor.setter
    def numbercolor(self, value):
        if value != '':
            self._uxo.value[COLORS][NUMBERCOLOR] = value


    def __getitem__(self, name):
        basename = name.replace('color', '')
        if basename in COLORNAMES:
            return self._uxo.value[COLORS][basename]
        raise Error(f'{self.__class__.__name__} ignored invalid color '
                    f'attribute name {name!r}')


    def __setitem__(self, name, value):
        basename = name.replace('color', '')
        if basename in COLORNAMES:
            if value != '':
                self._uxo.value[COLORS][basename] = value
        else:
            raise Error(f'{self.__class__.__name__} can\'t set invalid '
                        f'color attribute name {name!r}')


class Symbols(enum.IntEnum):
    DECIMAL = 1
    ROMAN = 2


class Error(Exception):
    pass


INITIALLYVISIBLE = 'initiallyvisible'
DECIMAL = 'SymbolsDecimal'
ROMAN = 'SymbolsRoman'
GENERAL = 'general'
FONTSIZE = 'fontsize'
SIZE = 'size'
NUMBERS = 'numbers'
COLORS = 'colors'
BGCOLOR1 = 'bg1'
BGCOLOR2 = 'bg2'
ANNOTATIONCOLOR = 'annotation'
CONFIRMEDCOLOR = 'confirmed'
NUMBERCOLOR = 'number'
COLORNAMES = {BGCOLOR1, BGCOLOR2, ANNOTATIONCOLOR, CONFIRMEDCOLOR,
              NUMBERCOLOR}
