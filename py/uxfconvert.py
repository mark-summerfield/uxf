#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

'''
A command line tool providing various conversions to/from UXF format.

If used as a module this also provides a public API:
    uxf_to_csv(infile, outfile)
    csv_to_uxf(infile, outfile, fieldnames=False)
    multi_csv_to_uxf(infiles, outfile, fieldnames=False):
    uxf_to_json(infile, outfile)
    json_to_uxf(infile, outfile)
    ini_to_uxf(infile, outfile)
    uxf_to_sqlite(infile, outfile)
    sqlite_to_uxf(infile, outfile)
    uxf_to_xml(infile, outfile)
    xml_to_uxf(infile, outfile)
'''

import argparse
import collections
import configparser
import csv
import datetime
import functools
import json
import pathlib
import shutil
import sqlite3
import sys
import textwrap
import xml.dom.minidom
import xml.sax
import xml.sax.handler

import uxf


def main():
    converter = _get_converter()
    try:
        converter()
    except (OSError, uxf.Error) as err:
        print(f'uxfconvert:error:{err}', file=sys.stderr)


def _get_converter():
    parser = argparse.ArgumentParser(usage=PREFIX + _get_usage())
    parser.add_argument('-d', '--dropunused', action='store_true',
                        help='drop unused imports and ttypes')
    parser.add_argument('-r', '--replaceimports', action='store_true',
                        help='replace imports with their used ttypes')
    parser.add_argument('-i', '--indent', type=int, default=2,
                        help='indent (0-8; default 2)')
    parser.add_argument('-w', '--wrapwidth', type=int, default=96,
                        help='wrapwidth (40-240; default 96)')
    parser.add_argument('-v', '--verbose', action='store_true')
    parser.add_argument(
        '-f', '--fieldnames', action='store_true',
        help='if set the first row of csv file(s) is read as field names; '
        'default: all rows are values')
    parser.add_argument('file', nargs='+',
                        help='infile(s) and outfile as shown above')
    config = parser.parse_args()
    if 0 <= config.indent <= 9:
        config.indent = (' ' * config.indent) if config.indent < 9 else '\t'
    return _prepare_converter(parser, config)


def _get_usage():
    try:
        term_width = shutil.get_terminal_size()[0]
    except AttributeError:
        term_width = 80
    return '\n\n'.join('\n'.join(textwrap.wrap(para.strip(), term_width))
                       for para in USAGE.strip().split('\n\n')
                       if para.strip())


def _prepare_converter(parser, config):
    if len(config.file) < 2:
        parser.error('least two filenames are required')
    config.convert = None
    if len(config.file) > 2:
        converter = _get_mulit_csv_converter(parser, config)
    else:
        converter = _get_other_converter(parser, config)
    if converter is None:
        parser.error('cannot perform the requested conversion')
    return converter


def _get_mulit_csv_converter(parser, config):
    if not config.file[-1].upper().endswith((DOT_UXF, DOT_UXF_GZ)):
        parser.error('multiple infiles may only be converted to .uxf')
    for name in config.file[:-1]:
        if not name.upper().endswith(DOT_CSV):
            parser.error('multiple infiles may only be .csv files')
    return lambda: multi_csv_to_uxf(
        config.file[:-1], config.file[-1], fieldnames=config.fieldnames,
        indent=config.indent, wrap_width=config.wrapwidth,
        verbose=config.verbose)


def _get_other_converter(parser, config):
    infile = config.file[0]
    uinfile = infile.upper()
    outfile = config.file[-1]
    uoutfile = outfile.upper()
    if (uinfile.endswith((DOT_UXF, DOT_UXF_GZ)) and
            uoutfile.endswith((DOT_UXF, DOT_UXF_GZ))):
        parser.error('cannot convert uxf to uxf, instead use: '
                     'python3 -m uxf infile.uxf outfile.uxf')
    kwargs = dict(verbose=config.verbose, drop_unused=config.dropunused,
                  replace_imports=config.replaceimports,
                  indent=config.indent, wrap_width=config.wrapwidth)
    if not uoutfile.endswith((DOT_CSV, DOT_INI, DOT_JSN, DOT_JSON,
                              DOT_SQLITE, DOT_XML)):
        if uinfile.endswith(DOT_CSV):
            return lambda: csv_to_uxf(infile, outfile,
                                      fieldnames=config.fieldnames,
                                      **kwargs)
        elif uinfile.endswith(DOT_INI):
            return lambda: ini_to_uxf(infile, outfile, **kwargs)
        elif uinfile.endswith((DOT_JSN, DOT_JSON)):
            return lambda: json_to_uxf(infile, outfile, **kwargs)
        elif uinfile.endswith(DOT_SQLITE):
            return lambda: sqlite_to_uxf(infile, outfile, **kwargs)
        elif uinfile.endswith(DOT_XML):
            return lambda: xml_to_uxf(infile, outfile, **kwargs)
    elif not uinfile.endswith((DOT_CSV, DOT_INI, DOT_JSN, DOT_JSON,
                               DOT_SQLITE, DOT_XML)):
        if uoutfile.endswith(DOT_CSV):
            return lambda: uxf_to_csv(infile, outfile, **kwargs)
        elif uoutfile.endswith((DOT_JSN, DOT_JSON)):
            return lambda: uxf_to_json(infile, outfile, **kwargs)
        elif uoutfile.endswith(DOT_SQLITE):
            return lambda: uxf_to_sqlite(infile, outfile, **kwargs)
        elif uoutfile.endswith(DOT_XML):
            return lambda: uxf_to_xml(infile, outfile, **kwargs)
    parser.error(f'cannot convert {infile} to {outfile}')


def uxf_to_csv(infile, outfile, *, verbose=True, replace_imports=False,
               drop_unused=False, indent='  ', wrap_width=96):
    on_event = functools.partial(uxf.on_event, verbose=verbose,
                                 prefix='uxfconvert')
    uxo = uxf.load(infile, on_event=on_event, drop_unused=drop_unused,
                   replace_imports=replace_imports)
    value = uxo.value
    if isinstance(value, uxf.Table):
        with open(outfile, 'w', encoding=UTF8, newline='') as file:
            writer = csv.writer(file, quoting=csv.QUOTE_NONNUMERIC)
            writer.writerow((field.name for field in value.fields))
            for row in value:
                writer.writerow(row)
    elif (isinstance(value, (list, uxf.List)) and value and
            isinstance(value[0], (list, uxf.List)) and value[0] and not
            isinstance(value[0][0], (dict, list, uxf.Map, uxf.List,
                                     uxf.Table))):
        with open(outfile, 'w', encoding=UTF8, newline='') as file:
            writer = csv.writer(file, quoting=csv.QUOTE_NONNUMERIC)
            for row in value:
                writer.writerow(row)
    else:
        raise SystemExit('can only convert a UXF containing a single table '
                         'or a single list of lists of scalars to csv')


def csv_to_uxf(infile, outfile, *, fieldnames=False, verbose=True,
               replace_imports=False, drop_unused=False, indent='  ',
               wrap_width=96):
    on_event = functools.partial(uxf.on_event, verbose=verbose,
                                 prefix='uxfconvert')
    value, filename, tclasses = _read_csv_to_data(infile, fieldnames)
    uxf.dump(outfile, uxf.Uxf(value, custom=filename, tclasses=tclasses),
             on_event=on_event,
             format=uxf.Format(indent=indent, wrap_width=wrap_width))


def _read_csv_to_data(infile, fieldnames):
    tclasses = {}
    data = None
    with open(infile, encoding=UTF8, newline='') as file:
        reader = csv.reader(file)
        for row in reader:
            if data is None:
                if fieldnames:
                    ttype = uxf.canonicalize(pathlib.Path(infile).stem)
                    data = uxf.table(ttype,
                                     [uxf.Field(name) for name in row])
                    tclasses[ttype] = data.tclass
                    continue
                else:
                    data = []
            data.append([uxf.naturalize(x) for x in row])
    return data, infile, tclasses


def multi_csv_to_uxf(infiles, outfile, *, fieldnames=False, verbose=True,
                     replace_imports=False, drop_unused=False, indent='  ',
                     wrap_width=96):
    data = []
    tclasses = {}
    for infile in infiles:
        datum, _, new_tclasses = _read_csv_to_data(infile, fieldnames)
        data.append(datum)
        if new_tclasses:
            tclasses.update(new_tclasses)
    on_event = functools.partial(uxf.on_event, verbose=verbose,
                                 prefix='uxfconvert')
    uxf.dump(outfile,
             uxf.Uxf(data, custom=' '.join(infiles), tclasses=tclasses),
             on_event=on_event,
             format=uxf.Format(indent=indent, wrap_width=wrap_width))


def uxf_to_json(infile, outfile, *, verbose=True, replace_imports=False,
                drop_unused=False, indent='  ', wrap_width=96):
    on_event = functools.partial(uxf.on_event, verbose=verbose,
                                 prefix='uxfconvert')
    uxo = uxf.load(infile, on_event=on_event, drop_unused=drop_unused,
                   replace_imports=replace_imports)
    d = {}
    if uxo.custom is not None:
        d[JSON_CUSTOM] = uxo.custom
    if uxo.comment is not None:
        d[JSON_COMMENT] = uxo.comment
    if uxo.imports:
        d[JSON_IMPORTS] = list(uxo.import_filenames)
    if uxo.tclasses: # sorted helps round-trip regression tests
        d[JSON_TCLASSES] = sorted(uxo.tclasses.values(),
                                  key=lambda t: t.ttype)
    d[JSON_DATA] = uxo.value
    with open(outfile, 'wt', encoding=UTF8) as file:
        json.dump(d, file, cls=_JsonEncoder, indent=len(indent))


class _JsonEncoder(json.JSONEncoder):

    def default(self, obj):
        if isinstance(obj, datetime.datetime):
            return {JSON_DATETIME: uxf.isoformat(obj)}
        if isinstance(obj, datetime.date):
            return {JSON_DATE: uxf.isoformat(obj)}
        if isinstance(obj, (bytes, bytearray)):
            return {JSON_BYTES: obj.hex().upper()}
        if isinstance(obj, (list, uxf.List)):
            comment = getattr(obj, COMMENT, None)
            vtype = getattr(obj, VTYPE, None)
            if comment is None and vtype is None:
                return list(obj)
            return {JSON_LIST: dict(comment=comment, vtype=vtype,
                                    list=list(obj))}
        if isinstance(obj, uxf.Map):
            return _json_encode_map(obj)
        if isinstance(obj, dict):
            return obj
        if isinstance(obj, uxf.Table):
            return {JSON_TABLE: dict(
                name=obj.ttype, comment=obj.comment,
                fields={field.name: field.vtype for field in obj.fields},
                records=obj.records)}
        if isinstance(obj, uxf.TClass):
            d = dict(name=obj.ttype)
            if obj.fields:
                d['fields'] = {field.name: field.vtype
                               for field in obj.fields}
            if obj.comment is not None:
                d[COMMENT] = obj.comment
            return {JSON_TCLASS: d}
        return json.JSONEncoder.default(self, obj)


def _json_encode_map(obj):
    comment = getattr(obj, COMMENT, None)
    ktype = getattr(obj, KTYPE, None)
    vtype = getattr(obj, VTYPE, None)
    d = {}
    itypes = {}
    for key, value in obj.items():
        if isinstance(key, (datetime.date, datetime.datetime)):
            skey = uxf.isoformat(key)
            itypes[skey] = UXF
        elif isinstance(key, int):
            skey = str(key)
            itypes[skey] = UXF
        elif isinstance(key, (bytes, bytearray)):
            skey = key.hex().upper()
            itypes[skey] = BYTES
        elif isinstance(key, str):
            skey = key
        else:
            raise SystemExit(f'invalid map key type: {key} of {type(key)}')
        d[skey] = value
    if not itypes and comment is None and ktype is None:
        return dict(obj)
    m = dict(comment=comment, ktype=ktype, vtype=vtype, map=d)
    if len(itypes) == len(d) and len(set(itypes.values())) == 1:
        m[ITYPE] = itypes.popitem()[1] # all use same non-str key
    elif itypes:
        m[ITYPES] = itypes
    return {JSON_MAP: m}


def json_to_uxf(infile, outfile, *, verbose=True, replace_imports=False,
                drop_unused=False, indent='  ', wrap_width=96):
    with open(infile, 'rt', encoding=UTF8) as file:
        d = json.load(file, object_hook=_json_naturalize)
    custom = d.get(JSON_CUSTOM)
    comment = d.get(JSON_COMMENT)
    imports_list = d.get(JSON_IMPORTS)
    imports = None if imports_list is None else _get_imports(imports_list)
    tclasses = None
    tclass_list = d.get(JSON_TCLASSES)
    if tclass_list:
        tclasses = {}
        for tclass in tclass_list:
            tclasses[tclass.ttype] = tclass
    data = d.get(JSON_DATA)
    on_event = functools.partial(uxf.on_event, verbose=verbose,
                                 prefix='uxfconvert')
    uxo = uxf.Uxf(data, custom=custom, tclasses=tclasses, comment=comment)
    uxo.imports = imports
    uxf.dump(outfile, uxo, on_event=on_event,
             format=uxf.Format(indent=indent, wrap_width=wrap_width))


def _json_naturalize(d):
    if JSON_DATETIME in d:
        return uxf.naturalize(d[JSON_DATETIME])
    if JSON_DATE in d:
        return uxf.naturalize(d[JSON_DATE])
    if JSON_BYTES in d:
        return bytes.fromhex(d[JSON_BYTES])
    if JSON_LIST in d:
        jlist = d[JSON_LIST]
        return uxf.List(jlist[LIST], vtype=jlist[VTYPE],
                        comment=jlist[COMMENT])
    if JSON_MAP in d:
        jmap = d[JSON_MAP]
        itype = jmap.get(ITYPE) # str or None
        itypes = jmap.get(ITYPES) # dict or None
        m = uxf.Map(ktype=jmap[KTYPE], vtype=jmap[VTYPE],
                    comment=jmap[COMMENT])
        for key, value in jmap[MAP].items():
            if itypes is not None:
                itype = itypes.get(key)
            if itype == BYTES:
                key = bytes.fromhex(key)
            else:
                key = key if itype is None else uxf.naturalize(key)
            m[key] = value
        return m
    if JSON_TABLE in d:
        jtable = d[JSON_TABLE]
        fields = [uxf.Field(name, vtype) for name, vtype in
                  jtable[FIELDS].items()]
        return uxf.Table(uxf.TClass(jtable[NAME], fields),
                         comment=jtable[COMMENT], records=jtable[RECORDS])
    if JSON_TCLASS in d:
        fields = d[JSON_TCLASS].get(FIELDS)
        if fields is not None:
            fields = [uxf.Field(name, vtype)
                      for name, vtype in fields.items()]
        return uxf.TClass(d[JSON_TCLASS][NAME], fields,
                          comment=d[JSON_TCLASS].get(COMMENT))
    return d


def ini_to_uxf(infile, outfile, *, verbose=True, replace_imports=False,
               drop_unused=False, indent='  ', wrap_width=96):
    on_event = functools.partial(uxf.on_event, verbose=verbose,
                                 prefix='uxfconvert')
    ini = configparser.ConfigParser()
    ini.read(infile)
    data = uxf.Map()
    for section in ini:
        d = ini[section]
        if d:
            m = data[section] = uxf.Map()
            for key, value in d.items():
                m[uxf.naturalize(key)] = uxf.naturalize(value)
    uxf.dump(outfile, uxf.Uxf(data, custom=infile), on_event=on_event,
             format=uxf.Format(indent=indent, wrap_width=wrap_width))


def uxf_to_sqlite(infile, outfile, *, verbose=True, replace_imports=False,
                  drop_unused=False, indent='  ', wrap_width=96):
    on_event = functools.partial(uxf.on_event, verbose=verbose,
                                 prefix='uxfconvert')
    uxo = uxf.load(infile, on_event=on_event, drop_unused=drop_unused,
                   replace_imports=replace_imports)
    if isinstance(uxo.value, uxf.Table):
        _uxf_to_sqlite(outfile, [uxo.value])
    elif (isinstance(uxo.value, (list, uxf.List)) and uxo.value and
            all(isinstance(v, uxf.Table) for v in uxo.value)):
        _uxf_to_sqlite(outfile, uxo.value)
    else:
        raise SystemExit('can only convert a UXF containing a single table '
                         'or a single list of Tables to SQLite')


def _uxf_to_sqlite(filename, tables):
    sqlite3.register_adapter(bool, lambda b: 'TRUE' if b else 'FALSE')
    sqlite3.register_adapter(datetime.date, lambda d: uxf.isoformat(d))
    sqlite3.register_adapter(datetime.datetime, lambda d: uxf.isoformat(d))
    db = None
    try:
        db = sqlite3.connect(filename)
        for table_index, table in enumerate(tables, 1):
            ttype = _create_table(db, table, table_index)
            _populate_table(db, table, ttype)
    finally:
        if db is not None:
            db.commit()
            db.close()


def _create_table(db, table, table_index):
    ttype = uxf.canonicalize(table.ttype)
    sql = ['CREATE TABLE IF NOT EXISTS ', ttype, ' (']
    types = ['TEXT'] * len(table.fields)
    if table.records:
        for i, value in enumerate(table.records[0]):
            if isinstance(value, float):
                types[i] = 'REAL'
            elif isinstance(value, bool):
                name = table.fields[i].name
                types[i] = f"TEXT CHECK({name} IN ('TRUE', 'FALSE'))"
            elif isinstance(value, int):
                types[i] = 'INT'
            elif isinstance(value, (bytes, bytearray)):
                types[i] = 'BLOB'
    sep = ''
    for i, field in enumerate(table.fields):
        sql.append(f'{sep}[{field.name}] {types[i]}')
        sep = ', '
    sql += [');']
    cursor = db.cursor()
    cursor.execute(''.join(sql))
    return ttype


def _populate_table(db, table, ttype):
    sql = ['INSERT INTO ', ttype, ' VALUES (',
           ', '.join('?' * len(table.fields)), ');']
    cursor = db.cursor()
    cursor.executemany(''.join(sql), table.records)


def sqlite_to_uxf(infile, outfile, *, verbose=True, replace_imports=False,
                  drop_unused=False, indent='  ', wrap_width=96):
    uxo = _sqlite_to_uxf(infile)
    on_event = functools.partial(uxf.on_event, verbose=verbose,
                                 prefix='uxfconvert')
    uxo.dump(outfile, on_event=on_event,
             format=uxf.Format(indent=indent, wrap_width=wrap_width))


def _sqlite_to_uxf(infile):
    db = None
    try:
        db = sqlite3.connect(infile)
        uxo = uxf.Uxf()
        ttypes = []
        comments = []
        cursor = db.cursor()
        sql = ("SELECT tbl_name, sql FROM sqlite_master "
               "WHERE type = 'table';")
        for ttype, comment in cursor.execute(sql):
            ttypes.append(ttype)
            comments.append(comment)
        for ttype, comment in zip(ttypes, comments):
            fields = []
            sql = 'SELECT name FROM pragma_table_info(?)'
            for row in cursor.execute(sql, (ttype,)):
                fields.append(row[0])
            table = uxf.table(ttype, fields, comment=comment)
            fields = [f'[{field}]' for field in fields]
            sql = f'SELECT {", ".join(fields)} FROM {ttype};'
            for row in cursor.execute(sql):
                table.append(
                    [uxf.naturalize(value) if isinstance(value, str) else
                     value for value in row])
            uxo.tclasses[table.ttype] = table.tclass
            uxo.value.append(table)
        return uxo
    finally:
        if db is not None:
            db.close()


def uxf_to_xml(infile, outfile, *, verbose=True, replace_imports=False,
               drop_unused=False, indent='  ', wrap_width=96):
    on_event = functools.partial(uxf.on_event, verbose=verbose,
                                 prefix='uxfconvert')
    uxo = uxf.load(infile, on_event=on_event, drop_unused=drop_unused,
                   replace_imports=replace_imports)
    _uxf_to_xml(uxo, outfile, indent=indent)


def _uxf_to_xml(uxo, outfile, indent):
    dom = xml.dom.minidom.getDOMImplementation()
    tree = dom.createDocument(None, 'uxf', None)
    root = tree.documentElement
    root.setAttribute('version', str(uxf.VERSION))
    if uxo.custom:
        root.setAttribute('custom', uxo.custom)
    if uxo.comment:
        _xml_add_comment(tree, root, uxo.comment)
    if uxo.imports:
        _xml_add_imports(tree, root, uxo.import_filenames)
    if uxo.tclasses:
        _xml_add_tclasses(tree, root, uxo.tclasses)
    _xml_add_value(tree, root, uxo.value)
    with open(outfile, 'wt', encoding=UTF8) as file:
        file.write(tree.toprettyxml(indent=indent))


def _xml_add_comment(tree, root, comment):
    element = tree.createElement('comment')
    if '\n' in comment and ']]>' not in comment:
        text_element = tree.createCDATASection(comment)
    else:
        text_element = tree.createTextNode(comment)
    element.appendChild(text_element)
    root.appendChild(element)


def _xml_add_imports(tree, root, import_filenames):
    imports_element = tree.createElement('imports')
    for filename in import_filenames: # don't sort!
        import_element = tree.createElement('import')
        import_element.setAttribute('filename', filename)
        imports_element.appendChild(import_element)
    root.appendChild(imports_element)


def _xml_add_tclasses(tree, root, tclasses):
    tclasses_element = tree.createElement('ttypes')
    for tclass in sorted(tclasses.values()):
        tclass_element = tree.createElement('ttype')
        tclass_element.setAttribute('name', tclass.ttype)
        if tclass.comment:
            _xml_add_comment(tree, tclass_element, tclass.comment)
        for field in tclass.fields:
            field_element = tree.createElement('field')
            field_element.setAttribute('name', field.name)
            if field.vtype is not None:
                field_element.setAttribute('vtype', field.vtype)
            tclass_element.appendChild(field_element)
        tclasses_element.appendChild(tclass_element)
    root.appendChild(tclasses_element)


def _xml_add_value(tree, root, value):
    if value.__class__.__name__.startswith('UXF_'):
        _xml_add_list(tree, root, value, tag='row')
    elif isinstance(value, (set, frozenset, tuple, collections.deque, list,
                            uxf.List)):
        _xml_add_list(tree, root, value)
    elif isinstance(value, (dict, uxf.Map)):
        _xml_add_map(tree, root, value)
    elif isinstance(value, uxf.Table):
        _xml_add_table(tree, root, value)
    else:
        _xml_add_scalar(tree, root, value)


def _xml_add_list(tree, root, lst, *, tag='list'):
    list_element = tree.createElement(tag)
    vtype = getattr(lst, 'vtype', None)
    if vtype is not None:
        list_element.setAttribute('vtype', vtype)
    comment = getattr(lst, 'comment', None)
    if comment is not None:
        _xml_add_comment(tree, list_element, comment)
    for value in lst:
        _xml_add_value(tree, list_element, value)
    root.appendChild(list_element)


def _xml_add_map(tree, root, map):
    map_element = tree.createElement('map')
    ktype = getattr(map, 'ktype', None)
    if ktype is not None:
        map_element.setAttribute('ktype', ktype)
    vtype = getattr(map, 'vtype', None)
    if vtype is not None:
        map_element.setAttribute('vtype', vtype)
    comment = getattr(map, 'comment', None)
    if comment is not None:
        _xml_add_comment(tree, map_element, comment)
    for key, value in map.items():
        key_element = tree.createElement('key')
        _xml_add_value(tree, key_element, key)
        map_element.appendChild(key_element)
        value_element = tree.createElement('value')
        _xml_add_value(tree, value_element, value)
        map_element.appendChild(value_element)
    root.appendChild(map_element)


def _xml_add_table(tree, root, table):
    table_element = tree.createElement('table')
    table_element.setAttribute('name', table.ttype)
    if table.comment is not None:
        _xml_add_comment(tree, table_element, table.comment)
    for value in table:
        _xml_add_value(tree, table_element, value)
    root.appendChild(table_element)


def _xml_add_scalar(tree, root, value):
    element = None
    if value is None:
        element = tree.createElement('null')
    elif isinstance(value, bool):
        element = tree.createElement('yes' if value else 'no')
    elif isinstance(value, int):
        element = tree.createElement('int')
        element.setAttribute('v', str(value))
    elif isinstance(value, float):
        element = tree.createElement('real')
        element.setAttribute('v', str(value))
    elif isinstance(value, datetime.datetime):
        element = tree.createElement('datetime')
        element.setAttribute('v', uxf.isoformat(value))
    elif isinstance(value, datetime.date):
        element = tree.createElement('date')
        element.setAttribute('v', uxf.isoformat(value))
    elif isinstance(value, str):
        element = tree.createElement('str')
        if '\n' in value and ']]>' not in value:
            text_element = tree.createCDATASection(value)
        else:
            text_element = tree.createTextNode(value)
        element.appendChild(text_element)
    elif isinstance(value, (bytes, bytearray)):
        element = tree.createElement('bytes')
        text_element = tree.createTextNode(value.hex().upper())
        element.appendChild(text_element)
    if element is not None:
        root.appendChild(element)
    else:
        raise SystemExit(f'invalid value type: {value} of {type(value)}')


def xml_to_uxf(infile, outfile, *, verbose=True, replace_imports=False,
               drop_unused=False, indent='  ', wrap_width=96):
    on_event = functools.partial(uxf.on_event, verbose=verbose,
                                 prefix='uxfconvert')
    uxo = _xml_to_uxf(infile)
    uxo.dump(outfile, on_event=on_event,
             format=uxf.Format(indent=indent, wrap_width=wrap_width))


def _xml_to_uxf(infile):
    handler = _UxfSaxHandler()
    parser = xml.sax.make_parser()
    parser.setContentHandler(handler)
    parser.parse(infile)
    return handler.uxo


class _UxfSaxHandler(xml.sax.handler.ContentHandler):

    def __init__(self):
        super().__init__()
        self.stack = None
        self.tclass = None
        self.imports_list = []
        self.incomment = False
        self.instr = False
        self.inbytes = False
        self.intclass = False
        self.comment = ''
        self.string = ''
        self.bytes = ''
        self.uxo = uxf.Uxf()


    def startElement(self, name, attributes):
        d = {key: value for key, value in attributes.items()}
        if name == 'uxf':
            self.uxo.custom = d.get('custom', '')
        elif name in {'imports', 'ttypes'}:
            pass
        elif name == 'import':
            self.imports_list.append(d['filename'])
        elif name == 'ttype':
            self.tclass = uxf.TClassBuilder(d['name'])
            self.intclass = True
        elif name == 'field':
            self.tclass.append(uxf.Field(d['name'], d.get('vtype')))
        elif name == 'map':
            container = uxf.Map(ktype=d.get('ktype'), vtype=d.get('vtype'))
            self.start_container(container)
        elif name == 'list':
            container = uxf.List(vtype=d.get('vtype'))
            self.start_container(container)
        elif name == 'table':
            tclass = self.uxo.tclasses[d['name']]
            container = uxf.Table(tclass)
            self.start_container(container)
        elif name in {'key', 'value'}:
            pass # container.append() doesn't need this distinction
        elif name == 'comment':
            self.incomment = True
            self.comment = ''
        elif name == 'str':
            self.instr = True
            self.string = ''
        elif name == 'bytes':
            self.inbytes = True
            self.bytes = ''
        elif name == 'int':
            self.stack[-1]._append(int(d['v']))
        elif name == 'real':
            self.stack[-1]._append(float(d['v']))
        elif name == 'date':
            self.stack[-1]._append(uxf.naturalize(d['v']))
        elif name == 'datetime':
            self.stack[-1]._append(uxf.naturalize(d['v']))
        elif name == 'null':
            self.stack[-1]._append(None)
        elif name == 'yes':
            self.stack[-1]._append(True)
        elif name == 'no':
            self.stack[-1]._append(False)


    def endElement(self, name):
        if name in {'uxf', 'ttypes', 'field', 'key', 'value', 'int', 'real',
                    'date', 'datetime', 'null', 'yes', 'no', 'import'}:
            pass
        elif name == 'imports': # got them all (if any)
            if self.imports_list:
                self.uxo.imports = _get_imports(self.imports_list)
        elif name == 'ttype':
            self.uxo.tclasses[self.tclass.ttype] = self.tclass.build()
            self.tclass = None
            self.intclass = False
        elif name in {'map', 'list', 'table'}:
            self.stack.pop()
        elif name == 'comment':
            if self.intclass:
                self.tclass.comment = self.comment
            elif self.stack:
                self.stack[-1].comment = self.comment
            else:
                self.uxo.comment = self.comment
            self.comment = ''
            self.incomment = False
        elif name == 'str':
            self.stack[-1]._append(self.string)
            self.string = ''
            self.instr = False
        elif name == 'bytes':
            self.stack[-1]._append(bytes.fromhex(self.bytes))
            self.bytes = ''
            self.inbytes = False


    def characters(self, text):
        if self.inbytes:
            self.bytes += text
        elif self.instr:
            self.string += text
        elif self.incomment:
            self.comment += text


    def start_container(self, container):
        if self.stack is None:
            self.uxo.value = container
            self.stack = [container]
        elif self.stack:
            self.stack[-1]._append(container)
        self.stack.append(container)


def _get_imports(imports_list):
    imports = None
    if imports_list:
        uxt = ['uxf 1\n']
        for import_filename in imports_list:
            uxt.append(f'!{import_filename}\n')
        uxt.append('[]\n')
        uxt = ''.join(uxt)
        uxo = uxf.loads(uxt)
        imports = uxo.imports
    return imports


BYTES = 'bytes'
COMMENT = 'comment'
DOT_CSV = '.CSV'
DOT_INI = '.INI'
DOT_JSN = '.JSN'
DOT_JSON = '.JSON'
DOT_SQLITE = '.SQLITE'
DOT_UXF = '.UXF'
DOT_UXF_GZ = '.UXF.GZ'
DOT_XML = '.XML'
FIELDS = 'fields'
ITYPE = 'itype'
ITYPES = 'itypes'
JSON_CUSTOM = 'UXF^custom'
JSON_COMMENT = 'UXF^comment'
JSON_IMPORTS = 'UXF^imports'
JSON_TCLASSES = 'UXF^ttypes'
JSON_TCLASS = 'UXF^ttype'
JSON_DATA = 'UXF^data'
JSON_BYTES = 'UXF^bytes'
JSON_DATE = 'UXF^date'
JSON_DATETIME = 'UXF^datetime'
JSON_LIST = 'UXF^list'
JSON_MAP = 'UXF^map'
JSON_TABLE = 'UXF^table'
KTYPE = 'ktype'
LIST = 'list'
MAP = 'map'
NAME = 'name'
RECORDS = 'records'
UTF8 = 'utf-8'
UXF = 'uxf'
VTYPE = 'vtype'

PREFIX = '''
uxfconvert.py <infile.uxf[.gz]> <outfile.{csv,json,sqlite,xml}>
uxfconvert.py [-d|--dropunused] [-r|--replaceimports] [-f|--fieldnames] \
[-iI|--indent=I] [-wW|--wrapwidth=W] <infile.{csv,ini,json,sqlite,xml}> \
<outfile.uxf[.gz]>
uxfconvert.py [-d|--dropunused] [-r|--replaceimports] [-f|--fieldnames] \
[-iI|--indent=I] [-wW|--wrapwidth=W] <infile1.csv> [infile2.csv ... \
infileM.csv] <outfile.uxf[.gz]>

'''

USAGE = '''Converts to and from uxf format.

If the outfile is .uxf.gz, the output will be gzip compressed UXF.

Use -d or --dropunused to drop unused ttype definitions and imports.

Use -r or --replaceimports to replace imports with ttype definitions
to make the outfile standalone (i.e., not dependent on any imports).

Indent spaces defaults to 2 and accepts a range of 0-9 (9 means use a tab).

Wrapwidth defaults to 96 and accepts a range of 40-240.

For indent and wrapwidth the default is silently used if an out of range
value is given. Indent applies when converting to JSON, UXF, and XML;
wrapwidth applies when converting to UXF.

If fieldnames is set and the infile(s) is(are) csv the first row of each
infile will be read as field (column) names; otherwise all rows will be
assumed to contain values.

To produce compact uxf output use a .gz suffix and -i0.

Converting from multiple csv files to uxf, the outfile will either be a
list of tables (if the fieldnames option is given), or a list of lists of
scalars otherwise. Converting from one csv file will produce a uxf with a
table (if fieldnames is set) or a list of lists of scalars. Converting from
uxf to csv can only be done if the uxf contains a single scalar table or a
single list of lists of scalars.

Converting from ini to uxf is purely for example purposes (e.g., ini
comments are dropped). In a real application (e.g., migrating from ini to
uxf), a custom ini parser would be needed. Converting uxf to ini is not
supported.

Converting sqlite to uxf is purely for example purposes and only converts
the sql tables and so won't roundtrip. Converting uxf to sqlite is again
only for example purposes and only supported if the uxf file is a single
scalar table or a list of scalar tables.

Converting from uxf to json and back (i.e., using uxfconvert.py's own json
format) roundtrips with perfect fidelity.

Converting from uxf to xml and back (i.e., using uxfconvert.py's own xml
format) roundtrips with almost perfect fidelity (apart, sometimes from
whitespace differences in strings).

Support for uxf to uxf conversions is provided by the uxf.py module itself,
which can be run directly or via python, e.g., `uxf.py infile.uxf
outfile.uxf` or `python3 -m uxf infile.uxf outfile.uxf` with the same
compression option as here, plus additional options (use uxf.py's -h or
--help for details).'''


if __name__ == '__main__':
    main()
