#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import sys
import os


try:
    PATH = os.path.abspath(os.path.dirname(__file__))
    sys.path.append(os.path.abspath(os.path.join(PATH, '../')))
    import uxf
    os.chdir(os.path.join(PATH, '../../testdata')) # move to test data
finally:
    pass


def on_error(lino, code, message, *, filename='-', fail=True,
             verbose=False):
    raise uxf.Error(f'uxf.py:{filename}:{lino}:#{code}:{message}')


def main():
    regression = False
    if len(sys.argv) > 1 and sys.argv[1] in {'-r', '--regression'}:
        regression = True
    total = ok = 0

    try:
        total += 1
        e = 100
        uxf.Uxf('data')
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 110
        uxf.loads('not a uxf file', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 120
        uxf.loads('uxf\n', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 130
        uxf.loads('UXF 1.0\n', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 141
        uxf.loads('uxf 9.0\n', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 151
        uxf.loads('uxf 1.0x\n', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 160
        uxf.loads('uxf 1.0\n# Not a comment', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 170
        uxf.loads('uxf 1.0\n* invalid', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 170
        uxf.loads('uxf 1.0\n[1 2 5_invalid]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 176 # see also test_imports.py
        uxf.load('i64.uxi', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 180
        uxf.loads('uxf 1.0\n[# 123]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 190
        uxf.loads('uxf 1.0\n[123 #<comment>]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 190
        uxf.loads('uxf 1.0\n{1 2 #<3> 4}', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 200
        uxf.loads('uxf 1.0\n[(:AB CD EF GH:)]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 220
        uxf.loads('uxf 1.0\n[7.8.9]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 220
        uxf.loads('uxf 1.0\n[2020-02-20T20e20]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 231
        uxf.loads('uxf 1.0\n[2020-02-20T20:20:20-07:31T]',
                  on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 270
        uxf.loads('uxf 1.0\n[(:AB 12:]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 290
        uxf.loads('uxf 1.0\n=p q\n{(p 1) 8}', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    table = uxf.table('Pair', ('first', 'second'))
    uxo = uxf.Uxf({})
    uxo.value['key'] = 'value'

    try:
        total += 1
        e = 290
        uxo.value._append(table)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 294
        uxf.loads('uxf 1.0\n{7.9 8}', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 294
        uxo.value._append(3.8)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 298
        uxf.Table(uxf.TClass(''), records=(1, 2))
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 300
        _ = uxf.Field('1st')
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 304
        _ = uxf.Field('int')
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 310
        _ = uxf.Field('$1st')
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 320
        _ = uxf.Table(records=(1, 2))
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 320
        t = uxf.Table(records=[1])
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 334
        _ = uxf.Table(uxf.TClass('test'), records=(1, 2))
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        uxf.loads('''uxf 1.0
=Fieldless
=Single field
[
  (Single)
  (Single 1)
  (Fieldless)
]''', on_error=on_error)
        ok += 1 # Should succeed
        total += 1
        e = 334
        uxf.loads('''uxf 1.0
=Fieldless
=Single field
[
  (Single)
  (Single 1)
  (Fieldless)
  (Fieldless 1)
]''', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 334
        _ = uxf.Table(uxf.TClass('test'), records=(1, 2))
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 334
        t = uxf.Table()
        t._append(1)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 334
        t = uxf.Table(uxf.TClass('t1'))
        t._append(1)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 334
        t = uxf.table('t1', ('a', 'b'))
        t.tclass.fields = []
        t._append(1)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 402
        uxf.loads('uxf 1.0\n(:AB:)', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 422
        uxf.load('i67.uxi', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 422
        uxf.load('i69.uxi', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 442
        uxf.loads('uxf 1.0\n{int T 5 <x>}', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 442
        uxf.loads('uxf 1.0\n{int p}', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 446
        uxf.loads('uxf 1.0\n[q]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 446
        uxf.loads('uxf 1.0\n[T 5]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 450
        uxf.loads('uxf 1.0\n(T 5)', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 450
        uxf.loads('uxf 1.0\n=T a\n=U b\n(T (u 1))', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 450
        uxf.loads('uxf 1.0\n(r)', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 456
        uxf.loads('''uxf 1.0
=p x:int y:int
=q a:real b:real
{str p
  <one> (#<ok> p 1 2 -3 4 5 6)
  <four> (#<wrong> q 8.1 -9.3)
  <five> (#<ok2> p -7 -6)
}''', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 460
        uxf.loads('uxf 1.0\n[-7F]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 460
        uxf.loads('uxf 1.0\n{p}', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 470
        uxf.loads('uxf 1.0\n[int real]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 480
        uxf.loads('uxf 1.0\n{int real str}', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 486
        uxf.loads('''uxf 1.0
=p x:int y:int
{str p <one> (#<ok> p 1 2 -3 4 5 6)
<three> (#<worse> p 11 -12 <-1> <13>)''', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 496
        uxf.loads('uxf 1.0\n=p x:real y:int\n(p 1 2.0)', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 498
        uxf.loads('uxf 1.0\n=p x:int y:real\n(p 1.0 2)', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 498
        uxf.loads('''uxf 1.0
=p x:int y:int
{str p <one> (#<ok> p 1 2 -3 4 5 6)
<two> (#<bad> p 7 -8 9.0 10)}''', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 498
        uxf.loads('''uxf 1.0
=p x:int y:int
{str p <one> (#<ok> p 1 2 -3 4 5 6)
<two> (#<bad> p 7 -8 9.0 10)}''', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 512
        uxo = uxf.loads('uxf 1.0\n[1 2 3}', on_error=on_error)
        print(uxo.dumps(on_error=on_error))
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 522
        uxf.loads('uxf 1.0\np a b\n(p 1 2)', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 524
        uxf.loads('uxf 1.0\nint', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 550
        uxf.loads('uxf 1.0\n!http://www.qtrac.eu/missing.uxf\n[]',
                  on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 560
        uxf.loads('uxf 1.0\n!system-missing\n[]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 561
        uxo = uxf.Uxf()
        uxo.value = [3+2j] # noqa: E226
        _ = uxo.dumps(on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try: # attempt to do circular import #1
        total += 1
        e = 580
        uxf.load('i65.uxi', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try: # attempt to do circular import #2
        total += 1
        e = 580
        uxo = uxf.load('i66.uxi', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    try:
        total += 1
        e = 586
        uxf.loads('uxf 1.0\n!missing.uxf\n[]', on_error=on_error)
        fail(f'test_errors • #{e} FAIL', regression)
    except uxf.Error as err:
        ok += got_error(e, err, regression)

    if not regression:
        result = 'OK' if total == ok else 'FAIL'
        print(f'{ok}/{total} {result}')
    else:
        print(f'total={total} ok={ok}')


def got_error(code, err, regression):
    err = str(err)
    code = f'#{code}:'
    if code not in err:
        fail(f'test_errors • expected {code} got, {err!r} FAIL',
             regression)
        return 0
    return 1


def fail(message, regression):
    if not regression:
        print(message)


if __name__ == '__main__':
    main()
