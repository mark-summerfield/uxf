#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import datetime
import enum
import io
import sys
from xml.sax.saxutils import escape

import uxf


def main():
    if len(sys.argv) == 1 or sys.argv[1] in {'-h', '--help'}:
        raise SystemExit('usage: oppen.py <file.uxf>')
    uxo = uxf.load(sys.argv[1], on_event=lambda *_, **__: None)
    pp = PrettyPrinter(wrap_width=76, realdp=3)
    uxo.visit(pp)
    pp.pprint(out=sys.stdout)


# TODO move to uxf.py if successful

class PrettyPrinter: # Functor that can be used as a visitor

    def __init__(self, wrap_width=96, realdp=None):
        self.tokens = []
        self.wrap_width = wrap_width
        self.realdp = realdp
        self.depth = 0


    def __call__(self, kind, value):
        if kind is uxf.VisitKind.UXF_BEGIN:
            header = 'uxf 1.0'
            if value.custom:
                header += f' {value.custom}'
            self.puts(f'{header}')
            self.nl()
            if value.comment:
                self.comment(value.comment)
                self.nl()
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
            if self.tokens[-1].kind is TokenKind.WS:
                self.tokens.pop() # Don't need WS before closer
            self.puts(']')
            self.end()
            self.depth -= 1
        elif kind is uxf.VisitKind.MAP_BEGIN:
            self.depth += 1
            self.map_begin(value)
        elif kind is uxf.VisitKind.MAP_END:
            if self.tokens[-1].kind is TokenKind.WS:
                self.tokens.pop() # Don't need WS before closer
            self.puts('}')
            self.end()
            self.depth -= 1
        elif kind is uxf.VisitKind.TABLE_BEGIN:
            self.depth += 1
            self.table_begin(value)
        elif kind is uxf.VisitKind.TABLE_END:
            if self.tokens[-1].kind is TokenKind.WS:
                self.tokens.pop() # Don't need WS before closer
            self.puts(')')
            self.end()
            self.depth -= 1
        elif kind is uxf.VisitKind.RECORD_BEGIN:
            self.depth += 1
            self.begin()
        elif kind is uxf.VisitKind.RECORD_END:
            self.end()
            self.nl()
            self.depth -= 1
        elif kind is uxf.VisitKind.VALUE:
            self.scalar(value)
            self.ws()


    def begin(self):
        self.tokens.append(Token(TokenKind.BEGIN, depth=self.depth))


    def end(self):
        self.tokens.append(Token(TokenKind.END, depth=self.depth))


    def puts(self, s):
        self.tokens.append(Token(TokenKind.STRING, s, depth=self.depth))


    def ws(self):
        self.tokens.append(Token(TokenKind.WS, depth=self.depth))


    def nl(self):
        self.tokens.append(Token(TokenKind.NL, depth=self.depth))


    def eof(self):
        self.tokens.append(Token(TokenKind.EOF, depth=self.depth))


    def list_begin(self, value):
        self.begin()
        self.puts('[')
        if value.comment:
            self.comment(value.comment)
        if value.vtype:
            if value.comment:
                self.ws()
            self.puts(value.vtype)


    def map_begin(self, value):
        self.begin()
        self.puts('{')
        if value.comment:
            self.comment(value.comment)
        if value.ktype:
            if value.comment:
                self.ws()
            self.puts(value.ktype)
            if value.vtype:
                self.puts(f' {value.vtype}')


    def table_begin(self, value):
        self.begin()
        self.puts('(')
        if value.comment:
            self.comment(value.comment)
        self.puts(value.ttype)


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
            ampersand = False
            span = self.wrap_width - 2
            while text: # Try to split on words or newlines first
                i = text.rfind(' ', 0, span)
                if i == -1:
                    i = text.rfind('\n', 0, span)
                if i > -1:
                    i += 1 # include the found whitespace
                    if ampersand:
                        self.ampersand()
                    self.puts(f'{prefix}<{text[:i]}>')
                    text = text[i:]
                    ampersand = True
                    prefix = ''
                else:
                    break
            # if we can't split on words, split anywhere
            if text:
                for i in range(0, len(text), span):
                    if ampersand:
                        self.ampersand()
                    self.puts(f'{prefix}<{text[i:i + span]}>')
                    ampersand = True
                    prefix = ''
        else:
            self.puts(f'{prefix}<{text}>')


    def ampersand(self):
        self.ws()
        self.puts('&')
        self.ws()


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


    def pprint(self, out=None):
        out = out or io.StringIO()
        # TODO pprint using Oppen algorithm
        out.write(' TOKENS '.center(40, '-'))
        out.write('\n')
        for token in self.tokens:
            out.write(f'{token}\n')
        out.write(' === '.center(40, '-'))
        out.write('\n')




@enum.unique
class TokenKind(enum.Enum):
    BEGIN = enum.auto()
    END = enum.auto()
    STRING = enum.auto()
    WS = enum.auto() # output either ' ' or '\n' at pprint's option
    NL = enum.auto() # output '\n'
    EOF = enum.auto()


class Token:

    def __init__(self, kind, value='', *, depth=0):
        self.kind = kind
        self.value = value
        self.depth = depth


    @property
    def is_multiline(self):
        return '\n' in self.value


    def size(self):
        if self.is_multiline:
            return self.value.find('\n') + 1
        return len(self.value)


    def __repr__(self):
        indent = self.depth * '   '
        if self.value == '':
            return f'{indent}{self.__class__.__name__}({self.kind.name})'
        return (f'{indent}{self.__class__.__name__}({self.kind.name}, '
                f'{self.value!r})')


if __name__ == '__main__':
    main()
