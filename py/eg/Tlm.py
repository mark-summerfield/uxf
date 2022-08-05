#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

'''
An example of using UXF as both a "native" file format and as an exchange
format for importing and exporting. The main class, Tlm holds a track list
and a list of history strings. The Tlm can load and save in its own TLM
format, and also seamlessly, UXF format.

Loading needs ~60 lines for TLM and ~23 lines for UXF.
This is because the uxf module does most of the parsing.
Saving needs ~18 lines for TLM and for UXF.
'''

import collections
import contextlib
import enum
import gzip
import os
import pathlib
import sys

try:
    import mutagen
except ImportError:
    mutagen = None

try:
    import uxf
except ImportError: # needed for development
    sys.path.append(os.path.abspath(os.path.dirname(__file__) + '/..'))
    import uxf


def main():
    if len(sys.argv) != 3 or sys.argv[1] in {'-h', '--help'}:
        raise SystemExit('''
usage: Tlm.py <infile.{tlm,uxf,uxf.gz}> <outfile.{tlm,uxf,uxf.gz}>

Converts between TLM and UXF TLM formats.
e.g., Tlm.py tlm-eg.uxf test.tlm''')
    infilename = sys.argv[1]
    outfilename = sys.argv[2]
    with contextlib.suppress(FileNotFoundError):
        if os.path.samefile(infilename, outfilename):
            raise SystemExit('can\'t overwrite infile with outfile')
    model = Model(infilename)
    model.save(filename=outfilename)


class Error(Exception):
    pass


class Model:

    def __init__(self, filename=None):
        self.clear()
        self._filename = filename
        if self._filename is not None and os.path.exists(self._filename):
            self.load()


    @property
    def filename(self):
        return self._filename


    @filename.setter
    def filename(self, filename):
        self._filename = filename
        if os.path.exists(filename):
            self.load()


    def clear(self):
        self.tree = Group('')
        self.history = collections.deque()


    def load(self, filename=None):
        if filename is not None:
            self._filename = filename
        if not self._load_uxf():
            self._load_tlm()


    def _load_tlm(self):
        with open(self._filename, 'rb') as file:
            opener = (open if file.read(5) == TLM_MAGIC.encode() else
                      gzip.open)
        self.clear()
        stack = [self.tree]
        prev_indent = 0
        state = _State.WANT_MAGIC
        with opener(self._filename, 'rt', encoding='utf-8') as file:
            for lino, line in enumerate(file, 1):
                line = line.rstrip()
                if not line:
                    continue # ignore blank lines
                if state is _State.IN_TRACKS and line == '\fHISTORY':
                    state = _State.IN_HISTORY
                elif state is _State.WANT_MAGIC:
                    if not line.startswith(TLM_MAGIC):
                        raise Error(f'error:{lino}: not a .tlm file')
                    # We're ignoring the version
                    state = _State.WANT_TRACK_HEADER
                elif state is _State.WANT_TRACK_HEADER:
                    if line != '\fTRACKS':
                        raise Error(f'error:{lino}: missing TRACKS')
                    state = _State.IN_TRACKS
                elif state is _State.IN_TRACKS:
                    if line.startswith(INDENT):
                        prev_indent = self._read_group(stack, prev_indent,
                                                       lino, line)
                    elif not line.startswith('\f'):
                        self._read_track(stack[-1], lino, line)
                elif state is _State.IN_HISTORY:
                    self.history.append(line)
                else:
                    raise Error(f'error:{lino}: invalid .tlm file')


    def _read_group(self, stack, prev_indent, lino, line):
        name = line.lstrip(INDENT)
        indent = len(line) - len(name)
        group = Group(name)
        if indent == 1:
            self.tree.append(group)
            stack[:] = [self.tree, group]
        elif indent > prev_indent: # child
            stack[-1].append(group)
            stack.append(group)
        elif indent <= prev_indent: # same level or higher
            for _ in range(prev_indent - indent + 1):
                stack.pop() # move back up to same or higher parent
            stack[-1].append(group)
            stack.append(group)
        return indent


    def _read_track(self, group, lino, line):
        try:
            filename, secs = line.split('\t', maxsplit=1)
            group.append(Track(filename, float(secs)))
        except ValueError as err:
            raise Error(f'error:{lino}: failed to read track: {err}')


    def _load_uxf(self):
        try:
            uxo = uxf.load(self._filename)
            stack = [self.tree]
            for group, kids in uxo.value.items():
                if group == UXF_HISTORY:
                    for history in kids:
                        self.history.append(history)
                else:
                    self._populate_tree_from_uxo(stack, group, kids)
            return True
        except uxf.Error:
            return False


    def _populate_tree_from_uxo(self, stack, group, kids):
        parent = stack[-1]
        group = Group(group)
        parent.append(group)
        for name, value in kids.items():
            if isinstance(value, (dict, uxf.Map)):
                stack.append(group)
                self._populate_tree_from_uxo(stack, name, value)
                stack.pop()
            else:
                group.append(Track(name, value))


    def save(self, *, filename=None, compress=True):
        if filename is not None:
            self._filename = filename
        if self._filename.upper().endswith('.TLM'):
            self._save_as_tlm(compress)
        else:
            self._save_as_uxf()


    def _save_as_tlm(self, compress):
        opener = gzip.open if compress else open
        with opener(self._filename, 'wt', encoding='utf-8') as file:
            file.write('\fTLM\t100\n\fTRACKS\n')
            self._write_tree(file, self.tree)
            file.write('\fHISTORY\n')
            for history in self.history:
                file.write(f'{history}\n')


    def _write_tree(self, file, tree, depth=1):
        pad = depth * INDENT
        for kid in tree.kids:
            if isinstance(kid, Group):
                file.write(f'{pad}{kid.name}\n')
                self._write_tree(file, kid, depth + 1)
            else:
                file.write(f'{kid.filename}\t{kid.secs:.03f}\n')


    def _save_as_uxf(self):
        t_track = uxf.TClass('Track', (uxf.Field('filename', 'str'),
                                       uxf.Field('secs', 'real')))
        t_group = uxf.TClass('Group', (uxf.Field('name', 'str'),
                                       uxf.Field('items')))
        uxo = uxf.Uxf(custom='TLM 1.2')
        stack = uxo.value # root is List
        self._write_tree_uxf(stack, self.tree, t_track, t_group)
        # TODO add history as last item in root
        opener = (gzip.open if self._filename.upper().endswith('.GZ') else
                  open)
        with opener(self._filename, 'wt', encoding='utf-8') as file:
            file.write(uxo.dumps())


    def _write_tree_uxf(self, stack, tree, t_track, t_group):
        parent = stack[-1] if stack else stack
        track = None
        for kid in tree.kids:
            if isinstance(kid, Group):
                track = None
                group = uxf.Table(t_group)
                group.append((kid.name, []))
                parent.append(group)
                stack.append(group.last.items)
                self._write_tree_uxf(stack, kid, t_track, t_group)
                stack.pop()
            else:
                if track is None:
                    track = uxf.Table(t_track)
                    parent.append(track)
                track.append((kid.filename, kid.secs))


    # TODO DELETE
    def _save_as_uxf1(self):
        uxo = uxf.Uxf({}, custom='TLM 1.1')
        stack = [uxo.value] # root is Map
        self._write_tree_uxf1(stack, self.tree)
        uxo.value[UXF_HISTORY] = self.history
        opener = (gzip.open if self._filename.upper().endswith('.GZ') else
                  open)
        with opener(self._filename, 'wt', encoding='utf-8') as file:
            file.write(uxo.dumps())


    # TODO DELETE
    def _write_tree_uxf1(self, stack, tree):
        parent = stack[-1]
        for kid in tree.kids:
            if isinstance(kid, Group):
                child = parent[kid.name] = {}
                stack.append(child)
                self._write_tree_uxf(stack, kid)
                stack.pop()
            else:
                parent[kid.filename] = kid.secs


    def paths(self):
        for path in self._paths(self.tree, ''):
            yield path


    def _paths(self, tree, prefix):
        prefix = f'{prefix}/{tree.name}' if prefix else tree.name
        if prefix:
            yield prefix
        for kid in tree.kids:
            if isinstance(kid, Group):
                for path in self._paths(kid, prefix):
                    yield path
            elif kid.treename:
                yield f'{prefix}/{kid.treename}'


    def secs_for(self, tree=None):
        if tree is None:
            tree = self.tree
        return self._secs_for(tree)


    def _secs_for(self, tree):
        secs = 0.0
        for kid in tree.kids:
            if isinstance(kid, Group):
                secs += self._secs_for(kid)
            else:
                secs += kid.secs
        return secs


@enum.unique
class _State(enum.Enum):
    WANT_MAGIC = enum.auto()
    WANT_TRACK_HEADER = enum.auto()
    IN_TRACKS = enum.auto()
    IN_HISTORY = enum.auto()


class Group:

    def __init__(self, name):
        self.name = name
        self.kids = []


    def __repr__(self):
        return f'Group({self.name!r})'


    def subgroup(self, group_name):
        for kid in self.kids:
            if kid.name == group_name:
                return kid


    def append(self, group_or_track):
        self.kids.append(group_or_track)


class Track:

    def __init__(self, filename, secs):
        self._filename = filename
        self._title = None
        self._secs = secs
        self._album = None
        self._artist = None
        self._number = 0


    def __repr__(self):
        return f'Track({self.filename!r}, {self.secs:0.3f})'


    def _populate_metadata(self):
        if mutagen is None:
            return

        def get_meta_item(meta, name):
            try:
                return meta[name][0]
            except (KeyError, IndexError):
                pass

        try:
            meta = mutagen.File(self._filename)
            if meta is not None:
                self._title = get_meta_item(meta, 'title')
                self._secs = meta.info.length
                self._album = get_meta_item(meta, 'album')
                self._artist = get_meta_item(meta, 'artist')
                try:
                    self._number = int(meta['tracknumber'][0])
                except (IndexError, ValueError):
                    self._number = 0
                return
        except (mutagen.MutagenError, FileNotFoundError):
            pass
        if self._title is None:
            self._title = (
                os.path.splitext(os.path.basename(self._filename))[0]
                .replace('-', ' ').replace('_', ' '))


    @property
    def filename(self):
        return self._filename


    @property
    def treename(self):
        return (pathlib.Path(self.filename).stem.replace('-', ' ')
                .replace('_', ' ').lstrip('0123456789 '))


    @property
    def title(self):
        if self._title is None:
            self._populate_metadata()
        return self._title


    @property
    def secs(self):
        if self._secs <= 0:
            self._populate_metadata()
        return self._secs


    @property
    def album(self):
        if self._album is None:
            self._populate_metadata()
        return self._album


    @property
    def artist(self):
        if self._artist is None:
            self._populate_metadata()
        return self._artist


    @property
    def number(self):
        if self._number == 0:
            self._populate_metadata()
        return self._number


TLM_MAGIC = '\fTLM\t'
INDENT = '\v'
UXF_HISTORY = '__HISTORY__'


if __name__ == '__main__':
    main()
