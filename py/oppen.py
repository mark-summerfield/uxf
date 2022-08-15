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

    def __init__(self, *, wrap_width=96, realdp=None, indent='   ',
                 on_event=uxf.on_event):
        self.wrap_width = wrap_width
        self.realdp = realdp
        self.indent = indent
        self.on_event = on_event
        self.lino = 0 # for on_event
        self.tokens = []
        self.depth = 0
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
            self.begin()
        elif kind is uxf.VisitKind.ITEM_END:
            self.end()
        elif kind is uxf.VisitKind.TABLE_BEGIN:
            self.handle_table_begin(value)
        elif kind is uxf.VisitKind.TABLE_END:
            self.handle_table_end()
        elif kind is uxf.VisitKind.RECORD_BEGIN:
            self.begin()
        elif kind is uxf.VisitKind.RECORD_END:
            self.end()
            self.rnl()
        elif kind is uxf.VisitKind.VALUE:
            self.handle_scalar(value)


    def begin(self):
        if self.tokens and self.tokens[-1].kind is TokenKind.END:
            self.rws()
        self.tokens.append(Token(TokenKind.BEGIN, depth=self.depth))


    def end(self, *, num_records=None): # Don't need RWS before END
        if self.tokens and self.tokens[-1].kind is TokenKind.RWS:
            self.tokens.pop()
        self.tokens.append(Token(TokenKind.END, depth=self.depth,
                                 num_records=num_records))


    def puts(self, s, num_records=None):
        self.tokens.append(Token(TokenKind.STRING, s, depth=self.depth,
                                 num_records=num_records))


    def rws(self): # Don't need duplicate RWS; don't need RWS if RNL present
        if self.tokens:
            pos = -1
            if (self.tokens[pos].kind is TokenKind.END and
                    len(self.tokens) > 1):
                pos -= 1
            if self.tokens[pos].kind in {TokenKind.RWS, TokenKind.RNL}:
                return
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
        self.begin()
        self.puts('[')
        if value.comment:
            self.handle_comment(value.comment)
        if value.vtype:
            if value.comment:
                self.rws()
            self.puts(value.vtype)
        self.depth += 1


    def handle_list_end(self):
        if self.tokens[-1].kind is TokenKind.RWS:
            self.tokens.pop() # Don't need RWS before closer
        self.depth -= 1
        self.puts(']')
        self.end()


    def handle_map_begin(self, value):
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
        self.depth += 1


    def handle_map_end(self):
        if self.tokens[-1].kind is TokenKind.RWS:
            self.tokens.pop() # Don't need RWS before closer
        self.depth -= 1
        self.puts('}')
        self.end()


    def handle_table_begin(self, value):
        self.table_row_counts.append(len(value))
        self.begin()
        self.puts('(')
        if value.comment:
            self.handle_comment(value.comment)
        self.puts(value.ttype, num_records=len(value))
        if len(value) == 1:
            self.rws()
        elif len(value) > 1:
            self.depth += 1
            self.rnl()


    def handle_table_end(self):
        if self.tokens[-1].kind is TokenKind.RWS:
            self.tokens.pop() # Don't need RWS before closer
        self.depth -= 1
        self.puts(')')
        self.end()
        if self.table_row_counts[-1] > 1:
            self.rnl()
        self.table_row_counts.pop()


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
        self.pos = 0
        self.tp = 0
        self.end_nl = False


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
        # end delete

        self.pos = 0
        self.tp = 0
        while self.tp < len(self.tokens):
            token = self.tokens[self.tp]
            self.tp += 1
            if token.kind is TokenKind.BEGIN:
                self.begin(token)
            elif token.kind is TokenKind.STRING:
                self.string(token)
            elif token.kind is TokenKind.RWS:
                self.rws()
            elif token.kind is TokenKind.RNL:
                self.rnl()
            elif token.kind is TokenKind.END:
                pass
            elif token.kind is TokenKind.EOF:
                break
        if not self.end_nl:
            self.write('\n')


    def begin(self, token):
        tab = self.indent * token.depth
        needed = self.pos if self.pos else len(tab)
        i = self.find_matching_end(needed, self.tp, token.depth)
        if i > -1: # found & fits on line
            if self.pos == 0:
                self.write(tab)
            self.write_tokens_to(i)
        elif self.pos: # try to fit begin…end on its own wrapped line
            i = self.find_matching_end(len(tab), self.tp, token.depth)
            if i > -1: # found & will fit on next line even with indent
                self.rnl()
                self.write(tab)
                self.write_tokens_to(i)
            # else: # skip this begin and continue from the next token


    def find_matching_end(self, needed, i, depth):
        while needed < self.width and i < len(self.tokens):
            token = self.tokens[i]
            i += 1
            if token.kind is TokenKind.END:
                if token.depth == depth: # matching end
                    return i
            elif token.kind in {TokenKind.RNL, TokenKind.EOF}:
                return i # de-facto; forced onto newline anyway or EOF
            elif token.kind is TokenKind.RWS:
                needed += 1
            elif token.kind is TokenKind.STRING:
                needed += len(token.text)
                if token.is_multiline:
                    return i # de-facto; forced onto newline anyway
        return -1


    def write_tokens_to(self, i):
        while self.tp < i: # room for more
            token = self.tokens[self.tp]
            self.tp += 1
            if token.kind is TokenKind.STRING:
                self.write(token.text)
                if token.is_multiline:
                    break
            elif token.kind is TokenKind.RWS:
                self.rws()
            elif token.kind is TokenKind.RNL:
                self.rnl()
            elif token.kind in {TokenKind.BEGIN, TokenKind.END}:
                pass
            elif token.kind is TokenKind.EOF:
                break


    def string(self, token):
        tab = self.indent * token.depth
        if token.is_multiline:
            self.multiline(token)
        else:
            if self.pos: # in a line
                if self.pos + len(token.text) < self.width: # fits on line
                    self.write(token.text)
                    return
                else:
                    self.write('\n')
            if len(tab) + len(token.text) < self.width:
                self.write(tab) # fits after indent
            self.write(token.text)


    def multiline(self, token): # write direct & handle pos
        if self.pos: # in a line:
            first, rest = token.text.split('\n', 1)
            if self.pos + len(first) < self.width:
                self.out.write(first)
                self.out.write('\n')
                self.out.write(rest)
                self.set_pos(rest)
            else:
                self.out.write('\n')
                self.out.write(token.text)
                self.set_pos(token.text)
        else: # newline
            self.out.write(token.text)
            self.set_pos(token.text)


    def rws(self):
        if self.pos > 0: # safe to ignore RWS at start of line
            if self.pos + self.peek_len(self.tp + 1) < self.width:
                self.write(' ')
            else:
                self.write('\n')


    def rnl(self):
        self.write('\n')


    def prev(self):
        return self.peek(self.tp - 1)


    def next(self):
        return self.peek(self.tp + 1)


    def peek(self, i):
        return self.tokens[i] if 0 <= i < len(self.tokens) else None


    def peek_len(self, i):
        return len(self.tokens[i].text) if 0 <= i < len(self.tokens) else 0


    def set_pos(self, text):
        if text.endswith('\n'):
            self.pos = 0
            self.end_nl = True
        else:
            self.end_nl = False
            i = text.rfind('\n')
            if i > -1:
                text = text[i + i:]
            self.pos += len(text)


    def write(self, text):
        self.out.write(text)
        self.set_pos(text)


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
