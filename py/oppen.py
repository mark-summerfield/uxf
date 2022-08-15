#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import datetime
import enum
import functools
import io
import sys
from xml.sax.saxutils import escape

import uxf
from uxf import _EventMixin


def main():
    if len(sys.argv) == 1 or sys.argv[1] in {'-h', '--help'}:
        raise SystemExit('usage: oppen.py <file.uxf>')
    filename = sys.argv[1]
    on_event = functools.partial(uxf.on_event, verbose=False,
                                 filename=filename)
    uxo = uxf.load(filename, on_event=uxf.on_event)
    pp = _PrettyPrinter(wrap_width=16, realdp=3, on_event=on_event)
    uxo.visit(pp)
    pp.pprint(out=sys.stdout)


# TODO move to uxf.py if successful & change version to 2.4.0

class _PrettyPrinter(_EventMixin): # Functor that can be used as a visitor

    def __init__(self, *, wrap_width=96, realdp=None, indent='  ',
                 on_event=uxf.on_event):
        self.wrap_width = wrap_width
        self.realdp = realdp
        self.indent = indent
        self.on_event = on_event
        self.lino = 0 # for on_event
        self.tokens = []
        self.depth = -1
        self.table_row_counts = []


    @property
    def wrap_width(self):
        return self._wrap_width


    @wrap_width.setter
    def wrap_width(self, value):
        if value is not None and 40 <= value <= 999:
            self._wrap_width = value # only allow 40-999
        else:
            self._wrap_width = 96 # default


    @property
    def realdp(self):
        return self._realdp


    @realdp.setter
    def realdp(self, value):
        if value is None or 0 <= value <= 15:
            self._realdp = value # only allow None or 0-15
        else:
            self._realdp = None # default i.e., output 'natural' decimals


    def __call__(self, kind, value):
        if kind is uxf.VisitKind.UXF_BEGIN:
            self.handle_header(value)
        elif kind is uxf.VisitKind.UXF_END:
            self.eof()
        elif kind is uxf.VisitKind.LIST_BEGIN:
            self.handle_list_begin(value)
        elif kind is uxf.VisitKind.LIST_END:
            self.handle_list_end()
        elif kind is uxf.VisitKind.MAP_BEGIN:
            self.handle_map_begin(value)
        elif kind is uxf.VisitKind.MAP_END:
            self.handle_map_end()
        elif kind is uxf.VisitKind.ITEM_BEGIN:
            self.handle_item_begin()
        elif kind is uxf.VisitKind.ITEM_END:
            self.handle_item_end()
        elif kind is uxf.VisitKind.TABLE_BEGIN:
            self.handle_table_begin(value)
        elif kind is uxf.VisitKind.TABLE_END:
            self.handle_table_end()
        elif kind is uxf.VisitKind.RECORD_BEGIN:
            self.handle_record_begin()
        elif kind is uxf.VisitKind.RECORD_END:
            self.handle_record_end()
        elif kind is uxf.VisitKind.VALUE:
            self.handle_scalar(value)


    def begin(self):
        if self.tokens and self.tokens[-1].kind is TokenKind.END:
            self.rws()
        self.tokens.append(Token(TokenKind.BEGIN, depth=self.depth))


    def end(self, *, num_records=None):
        self.tokens.append(Token(TokenKind.END, depth=self.depth,
                                 num_records=num_records))


    def puts(self, s, num_records=None):
        self.tokens.append(Token(TokenKind.STRING, s, depth=self.depth,
                                 num_records=num_records))


    def rws(self): # Don't need duplicate RWS; don't need RWS if RNL present
        if not self.tokens or self.tokens[-1] not in {TokenKind.RWS,
                                                      TokenKind.RNL}:
            self.tokens.append(Token(TokenKind.RWS, depth=self.depth))


    def rnl(self): # Don't need RWS before newline; don't need dup RNL
        if self.tokens and self.tokens[-1].kind is TokenKind.RWS:
            self.tokens.pop()
        if not self.tokens or self.tokens[-1] is not TokenKind.RNL:
            self.tokens.append(Token(TokenKind.RNL, depth=self.depth))


    def eof(self):
        self.tokens.append(Token(TokenKind.EOF, depth=self.depth))


    def handle_header(self, value):
        header = 'uxf 1.0'
        if value.custom:
            header += f' {value.custom}'
        self.puts(f'{header}\n')
        if value.comment:
            self.handle_str(value.comment, prefix='#', suffix='\n')
        if value.imports:
            self.handle_imports(value.import_filenames)
        if value.tclasses:
            self.handle_tclasses(value.tclasses, value.imports)


    def handle_imports(self, imports):
        widest = 0
        for filename in imports:
            self.puts(f'!{filename}\n')
            if len(filename) > widest:
                widest = len(filename)
        widest += 1 # to allow for '!'
        if widest > self.wrap_width:
            self.wrap_width = widest
            self.warning(563, 'import forced wrap_width to be increased to '
                         f'{widest}')


    def handle_tclasses(self, tclasses, imports):
        widest = 0
        for ttype, tclass in sorted(tclasses.items(),
                                    key=lambda t: t[0].upper()):
            if imports and ttype in imports:
                continue # defined in an import
            self.puts('=')
            if tclass.comment:
                self.handle_comment(tclass.comment)
                self.rws()
            self.puts(tclass.ttype)
            if len(tclass.ttype) > widest:
                widest = len(tclass.ttype)
            for field in tclass.fields:
                self.rws()
                text = field.name
                if field.vtype is not None:
                    text += f':{field.vtype}'
                self.puts(text)
                if len(text) > widest:
                    widest = len(text)
            self.puts('\n')
        widest += 1 # to allow for '='
        if widest > self.wrap_width:
            self.wrap_width = widest
            self.warning(564, 'ttype forced wrap_width to be increased to '
                         f'{widest}')


    def handle_list_begin(self, value):
        self.depth += 1
        self.begin()
        self.puts('[')
        if value.comment:
            self.handle_comment(value.comment)
        if value.vtype:
            if value.comment:
                self.rws()
            self.puts(value.vtype)


    def handle_list_end(self):
        if self.tokens[-1].kind is TokenKind.RWS:
            self.tokens.pop() # Don't need RWS before closer
        self.puts(']')
        self.end()
        self.depth -= 1


    def handle_map_begin(self, value):
        self.depth += 1
        self.begin()
        self.puts('{')
        if value.comment:
            self.handle_comment(value.comment)
        if value.ktype:
            if value.comment:
                self.rws()
            self.puts(value.ktype)
            if value.vtype:
                self.puts(f' {value.vtype}')


    def handle_map_end(self):
        if self.tokens[-1].kind is TokenKind.RWS:
            self.tokens.pop() # Don't need RWS before closer
        self.puts('}')
        self.end()
        self.depth -= 1


    def handle_item_begin(self):
        self.depth += 1
        self.begin()


    def handle_item_end(self):
        self.end()
        self.depth -= 1


    def handle_table_begin(self, value):
        self.depth += 1
        self.table_row_counts.append(len(value))
        self.begin()
        self.puts('(')
        if value.comment:
            self.handle_comment(value.comment)
        self.puts(value.ttype, num_records=len(value))
        if len(value) == 1:
            self.rws()
        elif len(value) > 1:
            self.rnl()


    def handle_table_end(self):
        if self.tokens[-1].kind is TokenKind.RWS:
            self.tokens.pop() # Don't need RWS before closer
        self.puts(')')
        self.end()
        self.table_row_counts.pop()
        self.depth -= 1


    def handle_record_begin(self):
        self.depth += 1
        self.begin()


    def handle_record_end(self):
        self.rnl()
        self.end()
        self.depth -= 1


    def handle_real(self, value):
        if self.realdp is not None:
            value = round(value, self.realdp)
        text = str(value)
        if '.' not in text and 'e' not in text and 'E' not in text:
            text += '.0'
        self.puts(text)


    def handle_comment(self, value):
        self.handle_str(value, prefix='#')


    def handle_str(self, value, *, prefix='', suffix=''):
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
            self.rnl() # newline always follows multiline bytes or str
        else:
            self.puts(f'{prefix}<{text}>{suffix}')


    def ampersand(self):
        self.rws()
        self.puts('&')
        self.rws()


    def handle_bytes(self, value):
        text = value.hex().upper()
        if len(text) + 4 >= self.wrap_width:
            span = self.wrap_width - 2
            self.puts('(:')
            for i in range(0, len(text), span):
                self.puts(text[i:i + span])
            self.puts(':)')
            self.rnl() # newline always follows multiline bytes or str
        else:
            self.puts(f'(:{text}:)')


    def handle_scalar(self, value):
        if value is None:
            self.puts('?')
        elif isinstance(value, bool):
            self.puts('yes' if value else 'no')
        elif isinstance(value, int):
            self.puts(str(value))
        elif isinstance(value, float):
            self.handle_real(value)
        elif isinstance(value, (datetime.date, datetime.datetime)):
            self.puts(value.isoformat()[:19]) # 1-second resolution
        elif isinstance(value, str):
            self.handle_str(value)
        elif isinstance(value, (bytes, bytearray)):
            self.handle_bytes(value)
        else:
            self.warning(561, 'unexpected value of type '
                         f'{value.__class__.__name__}: {value!r}; consider '
                         'using a ttype')
        self.rws()


    def pprint(self, out=None):
        if not self.tokens:
            return
        out = out or io.StringIO()
        writer = _Writer(self.tokens, out, wrap_width=self.wrap_width,
                         realdp=self.realdp, indent=self.indent)
        writer.pprint()


class _Writer:

    def __init__(self, tokens, out, *, wrap_width, realdp, indent):
        self.tokens = tokens
        self.out = out
        self.width = wrap_width
        self.realdp = realdp
        self.indent = indent
        self.wrote = ''
        self.pos = 0
        self.tp = 0


    def pprint(self):
        if not self.tokens:
            return

        # TODO delete
        if 1:
            sys.stderr.write(' TOKENS '.center(40, '-'))
            sys.stderr.write('\n')
            for i, token in enumerate(self.tokens):
                sys.stderr.write(f'{token}\n')
            sys.stderr.write(' === '.center(40, '-'))
            sys.stderr.write('\n')

        self.pos = 0
        self.tp = 0
        while self.tp < len(self.tokens):
            token = self.tokens[self.tp]
            self.tp += 1
            if token.kind is TokenKind.BEGIN:
                self.begin(token)
            elif token.kind is TokenKind.END:
                pass
            elif token.kind is TokenKind.STRING:
                self.string(token)
            elif token.kind is TokenKind.RWS:
                self.rws()
            elif token.kind is TokenKind.RNL:
                self.rnl()
            elif token.kind is TokenKind.EOF:
                break
        self.write('\n')


    def begin(self, token):
        tab = self.indent * token.depth
        if self.pos: # in a line
            peek = self.prev()
            if peek is not None and peek.kind is TokenKind.END:
                self.pos += self.write(' ')
            needed = self.pos
        else:
            needed = len(tab)
        i = self.tp + 1
        while needed < self.width and i < len(self.tokens):
            tok = self.tokens[i]
            i += 1
            if tok.kind is TokenKind.END:
                if tok.depth == token.depth: # matching end
                    break
            elif tok.kind in {TokenKind.RNL, TokenKind.EOF}:
                break # forced onto newline anyway or EOF
            elif tok.kind is TokenKind.RWS:
                needed += 1
            elif tok.kind is TokenKind.STRING:
                needed += len(tok.text)
        if self.tp < i and self.pos == 0:
            self.pos = self.write(tab)
        while self.tp < i: # room for more
            tok = self.tokens[self.tp]
            self.tp += 1
            if tok.kind is TokenKind.STRING:
                self.pos += self.write(tok.text)
            elif tok.kind is TokenKind.RWS:
                self.pos += self.write(' ')


    def string(self, token):
        tab = self.indent * token.depth
        if token.is_multiline:
            if self.pos: # in a line:
                first, rest = token.text.split('\n', 1)
                if self.pos + len(first) < self.width:
                    self.write(first)
                    self.write('\n')
                    self.write(rest)
                    i = rest.rfind('\n')
                    text = rest
            else: # newline
                text = token.text
                self.write(text)
                i = text.rfind('\n')
            if i > -1:
                self.pos = len(text[i:])
            else:
                self.pos = len(text)
        else:
            if self.pos: # in a line
                if self.pos + len(token.text) < self.width: # fits on line
                    self.pos += self.write(token.text)
                    return
                else:
                    self.write('\n')
                    self.pos = 0
            if len(tab) + len(token.text) < self.width:
                self.pos = self.write(tab) # fits after indent
            self.pos += self.write(token.text)


    def rws(self):
        if self.pos != 0: # safe to ignore RWS at start of line
            if self.pos + self.peek_len(self.tp + 1) < self.width:
                self.pos += self.write(' ')
            else:
                self.write('\n')
                self.pos = 0


    def rnl(self):
        self.write('\n')
        self.pos = 0


    def prev(self):
        return self.peek(self.tp - 1)


    def next(self):
        return self.peek(self.tp + 1)

    def peek(self, i):
        return self.tokens[i] if 0 <= i < len(self.tokens) else None


    def peek_len(self, i):
        return len(self.tokens[i].text) if 0 <= i < len(self.tokens) else 0


    def write(self, text):
        if text == ' ' and self.wrote.endswith((' ', '\n')):
            return 0
        self.out.write(text)
        self.wrote = text
        return len(text)


@enum.unique
class TokenKind(enum.Enum):
    BEGIN = '▶'
    END = '◀'
    STRING = ' '
    RWS = '␣ ' # required whitespace: output either ' ' or '\n'
    RNL = '⏎ ' # required newline: output '\n'
    EOF = '■'


class Token:

    def __init__(self, kind, text='', *, depth=0, num_records=0):
        self.kind = kind
        self.text = text
        self.depth = depth
        self.num_records = num_records


    @property
    def is_multiline(self):
        return '\n' in self.text


    def __repr__(self):
        text = self.depth * '   '
        if self.num_records:
            text += f'{self.num_records} × '
        text += self.kind.name # .text
        if self.text:
            text += f' {self.text!r}'
        return text


if __name__ == '__main__':
    main()
