#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import gzip
import os
import sqlite3
from xml.sax.saxutils import escape


def main():
    infile = 'playlists-epd.uxf'
    with sqlite3.connect(os.path.expanduser('~/data/playlists.epd')) as db:
        cursor = db.cursor()
        with gzip.open(infile, 'wt', encoding='utf-8') as out:
            out.write('uxf 1 EPD (SQLite)\n')
            out.write('= Categories CID Title Selected\n')
            out.write('= Playlists PID Title CID Selected\n')
            out.write('= Tracks TID Title Seconds Filename Selected PID\n')
            out.write('[\n') # simple list of SQL tables
            out.write('  (Categories\n')
            for cid, title, selected in cursor.execute(
                    'SELECT cid, title, selected FROM categories '
                    'ORDER BY title'):
                selected = 'yes' if selected else 'no'
                out.write(f'    {cid} <{escape(title)}> {selected}\n')
            out.write('  )\n') # end of categories
            out.write('  (Playlists\n')
            for pid, title, cid, selected in cursor.execute(
                    'SELECT pid, title, cid, selected FROM playlists '
                    'ORDER BY title'):
                selected = 'yes' if selected else 'no'
                out.write(
                    f'    {pid} <{escape(title)}> {cid} {selected}\n')
            out.write('  )\n') # end of playlists
            out.write('  (Tracks\n')
            for tid, title, seconds, filename, selected, pid in (
                    cursor.execute(
                        'SELECT tid, title, seconds, filename, selected, '
                        'pid FROM tracks ORDER BY pid, title')):
                selected = 'yes' if selected else 'no'
                out.write(
                    f'    {tid} <{escape(title)}> {seconds:.1f} '
                    f'<{escape(filename)}> {selected} {pid}\n')
            out.write('  )\n') # end of tracks
            out.write(']\n') # end of simple list of SQL tables
    print('wrote', infile)


if __name__ == '__main__':
    main()
