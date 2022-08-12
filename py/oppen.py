#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import datetime
import enum
import functools
import sys
from xml.sax.saxutils import escape

import uxf


def main():
    if len(sys.argv) == 1 or sys.argv[1] in {'-h', '--help'}:
        raise SystemExit('usage: oppen.py <file.uxf>')
    wrap_width = 76
    realdp = 3
    uxo = uxf.load(sys.argv[1], on_event=lambda *_, **__: None)
    tokens = Tokens()
    state_visitor = functools.partial(visitor, tokens=tokens, realdp=realdp,
                                      wrap_width=wrap_width)
    uxo.visit(state_visitor)
    print('TOKENS')
    for token in tokens.tokens:
        print(f'  {token}')
    print('-' * 40)
    # TODO output header line (do this last since easiest)
    # TODO pprint using Oppen algorithm


def visitor(kind, value, *, tokens, wrap_width=96, realdp=6):
    if kind is uxf.VisitKind.UXF_BEGIN:
        # TODO imports
        # TODO ttypes
        tokens.begin()
    elif kind is uxf.VisitKind.UXF_END:
        tokens.end()
        tokens.eof()
    elif kind is uxf.VisitKind.LIST_BEGIN:
        _list_begin(tokens, value)
    elif kind is uxf.VisitKind.LIST_END:
        tokens.brk()
        tokens.puts(']')
        tokens.end()
    elif kind is uxf.VisitKind.MAP_BEGIN:
        _map_begin(tokens, value)
    elif kind is uxf.VisitKind.MAP_END:
        tokens.brk()
        tokens.puts('}')
        tokens.end()
    elif kind is uxf.VisitKind.TABLE_BEGIN:
        _table_begin(tokens, value)
    elif kind is uxf.VisitKind.TABLE_END:
        tokens.brk()
        tokens.puts(')')
        tokens.end()
    elif kind is uxf.VisitKind.RECORD_BEGIN:
        tokens.begin()
    elif kind is uxf.VisitKind.RECORD_END:
        tokens.nl()
        tokens.end()
    elif kind in {uxf.VisitKind.MAP_KEY, uxf.VisitKind.VALUE}:
        _scalar(tokens, value, wrap_width, realdp)


def _list_begin(tokens, value):
    tokens.begin()
    tokens.puts('[')
    if value.comment:
        tokens.puts(f'#<{escape(value.comment)}>')
    if value.vtype:
        if value.comment:
            tokens.brk()
        tokens.puts(value.vtype)
    tokens.brk()


def _map_begin(tokens, value):
    tokens.begin()
    tokens.puts('{')
    if value.comment:
        tokens.puts(f'#<{escape(value.comment)}>')
    if value.ktype:
        if value.comment:
            tokens.brk()
        tokens.puts(value.ktype)
        if value.vtype:
            tokens.puts(f' {value.vtype}')
    tokens.brk()


def _table_begin(tokens, value):
    tokens.begin()
    tokens.puts('(')
    if value.comment:
        tokens.puts(f'#<{escape(value.comment)}> ')
    tokens.puts(value.ttype)
    tokens.brk()


def _real(tokens, value, realdp):
    if realdp is not None:
        value = round(value, realdp)
    text = str(value)
    if '.' not in text and 'e' not in text and 'E' not in text:
        text += '.0'
    tokens.puts(text)


def _str(tokens, value, wrap_width):
    text = escape(value)
    if wrap_width and len(text) + 2 >= wrap_width:
        sep = ''
        span = wrap_width - 2
        while text: # Try to split on words or newlines first
            i = text.rfind(' ', 0, span)
            if i == -1:
                i = text.rfind('\n', 0, span)
            if i > -1:
                i += 1 # include the found whitespace
                if sep:
                    tokens.brk()
                    tokens.puts(sep)
                    tokens.brk()
                tokens.puts(f'<{text[:i]}>')
                text = text[i:]
                sep = '&'
            else:
                break
        # if we can't split on words, split anywhere
        if text:
            for i in range(0, len(text), span):
                if sep:
                    tokens.brk()
                    tokens.puts(sep)
                    tokens.brk()
                tokens.puts(f'<{text[i:i + span]}>')
                sep = '&'
    else:
        tokens.puts(f'<{text}>')


def _bytes(tokens, value, wrap_width):
    text = value.hex().upper()
    if len(text) + 4 >= wrap_width:
        span = wrap_width - 2
        tokens.puts('(:')
        for i in range(0, len(text), span):
            tokens.puts(text[i:i + span])
        tokens.puts(':)')
    else:
        tokens.puts(f'(:{text}:)')


def _scalar(tokens, value, wrap_width, realdp):
    if value is None:
        tokens.puts('?')
    elif isinstance(value, bool):
        tokens.puts('yes' if value else 'no')
    elif isinstance(value, int):
        tokens.puts(str(value))
    elif isinstance(value, float):
        _real(tokens, value, realdp)
    elif isinstance(value, (datetime.date, datetime.datetime)):
        tokens.puts(value.isoformat()[:19]) # 1-second resolution
    elif isinstance(value, str):
        _str(tokens, value, wrap_width)
    elif isinstance(value, (bytes, bytearray)):
        _bytes(tokens, value, wrap_width)
    else:
        print(561, 'unexpected value of type '
                   f'{value.__class__.__name__}: {value!r};'
                   'consider using a ttype')


class Tokens:

    def __init__(self):
        self.tokens = []


    def begin(self):
        self.tokens.append(Token(TokenKind.BEGIN))


    def end(self):
        self.tokens.append(Token(TokenKind.END))


    def eof(self):
        self.tokens.append(Token(TokenKind.EOF))


    def puts(self, s):
        self.tokens.append(Token(TokenKind.STRING, s))


    def brk(self):
        self.tokens.append(Token(TokenKind.BREAK))


    def nl(self):
        self.tokens.append(Token(TokenKind.NEWLINE))


@enum.unique
class TokenKind(enum.Enum):
    BEGIN = enum.auto()
    END = enum.auto()
    BREAK = enum.auto()
    NEWLINE = enum.auto()
    STRING = enum.auto()
    EOF = enum.auto()


class Token:

    def __init__(self, kind, value=''):
        self.kind = kind
        self.value = value


    def __repr__(self):
        if self.value == '':
            return f'{self.__class__.__name__}({self.kind.name})'
        return (f'{self.__class__.__name__}({self.kind.name}, '
                f'{self.value!r})')


if __name__ == '__main__':
    main()
