#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

'''
This reads slides.sld and outputs slides/index.html and slides/N.html where
N is a slide number.

This program is just an illustration of the flexibility of the UXF format.
It also shows how even an empty table can be useful (e.g., the nl tclass).
'''

import base64
import os
import shutil
import sys
from xml.sax.saxutils import escape

try:
    import uxf
except ImportError: # needed for development
    sys.path.append(os.path.abspath(os.path.dirname(__file__) + '/..'))
    import uxf


def main():
    infile, outdir = get_args()
    shutil.rmtree(outdir, ignore_errors=True)
    os.mkdir(outdir)
    uxo = uxf.load(infile)
    titles = []
    slides = uxo.value
    for index, slide in enumerate(slides, 1):
        titles.append(write_slide(outdir, index, slide, len(slides)))
    index += 1
    titles.append(write_uxf_source(outdir, index, infile))
    index += 1
    titles.append(write_py_source(outdir, index))
    write_index(outdir, titles)


def get_args():
    if len(sys.argv) < 3 or sys.argv[1] in {'-h', '--help'}:
        raise SystemExit('usage: slides.py <infile.sld> <outdir>]')
    infile = sys.argv[1]
    outdir = sys.argv[2]
    return infile, outdir


def write_slide(outdir, index, slide, last):
    parts = ['<html><title>']
    doc_title = title = html_for_block(slide[0])
    while len(doc_title) > 1:
        doc_title = doc_title[1:-1]
    parts += doc_title
    parts.append('</title><body>')
    parts += title
    for block in slide[1:]:
        parts += html_for_block(block)
    parts.append(f'<a href="{index - 1}.html">Prev</a>' if index > 1 else
                 '<a href="index.html">Prev</a>')
    parts.append('&nbsp;<a href="index.html">Contents</a>&nbsp;')
    parts.append(f'<a href="{index + 1}.html">Next</a>' if index != last
                 else '<font color="gray">Next</font>')
    parts.append('</body></html>')
    with open(f'{outdir}/{index}.html', 'wt', encoding='utf-8') as file:
        file.write('\n'.join(parts))
    return escape(doc_title[0])


def html_for_block(block):
    if isinstance(block, str):
        return [escape(block)]
    if isinstance(block, uxf.List):
        parts = []
        for value in block:
            parts += html_for_block(value)
        return parts
    # ∴ must be a Table
    parts = []
    end = None
    if block.ttype == 'B':
        parts.append('<ul><li>')
        end = '</li></ul>'
    elif block.ttype in {'h1', 'h2'}:
        parts.append(f'<{block.ttype}>')
        end = f'</{block.ttype}>'
    elif block.ttype == 'i':
        parts.append('<i>')
        end = '</i>'
    elif block.ttype == 'img':
        record = block[0]
        data = base64.urlsafe_b64encode(record.image).decode('ascii')
        parts.append(f'<img src="data:image/png;base64,{data}" />')
        parts += html_for_block(record.content)
        if end is not None:
            parts.append(end)
        return parts
    elif block.ttype == 'm':
        parts.append('<tt>')
        end = '</tt>'
    elif block.ttype == 'nl':
        parts.append('<br />')
    elif block.ttype == 'p':
        parts.append('<p>')
        end = '</p>'
    elif block.ttype == 'pre':
        parts.append('<pre>')
        end = '</pre>'
    elif block.ttype == 'url':
        record = block[0]
        parts.append(f'<a href="{record.link}">')
        parts += html_for_block(record.content)
        end = '</a>'
        if end is not None:
            parts.append(end)
        return parts
    for record in block:
        parts += html_for_block(record.content)
    if end is not None:
        parts.append(end)
    return parts


def write_uxf_source(outdir, index, infile):
    with open(infile, 'rt', encoding='utf-8') as file:
        text = file.read()
    title = escape(os.path.basename(infile))
    with open(f'{outdir}/{index}.html', 'wt', encoding='utf-8') as file:
        file.write(f'<html><title>{title}</title><body>\n<h1>{title}</h1>')
        file.write(f'<pre>\n{escape(text)}\n</pre>')
    return title


def write_py_source(outdir, index):
    with open(__file__, 'rt', encoding='utf-8') as file:
        text = file.read()
    title = escape(os.path.basename(__file__))
    with open(f'{outdir}/{index}.html', 'wt', encoding='utf-8') as file:
        file.write(f'<html><title>{title}</title><body>\n<h1>{title}</h1>')
        file.write(f'<pre>\n{escape(text)}\n</pre>')
    return title


def write_index(outdir, titles):
    with open(f'{outdir}/index.html', 'wt', encoding='utf-8') as file:
        title = escape(titles[0])
        file.write(
            f'<html><title>{title}</title><body>\n<h1>{title}</h1><ol>')
        for i, title in enumerate(titles, 1):
            file.write(f'<li><a href="{i}.html">{title}</a></li>\n')
        file.write('</ol></body></html>')


if __name__ == '__main__':
    main()
