#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import datetime
import functools
import math
import os
import statistics
import sys
import time

try:
    PATH = os.path.abspath(os.path.dirname(__file__))
    sys.path.append(os.path.abspath(os.path.join(PATH, '../')))
    import uxf
    sys.path.append(os.path.abspath(os.path.join(PATH, '../eg/')))
    import eq
    sys.path.append(os.path.abspath(os.path.join(PATH, '../t/')))
    import gen
    os.chdir(os.path.join(PATH, '../../testdata')) # move to testdata
finally:
    pass


def main():
    scale = 7
    verbose = True
    for arg in sys.argv[1:]:
        if arg in {'-h', '--help'}:
            raise SystemExit('usage: benchmark.py [-q|--quiet] [scale]\n'
                             'default scale is 7 → ~1MB')
        if arg in {'-q', '--quiet'}:
            verbose = False
        elif uxf.isasciidigit(arg):
            scale = int(arg)
    on_event = functools.partial(uxf.on_event, verbose=False,
                                 prefix='benchmark')
    print(f'scale={scale} ', end='', flush=True)

    uxt1s = []
    for _ in range(scale):
        uxt1s.append(gen.generate(scale=scale))
    mean_bytes = round(statistics.fmean(len(x.encode()) for x in uxt1s) //
                       1000)
    print(f'~{mean_bytes:,} KB ', end='', flush=True)
    mean_lines = round(statistics.fmean(len(x.splitlines()) for x in uxt1s))
    print(f'~{mean_lines:,} lines ', end='', flush=True)

    t = time.monotonic()
    uxos = []
    for uxt1 in uxt1s:
        uxos.append(uxf.loads(uxt1, on_event=on_event))
    load_t = time.monotonic() - t
    print(f'load={load_t:.03f}s ', end='', flush=True)

    t = time.monotonic()
    uxt2s = []
    for uxo in uxos:
        uxt2s.append(uxo.dumps())
    dump_t = time.monotonic() - t

    total = load_t + dump_t
    print(f'dump={dump_t:0.03f}s (total={total:0.03f}s', end='')

    d = dict(drop_unused=True, replace_imports=True, on_event=on_event)
    ok = 0
    for i in range(scale):
        uxo1 = uxf.loads(uxt1s[i], **d)
        uxo2 = uxf.loads(uxt2s[i], **d)
        if eq.eq(uxo1, uxo2):
            ok += 1

    if ok == scale:
        filename, uxo = get_timings()
        unix = not sys.platform.startswith('win')
        timings = sum(1 for result in uxo.value if result.scale == scale and
                      result.unix == unix)
        print(f' timings={timings:,}) OK')
        record = uxo.value.RecordClass(scale, load_t, dump_t,
                                       datetime.datetime.now(), unix)
        post_process_result(filename, uxo, scale, record, verbose)
    else:
        print(') uxo1 != uxo2') # we don't save bad results


def get_timings():
    filename = os.path.abspath('../py/t/benchmark.uxf.gz')
    try:
        return filename, uxf.load(filename)
    except uxf.Error:
        return filename, uxf.loads('''uxf 1.0 benchmark.py timings
=Result scale:int load:real dump:real when:datetime unix:bool
(Result)
''')


def post_process_result(filename, uxo, scale, record, verbose):
    load_times = []
    dump_times = []
    for result in uxo.value:
        if result.scale == scale and result.unix == record.unix:
            load_times.append(result.load)
            dump_times.append(result.dump)
    uxo.value.append(record)
    while len(uxo.value.records) > 10000:
        uxo.value.records.pop(0)
    uxo.dump(filename, format=uxf.Format(realdp=3))
    if verbose and load_times:
        show_results('load', scale, record.load, load_times)
        show_results('dump', scale, record.dump, dump_times)


def show_results(what, scale, this, times):
    the_mean = statistics.fmean(times)
    the_min = min(times)
    the_max = max(times)
    c = char_for(this, the_min, the_mean, the_max)
    print(f'{what} min={the_min:.03f}s mean={the_mean:.03f}s '
          f'max={the_max:.03f}s ', end='')
    if scale == 1:
        print(f'this=each={this:.03f}s {c}')
    else:
        print(f'this={this:.03f}s each={this/scale:.03f}s {c}')


def char_for(this, min, mean, max):
    if this < min:
        return '✔✔'
    if this < mean:
        return '✔'
    if this > max:
        return '✖✖'
    if this > mean:
        return '✖'
    if math.isclose(this, mean):
        return '='
    return '~'


if __name__ == '__main__':
    main()
