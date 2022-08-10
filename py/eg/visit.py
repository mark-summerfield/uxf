#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

'''
This example shows a use-case for the Uxf.visit() method.

If you use it with testdata/t5.uxf you'll find it reveals that there is an
inconsistency in the data which would not otherwise be apparent.
'''

import functools
import pathlib
import sys

try:
    import uxf
except ImportError: # needed for development
    sys.path.append(str(pathlib.Path(__file__).parent.parent.resolve()))
    import uxf


def main():
    if len(sys.argv) < 3 or sys.argv[1] in {'-h', '--help'}:
        raise SystemExit('''usage: visit.py <infile.uxf> <outfile.txt|->
suggested infile: testdata/t5.uxf''')
    tracks = uxf.load(sys.argv[1])
    state = State()
    state_visitor = functools.partial(visitor, state=state)
    tracks.visit(state_visitor)
    print_tree(state.tree, sys.argv[2])


def print_tree(tree, outfile):
    out, close = get_outstream(outfile)
    try:
        for category in tree:
            out.write(f'{category}\n')
            for playlist in tree[category]:
                out.write(f'  {playlist}\n')
                for name, secs in tree[category][playlist]:
                    out.write(f'    {name} ({secs:.1f}s)\n')
    finally:
        if close:
            out.close()


def get_outstream(outfile):
    if outfile == '-':
        return sys.stdout, False
    return open(outfile, 'wt', encoding='utf-8'), True


def visitor(kind, value, *, state):
    if kind is uxf.VisitKind.TABLE_BEGIN:
        handle_table(state, value)
    elif kind is uxf.VisitKind.TABLE_END:
        state.in_categories = state.in_playlists = state.in_tracks = False
    elif kind is uxf.VisitKind.RECORD_BEGIN:
        state.record.clear()
    elif kind is uxf.VisitKind.RECORD_END:
        if state.in_categories:
            handle_category(state)
        elif state.in_playlists:
            handle_playlist(state)
        elif state.in_tracks:
            handle_track(state)
        state.record.clear()
    elif kind is uxf.VisitKind.VALUE:
        if value is not None:
            state.record.append(value)


def handle_table(state, value):
    state.in_categories = state.in_playlists = state.in_tracks = False
    if value.ttype == 'Categories':
        state.in_categories = True
    elif value.ttype == 'Playlists':
        state.in_playlists = True
    elif value.ttype == 'Tracks':
        state.in_tracks = True


def handle_category(state):
    if len(state.record) > 1:
        cid = state.record[CATEGORY_CID]
        title = state.record[CATEGORY_TITLE]
        state.tree[title] = {} # playlists for category
        state.category_for_cid[cid] = title


def handle_playlist(state):
    if len(state.record) > 2:
        pid = state.record[PLAYLIST_PID]
        title = state.record[PLAYLIST_TITLE]
        cid = state.record[PLAYLIST_CID]
        category = state.category_for_cid[cid]
        state.tree[category][title] = [] # tracks for playlist
        state.playlist_for_pid[pid] = title
        state.cid_for_pid[pid] = cid


def handle_track(state):
    if len(state.record) > 5:
        title = state.record[TRACK_TITLE]
        secs = state.record[TRACK_SECS]
        pid = state.record[TRACK_PID]
        if pid not in state.cid_for_pid:
            print(f'skipping track {state.record!r}: no playlist')
        else:
            cid = state.cid_for_pid[pid]
            playlist = state.playlist_for_pid[pid]
            category = state.category_for_cid[cid]
            state.tree[category][playlist].append((title, secs))


class State:

    def __init__(self):
        # self.tree keys are categories, values are dicts
        # these in turn have playlist keys and values are lists of tracks as
        # (title, secs) 2-tuples
        self.tree = {}
        self.category_for_cid = {}
        self.cid_for_pid = {}
        self.playlist_for_pid = {}
        self.in_categories = False
        self.in_playlists = False
        self.in_tracks = False
        self.record = []


CATEGORY_CID = 0
CATEGORY_TITLE = 1
PLAYLIST_PID = 0
PLAYLIST_TITLE = 1
PLAYLIST_CID = 2
TRACK_TITLE = 1
TRACK_SECS = 2
TRACK_PID = 5


if __name__ == '__main__':
    main()
