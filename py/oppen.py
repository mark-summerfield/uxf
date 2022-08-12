#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import datetime
import enum
import sys
from xml.sax.saxutils import escape

import uxf


def main():
    if len(sys.argv) == 1 or sys.argv[1] in {'-h', '--help'}:
        raise SystemExit('usage: oppen.py <file.uxf>')
    uxo = uxf.load(sys.argv[1], on_event=lambda *_, **__: None)
    lexer = PrettyPrintLexer(wrap_width=76, realdp=3)
    uxo.visit(lexer)
    print(' TOKENS '.center(40, '-'))
    for token in lexer.tokens:
        print(token)
    print(' === '.center(40, '-'))
    # TODO pprint using Oppen algorithm


class PrettyPrintLexer: # Functor that can be used as a visitor

    def __init__(self, wrap_width=96, realdp=None):
        self.tokens = []
        self.wrap_width = wrap_width
        self.realdp = realdp
        self.depth = 0 # for debugging


    def __call__(self, kind, value):
        if kind is uxf.VisitKind.UXF_BEGIN:
            header = 'uxf 1.0'
            if value.custom:
                header += f' {value.custom}'
            self.puts(f'{header}\n')
            if value.comment:
                self.comment(value.comment)
            self.begin()
            self.depth += 1
            self.puts('TODO: imports') # TODO
            self.puts('TODO: ttype defs') # TODO
            self.depth -= 1
            self.end()
        elif kind is uxf.VisitKind.UXF_END:
            self.eof()
        elif kind is uxf.VisitKind.LIST_BEGIN:
            self.depth += 1
            self.list_begin(value)
        elif kind is uxf.VisitKind.LIST_END:
            self.puts(']')
            self.end()
            self.depth -= 1
        elif kind is uxf.VisitKind.MAP_BEGIN:
            self.depth += 1
            self.map_begin(value)
        elif kind is uxf.VisitKind.MAP_END:
            self.puts('}')
            self.end()
            self.depth -= 1
        elif kind is uxf.VisitKind.TABLE_BEGIN:
            self.depth += 1
            self.table_begin(value)
        elif kind is uxf.VisitKind.TABLE_END:
            self.puts(')')
            self.end()
            self.depth -= 1
        elif kind is uxf.VisitKind.RECORD_BEGIN:
            self.depth += 1
            self.begin()
        elif kind is uxf.VisitKind.RECORD_END:
            self.end()
            self.brk()
            self.depth -= 1
        elif kind is uxf.VisitKind.VALUE:
            self.scalar(value)


    def begin(self):
        self.tokens.append(Token(TokenKind.BEGIN, depth=self.depth))


    def end(self):
        self.tokens.append(Token(TokenKind.END, depth=self.depth))


    def eof(self):
        self.tokens.append(Token(TokenKind.EOF, depth=self.depth))


    def puts(self, s):
        self.tokens.append(Token(TokenKind.STRING, s, depth=self.depth))


    def sep(self):
        self.tokens.append(Token(TokenKind.STRING, ' ', depth=self.depth))


    def brk(self):
        self.tokens.append(Token(TokenKind.BREAK, depth=self.depth))


    def list_begin(self, value):
        self.begin()
        self.puts('[')
        if value.comment:
            self.comment(value.comment)
        if value.vtype:
            if value.comment:
                self.sep()
            self.puts(value.vtype)
        if len(value):
            self.sep()


    def map_begin(self, value):
        self.begin()
        self.puts('{')
        if value.comment:
            self.comment(value.comment)
        if value.ktype:
            if value.comment:
                self.sep()
            self.puts(value.ktype)
            if value.vtype:
                self.puts(f' {value.vtype}')
        if len(value):
            self.sep()


    def table_begin(self, value):
        self.begin()
        self.puts('(')
        if value.comment:
            self.comment(value.comment)
        self.puts(value.ttype)
        if len(value):
            self.sep()


    def real(self, value):
        if self.realdp is not None:
            value = round(value, self.realdp)
        text = str(value)
        if '.' not in text and 'e' not in text and 'E' not in text:
            text += '.0'
        self.puts(text)


    def comment(self, value):
        self.str_(value, prefix='#')


    def str_(self, value, *, prefix=''):
        text = escape(value)
        if self.wrap_width and len(text) + 2 >= self.wrap_width:
            sep = ''
            span = self.wrap_width - 2
            while text: # Try to split on words or newlines first
                i = text.rfind(' ', 0, span)
                if i == -1:
                    i = text.rfind('\n', 0, span)
                if i > -1:
                    i += 1 # include the found whitespace
                    if sep:
                        self.puts(sep)
                    self.puts(f'{prefix}<{text[:i]}>')
                    text = text[i:]
                    sep = ' & '
                    prefix = ''
                else:
                    break
            # if we can't split on words, split anywhere
            if text:
                for i in range(0, len(text), span):
                    if sep:
                        self.puts(sep)
                    self.puts(f'{prefix}<{text[i:i + span]}>')
                    sep = ' & '
                    prefix = ''
        else:
            self.puts(f'{prefix}<{text}>')


    def bytes_(self, value):
        text = value.hex().upper()
        if len(text) + 4 >= self.wrap_width:
            span = self.wrap_width - 2
            self.puts('(:')
            for i in range(0, len(text), span):
                self.puts(text[i:i + span])
            self.puts(':)')
        else:
            self.puts(f'(:{text}:)')


    def scalar(self, value):
        if value is None:
            self.puts('?')
        elif isinstance(value, bool):
            self.puts('yes' if value else 'no')
        elif isinstance(value, int):
            self.puts(str(value))
        elif isinstance(value, float):
            self.real(value)
        elif isinstance(value, (datetime.date, datetime.datetime)):
            self.puts(value.isoformat()[:19]) # 1-second resolution
        elif isinstance(value, str):
            self.str_(value)
        elif isinstance(value, (bytes, bytearray)):
            self.bytes_(value)
        else:
            print(561, 'unexpected value of type '
                  f'{value.__class__.__name__}: {value!r}; consider '
                  'using a ttype')



@enum.unique
class TokenKind(enum.Enum):
    BEGIN = enum.auto()
    END = enum.auto()
    BREAK = enum.auto()
    STRING = enum.auto()
    EOF = enum.auto()


class Token:

    def __init__(self, kind, value='', *, depth=0):
        self.kind = kind
        self.value = value
        self.depth = depth # for debugging


    def __len__(self):
        '''for strings with embedded newlines the length is effectively that
        of the string's last line'''
        i = self.value.rfind('\n')
        if i == -1:
            return len(self.value)
        return len(self.value[i + 1:])


    def __repr__(self):
        indent = self.depth * '   '
        if self.value == '':
            return f'{indent}{self.__class__.__name__}({self.kind.name})'
        return (f'{indent}{self.__class__.__name__}({self.kind.name}, '
                f'{self.value!r})')


if __name__ == '__main__':
    main()
