#!/usr/bin/env py310
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import datetime
import functools
import sys
from xml.sax.saxutils import escape

import uxf

try:
    sys.path.append('/home/mark/opt')
    from oppen_pretty_printer import pprint, Tokens
except ImportError:
    raise


def main():
    if len(sys.argv) == 1 or sys.argv[1] in {'-h', '--help'}:
        raise SystemExit('usage: tp.py <filename.uxf>')
    line_width = 76
    uxo = uxf.load(sys.argv[1], on_event=lambda *_, **__: None)
    tokens = []
    state_visitor = functools.partial(visitor, tokens=tokens,
                                      line_width=line_width)
    uxo.visit(state_visitor)
    print(tokens)
    print()
    pprint(tokens, line_width)


def visitor(kind, value, *, tokens, line_width=96, realdp=6):
    begin = lambda: tokens.append(Tokens.BEGIN())
    end = lambda: tokens.append(Tokens.END())
    eof = lambda: tokens.append(Tokens.EOF())
    nl = lambda: tokens.append(Tokens.LINEBREAK)
    onl = lambda: tokens.append(Tokens.BREAK())
    puts = lambda s: tokens.append(Tokens.STRING(s))
    sep = lambda: tokens.append(Tokens.STRING(' '))
    if kind is uxf.VisitKind.UXF_BEGIN:
        begin()
    elif kind is uxf.VisitKind.UXF_END:
        end()
        eof()
    elif kind is uxf.VisitKind.MAP_BEGIN:
        begin()
        puts('{')
        if value.comment:
            puts(f'#<{escape(value.comment)}>')
            onl()
        if value.ktype:
            if value.comment:
                sep()
                puts(' ')
            puts(value.ktype)
            if value.vtype:
                puts(f' {value.vtype}')
            onl()
    elif kind is uxf.VisitKind.MAP_END:
        puts('}')
        end()
    elif kind is uxf.VisitKind.LIST_BEGIN:
        begin()
        puts('[')
        if value.comment:
            puts(f'#<{escape(value.comment)}>')
            onl()
        if value.vtype:
            if value.comment:
                sep()
                puts(' ')
            puts(value.vtype)
            onl()
    elif kind is uxf.VisitKind.LIST_END:
        puts(']')
        end()
    elif kind is uxf.VisitKind.TABLE_BEGIN:
        begin()
        puts('(')
        if value.comment:
            puts(f'#<{escape(value.comment)}> ')
        puts(value.ttype)
        onl()
    elif kind is uxf.VisitKind.TABLE_END:
        puts(')')
        end()
    elif kind is uxf.VisitKind.RECORD_BEGIN:
        begin()
    elif kind is uxf.VisitKind.RECORD_END:
        nl()
        end()
    elif kind in {uxf.VisitKind.MAP_KEY, uxf.VisitKind.VALUE}:
        if value is None:
            puts('?')
        elif isinstance(value, bool):
            puts('yes' if value else 'no')
        elif isinstance(value, int):
            puts(str(value))
        elif isinstance(value, float):
            if realdp is not None:
                value = round(value, realdp)
            text = str(value)
            if '.' not in text and 'e' not in text and 'E' not in text:
                text += '.0'
            puts(text)
        elif isinstance(value, (datetime.date, datetime.datetime)):
            puts(value.isoformat()[:19]) # 1-second resolution
        elif isinstance(value, str):
            s = f'<{escape(value)}>'
            if len(s) <= line_width:
                puts(s)
            else: # TODO change to a multistring
                for i in range(0, len(s), line_width):
                    puts(s[i:i + line_width])
        elif isinstance(value, (bytes, bytearray)):
            # TODO chunkize if len() > line_width
            puts(f'(:{value.hex().upper()}:)')
        else:
            print(561, 'unexpected value of type '
                       f'{value.__class__.__name__}: {value!r};'
                       'consider using a ttype')
        onl()


if __name__ == '__main__':
    main()
