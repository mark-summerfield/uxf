#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

'''The API documentation is in the accompanying README.md file.'''

import collections
import datetime
import enum
import functools
import gzip
import io
import math
import os
import pathlib
import sys
import urllib.request
from xml.sax.saxutils import escape, unescape

import editabletuple

__version__ = '2.1.1' # uxf module version
VERSION = 1.0 # UXF file format version

UTF8 = 'utf-8'
_MAX_IDENTIFIER_LEN = 60
_SEP = ' '
_KTYPES = frozenset({'int', 'date', 'datetime', 'str', 'bytes'})
_VTYPES = frozenset(_KTYPES | {'bool', 'real'})
_ANY_VTYPES = frozenset(_VTYPES | {'list', 'map', 'table'})
_BOOL_FALSE = frozenset({'no'})
_BOOL_TRUE = frozenset({'yes'})
_BOOLS = frozenset(_BOOL_FALSE | _BOOL_TRUE)
_BAREWORDS = frozenset(_ANY_VTYPES | _BOOLS)
RESERVED_WORDS = frozenset(_ANY_VTYPES | {'null'} | _BOOLS)
_MISSING = object()


@enum.unique
class Event(enum.Enum):
    '''
    - REPAIR a fix was applied, e.g., an int was converted to a float, a
      string containing a date was converted to a date, etc.;
    - WARNING problem can't be automatically fixed but can be coped with;
    - ERROR problem can't be automatically fixed and means UXF may be
      invalid;
    - FATAL unrecoverable problem.
    '''
    REPAIR = enum.auto()
    WARNING = enum.auto()
    ERROR = enum.auto()
    FATAL = enum.auto()


@enum.unique
class Compare(enum.Flag):
    '''
    - EXACT use == or != instead;
    - IGNORE_COMMENTS ignore differences in comments
    - IGNORE_UNUSED_TTYPES ignore unused ttypes
    - IGNORE_IMPORTS ignore whether ttypes are imported or defined
    - EQUIVALENT this is the _or_ of the three ignores above
    - IGNORE_KVTYPES ignore ktypes, vtypes, and field vtypes (rarely
      appropriate)
    - UNTYPED_EQUIVALENT this is the _or_ of all the ignores above (rarely
      appropriate)
    '''
    EXACT = 0
    IGNORE_COMMENTS = enum.auto()
    IGNORE_UNUSED_TTYPES = enum.auto()
    IGNORE_IMPORTS = enum.auto()
    EQUIVALENT = (IGNORE_COMMENTS | IGNORE_UNUSED_TTYPES | IGNORE_IMPORTS)
    IGNORE_KVTYPES = enum.auto()
    UNTYPED_EQUIVALENT = EQUIVALENT | IGNORE_KVTYPES


def on_event(event: Event, code: int, message: str, *, filename='-',
             lino=0, verbose=True, prefix='uxf'):
    '''The default event handler.'''
    error = Error(event, code, message, filename=filename, lino=lino,
                  verbose=verbose, prefix=prefix)
    if event is Event.FATAL:
        raise error
    if verbose:
        print(str(error), file=sys.stderr)


def _raise_error(code, message):
    raise Error(Event.FATAL, code, message)


def _validate_format(name, value): # If invalid we return the valid default
    if name == 'indent':
        return value if (value == '' or (
                         value.isspace() and len(value) < 33)) else '  '
    if name == 'wrap_width':
        return value if (value is None or value == 0 or
                         40 <= value <= 240) else 96
    if name == 'realdp':
        return value if (value is None or 0 <= value <= 15) else None
    if name == 'max_short_len':
        return value if 24 <= value <= 60 else 32


Format = editabletuple.editableobject(
    'Format', 'indent', 'wrap_width', 'realdp', 'max_short_len',
    defaults=('  ', 96, None, 32), validator=_validate_format,
    doc='''Specifies various aspects of how a UXF file is dumped to file or
to a string.
`indent` defaults to 2 spaces and may be an empty string or up to 32 spaces
`wrap_width` defaults to 96 characters and may be None (use the default) \
or 40<=240
`realdp` defaults to None which means use however many digits after the
decimal place are needed to represent the given `real` (i.e., Python
`float`); if not None specify an int 0<=15
`max_short_len` defaults to 32 and must be 24<=60''')


class Uxf:

    def __init__(self, value=None, *, custom='', tclasses=None,
                 comment=None, on_event=on_event):
        '''value may be a list, List, tuple, dict, Map, or Table and will
        default to a List if not specified; if given tclasses must be a dict
        whose values are TClasses and whose corresponding keys are the
        TClasses' ttypes (i.e., their names); if given the comment is a
        file-level comment that follows the uxf header and precedes any
        TClasses and the value.
        on_event is used instead of raise to give users more control'''
        self.on_event = on_event
        self.value = value
        self.custom = custom
        self.comment = comment
        self._tclasses = {} # tclasses key=ttype value=TClass
        self.tclasses = tclasses if tclasses is not None else {}
        self.imports = {} # key=ttype value=import text


    def add_tclasses(self, tclass, *tclasses):
        for tclass in (tclass,) + tclasses:
            if not tclass.ttype:
                self.on_event(Event.FATAL, 200,
                              'cannot add an unnamed TClass')
                return # in case user on_event doesn't raise
            _add_to_tclasses(self.tclasses, tclass, lino=0, code=690,
                             on_event=self.on_event)


    @property
    def tclasses(self):
        return self._tclasses


    @tclasses.setter
    def tclasses(self, tclasses):
        self.tclasses.clear()
        for ttype, tclass in tclasses.items():
            if not ttype:
                self.on_event(Event.FATAL, 694,
                              'cannot set an unnamed TClass')
                return # in case user on_event doesn't raise
            self._tclasses[ttype] = tclass


    @property
    def import_filenames(self):
        '''A utility useful for some UXF processors. This yields all the
        unique import filenames.'''
        seen = set()
        for filename in self.imports.values(): # don't sort!
            if filename not in seen:
                yield filename
                seen.add(filename)


    @property
    def value(self):
        '''The List, Map, or Table that holds all the Uxf's data.'''
        return self._value


    @value.setter
    def value(self, value):
        '''Replaces all the Uxf's data with the given collection.'''
        value = List() if value is None else _maybe_to_uxf_collection(value)
        if not _is_uxf_collection(value):
            self.on_event(
                Event.FATAL, 100, 'Uxf value must be a list, List, dict, '
                f'Map, or Table, got {value.__class__.__name__}')
            return # in case user on_event doesn't raise
        self._value = value


    def dump(self, filename_or_filelike, *, on_event=on_event,
             format=Format()):
        '''Convenience method that wraps the module-level dump() function'''
        dump(filename_or_filelike, self, on_event=on_event, format=format)


    def dumps(self, *, on_event=on_event, format=Format()):
        '''Convenience method that wraps the module-level dumps()
        function'''
        return dumps(self, on_event=on_event, format=format)


    def load(self, filename_or_filelike, *, on_event=on_event,
             drop_unused=False, replace_imports=False):
        '''Convenience method that wraps the module-level load()
        function'''
        filename = (filename_or_filelike if isinstance(filename_or_filelike,
                    (str, pathlib.Path)) else '-')
        value, custom, tclasses, imports, comment = _loads(
            _read_text(filename_or_filelike, on_event), filename,
            on_event=on_event, drop_unused=drop_unused,
            replace_imports=replace_imports)
        self.value = value
        self.custom = custom
        self.tclasses = tclasses
        self.comment = comment


    def loads(self, uxt, filename='-', *, on_event=on_event,
              drop_unused=False, replace_imports=False):
        '''Convenience method that wraps the module-level loads()
        function'''
        value, custom, tclasses, imports, comment = _loads(
            uxt, filename, on_event=on_event, drop_unused=drop_unused,
            replace_imports=replace_imports)
        self.value = value
        self.custom = custom
        self._tclasses = tclasses
        self.imports = imports
        self.comment = comment


    def is_equivalent(self, other, compare=Compare.EXACT):
        '''Returns True if this Uxf is equivalent to the other Uxf;
        otherwise returns False.
        Use == or != for exact comparisons.
        Use this to ignore differences in comments Compare.IGNORE_COMMENTS,
        or to ignore differences in imports (e.g., if one has a ttype
        defined and the other the same ttype imported)
        Compare.IGNORE_IMPORTS, or to ignore differences in unused ttypes,
        (e.g., if one defines ir imports a ttype that isn't used and the
        other doesn't) Compare.IGNORE_UNUSED_TTYPES. To ignore any two of
        these differences combine with | and to ignore all of them use
        Compare.EQUIVALENT.
        '''
        if not isinstance(other, self.__class__):
            return False
        if self.custom != other.custom:
            return False
        if (Compare.IGNORE_COMMENTS not in compare and
                self.comment != other.comment):
            return False
        if (Compare.IGNORE_IMPORTS not in compare and
                self.imports != other.imports):
            return False
        if (Compare.IGNORE_UNUSED_TTYPES not in compare and
                self.tclasses != other.tclasses):
            # This means that we only compare actually used ttypes when
            # comparing any tables.
            return False
        return self.value.is_equivalent(other.value, compare)


    def __eq__(self, other):
        '''Returns True if this Uxf is the same as the other Uxf; otherwise
        returns False.
        See also is_equivalent()'''
        if not isinstance(other, self.__class__):
            return False
        if self.custom != other.custom:
            return False
        if self.comment != other.comment:
            return False
        if self.imports != other.imports:
            return False
        if self.tclasses != other.tclasses:
            return False
        return self.value == other.value # These are Lists, Maps, or Tables


def load(filename_or_filelike, *, on_event=on_event, drop_unused=False,
         replace_imports=False, _imported=None, _is_import=False):
    '''
    Returns a Uxf object.

    filename_or_filelike is sys.stdin or a filename or an open readable file
    (text mode UTF-8 encoded, optionally gzipped).
    '''
    filename = (filename_or_filelike if isinstance(filename_or_filelike,
                (str, pathlib.Path)) else '-')
    value, custom, tclasses, imports, comment = _loads(
        _read_text(filename_or_filelike, on_event), filename,
        on_event=on_event, drop_unused=drop_unused,
        replace_imports=replace_imports, _imported=_imported,
        _is_import=_is_import)
    uxo = Uxf(value, custom=custom, tclasses=tclasses, comment=comment,
              on_event=on_event)
    uxo.imports = imports
    return uxo


def loads(uxt, filename='-', *, on_event=on_event, drop_unused=False,
          replace_imports=False, _imported=None, _is_import=False):
    '''
    Returns a Uxf object.

    uxt must be a string of UXF data.
    '''
    value, custom, tclasses, imports, comment = _loads(
        uxt, filename, on_event=on_event, drop_unused=drop_unused,
        replace_imports=replace_imports, _imported=_imported,
        _is_import=_is_import)
    uxo = Uxf(value, custom=custom, tclasses=tclasses, comment=comment,
              on_event=on_event)
    uxo.imports = imports
    return uxo


def _loads(uxt, filename='-', *, on_event=on_event, drop_unused=False,
           replace_imports=False, _imported=None, _is_import=False):
    tokens, custom, text = _tokenize(uxt, filename, on_event=on_event)
    value, comment, tclasses, imports = _parse(
        tokens, filename, on_event=on_event, drop_unused=drop_unused,
        replace_imports=replace_imports, _imported=_imported,
        _is_import=_is_import)
    return value, custom, tclasses, imports, comment


def _tokenize(uxt, filename='-', *, on_event=on_event):
    lexer = _Lexer(filename, on_event=on_event)
    tokens = lexer.tokenize(uxt)
    return tokens, lexer.custom, uxt


def _read_text(filename_or_filelike, on_event):
    if not isinstance(filename_or_filelike, (str, pathlib.Path)):
        return filename_or_filelike.read()
    try:
        try:
            with gzip.open(filename_or_filelike, 'rt',
                           encoding=UTF8) as file:
                return file.read()
        except gzip.BadGzipFile:
            with open(filename_or_filelike, 'rt', encoding=UTF8) as file:
                return file.read()
    except OSError as err:
        on_event(Event.FATAL, 102, f'failed to read UXF text: {err}')


class _EventMixin:

    def warning(self, code, message):
        self.on_event(Event.WARNING, code, message, lino=self.lino)


    def repair(self, code, message):
        self.on_event(Event.REPAIR, code, message, lino=self.lino)


    def error(self, code, message):
        self.on_event(Event.ERROR, code, message, lino=self.lino)


    def fatal(self, code, message):
        self.on_event(Event.FATAL, code, message, lino=self.lino)


class _Lexer(_EventMixin):

    def __init__(self, filename, *, on_event=on_event):
        self.filename = filename
        self.on_event = functools.partial(
            on_event, filename=os.path.basename(filename))
        self.clear()


    def clear(self):
        self.pos = 0 # current
        self.lino = 0
        self.custom = None
        self.in_tclass = False
        self.tokens = []


    def tokenize(self, uxt):
        self.text = uxt
        self.scan_header()
        self.maybe_read_file_comment()
        while not self.at_end():
            self.scan_next()
        self.add_token(_Kind.EOF)
        return self.tokens


    def scan_header(self):
        i = self.text.find('\n')
        if i == -1:
            self.fatal(110, 'missing UXF file header or empty file')
            return # in case user on_event doesn't raise
        self.pos = i
        parts = self.text[:i].split(None, 2)
        if len(parts) < 2:
            self.fatal(120, 'invalid UXF file header')
            return # in case user on_event doesn't raise
        if parts[0] != 'uxf':
            self.fatal(130, 'not a UXF file')
            return # in case user on_event doesn't raise
        try:
            version = float(parts[1])
            if version > VERSION:
                self.warning(141,
                             f'version ({version}) > current ({VERSION})')
        except ValueError:
            self.error(151, 'failed to read UXF file version number')
        if len(parts) > 2:
            self.custom = parts[2]


    def maybe_read_file_comment(self):
        self.skip_ws()
        if not self.at_end() and self.text[self.pos] == '#':
            self.pos += 1 # skip the #
            if self.peek() == '<':
                self.pos += 1 # skip the leading <
                value = self.match_to('>', what='comment string')
                self.add_token(_Kind.FILE_COMMENT, unescape(value))
            else:
                self.error(160, 'invalid comment syntax: expected \'<\', '
                           f'got {self.peek()}')


    def at_end(self):
        return self.pos >= len(self.text)


    def scan_next(self):
        c = self.getch()
        if c.isspace(): # ignore insignificant whitespace
            if c == '\n':
                self.lino += 1
        elif c == '(':
            if self.peek() == ':':
                self.pos += 1
                self.read_bytes()
            else:
                self.check_in_tclass()
                self.add_token(_Kind.TABLE_BEGIN)
        elif c == ')':
            self.add_token(_Kind.TABLE_END)
        elif c == '[':
            self.check_in_tclass()
            self.add_token(_Kind.LIST_BEGIN)
        elif c == '=':
            self.check_in_tclass() # allow for fieldless TClasses
            self.add_token(_Kind.TCLASS_BEGIN)
            self.in_tclass = True
        elif c == ']':
            self.add_token(_Kind.LIST_END)
        elif c == '{':
            self.check_in_tclass()
            self.add_token(_Kind.MAP_BEGIN)
        elif c == '}':
            self.in_tclass = False
            self.add_token(_Kind.MAP_END)
        elif c == '?':
            self.add_token(_Kind.NULL)
        elif c == '!':
            self.read_imports()
        elif c == '#':
            self.read_comment()
        elif c == '<':
            self.read_string()
        elif c == ':':
            self.read_field_vtype()
        elif c == '-' and isasciidigit(self.peek()):
            c = self.getch() # skip the - and get the first digit
            self.read_negative_number(c)
        elif isasciidigit(c):
            self.read_positive_number_or_date(c)
        elif c.isalpha():
            self.read_name()
        else:
            self.error(170, f'invalid character encountered: {c!r}')


    def check_in_tclass(self):
        if self.in_tclass:
            self.in_tclass = False
            self.add_token(_Kind.TCLASS_END)


    def read_imports(self):
        this_file = _full_filename(self.filename)
        path = os.path.dirname(this_file)
        while True:
            value = self.match_to('\n', what='import')
            value = value.strip()
            if this_file == _full_filename(value, path):
                self.fatal(176, 'a UXF file cannot import itself')
                return # in case user on_event doesn't raise
            else:
                self.add_token(_Kind.IMPORT, value)
            if self.peek() == '!':
                self.getch() # skip !
            else:
                break # imports finished


    def read_comment(self):
        if self.tokens and self.tokens[-1].kind in {
                _Kind.LIST_BEGIN, _Kind.MAP_BEGIN,
                _Kind.TABLE_BEGIN, _Kind.TCLASS_BEGIN}:
            if self.peek() != '<':
                self.error(180, 'a str must follow the # comment '
                           f'introducer, got {self.peek()!r}')
            self.pos += 1 # skip the leading <
            value = self.match_to('>', what='comment string')
            if value:
                self.tokens[-1].comment = unescape(value)
        else:
            self.error(190, 'comments may only occur at the start of '
                       'Lists, Maps, Tables, and TClasses')


    def read_string(self):
        value = self.match_to('>', what='string')
        self.add_token(_Kind.STR, unescape(value))


    def read_bytes(self):
        value = self.match_to(':)', what='bytes')
        try:
            self.add_token(_Kind.BYTES, bytes.fromhex(value))
        except ValueError as err:
            self.error(200, f'expected bytes, got {value!r}: {err}')


    def read_negative_number(self, c):
        is_real = False
        start = self.pos - 1
        while not self.at_end() and (c in '.eE' or isasciidigit(c)):
            if c in '.eE':
                is_real = True
            c = self.text[self.pos]
            self.pos += 1
        convert = float if is_real else int
        self.pos -= 1 # wind back to terminating non-numeric char
        text = self.text[start:self.pos]
        try:
            value = convert(text)
            self.add_token(_Kind.REAL if is_real else _Kind.INT,
                           -value)
        except ValueError as err:
            self.error(210, f'invalid number: {text!r}: {err}')


    def read_positive_number_or_date(self, c):
        start = self.pos - 1
        is_real, is_datetime, hyphens = self.read_number_or_date_chars(c)
        text = self.text[start:self.pos]
        convert, token = self.get_converter_and_token(is_real, is_datetime,
                                                      hyphens, text)
        try:
            value = convert(text)
            self.add_token(token, value)
        except ValueError as err:
            if is_datetime and len(text) > 19:
                self.reread_datetime(text, convert)
            else:
                self.error(220,
                           f'invalid number or date/time: {text!r}: {err}')


    def read_number_or_date_chars(self, c):
        is_real = is_datetime = False
        hyphens = 0
        while not self.at_end() and (c in '-+.:eET' or isasciidigit(c)):
            if c in '.eE':
                is_real = True
            elif c == '-':
                hyphens += 1
            elif c in ':T':
                is_datetime = True
            c = self.text[self.pos]
            self.pos += 1
        self.pos -= 1 # wind back to terminating non-numeric non-date char
        return is_real, is_datetime, hyphens


    def get_converter_and_token(self, is_real, is_datetime, hyphens, text):
        if is_datetime:
            return datetime.datetime.fromisoformat, _Kind.DATE_TIME
        if hyphens == 2:
            return datetime.date.fromisoformat, _Kind.DATE
        if is_real:
            return float, _Kind.REAL
        return int, _Kind.INT


    def reread_datetime(self, text, convert):
        try:
            value = convert(text[:19])
            self.add_token(_Kind.DATE_TIME, value)
            self.error(231, 'skipped timezone data, '
                       f'used {text[:19]!r}, got {text!r}')
        except ValueError as err:
            self.error(240, f'invalid datetime: {text!r}: {err}')


    def read_name(self):
        match = self.match_any_of(_BAREWORDS)
        if match in _BOOL_FALSE:
            self.add_token(_Kind.BOOL, False)
            return
        if match in _BOOL_TRUE:
            self.add_token(_Kind.BOOL, True)
            return
        if match in _ANY_VTYPES:
            self.add_token(_Kind.TYPE, match)
            return
        start = self.pos - 1
        if self.text[start] == '_' or self.text[start].isalpha():
            self.read_ttype_or_identifier(start)
        else:
            i = self.text.find('\n', self.pos)
            text = self.text[self.pos - 1:i if i > -1 else self.pos + 8]
            self.error(250, f'expected const or identifier, got {text!r}')


    def read_ttype_or_identifier(self, start):
        identifier = self.match_identifier(start, 'identifier')
        if self.in_tclass:
            top = self.tokens[-1] # if in_tclass there must be a prev token
            if top.kind is _Kind.TCLASS_BEGIN and top.value is None:
                top.value = identifier
            else:
                self.add_token(_Kind.FIELD, identifier)
        else:
            self.add_token(_Kind.IDENTIFIER, identifier)


    def read_field_vtype(self):
        self.skip_ws()
        identifier = self.match_identifier(self.pos, 'field vtype')
        if (self.in_tclass and self.tokens and
                self.tokens[-1].kind is _Kind.FIELD):
            self.tokens[-1].vtype = identifier
        else:
            self.error(248, f'expected field vtype, got {identifier!r}')


    def peek(self):
        return '\0' if self.at_end() else self.text[self.pos]


    def skip_ws(self):
        while self.pos < len(self.text) and self.text[self.pos].isspace():
            if self.text[self.pos] == '\n':
                self.lino += 1
            self.pos += 1


    def getch(self): # advance
        c = self.text[self.pos]
        self.pos += 1
        return c


    def match_identifier(self, start, what):
        while self.pos < len(self.text):
            if (not self.text[self.pos].isalnum() and
                    self.text[self.pos] != '_'):
                break
            self.pos += 1
        identifier = self.text[start:self.pos][:_MAX_IDENTIFIER_LEN]
        if identifier:
            return identifier
        text = self.text[start:start + 10]
        self.error(260, f'expected {what}, got {text}…')


    def match_to(self, target, *, what):
        if not self.at_end():
            i = self.text.find(target, self.pos)
            if i > -1:
                text = self.text[self.pos:i]
                self.lino += text.count('\n')
                self.pos = i + len(target) # skip past target
                return text
        self.error(270, f'unterminated {what}')


    def match_any_of(self, targets):
        if self.at_end():
            return None
        start = self.pos - 1
        for target in sorted(targets, key=lambda t: (len(t), t),
                             reverse=True):
            if self.text.startswith(target, start):
                self.pos += len(target) - 1 # skip past target
                return target


    def add_token(self, kind, value=None):
        if not self.in_tclass and self.tokens:
            if self.subsumed(kind, value):
                return
        self.tokens.append(_Token(kind, value, lino=self.lino))


    def subsumed(self, kind, value):
        top = self.tokens[-1]
        if kind in {_Kind.IDENTIFIER, _Kind.TYPE}:
            if top.kind is _Kind.LIST_BEGIN:
                return self.subsume_list_vtype(top, value)
            elif top.kind is _Kind.MAP_BEGIN:
                return self.subsume_map_type(kind, top, value)
            elif top.kind is _Kind.TABLE_BEGIN and kind is _Kind.IDENTIFIER:
                return self.subsume_table_ttype(kind, top, value)
        return False


    def subsume_list_vtype(self, top, value):
        if top.vtype is None:
            _check_type_name(value, self.error)
            top.vtype = value
        else:
            self.error(272, f'expected value got type, {value}')
        return True


    def subsume_map_type(self, kind, top, value):
        if top.ktype is None:
            if kind is _Kind.IDENTIFIER:
                self.error(273, f'expected ktype got, {value}')
            _check_ktype(value, self.error)
            top.ktype = value
        elif top.vtype is None:
            _check_type_name(value, self.error)
            top.vtype = value
        else:
            self.error(276, f'expected first map key got type, {value}')
        return True


    def subsume_table_ttype(self, kind, top, value):
        if top.ttype is None:
            top.ttype = value
        else:
            self.error(274, f'expected value got type, {value}')
        return True


class Error(Exception):

    def __init__(self, event: Event, code: int, message: str, *,
                 filename='-', lino=0, verbose=True, prefix='uxf'):
        self.event = event
        self.code = code
        self.message = message
        self.filename = filename
        self.lino = lino
        self.verbose = verbose
        self.prefix = prefix


    def __str__(self):
        return (f'{self.prefix}:{self.event.name[0]}{self.code}:'
                f'{self.filename}:{self.lino}:{self.message}')


class _Token:

    def __init__(self, kind, value=None, *, lino=0, comment=None,
                 ttype=None, ktype=None, vtype=None):
        self.kind = kind
        self.value = value # literal, i.e., correctly typed item
        self.lino = lino
        self.comment = comment
        self.ttype = ttype
        self.ktype = ktype
        self.vtype = vtype


    def __str__(self):
        parts = [self.kind.name]
        if self.value is not None:
            parts.append(f'={self.value!r}')
        return f'{self.lino}:{"".join(parts)}'


    def __repr__(self):
        parts = [f'{self.__class__.__name__}({self.kind.name}']
        if self.value is not None:
            parts.append(f', value={self.value!r}')
        if self.ttype is not None:
            parts.append(f', ttype={self.ttype!r}')
        if self.ktype is not None:
            parts.append(f', ktype={self.ktype!r}')
        if self.vtype is not None:
            parts.append(f', vtype={self.vtype!r}')
        parts.append(')')
        return ''.join(parts)


@enum.unique
class _Kind(enum.Enum):
    IMPORT = enum.auto()
    TCLASS_BEGIN = enum.auto()
    FIELD = enum.auto()
    TCLASS_END = enum.auto()
    TABLE_BEGIN = enum.auto()
    TABLE_END = enum.auto()
    LIST_BEGIN = enum.auto()
    LIST_END = enum.auto()
    MAP_BEGIN = enum.auto()
    MAP_END = enum.auto()
    FILE_COMMENT = enum.auto()
    NULL = enum.auto()
    BOOL = enum.auto()
    INT = enum.auto()
    REAL = enum.auto()
    DATE = enum.auto()
    DATE_TIME = enum.auto()
    STR = enum.auto()
    BYTES = enum.auto()
    TYPE = enum.auto()
    IDENTIFIER = enum.auto()
    EOF = enum.auto()


    @property
    def is_scalar(self):
        return self in {_Kind.NULL, _Kind.BOOL, _Kind.INT, _Kind.REAL,
                        _Kind.DATE, _Kind.DATE_TIME, _Kind.STR, _Kind.BYTES}


class List(collections.UserList):

    def __init__(self, seq=None, *, vtype=None, comment=None):
        '''Takes an optional sequence (list, tuple, iterable)
        .data holds the actual list
        .comment holds an optional read-only comment
        .vtype holds a read-only UXF type name ('int', 'real', …)'''
        super().__init__(seq)
        if vtype is not None:
            _check_type_name(vtype)
        self._vtype = vtype
        self._comment = comment


    @property
    def vtype(self):
        if self._vtype == '':
            self._vtype = None
        return self._vtype


    @property
    def comment(self):
        if self._comment == '':
            self._comment = None
        return self._comment


    def is_equivalent(self, other, compare=Compare.EXACT):
        '''Returns True if this List is equivalent to the other List;
        otherwise returns False.
        Use == or != for exact comparisons.
        Use this to ignore differences in comments Compare.IGNORE_COMMENTS
        or Compare.EQUIVALENT.
        '''
        if not isinstance(other, self.__class__):
            return False
        if (Compare.IGNORE_KVTYPES not in compare and
                self.vtype != other.vtype):
            return False
        if (Compare.IGNORE_COMMENTS not in compare and
                self.comment != other.comment):
            return False
        if len(self.data) != len(other.data):
            return False
        for avalue, bvalue in zip(self.data, other.data):
            if not _is_equivalent_value(avalue, bvalue, compare):
                return False
        return True


    def __eq__(self, other):
        '''Returns True if this List is the same as the other List;
        otherwise returns False.
        See also is_equivalent()'''
        if not isinstance(other, self.__class__):
            return False
        if self.vtype != other.vtype:
            return False
        if self.comment != other.comment:
            return False
        if len(self.data) != len(other.data):
            return False
        for avalue, bvalue in zip(self.data, other.data):
            if not _is_equivalent_value(avalue, bvalue, Compare.EXACT):
                return False
        return True


def _is_equivalent_value(avalue, bvalue, compare):
    avalue = _maybe_to_uxf_collection(avalue)
    bvalue = _maybe_to_uxf_collection(bvalue)
    if _is_uxf_collection(avalue) and _is_uxf_collection(bvalue):
        return avalue.is_equivalent(bvalue, compare)
    elif isinstance(avalue, float) and isinstance(bvalue, float):
        return math.isclose(avalue, bvalue)
    return avalue == bvalue


def _maybe_to_uxf_collection(value):
    if isinstance(value, dict):
        return Map(value)
    elif isinstance(value, (set, frozenset, tuple, collections.deque,
                            list)):
        return List(value)
    return value


class Map(collections.UserDict):

    def __init__(self, d=None, *, ktype=None, vtype=None, comment=None):
        '''Takes an optional dict
        .data holds the actual dict
        .comment holds an optional comment
        .ktype may only be bytes, date, datetime, int, str — or None meaning
        any valid ktype
        .vtype may hold a UXF type name ('int', 'str', …) — or None meaning
        any valid vtype
        A Map whose ktype is None _must_ have a vtype of None.
        '''
        super().__init__(d)
        if ktype is not None:
            _check_ktype(ktype)
        self._ktype = ktype
        if vtype is not None:
            if ktype is None:
                _raise_error(
                    299, 'a map may only have a vtype if it has a ktype')
            _check_type_name(vtype)
        self._vtype = vtype
        self._comment = comment
        self._pending_key = _MISSING


    @property
    def ktype(self):
        if self._ktype == '':
            self._ktype = None
        return self._ktype


    @property
    def vtype(self):
        if self._vtype == '':
            self._vtype = None
        return self._vtype


    @property
    def comment(self):
        if self._comment == '':
            self._comment = None
        return self._comment


    def _append(self, value):
        '''This is for UXF readers; instead use: map[key] = value

        If there's no pending key, sets the value as the pending key;
        otherwise adds a new item with the pending key and this value and
        clears the pending key.'''
        if self._pending_key is _MISSING:
            if not _is_key_type(value):
                prefix = ('map keys may only be of type int, date, '
                          'datetime, str, or bytes, got ')
                if isinstance(value, Table):
                    _raise_error(290, f'{prefix}a Table ( … ), maybe '
                                 'bytes (: … :) was intended?')
                else:
                    _raise_error(294, f'{prefix}{value.__class__.__name__} '
                                 f'{value!r}')
            self._pending_key = value
        else:
            self.data[self._pending_key] = value
            self._pending_key = _MISSING


    @property
    def _next_is_key(self):
        return self._pending_key is _MISSING


    def is_equivalent(self, other, compare=Compare.EXACT):
        '''Returns True if this Map is equivalent to the other Map;
        otherwise returns False.
        Use == or != for exact comparisons.
        Use this to ignore differences in comments Compare.IGNORE_COMMENTS
        or Compare.EQUIVALENT.
        '''
        if not isinstance(other, self.__class__):
            return False
        if (Compare.IGNORE_KVTYPES not in compare and
                self.ktype != other.ktype):
            return False
        if (Compare.IGNORE_KVTYPES not in compare and
                self.vtype != other.vtype):
            return False
        if (Compare.IGNORE_COMMENTS not in compare and
                self.comment != other.comment):
            return False
        if len(self.data) != len(other.data):
            return False
        for ((akey, avalue), (bkey, bvalue)) in zip(
                sorted(self.data.items(), key=_by_key),
                sorted(other.data.items(), key=_by_key)):
            if akey != bkey:
                return False
            if not _is_equivalent_value(avalue, bvalue, compare):
                return False
        return True


    def __eq__(self, other):
        '''Returns True if this Map is the same as the other Map; otherwise
        returns False.
        See also is_equivalent()'''
        if not isinstance(other, self.__class__):
            return False
        if self.ktype != other.ktype:
            return False
        if self.vtype != other.vtype:
            return False
        if self.comment != other.comment:
            return False
        if len(self.data) != len(other.data):
            return False
        for ((akey, avalue), (bkey, bvalue)) in zip(
                sorted(self.data.items(), key=_by_key),
                sorted(other.data.items(), key=_by_key)):
            if akey != bkey:
                return False
            if not _is_equivalent_value(avalue, bvalue, Compare.EXACT):
                return False
        return True


def _check_ktype(ktype, callback=None):
    if callback is None:
        callback = _raise_error
    if ktype not in _KTYPES:
        ktypes = ', '.join(sorted(_KTYPES))
        return callback(308,
                        f'a ktype must be one of ({ktypes}), got {ktype}')


def _check_name(name, callback=None):
    if callback is None:
        callback = _raise_error
    if name in RESERVED_WORDS:
        return callback(304, 'names cannot be the same as built-in type '
                        f'names or constants, got {name}')
    _check_type_name(name, callback)


def _check_type_name(name, callback=None):
    if callback is None:
        callback = _raise_error
    if not name:
        return callback(298, 'names must be nonempty')
    if name[0] != '_' and not name[0].isalpha():
        return callback(300, 'names must start with a letter or '
                        f'underscore, got {name}')
    if name in _BOOLS:
        return callback(302, f'names may not be yes or no got {name}')
    if len(name) > _MAX_IDENTIFIER_LEN:
        return callback(306, f'names may at most {_MAX_IDENTIFIER_LEN} char'
                        f'acters long, got {name} ({len(name)} characters)')
    for c in name:
        if not (c == '_' or c.isalnum()):
            return callback(310, 'names may only contain letters, digits, '
                            f'or underscores, got {name}')


class TClass:

    def __init__(self, ttype, fields=None, *, comment=None):
        '''The type of a Table
        .ttype holds the tclass's read-only name (equivalent to a vtype or
        ktype name); it may not be the same as a built-in type name or
        constant
        .fields holds a read-only sequence of Fields which may be supplied
        as field names (so their vtype's are None) or Field objects
        .comment holds an optional read-only comment

        This is best to use when you want to pass a sequence of fields:

            fields = [uxf.Field('x', 'int'), uxf.Field('y', 'int')]
            point_tclass = TClass('point', fields)

        Or no fields at all:

            fieldless_class = TClass('SomeEnumValue')

        TClasses are immutable.')
        '''
        _check_name(ttype)
        self._ttype = ttype
        self._fields = []
        self._comment = comment
        self._RecordClass = None
        if fields is not None:
            seen = set()
            for field in fields:
                if isinstance(field, str):
                    field = Field(field)
                self._fields.append(field)
                name = self._fields[-1].name
                if name in seen:
                    _raise_error(336, 'can\'t have duplicate table tclass '
                                 f'field names, got {name!r} twice')
                else:
                    seen.add(name)
            self._RecordClass = editabletuple.editabletuple(
                f'UXF_{self.ttype}', # prefix avoids name clashes
                *[field.name for field in self.fields])


    @property
    def ttype(self):
        return self._ttype


    @property
    def fields(self):
        return self._fields


    @property
    def comment(self):
        if self._comment == '':
            self._comment = None
        return self._comment

    @property
    def RecordClass(self):
        return self._RecordClass


    @property
    def isfieldless(self):
        return not bool(self.fields)


    def is_equivalent(self, other, compare=Compare.EXACT):
        '''Returns True if this TClass is equivalent to the other TClass;
        otherwise returns False.
        Use == or != for exact comparisons.
        Use this to ignore differences in comments Compare.IGNORE_COMMENTS,
        or differences in field vtypes Compare.IGNORE_KVTYPES, or both
        Compare.UNTYPED_EQUIVALENT
        '''
        if not isinstance(other, self.__class__):
            return False
        if self.ttype != other.ttype:
            return False
        if (Compare.IGNORE_COMMENTS not in compare and
                self.comment != other.comment):
            return False
        if Compare.IGNORE_KVTYPES not in compare:
            return self.fields == other.fields
        if len(self.fields) != len(other.fields):
            return False
        for afield, bfield in zip(self.fields, other.fields):
            if afield.name != bfield.name: # ignore vtype
                return False
        return True


    def __eq__(self, other):
        '''Returns True if this TClass is the same as the other TClass;
        otherwise returns False.
        See also is_equivalent()'''
        if not isinstance(other, self.__class__):
            return False
        if self.ttype != other.ttype:
            return False
        if self.comment != other.comment:
            return False
        return self.fields == other.fields


    def __lt__(self, other): # case-insensitive when possible
        uttype = self.ttype.upper()
        uother = other.ttype.upper()
        if uttype != uother:
            return uttype < uother
        return self.ttype < other.ttype


    def __len__(self):
        return len(self.fields)


    def __repr__(self):
        if self.fields:
            fields = ', '.join(repr(field) for field in self.fields)
            fields = f' {fields}, '
        else:
            fields = ''
        return (f'{self.__class__.__name__}({self.ttype!r},{fields}'
                f'comment={self.comment!r})')


class TClassBuilder:

    def __init__(self, ttype, fields=None, *, comment=None):
        self._ttype = ttype
        self._fields = []
        if fields is not None:
            for field in fields:
                self.append(field)
        self._comment = comment


    @property
    def ttype(self):
        return self._ttype


    @property
    def comment(self):
        if self._comment == '':
            self._comment = None
        return self._comment


    @property
    def fields(self):
        return self._fields


    def append(self, field):
        if isinstance(field, str):
            field = Field(field)
        self.fields.append(field)
        name = self.fields[-1].name
        for field in self.fields[:-1]:
            if field.name == name:
                _raise_error(338, 'can\'t append duplicate table tclass '
                             f'field names, got {name!r}')


    def build(self):
        return TClass(self.ttype, self.fields, comment=self.comment)


class Field:

    __slots__ = ('_name', '_vtype')

    def __init__(self, name, vtype=None):
        '''The type of one field in a Table
        .name holds the field's read-only name (equivalent to a vtype or
        ktype name); may not be the same as a built-in type name or constant
        .vtype holds a read-only UXF type name ('int', 'real', …) or None
        (meaning any UXF type is acceptable)
        Fields are immutable.'''
        _check_name(name)
        if vtype is not None:
            _check_type_name(vtype)
        self._name = name
        self._vtype = vtype


    @property
    def name(self):
        return self._name


    @property
    def vtype(self):
        if self._vtype == '':
            self._vtype = None
        return self._vtype


    def __eq__(self, other):
        return self.name == other.name and self.vtype == other.vtype


    def __lt__(self, other):
        if self.name != other.name:
            return self.name < other.name
        return self.vtype < other.vtype


    def __repr__(self):
        return f'{self.__class__.__name__}({self.name!r}, {self.vtype!r})'


def table(ttype, fields=(), *, comment=None, tclass_comment=None):
    '''Convenience function for creating empty tables with a new tclass.
    See also the Table constructor.'''
    return Table(TClass(ttype, fields, comment=tclass_comment),
                 comment=comment)


class Table:
    '''Used to store a UXF table.

    A Table has an optional list of fields (name, optional type) and (if it
    has fields), a records list which is a list of lists of values, with
    each sublist having the same number of items as the number of fields. It
    also has a .comment attribute. (Note that the lists in a Table are plain
    lists not UXF Lists.)

    Table's API is very similar to the list API, only it works in terms of
    whole records rather than individual field values. However, field values
    can be directly accessed using the field name or index.

    When a Table is iterated each record (row) is returned as a custom class
    instance, and in the process each record is converted from a list to a
    custom class instance if it isn't one already. The custom class allows
    fields to be accessed by name and by index.

    Some tables are fieldless, for example to represent enumerations.
    '''

    def __init__(self, tclass, *, records=None, comment=None):
        '''
        A Table must be created with a TClass but may be created with no
        records.

        .tclass is the table's read-only TClass

        .records can be a flat list of values (which will be put into a list
        of lists with each sublist being len(fields) long), or a list of
        lists in which case each list is _assumed_ to be len(fields) i.e.,
        len(tclass.fields), long

        .comment is an optional read-only str.

        .RecordClass is a read-only property holding a dynamically created
        custom class that is used when accessing a single record via [] or
        when iterating a table's records.

        See also the table() convenience function.
        '''
        self._tclass = tclass
        self.records = []
        self._comment = comment
        if records:
            if tclass is None:
                _raise_error(320, 'can\'t create a nonempty table '
                             'without fields')
            elif not tclass.ttype:
                _raise_error(330, 'can\'t create an unnamed nonempty table')
            if isinstance(records, (list, List)):
                self.records = list(records)
            else:
                for value in records:
                    self._append(value)


    @property
    def tclass(self):
        return self._tclass


    @property
    def comment(self):
        if self._comment == '':
            self._comment = None
        return self._comment


    @property
    def RecordClass(self):
        if self.tclass.RecordClass is None:
            _raise_error(332, 'cannot have records in a table with a '
                         'fieldless tclass')
        return self.tclass.RecordClass


    @property
    def ttype(self):
        return self.tclass.ttype if self.tclass is not None else None


    @property
    def fields(self):
        return self.tclass.fields if self.tclass is not None else []


    def field(self, column):
        return self.tclass.fields[column]


    @property
    def isfieldless(self):
        return self.tclass.isfieldless


    @property
    def _next_vtype(self):
        if self.isfieldless:
            return None
        if not self.records:
            return self.tclass.fields[0].vtype
        else:
            if len(self.records[-1]) == len(self.tclass):
                return self.tclass.fields[0].vtype
            return self.tclass.fields[len(self.records[-1])].vtype


    def _append(self, value):
        '''This is for UXF readers; instead use: table.append(record)

        Use to append a value to the table. The value will be added to
        the last record (row) if that isn't full, or as the first value in a
        new record (row)'''
        if not self.fields:
            _raise_error(334, 'can\'t append to a fieldless table')
        if not self.records or len(self.records[-1]) >= len(self.tclass):
            self.records.append([])
        self.records[-1].append(value)


    @property
    def is_scalar(self):
        for field in self.fields:
            if field.vtype is None:
                break # any type allowed so need to check records themselves
            if field.vtype not in _VTYPES:
                return False # non-scalar expected so not a scalar table
        else:
            return True # all vtypes specified and all scalar
        for record in self.records:
            for x in record:
                if not is_scalar(x):
                    return False
        return True


    def _classify(self, row):
        record = self.records[row]
        if not isinstance(record, self.RecordClass):
            record = self.records[row] = self.RecordClass(*record)
        return record


    def append(self, record):
        '''Appends a record (either a RecordClass tuple or a sequence of
        field values) to the table'''
        if not isinstance(record, self.RecordClass):
            record = self.RecordClass(*record)
        self.records.append(record)


    def insert(self, row, record):
        '''Inserts a record (either a RecordClass tuple or a sequence of
        field values) into the table at the given row position'''
        if not isinstance(record, self.RecordClass):
            record = self.RecordClass(*record)
        self.records.insert(row, record)


    @property
    def first(self):
        if self.records:
            return self[0]
        # else return None


    @property
    def second(self):
        if self.records:
            return self[1]
        # else return None


    @property
    def third(self):
        if self.records:
            return self[2]
        # else return None


    @property
    def fourth(self):
        if self.records:
            return self[3]
        # else return None


    @property
    def last(self):
        if self.records:
            return self[-1]
        # else return None


    def get(self, row):
        '''Return the row-th record as a custom class or None if row is out
        of range.

        If a record is returned it is an object reference to an
        editabletuple so any changes made to its fields will be reflected in
        the table.'''
        if 0 <= row < len(self.records):
            return self._classify(row)
        # else: return None


    def __getitem__(self, row):
        '''Return the row-th record as a custom class.

        The returned record is an object reference to an editabletuple so
        any changes made to its fields will be reflected in the table.'''
        return self._classify(row)


    def __setitem__(self, row, record):
        '''Replace the row-th record as a custom class'''
        if not isinstance(record, self.RecordClass):
            record = self.RecordClass(*record)
        self.records[row] = record


    def __delitem__(self, row):
        '''Deletes the table's row-th record'''
        del self.records[row]


    def __iter__(self):
        if not self.records:
            return
        for i in range(len(self.records)):
            record = self.records[i]
            if not isinstance(record, self.RecordClass):
                record = self.records[i] = self.RecordClass(*record)
            yield record


    def __len__(self):
        return len(self.records)


    def is_equivalent(self, other, compare=Compare.EXACT):
        '''Returns True if this Table is equivalent to the other Table;
        otherwise returns False.
        Use == or != for exact comparisons.
        Use this to ignore differences in comments Compare.IGNORE_COMMENTS
        or Compare.EQUIVALENT.
        '''
        if not isinstance(other, self.__class__):
            return False
        if not self.tclass.is_equivalent(other.tclass, compare):
            return False
        if (Compare.IGNORE_COMMENTS not in compare and
                self.comment != other.comment):
            return False
        if len(self.records) != len(other.records):
            return False
        for arecord, brecord in zip(self.records, other.records):
            for avalue, bvalue in zip(arecord, brecord):
                if not _is_equivalent_value(avalue, bvalue, compare):
                    return False
        return True


    def __eq__(self, other):
        '''Returns True if this Table is the same as the other Table;
        otherwise returns False.
        See also is_equivalent()'''
        if not isinstance(other, self.__class__):
            return False
        if self.comment != other.comment:
            return False
        if self.tclass != other.tclass:
            return False
        if len(self.records) != len(other.records):
            return False
        for arecord, brecord in zip(self.records, other.records):
            for avalue, bvalue in zip(arecord, brecord):
                if not _is_equivalent_value(avalue, bvalue, Compare.EXACT):
                    return False
        return True


    def __str__(self):
        parts = []
        if self.tclass is not None:
            parts.append(f'ttype={self.ttype!r}')
            if self.fields:
                parts.append(f'fields={self.fields!r}')
        else:
            parts.append('(no fields)')
        if self.comment:
            parts.append(f'comment={self.comment!r}')
        parts.append(f'({len(self.records)} records)')
        return ' '.join(parts)


    def __repr__(self):
        return (f'{self.__class__.__name__}({self.tclass!r}, '
                f'records={self.records!r}, comment={self.comment!r})')


def _parse(tokens, filename='-', *, on_event=on_event, drop_unused=False,
           replace_imports=False, _imported=None, _is_import=False):
    parser = _Parser(filename, on_event=on_event, drop_unused=drop_unused,
                     replace_imports=replace_imports, _imported=_imported,
                     _is_import=_is_import)
    data, comment = parser.parse(tokens)
    return data, comment, parser.tclasses, parser.imports


class _Parser(_EventMixin):

    def __init__(self, filename, *, on_event=on_event, drop_unused=False,
                 replace_imports=False, _imported=None, _is_import=False):
        self.filename = filename
        self.on_event = functools.partial(
            on_event, filename=os.path.basename(filename))
        self.drop_unused = drop_unused
        self.replace_imports = replace_imports
        self._is_import = _is_import
        self.clear()
        if _imported is not None:
            self.imported = _imported
        if filename and filename != '-':
            filename = _full_filename(filename)
            if filename in self.imported:
                self.fatal(400, f'already imported {filename}')
                return # in case user on_event doesn't raise
            self.imported.add(filename)


    def clear(self):
        self.stack = []
        self.imports = {} # key=ttype value=import text
        self.imported = set() # to avoid reimports or self import
        self.tclasses = {} # key=ttype value=TClass
        self.lino_for_tclass = {} # key=ttype value=lino
        self.used_tclasses = set()
        self.pos = -1
        self.lino = 0


    def parse(self, tokens):
        if not tokens:
            return
        self.tokens = tokens
        data = None
        comment = self._parse_file_comment()
        self._parse_imports()
        self._parse_tclasses()
        for i, token in enumerate(self.tokens):
            self.lino = token.lino
            kind = token.kind
            collection_start = self._is_collection_start(kind)
            if data is None and not collection_start:
                self.error(402,
                           f'expected a map, list, or table, got {token}')
            if collection_start:
                next_value = (self.tokens[i + 1].value
                              if i + 1 < len(self.tokens) else None)
                self._on_collection_start(token, next_value)
                if data is None:
                    data = self.stack[0]
            elif self._is_collection_end(kind):
                self._on_collection_end(token)
            elif kind is _Kind.IDENTIFIER: # All the valid ones are subsumed
                self._handle_incorrect_identifier(token)
            elif kind is _Kind.STR:
                self._handle_str(token)
            elif kind.is_scalar:
                self._handle_scalar(token)
            elif kind is _Kind.EOF:
                break
            else:
                self.error(410, f'unexpected token, got {token}')
        if not self._is_import:
            self._check_tclasses()
        return data, comment


    def _check_tclasses(self):
        imported = set(self.imports.keys())
        if self.replace_imports:
            self._replace_imports(imported)
        defined = set(self.tclasses.keys())
        if self.drop_unused:
            self._drop_unused(defined)
        unused = defined - self.used_tclasses
        unused -= imported # don't warn on unused imports
        unused = {ttype for ttype in unused # don't warn on fieldless
                  if not self.tclasses[ttype].isfieldless}
        if unused:
            self._report_problem(unused, 422, 'unused ttype')
        undefined = self.used_tclasses - defined
        if undefined:
            self._report_problem(undefined, 424, 'undefined ttype')


    def _replace_imports(self, imported):
        for ttype in imported:
            if ttype not in self.used_tclasses:
                del self.tclasses[ttype] # drop unused imported ttype
        self.imports.clear()
        imported.clear()


    def _drop_unused(self, defined):
        ttypes_for_filename = collections.defaultdict(set)
        for ttype, filename in self.imports.items():
            ttypes_for_filename[filename].add(ttype)
        for ttype in list(self.tclasses):
            if ttype not in self.used_tclasses:
                del self.tclasses[ttype] # drop unused ttype def
                defined.discard(ttype)
                for filename in list(ttypes_for_filename):
                    ttypes_for_filename[filename].discard(ttype)
        for filename, ttypes in ttypes_for_filename.items():
            if not ttypes:
                for ttype, ifilename in list(self.imports.items()):
                    if filename == ifilename:
                        del self.imports[ttype] # drop unused import


    def _report_problem(self, diff, code, what):
        handle = self.warning if code == 422 else self.error
        diff = sorted(diff)
        if len(diff) == 1:
            handle(code, f'{what}: {diff[0]!r}')
        else:
            diff = ', '.join(repr(t) for t in diff)
            handle(code, f'{what}s: {diff}')


    def _verify_type_identifier(self, vtype):
        if vtype is not None:
            if vtype in _ANY_VTYPES:
                return # built-in type
            tclass = self.tclasses.get(vtype)
            if tclass is None:
                self.error(446, f'expected list vtype, got {vtype}')
            else:
                self.used_tclasses.add(tclass.ttype)


    def _verify_ttype_identifier(self, tclass, next_value):
        if tclass is None: # A table with no tclass is invalid
            value = f', got {next_value}' if next_value is not None else ''
            self.fatal(450, f'expected table ttype{value}')
        else:
            self.used_tclasses.add(tclass.ttype)
            if len(self.stack) > 1:
                parent = self.stack[-1]
                vtype = getattr(parent, 'vtype', None)
                if (vtype is not None and vtype != 'table' and
                        vtype != tclass.ttype):
                    self.error(456, f'expected table value of type {vtype},'
                               f' got value of type {tclass.ttype}')


    def _handle_incorrect_identifier(self, token):
        if token.value.upper() in {'TRUE', 'FALSE'}:
            self.error(458, 'boolean values are represented by yes or no')
        else:
            self.error(460, 'ttypes may only appear at the start of a '
                       f'map (as the value type), list, or table, {token}')


    def _handle_str(self, token):
        value = token.value
        vtype, message = self.typecheck(value)
        if value is not None and vtype is not None and vtype in {
                'bool', 'int', 'real', 'date', 'datetime'}:
            new_value = naturalize(value)
            if new_value != value:
                self.repair(486,
                            f'converted str {value!r} to {vtype} {value}')
                value = new_value
            else:
                self.error(488, message)
        if not self.stack:
            self.fatal(489, 'invalid UXF data')
            return # in case user on_event doesn't raise
        append_to_parent(self.stack[-1], value)


    def _handle_scalar(self, token):
        value = token.value
        vtype, message = self.typecheck(value)
        if value is not None and vtype is not None:
            if vtype == 'real' and isinstance(value, int):
                v = float(value)
                self.repair(496, f'converted int {value} to real {v}')
                value = v
            elif vtype == 'int' and isinstance(value, float):
                v = round(value)
                self.repair(498, f'converted real {value} to int {v}')
                value = v
            else:
                self.error(500, message)
        if not self.stack:
            self.fatal(501, 'invalid UXF data')
            return # in case user on_event doesn't raise
        append_to_parent(self.stack[-1], value)


    def _on_collection_start(self, token, next_value):
        kind = token.kind
        if kind is _Kind.LIST_BEGIN:
            self._verify_type_identifier(token.vtype)
            value = List(vtype=token.vtype, comment=token.comment)
        elif kind is _Kind.MAP_BEGIN:
            if token.ktype is not None and token.ktype not in _KTYPES:
                self.error(448, f'expected map ktype, got {token.ktype}')
            self._verify_type_identifier(token.vtype)
            value = Map(ktype=token.ktype, vtype=token.vtype,
                        comment=token.comment)
        elif kind is _Kind.TABLE_BEGIN:
            tclass = self.tclasses.get(token.ttype)
            self._verify_ttype_identifier(tclass, next_value)
            value = Table(tclass, comment=token.comment)
        else:
            self.error(504, 'expected to create map, list, or table, '
                       f'got {token}')
        if self.stack:
            _, message = self.typecheck(value)
            if message is not None:
                self.error(506, message)
            # add the collection to the parent
            append_to_parent(self.stack[-1], value)
        self.stack.append(value) # make the collection the current parent


    def _on_collection_end(self, token):
        if not self.stack:
            self.error(510, f'unexpected {token} suggests unmatched map, '
                       'list, or table start/end pair')
            return
        parent = self.stack[-1]
        if token.kind is _Kind.LIST_END:
            Class = List
            end = ']'
        elif token.kind is _Kind.MAP_END:
            Class = Map
            end = '}'
        elif token.kind is _Kind.TABLE_END:
            Class = Table
            end = ')'
        if not isinstance(parent, Class):
            self.error(512, f'expected {end!r}, got {token.value!r}')
        self.stack.pop()


    def _is_collection_start(self, kind):
        return kind in {_Kind.MAP_BEGIN, _Kind.LIST_BEGIN,
                        _Kind.TABLE_BEGIN}


    def _is_collection_end(self, kind):
        return kind in {_Kind.MAP_END, _Kind.LIST_END,
                        _Kind.TABLE_END}


    def _parse_file_comment(self):
        if self.tokens:
            token = self.tokens[0]
            self.lino = token.lino
            if token.kind is _Kind.FILE_COMMENT:
                self.tokens = self.tokens[1:]
                return token.value
        return None


    def _parse_imports(self):
        offset = 0
        for index, token in enumerate(self.tokens):
            self.lino = token.lino
            if token.kind is _Kind.IMPORT:
                self._handle_import(token.value)
                offset = index + 1
        self.tokens = self.tokens[offset:]


    def _parse_tclasses(self):
        tclass_builder = None
        offset = 0
        lino = 0
        for index, token in enumerate(self.tokens):
            self.lino = token.lino
            if token.kind is _Kind.TCLASS_BEGIN:
                tclass_builder = TClassBuilder(token.value,
                                               comment=token.comment)
                lino = self.lino
            elif token.kind is _Kind.FIELD:
                if tclass_builder is None:
                    self.fatal(524, 'Field outside TClass')
                    return # in case user on_event doesn't raise
                tclass_builder.append(Field(token.value, token.vtype))
            elif token.kind is _Kind.TCLASS_END:
                if not self._handle_tclass_end(tclass_builder, lino):
                    return # in case user on_event doesn't raise
                offset = index + 1
                tclass_builder = None
                lino = 0
            else:
                break # no TClasses at all
        self.tokens = self.tokens[offset:]


    def _handle_tclass_end(self, tclass_builder, lino):
        if tclass_builder is not None:
            if tclass_builder.ttype is None:
                self.fatal(526, 'TClass without ttype')
                return False # in case user on_event doesn't raise
            tclass = tclass_builder.build()
            _add_to_tclasses(self.tclasses, tclass, lino=self.lino,
                             code=528, on_event=self.on_event)
            self.lino_for_tclass[tclass.ttype] = lino
        return True


    def _handle_import(self, value):
        filename = text = None
        try:
            if value.startswith(('http://', 'https://')):
                text = self._url_import(value)
            elif '.' not in value:
                return self._system_import(value)
            else:
                filename = value
            if filename is not None:
                uxo = self._load_import(filename)
            elif text is not None:
                uxo = self._parse_url_text(text, value)
            else:
                self.fatal(540, 'there are no ttype definitions to import '
                           f'{value!r} ({filename!r})')
                return # should never get here
            if uxo is None:
                self.fatal(541, 'invalid UXF data')
                return # in case user on_event doesn't raise
            for ttype, tclass in uxo.tclasses.items():
                if _add_to_tclasses(self.tclasses, tclass, lino=self.lino,
                                    code=544, on_event=self.on_event):
                    self.imports[ttype] = value
        except _AlreadyImported:
            pass # don't reimport & errors already handled


    def _url_import(self, url):
        if url in self.imported:
            raise _AlreadyImported # don't reimport
        try:
            with urllib.request.urlopen(url) as file:
                return file.read().decode()
        except (UnicodeDecodeError, OSError) as err:
            if url == 'http://www.qtrac.eu/ttype-eg.uxf':
                return _TTYPE_EG_UXF # Let tests work w/o internet connect'n
            self.error(550, f'failed to import {url!r}: {err}')
            raise _AlreadyImported
        finally:
            self.imported.add(url) # don't want to retry


    def _parse_url_text(self, text, filename):
        try:
            return loads(text, filename=filename,
                         on_event=self._on_import_error,
                         _imported=self.imported, _is_import=True)
        except Error as err:
            self.error(530, f'failed to import {filename!r}: {err}')


    def _system_import(self, value):
        if value == 'complex':
            self._system_import_tclass(_ComplexTClass, value)
        elif value == 'fraction':
            self._system_import_tclass(_FractionTClass, value)
        elif value == 'numeric':
            self._system_import_tclass(_ComplexTClass, value)
            self._system_import_tclass(_FractionTClass, value)
        else:
            self.error(560,
                       f'there is no system ttype import called {value!r}')
            raise _AlreadyImported


    def _system_import_tclass(self, tclass, name):
        if _add_to_tclasses(self.tclasses, tclass, lino=self.lino,
                            code=570, on_event=self.on_event):
            self.imports[tclass.ttype] = name


    def _load_import(self, filename):
        fullname = self._get_fullname(filename)
        if fullname in self.imported:
            raise _AlreadyImported # don't reimport
        try:
            return load(fullname, on_event=self._on_import_error,
                        _imported=self.imported, _is_import=True)
        except (FileNotFoundError, Error) as err:
            if fullname in self.imported:
                self.fatal(580, f'cannot do circular imports {fullname!r}')
                return # in case user on_event doesn't raise
            else:
                self.error(586, f'failed to import {fullname!r}: {err}')
            raise _AlreadyImported # couldn't import
        finally:
            self.imported.add(fullname) # don't want to retry


    def _get_fullname(self, filename):
        '''Searches in order: filename's path, cwd, UXF_PATHs'''
        paths = []
        if self.filename and self.filename != '-':
            paths.append(os.path.dirname(self.filename))
        if paths and paths[0] != '.':
            paths.append('.')
        uxf_paths = os.environ.get('UXF_PATH')
        if uxf_paths:
            paths += uxf_paths.split(
                ';' if sys.platform.startswith('win') else ':')
        for path in paths:
            fullname = _full_filename(filename, path)
            if fullname in self.imported:
                raise _AlreadyImported # don't reimport
            if os.path.isfile(fullname):
                return fullname # stop as soon as we find one
        return _full_filename(filename)


    def _on_import_error(self, event, code, message, *, filename='-',
                         lino=0, verbose=True):
        if code == 418: # we expect all ttypes to be unused here
            return
        self.on_event(event, code, message, filename=filename, lino=lino,
                      verbose=verbose)


    def typecheck(self, value):
        if not self.stack:
            self.fatal(590, 'invalid UXF data')
            return None, None # in case user on_event doesn't raise
        parent = self.stack[-1]
        if isinstance(parent, List):
            vtype = parent.vtype
        elif isinstance(parent, Map):
            vtype = parent.ktype if parent._next_is_key else parent.vtype
            if (vtype is not None and isinstance(value, Table) and vtype !=
                    value.ttype):
                self.error(454, f'expected table value of type {vtype},'
                           f' got table of type {value.ttype}')
                return None, None # handled
        elif isinstance(parent, Table):
            vtype = parent._next_vtype
        else:
            return None, f'expected collection, got {value}'
        if value is not None and vtype is not None:
            if vtype in _BUILT_IN_NAMES:
                if not isinstance(value, _TYPECHECK_CLASSES[vtype]):
                    message = (f'expected {vtype}, got '
                               f'{value.__class__.__name__} {value}')
                    return vtype, message
            elif vtype not in self.tclasses:
                message = (f'expected {vtype}, got '
                           f'{value.__class__.__name__} {value}')
                return vtype, message
        return None, None


def _add_to_tclasses(tclasses, tclass, *, lino, code, on_event):
    first_tclass = tclasses.get(tclass.ttype)
    if first_tclass is None: # this is the first definition of this ttype
        tclasses[tclass.ttype] = tclass
        return True
    if first_tclass == tclass:
        if tclass.comment and tclass.comment != first_tclass.comment:
            first_tclass.comment = tclass.comment # last comment wins
        return True # harmless duplicate
    else:
        on_event(Event.FATAL, code,
                 f'conflicting ttype definitions for {tclass.ttype}',
                 lino=lino)
    return False


_TYPECHECK_CLASSES = dict(
    collection=(List, Map, Table), bool=bool, bytes=(bytes, bytearray),
    date=datetime.date, datetime=datetime.datetime, int=int, list=List,
    map=Map, real=float, str=str, table=Table)
_BUILT_IN_NAMES = tuple(_TYPECHECK_CLASSES.keys())


def dump(filename_or_filelike, data, *, on_event=on_event, format=Format()):
    '''
    filename_or_filelike is sys.stdout or a filename or an open writable
    file (text mode UTF-8 encoded). If filename_or_filelike is a filename
    with a .gz suffix then the output will be gzip-compressed.

    data is a Uxf object, or a list, List, dict, Map, or Table, that this
    function will write to the filename_or_filelike in UXF format.
    '''
    close = False
    if isinstance(filename_or_filelike, (str, pathlib.Path)):
        filename_or_filelike = str(filename_or_filelike)
        opener = (gzip.open if filename_or_filelike[-3:].upper().endswith(
                  '.GZ') else open)
        file = opener(filename_or_filelike, 'wt', encoding=UTF8)
        close = True
    else:
        file = filename_or_filelike
    try:
        if not isinstance(data, Uxf):
            data = Uxf(data, on_event=on_event)
        _Writer(file, data, on_event, format)
    finally:
        if close:
            file.close()


def dumps(data, *, on_event=on_event, format=Format()):
    '''
    data is a Uxf object, or a list, List, dict, Map, or Table that this
    function will write to a string in UXF format which will then be
    returned.
    '''
    string = io.StringIO()
    if not isinstance(data, Uxf):
        data = Uxf(data, on_event=on_event)
    _Writer(string, data, on_event, format)
    return string.getvalue()


class _Writer:

    def __init__(self, file, uxo, on_event, format):
        self.file = file
        self.on_event = on_event
        self.format = format
        self.column = 0
        self.indent = 0
        self.closed = ''
        self.last_line = '\n'
        self.prev_last_line = '\n'
        self.pending = []
        self.write_header(uxo.custom)
        if uxo.comment is not None:
            self.file.write(f'#<{escape(uxo.comment)}>\n')
        if uxo.imports:
            self.write_imports(uxo.import_filenames)
        if uxo.tclasses:
            self.write_tclasses(uxo.tclasses, uxo.imports)
        self.write_value(uxo.value)
        self._write_pending('\n' if self.prev != '\n' else '', force=True)


    @property
    def prev(self):
        return self.last_line[-1] if self.last_line else '\0'


    @property
    def had_space(self):
        return self.last_line.isspace()


    def write_header(self, custom):
        self.file.write(f'uxf {VERSION}')
        if custom:
            self.file.write(f' {custom}')
        self.file.write('\n')


    def write_imports(self, import_filenames):
        for filename in import_filenames: # don't sort!
            self.file.write(f'!{filename}\n')


    def write_tclasses(self, tclasses, imports):
        for ttype, tclass in sorted(tclasses.items(),
                                    key=lambda t: t[0].upper()):
            if imports and ttype in imports:
                continue # defined in an import
            self.file.write('=')
            column = self._write_tclass_comment(1, tclass)
            column = self._write_tclass_ttype(column, tclass)
            for field in tclass.fields:
                column = self._write_tclass_field(column, field)
            self.file.write('\n')


    def _write_tclass_comment(self, column, tclass):
        if tclass.comment:
            text = f'#<{escape(tclass.comment)}> '
            column += len(text)
            self.file.write(text)
        return column


    def _write_tclass_ttype(self, column, tclass):
        text = f'{tclass.ttype}'
        if self.format.wrap_width:
            if column + len(text) > self.format.wrap_width:
                self.file.write(f'\n{self.format.indent}')
                column = len(self.format.indent)
            else:
                column += len(text)
        self.file.write(text)
        return column


    def _write_tclass_field(self, column, field):
        text = f' {field.name}'
        if field.vtype is not None:
            text += f':{field.vtype}'
        if self.format.wrap_width:
            if column + len(text) > self.format.wrap_width:
                self.file.write(f'\n{self.format.indent}')
                column = len(self.format.indent)
            else:
                column += len(text)
        self.file.write(text)
        return column


    def write_value(self, item):
        if isinstance(item, (set, frozenset, tuple, collections.deque)):
            item = List(item)
        if isinstance(item, (list, List)):
            self.write_list(item)
        elif isinstance(item, (dict, Map)):
            self.write_map(item)
        elif isinstance(item, Table):
            self.write_table(item)
        else:
            self.write_scalar(item)


    def write_list(self, item):
        closed = self._write_nl_if_closed()
        prefix = self._collection_prefix(item)
        text = f'{self.tab}[{prefix}'
        if len(item) == 0:
            self._write_close(text, ']')
            return
        text = self._write_open(text)
        if len(item) == 1 or _are_short(self.format.max_short_len, *item):
            sep = _SEP if prefix and not text.endswith(_SEP) else ''
            self._write_short_list(sep, item)
        else:
            self.indent += 1
            if not closed or self.prev != '\n':
                self._write_one(f'\n{self.format.indent * self.indent}')
            self._write_list(item)
            self.indent -= 1
            self._write_nl(']')


    def _write_short_list(self, sep, item):
        for value in item:
            self._write_one_value(value, sep)
            sep = _SEP
        self._write_one(']')


    def _write_list(self, item):
        sep = ''
        for value in item:
            self._write_one_value(value, sep)
            sep = _SEP


    def write_map(self, item):
        closed = self._write_nl_if_closed()
        prefix = self._collection_prefix(item)
        text = f'{self.tab}{{{prefix}'
        if len(item) == 0:
            self._write_close(text, '}')
            return
        text = self._write_open(text)
        sep = _SEP if prefix and not text.endswith(_SEP) else ''
        if len(item) == 1:
            self._write_single_item_map(sep, item)

        elif _are_short(self.format.max_short_len, *list(item.values())):
            self._write_short_map(sep, item)
        else:
            self.indent += 1
            if not closed:
                self._write_one(f'\n{self.format.indent * self.indent}')
            self._write_map(item)
            self.indent -= 1
            self._write_nl('}')


    def _write_single_item_map(self, sep, item):
        key, value = list(item.items())[0]
        if sep:
            self._write_one(sep)
        self.write_scalar(key)
        self._write_one_value(value)
        self._write_one('}')


    def _write_short_map(self, sep, item):
        for key, value in sorted(item.items(), key=_by_key):
            if sep:
                self._write_one(sep)
            self.write_scalar(key)
            self._write_one_value(value)
            sep = _SEP
        self._write_one('}')


    def _write_map(self, item):
        for key, value in sorted(item.items(), key=_by_key):
            self._write_pre_item_nl('')
            self.write_scalar(key)
            scalar = is_scalar(value)
            if not scalar:
                self.indent += 1
            self._write_one_value(value)
            if not scalar:
                self.indent -= 1


    def _write_one_value(self, value, sep=_SEP):
        if sep and not self.prev.isspace() and (
                is_scalar(value) or
                _is_short(value, self.format)):
            self._write_one(sep)
        self.write_value(value)


    def write_table(self, item):
        closed = self._write_nl_if_closed()
        prefix = self._collection_prefix(item)
        text = f'{self.tab}({prefix}'
        if len(item) == 0:
            self._write_close(text, ')')
            return
        text = self._write_open(text)
        if len(item) == 1:
            sep = _SEP if prefix and not text.endswith(_SEP) else ''
            self._write_single_record_table(item[0], sep)
        else:
            self.indent += 1
            if not closed:
                self._write_one(f'\n{self.format.indent * self.indent}')
            self._write_table(item)
            self.indent -= 1
            self._write_nl(')')


    def _write_single_record_table(self, record, sep):
        self._write_one(sep)
        self._write_record(record)
        self._write_one(')')


    def _write_table(self, item):
        for record in item:
            self._write_pre_item_nl('')
            self._write_record(record)


    def _write_record(self, record):
        sep = ''
        for value in record:
            self._write_one(sep)
            self._write_one_value(value, sep)
            sep = _SEP


    def write_scalar(self, item):
        if item is None:
            self._write_one('?')
        elif isinstance(item, bool):
            self._write_one('yes' if item else 'no')
        elif isinstance(item, int):
            self._write_one(str(item))
        elif isinstance(item, float):
            if self.format.realdp is not None:
                item = round(item, self.format.realdp)
            text = str(item)
            if '.' not in text and 'e' not in text and 'E' not in text:
                text += '.0'
            self._write_one(text)
        elif isinstance(item, (datetime.date, datetime.datetime)):
            self._write_one(isoformat(item)) # 1-second resolution
        elif isinstance(item, str):
            self._write_one(f'<{escape(item)}>')
        elif isinstance(item, (bytes, bytearray)):
            self._write_one(f'(:{item.hex().upper()}:)')
        else:
            self.on_event(Event.FATAL, 561, 'unexpected item of type '
                          f'{item.__class__.__name__}: {item!r};'
                          'consider using a ttype')
            return # in case user on_event doesn't raise
        return False


    @property
    def tab(self):
        if self.prev == '\n':
            return self.format.indent * self.indent
        return '' if self.prev.isspace() else _SEP


    def _write_nl(self, one):
        self._write_one('\n')
        self._write_one(self.format.indent * self.indent)
        self._write_one(one)


    def _write_pre_item_nl(self, one):
        self._write_one('\n')
        self._write_one(self.format.indent * self.indent)
        self._write_one(one)


    def _write_one(self, one):
        if (not one or one == self.prev == '\n') or (
                self.prev_last_line not in ']})' and
                not self.prev_last_line.endswith(':)') and one == '\n' and
                self.had_space):
            return
        wrap_width = self.format.wrap_width
        if (wrap_width and len(one) < wrap_width and not one.isspace() and
                self.column + len(one) > wrap_width and
                not ('\n' in self.prev_last_line and self.had_space)):
            self._write_pending('\n')
            tab = self.format.indent * self.indent
            self._write_pending(tab)
            self._update(f'\n{tab}')
        self._write_pending(one)
        self._update(one)
        one = one.rstrip()
        if not one.endswith(':)') and one.endswith((']', '}', ')')):
            self.closed = one[-1]
        elif not self.prev.isspace():
            self.closed = ''


    def _update(self, text):
        i = text.rfind('\n')
        if i == -1:
            self.column += len(text)
        else:
            self.column = len(text[i + 1:])
            text = text[i:]
        self.prev_last_line = self.last_line
        self.last_line = text


    def _write_pending(self, text, *, force=False):
        if text.endswith('\n'):
            while self.pending and self.pending[-1].isspace():
                self.pending.pop() # discard spurious spaces before newline
            self.pending.append(text)
            force = True
        if force:
            text = ''.join(self.pending)
            self.file.write(text)
            self._update(text)
            self.pending.clear()
        else:
            self.pending.append(text)


    def _write_open(self, text):
        if text.endswith('>'):
            text += _SEP # opening ended with a comment
        self._write_one(text)
        return text


    def _write_close(self, text, close):
        text += close
        self._write_one(text)


    def _write_nl_if_closed(self):
        closed = False
        if self.closed in ']})':
            self._write_one('\n')
            closed = True
        return closed


    def _collection_prefix(self, item):
        comment = getattr(item, 'comment', None)
        ktype = getattr(item, 'ktype', None)
        vtype = getattr(item, 'vtype', None)
        tclass = getattr(item, 'tclass', None)
        parts = []
        if comment is not None:
            parts.append(f'#<{escape(comment)}>')
        if ktype is not None:
            parts.append(ktype)
        if vtype is not None:
            parts.append(vtype)
        if tclass is not None:
            parts.append(tclass.ttype)
        return _SEP.join(parts) if parts else ''


def is_scalar(x):
    '''Returns True if x is None or a bool, int, float, datetime.date,
    datetime.datetime, str, bytes, or bytearray; otherwise returns False.'''
    return x is None or isinstance(
        x, (bool, int, float, datetime.date, datetime.datetime, str, bytes,
            bytearray))


def _is_key_type(x):
    return isinstance(x, (int, datetime.date, datetime.datetime, str,
                          bytes))


def _is_short(value, format):
    if isinstance(value, (bytes, bytearray, str)):
        return len(value) <= format.max_short_len
    if is_scalar(value) or len(value) == 1:
        return True
    if isinstance(value, (list, List)) and _are_short(format.max_short_len,
                                                      *value):
        return True
    if isinstance(value, (dict, Map)) and _are_short(
            format.max_short_len, *list(value.values())):
        return True
    return False


def _are_short(max_short_len, *items):
    for x in items:
        if isinstance(x, (bytes, bytearray, str)):
            if len(x) > max_short_len:
                return False
        elif x is not None and not isinstance(
                x, (bool, int, float, datetime.date, datetime.datetime)):
            return False
    return True


def _is_uxf_collection(value):
    return isinstance(value, (List, Map, Table))


def _full_filename(filename, path='.'):
    if os.path.isabs(filename):
        return filename
    return os.path.abspath(os.path.join(path, filename))


def _by_key(item):
    # Order is: bytes, dates, datetimes, ints, strs
    key = item[0]
    if isinstance(key, (bytes, bytearray)):
        return (1, key)
    if isinstance(key, datetime.datetime): # must compare before date
        return (3, key) # want datetimes _after_ dates
    if isinstance(key, datetime.date): # a date is also a datetime; not v.v.
        return (2, key)
    if isinstance(key, int):
        return (4, key)
    return (5, key)


class _AlreadyImported(Exception):
    pass


def append_to_parent(parent, value):
    '''Utility for UXF processors; see uxf.py and uxfconvert.py for examples
    of use.'''
    if isinstance(parent, (Map, Table)):
        parent._append(value)
    else:
        parent.append(value)


def naturalize(s):
    '''Given string s returns True if the string is 't', 'true', 'y', 'yes',
    or False if the string is 'f', 'false', 'n', 'no' (case-insensitive), or
    an int if s holds a parsable int, or a float if s holds a parsable
    float, or a datetime.datetime if s holds a parsable ISO8601 datetime
    string, or a datetime.date if s holds a parsable ISO8601 date string, or
    failing these returns the string s unchanged.
    '''
    u = s.upper()
    if u in {'T', 'TRUE', 'Y', 'YES'}:
        return True
    if u in {'F', 'FALSE', 'N', 'NO'}:
        return False
    try:
        return int(s)
    except ValueError:
        try:
            return float(s)
        except ValueError:
            try:
                if 'T' in s:
                    return datetime.datetime.fromisoformat(s)
                else:
                    return datetime.date.fromisoformat(s)
            except ValueError:
                return s


def canonicalize(name):
    '''Given a name, returns a name that is a valid table or field name.
    is_table_name must be True (the default) for tables since table names
    have additional constraints. (See uxfconvert.py for uses.)'''
    prefix = 'UXF_'
    cs = []
    if name[0] == '_' or name[0].isalpha():
        cs.append(name[0])
    else:
        cs.append(prefix)
    for c in name[1:]:
        if c.isspace() or c in '/\\,;:.-':
            if not cs or cs[-1] != '_':
                cs.append('_')
        elif c == '_' or c.isalnum():
            cs.append(c)
    name = ''.join(cs)
    if name in RESERVED_WORDS:
        name = prefix + name
    elif not name:
        name = prefix
    if name == prefix:
        name += str(canonicalize.count)
        canonicalize.count += 1
    return name[:_MAX_IDENTIFIER_LEN]
canonicalize.count = 1 # noqa: E305


def isasciidigit(s):
    '''Returns True if s matches /^[0-9]+$/.'''
    return s.isascii() and s.isdigit()


def isoformat(dt):
    '''Returns ISO8601 str representing the given date or datetime; in the
    latter case limited to 1-second resolution.'''
    return dt.isoformat()[:19]


_ComplexTClass = TClass('Complex', (Field('Real', 'real'),
                                    Field('Imag', 'real')))

_FractionTClass = TClass('Fraction', (Field('numerator', 'int'),
                                      Field('denominator', 'int')))

_TTYPE_EG_UXF = '''uxf 1.0 Slides 0.1.0
#<This is a simple example of an application-specific use of UXF.
See slides[12].py for examples of converting this format to HTML.>
= Slide title body
= #<title> h1 content
= #<subtitle> h2 content
= #<bullet item> B content
= #<para> p content
= #<image> img content image:bytes
= #<monospace inline> m content
= #<monospace block> pre content
= #<italic> i content
= url content link
= #<newline with no content> nl
[]
'''


if __name__ == '__main__':
    import argparse
    import contextlib
    import shutil
    import textwrap

    def get_usage(text):
        try:
            term_width = shutil.get_terminal_size()[0]
        except AttributeError:
            term_width = 80
        return '\n\n'.join('\n'.join(
            textwrap.wrap(para.strip(), term_width))
            for para in text.strip().split('\n\n') if para.strip())

    parser = argparse.ArgumentParser(usage=get_usage('''\
usage: uxf.py [-l|--lint] [-d|--dropunused] [-r|--replaceimports] \
[-iI|--indent=I] [-wW|--wrapwidth=W] <infile.uxf[.gz]> [<outfile.uxf[.gz]>]
   or: python3 -m uxf ...same options as above...

If an outfile is specified and ends .gz it will be gzip-compressed.
If outfile is - output will be to stdout.
If you just want linting either don't specify an outfile at all or
use -l or --lint. Lint errors and fixes go to stderr.

Use -d or --dropunused to drop unused ttype definitions and imports.

Use -r or --replaceimports to replace imports with ttype definitions
to make the outfile standalone (i.e., not dependent on any imports).

(-d, -l, and -r may be grouped, e.g., -ldr, -dl, etc.)

Indent defaults to 2 and accepts a range of 0-32.

Wrapwidth defaults to 96 and accepts 0 (no wrapping) or a range of 40-240.

For indent and wrapwidth the default is silently used if an out of range
value is given.

To get an uncompressed .uxf file run: `uxf.py infile.uxf.gz outfile.uxf` or
simply `gunzip infile.uxf.gz`.

To produce a compressed and compact .uxf file run:
`uxf.py -i0 -w0 infile.uxf outfile.uxf.gz`

Converting uxf to uxf will alphabetically order any ttypes.
However, the order of imports is preserved (with any duplicates removed)
to allow later imports to override earlier ones.
'''))
    parser.add_argument('-l', '--lint', action='store_true',
                        help='show lint errors')
    parser.add_argument('-d', '--dropunused', action='store_true',
                        help='drop unused imports and ttypes')
    parser.add_argument('-r', '--replaceimports', action='store_true',
                        help='replace imports with their used ttypes')
    parser.add_argument('-i', '--indent', type=int, default=2,
                        help='indent (0-32; default 2)')
    parser.add_argument('-w', '--wrapwidth', type=int, default=96,
                        help='wrapwidth (0 or 40-240; default 96)')
    parser.add_argument('infile', nargs=1, help='required UXF infile')
    parser.add_argument('outfile', nargs='?', help='optional UXF outfile')
    config = parser.parse_args()
    config.indent = ' ' * config.indent # change to spaces
    infile = config.infile[0]
    outfile = config.outfile
    if outfile is not None:
        with contextlib.suppress(FileNotFoundError):
            if os.path.samefile(infile, outfile):
                raise SystemExit(f'uxf.py:error:won\'t overwrite {outfile}')
    try:
        lint = config.lint or (config.lint is None and outfile is None)
        on_event = functools.partial(on_event, verbose=config.lint,
                                     filename=infile)
        uxo = load(infile, on_event=on_event, drop_unused=config.dropunused,
                   replace_imports=config.replaceimports)
        do_dump = outfile is not None
        outfile = sys.stdout if outfile == '-' else outfile
        on_event = functools.partial(on_event, verbose=config.lint,
                                     filename=outfile)
        if do_dump:
            format = Format(indent=config.indent,
                            wrap_width=config.wrapwidth)
            dump(outfile, uxo, on_event=on_event, format=format)
    except (OSError, Error) as err:
        message = str(err)
        if not message.startswith('uxf.py'):
            message = f'uxf.py:error:{message}'
        print(message, file=sys.stderr)
