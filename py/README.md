# Python UXF Library

Uniform eXchange Format (UXF) is a plain text human readable optionally
typed storage format. UXF is designed to make life easier for software
developers and data designers. It directly competes with csv, ini, json,
toml, and yaml formats. One key advantage of UXF is that it supports custom
(i.e., user-defined) types. This can result in more compact, more readable,
and easier to parse data. And in some contexts it may prove to be a
convenient alternative to sqlite or xml.

For details of the Uniform eXchange Format (UXF) supported by this library,
see the [UXF Overview](../README.md). ([PyPI link to UXF
Overview](https://github.com/mark-summerfield/uxf/blob/main/README.md).)

- [Introduction](#introduction)
- [Python UXF Types](#python-uxf-types)
- [API](#api)
    - [Reading and Writing UXF Data](#reading-and-writing-uxf-data)
    - [API Notes](#api-notes)
    - [Classes](#classes)
    - [Functions](#functions)
    - [Constants](#constants)
    - [Command Line Usage](#command-line-usage)
- [Changes](#changes)

## Introduction

The Python `uxf` library works out of the box with the standard library and
one dependency,
[editabletuple](https://github.com/mark-summerfield/editabletuple). It
requires Python 3.8 or later.

- Install: `python3 -m pip install uxf` (or download the wheel `.whl`
  files and do `python -m pip install uxf....whl
  editabletuple...whl` where ... varies)
- Run: `python3 -m uxf -h` _# this shows the command line help_
- Use: `import uxf` _# see the `uxf.py` module docs and [API](#api) for the API_

Using `uxf` as an executable (e.g., `python3 -m uxf ...`) provides a means
of doing `.uxf` to `.uxf` conversions (e.g., compress or uncompress or to
use the standard pretty print format). The executable can also be used for
linting, for deleting unused _ttypes_, and for replacing imports to ensure
that UXF files are stand-alone.

Installed alongside `uxf.py` are: `uxfconvert.py`, `uxfcompare.py`, and
`uxflint.py`. For example, `uxfconvert.py` can losslessly convert `.uxf` to
`.json` or `.xml` and back. It can also do some simple conversions to and
from `.csv`, to `.ini`, and to and from `.sqlite`, but these are really to
illustrate use of the `uxf.py` APIs. `uxfcompare.py` can compare two UXF
files, either for equality, i.e., they both contain the same data, with
lists and tables compared value by value in order, and maps compared by item
in key-order, or for equivalence. A UXF file can be linted using `python3 -m
uxf -l file.uxf`; but to lint any number of files, use `uxflint.py`.

Also see the UXF test files in the `../testdata` folder, the Python examples
in the `py/eg` folder, and the Python tests in the `py/t` folder.

If you just want to create a small standalone `.pyz`, you could simply copy
`uxf.py` and `editabletuple.py` into your project folder and include them in
your `.pyz` file.

## Python UXF Types

Most Python types convert losslessly to and from UXF types. In particular:

|**Python Type**     |**UXF type**|**Notes**|
|--------------------|------------|---------|
|`None`              | `null`     ||
|`bool`              | `bool`     ||
|`bytes`             | `bytes`    ||
|`bytearray`         | `bytes`    |Lossless conversion but type changes to Python `bytes`|
|`datetime.date`     | `date`     ||
|`datetime.datetime` | `datetime` |Loses any timezone and only preserves to the nearest 1-second|
|`int`               | `int`      ||
|`float`             | `real`     ||
|`str`               | `str`      ||
|`uxf.List`          | `list`     ||
|`uxf.Map`           | `map`      ||
|`uxf.Table`         | `table    `||

A [List](#list-class) is a Python `collections.UserList` subclass with
`.data` (the list)`, .comment` and `.vtype` attributes. A `.vtype` holds a
`str` representing the name of the corresponding UXF type (e.g., `int`,
`real`, etc.) or a `ttype` (the name of a custom table type; see
[TClass](#tclass-class)). If a `.vtype` is `None` it means that _any_ UXF
type may be stored.

Similarly a [Map](#map-class) is a Python `collections.UserDict` subclass
with `.data` (the dict), `.comment`, `.ktype`, and `.vtype` attributes. The
`.ktype` is a `str` like the `.vtype` except that it may only be `None`
(accept any valid _ktype_) or one of `bytes`, `date`, `datetime`, `int`, or
`str`. The `.vtype` works just the same as for [List](#list-class)s.

The [Table](#table-class) class has `.records`, `.comment`, and `.tclass`
attributes; with `.tclass` holding a [TClass](#tclass-class) which in turn
holds the table's `ttype` (i.e., its table type name), and the table's field
names and (optional) types. In all cases a type of `None` signifies that any
type valid for the context may be used.

Custom types can be represented using _ttypes_. For example, for points you
could define a _ttype_ such as: `=point x:real y:real`. Then you could
include single points like `(point 1.5 7.2)`, or many of them such as
`(point 1.5 7.2 8.3 -9.4 14.8 0.6)`.

Collection types such as `set`, `frozenset`, `tuple`, or `collections.deque`
are automatically converted to a [List](#list-class) when they are
encountered. Of course, these are one way conversions.

## API

The simplest part of the API loads and saves (dumps) UXF data from/to
strings or files.

### Reading and Writing UXF Data

    load(filename_or_filelike): -> uxo
    loads(uxt): -> uxo

The [load()](#load-def) function reads UXF data from a file or file-like
object, and the [loads()](#loads-def) function reads UXF data from a string.
These functions take several optional arguments; see [load()](#load-def) and
[loads()](#loads-def). The returned `uxo` (UXF object) is of type
[Uxf](#uxf-class).

    dump(filename_or_filelike, data)
    dumps(data) -> uxt

The [dump()](#dump-def) function writes the data in UXF format to a file or
file-like object, and the [dumps()](#dumps-def) function writes the data
into a string that's then returned (here called `uxt` to indicate UXF text).
These functions take several optional arguments; see [dump()](#dump-def) and
[dumps()](#dumps-def). The data can be a [Uxf](#uxfclass) object or a single
`list`, [List](#list-class), `dict`, [Map](#map-class), or
[Table](#table-class).

If the data contains values of types that aren't supported by UXF, they
could be transformed in advance (e.g., to a custom table type, a _ttype_).

See also the examples in the `py/eg` folder and the tests in the `py/t`
folder.

### API Notes

A [UXF](#uxf-class) object (called a `uxo` in these docs) has a `.value`
attribute that is always a [List](#list-class) or [Map](#map-class) or
[Table](#table-class). The first two have essentially the same APIs as
`list` and `dict` respectively. The [Table](#table-class) API is a little
similar to a `list`. Individual records can be accessed (read _and_ updated)
using `table[row]`, and individual files using `table[row][column]` or
`table[row].fieldname`. For small tables, records can be accessed with the
`first`, `second`, `third`, or `fourth` properties. The best way to append
new records is to use the [Table](#table-class)'s `append()` method.

The `uxf` module distinguishes between a _ttype_ (the name of a user-defined
table) and a [TClass](#tclass-class) (the Python class which represents a
user-defined table). A [TClass](#tclass-class) has `.ttype` and `.fields`
attributes.

For reading UXF data it is easiest to iterate, and to do so recursively, if
the data has nested collections, e.g., using [Uxf.visit()](#uxf.visit-def).
The `uxfconvert.py` tool and the `eg/slides2.py` example both show
alternative approaches to manual iteration.

Note that for imports, if the filename is relative it is searched for in the
same folder as the importing UXF file, and if not found there, in the
current folder, and if not there either, in each of the paths in `UXF_PATH`
(if any).

Note that the `__version__` is the module version (i.e., the version of this
implementation), while the `VERSION` is the maximum UXF version that this
module can read (and the UXF version that it writes).

### Classes

The classes are documented in convenience order. Here are alphabetically
ordered links:
[Error](#error-class),
[Field](#field-class),
[Format](#format-class),
[List](#list-class),
[Map](#map-class),
[Table](#table-class),
[TClass](#tclass-class),
[Uxf](#uxf-class).

<a name="uxf-class"></a>
#### Uxf

A `Uxf` object represents a UXF file in memory.

The easiest way to obtain a `Uxf` object is to use one of the module-level
functions, [load()](#load-def) or [loads()](#loads-def). Of course, you can
also create ``Uxf``s programatically (as shown in many of the examples and
tests).

##### Constructor

**`Uxf(value=None, *, custom='', tclasses=None, comment=None,
on_event=on_event)`**

The `Uxf` constructor takes an optional `value` (a `list`, `List`, `tuple`,
`dict`, `Map`, or `Table`) and optional (by keyword arguments) `custom`
string, `tclasses` (a `dict` whose names are _ttypes_ and whose values are
the corresponding [TClass](#tclass-class)es), and `comment` string. You can
also pass a custom [on\_event()](#on_event-def) handler.

To create a `Uxf` programmatically (rather than by using, say,
[load()](#load-def), you can either create it empty, or with some data and
optionally, some _ttypes_.

    value = ... # a list, uxf.List, dict, uxf.Map, or uxf.Table
    point_ttype = uxf.TClass('point', (uxf.Field('x', 'real'), uxf.Field('y', 'real')))
    uxo = uxf.Uxf(value, tclasses={point_ttype.ttype: point_ttype})

An alternative is to do this:

    uxo = uxf.loads('uxf 1.0\n=point x:real y:real\n[]')
    uxo.value = value

##### Properties

**`.value`**

A [List](#list-class), [Map](#map-class), or [Table](#table-class). Since
these classes can nest, a `Uxf` can represent any arbitrary data structure.

**`.custom`**

An opional `str` used for customizing the file format, i.e., the header text
that follows the initial `uxf 1.0` on the first line.

**`.tclasses`**

A possibly empty `dict` where each key is a `ttype` (i.e., the
`.tclass.ttype` which is a [TClass](#tclass-class)'s name—a `str`) and each
value is a [TClass](#tclass-class) object. See also
[tclass()](#uxf.tclass-def).

**`.comment`**

An optional file-level `str`.

**`.imports`**

A possibly empty `dict` where each key is a `ttype` and where each value is
a `str` holding that type's import text.

**`.import_filenames`**

A utility property useful for some UXF processors. It yields all the unique
import filenames.

##### Methods

<a name="Uxf.load-def"></a>
**`.load(filename_or_filelike, *, on_event=on_event, drop_unused=False, replace_imports=False)`**

Convenience method that wraps the module-level [load()](#load-def) function.

<a name="Uxf.loads-def"></a>
**`.loads(uxt, filename='-', *, on_event=on_event, drop_unused=False, replace_imports=False)`**

Convenience method that wraps the module-level [loads()](#loads-def)
function.

<a name="Uxf.dump-def"></a>
**`.dump(filename_or_filelike, *, on_event=on_event, format=Format())`**

Convenience method that wraps the module-level [dump()](#dump-def) function.

<a name="Uxf.dumps-def"></a>
**`.dumps(*, on_event=on_event, format=Format())`**

Convenience method that wraps the module-level [dumps()](#dumps-def)
function.

<a name="Uxf.is_equivalent-def"></a>
**`.is_equivalent(other: Uxf, compare=Compare.EXACT) -> bool`**

Method primarily used for regression testing. The [List](#list-class),
[Map](#map-class), [Table](#table-class), and [TClass](#tclass-class)
classes all provide this method.

There are seven comparison flags although normally only `Compare.EQUIVALENT`
is used:

- ``EXACT`` use `==` or `!=` instead;
- ``IGNORE_COMMENTS`` ignore differences in comments
- ``IGNORE_UNUSED_TTYPES`` ignore unused ttypes
- ``IGNORE_IMPORTS`` ignore whether ttypes are imported or defined
- ``EQUIVALENT`` this is the _or_ of the three ignores above
- ``IGNORE_KVTYPES`` ignore ktypes, vtypes, and field vtypes (rarely
  appropriate)
- ``UNTYPED_EQUIVALENT`` this is the _or_ of all the ignores above (rarely
  appropriate)

For exact equality use `==` or `!=`.

<a name="Uxf.visit-def"></a>
**`.visit(visitor)`**

This method iterates over every value in the Uxf's `value` (recursively) and
calls `visitor(Visit, value)` where Visit is an enum, and value is either a
a value or None.

Lists, Maps, and Tables also have `visit()` methods (that this method
calls).

<a name="Uxf.tclass-def"></a>
**`.tclass(ttype: str) -> bool`**

Returns the `TClass` whose _ttype_ is `ttype` or raises `KeyError`.

See also the `tclasses` property.

<a name="list-class"></a>
#### List

A class used to represent a UXF list. It is a `collections.UserList`
subclass which holds its list in the `.data` attribute and that also has
`.comment` and `.vtype` attributes.

Also provides ``is_equivalent()`` and support for `==` and `!=`, as well as
a `visit(visitor)` method.

##### Constructor

**`List(seq=None, *, vtype=None, comment=None)`**

The `seq` can be a `list`, `tuple`, or any iterable acceptable to the
built-in `list()` type. The _vtype_ is a `str` signifying the type of value
this list may hold; `None` means values may be of _any_ UXF-compatible type
(see [Python UXF Types](#python-uxf-types)). A `str` may be passed as the
`comment`. Both the `vtype` and `comment` properties are immutable after
construction.

See [Python UXF Types](#python-uxf-types) for more about _vtypes_.

<a name="map-class"></a>
#### Map

A class used to represent a UXF map. It is a `collections.UserDict` subclass
that holds its dict in the `.data` attribute and that also has `.comment`,
`.ktype`, and `.vtype` attributes. The `ktype, `vtype`, and `comment`
properties are immutable after construction.

Although Python ``dict``s are insertion-ordered, UXF ``Map``s are
key-ordered. To access keys, values, or items in UXF order use the
overridden `keys()`, `values()`, or `items()` methods rather than iterating
directly on the map. (The order works as follows: when two keys are of
different types they are ordered `bytes` `<` `date` `<` `datetime` `<` `int`
`<` `str`, and when two keys have the same types they are ordered using `<`
except for ``str``s which use case-insensitive `<`.)

Also provides ``is_equivalent()`` and support for `==` and `!=`, as well as
a `visit(visitor)` method.

##### Constructor

**`Map(d=None, *, ktype=None, vtype=None, comment=None)`**

The `d` can be a `dict` or any single value acceptable to the built-in
`dict()` type. The _ktype_ is a `str` signifying the type of key that must
be used; `None` means that any valid _ktype_, i.e., `'bytes'`, `'date'`,
`'datetime'`, `'int'`, or `'str'`. The _vtype_ is a `str` signifying the
type of value this `Map` may hold; `None` means values may be of _any_
UXF-compatible type (see [Python UXF Types](#python-uxf-types)). A `str` may
be passed as the `comment`.

See [Python UXF Types](#python-uxf-types) for more about _ktypes_ and
_vtypes_.

<a name="table-class"></a>
#### Table

A class used to store UXF Tables.

Also provides ``is_equivalent()`` and support for `==` and `!=`, as well as
a `visit(visitor)` method.

##### Constructor

**`Table(tclass=None, *, records=None, comment=None)`**

A `Table` can be created using the constructor, passing a
[TClass](#tclass-class), and optionally, records (a list of lists, where
each sublist has `len(tclass.fields)` values), and a comment (a `str`).
Alternativvely, use the [table()](#table-def) convenience function which
takes a _ttype_ (a `str`), and fields. Both the `tclass` (i.e., the `ttype`)
and `comment` properties are immutable after construction.

See [Python UXF Types](#python-uxf-types) for more about and _ttypes_.

##### Properties

**`.tclass`**

A [TClass](#tclass-class) which holds the table's _ttype_ and field names
(and optional field types). Immutable after construction.

**`.ttype`**

A convenience for `.tclass.ttype`. Immutable after construction.

**`.records`**

The table's data: a list of values where each value is either a list with
`len(table.fields)` values, or an
[editabletuple](https://github.com/mark-summerfield/editabletuple) of type
`.RecordClass`.

**`.RecordClass`**

The table's record class, an
[editabletuple](https://github.com/mark-summerfield/editabletuple), which
can be used to create a single record by calling it with each of the
record's fields' values (or with `*sequence` where `len(sequence)` equals
the number of fields). When a table record is accessed (e.g., when one row
of the table's list is returned), it is returned as an instance of this
class.

(This is actually a convenience method that returns
`table.tclass.RecordClass`.)

**`.fields`**

A convenience for `.tclass.fields`. Immutable after construction.

**`.first`**

The table's first record as a `RecordClass`
[editabletuple](https://github.com/mark-summerfield/editabletuple), or
`None` if the table is empty.

**`.second`**

The table's second record or `None`.

**`.third`**

The table's third record or `None`.

**`.fourth`**

The table's fourth record or `None`.

**`.last`**

The table's last record or `None`.

**`.isfieldless`**

This is `True` if the table's `TClass` is fieldless (and is a convenience
for ``.tclass.isfieldless``).

**`.is_scalar`**

This is `True` if all the table's values are (or ought to be given its
fields' types) scalars; otherwise it is `False`. Its main use is as a helper
for the [dump()](#dump-def) and [dumps()](#dumps-def) functions.

##### Operators

**`table[row]`**

The table's `row`-th record as a `RecordClass`
[editabletuple](https://github.com/mark-summerfield/editabletuple). The
returned record is editable, so to get or set a field use
`table[row][column]` or `table[row].fieldname`.

To replace an entire record, use `table[row] = record` where `record` is a
sequence (e.g., a `tuple` of length `len(table.fields)`), or a
`table.RecordClass` instance. (See also `append()` below.)

To delete an entire record, use `del table[row]`.

**`iter(table)`**

For example, `for record in table:`. Yields every record in the table as a
`RecordClass` instance.

**`len(table)`**

The number of records (rows) in the table.

##### Methods

**`.append(record)`**

Appends a new row to the table's list of lists as an 
[editabletuple](https://github.com/mark-summerfield/editabletuple) of type
`.RecordClass`. The row can be provided as a `RecordClass` editabletuple or
as a sequence of field values (which will then be converted to the table's
`RecordClass`).

**`.insert(row, record)`**

Inserts the given record (or sequence of field values which is converted to
a `RecordClass` instance) into the table at the given `row` position.

**`.field(column)`**

This function is a convenience for `.tclass.fields[column]`.

**`.get(row)`**

Returns the table's `row`-th record as a `RecordClass`
[editabletuple](https://github.com/mark-summerfield/editabletuple) _or_
`None` if `row` is out of range. If a record is returned it is editable.

<a name="tclass-class"></a>
#### TClass

This class is used to store a [Table](#table-class)'s _ttype_ (i.e., its
name) and fields.

Also provides ``is_equivalent()`` and support for `==` and `!=`.

##### Constructor

**`TClass(ttype, fields=None, *, comment=None)`**

The `ttype` must be a `str` that holds the ``TClass``'s name (equivalent to
a _vtype_ or _ktype_ name); it may not be the same as a built-in type name
or constant. Leave `fields` as `None` for a fieldless table; otherwise pass
a sequence of field names (``str``'s) or ``Field``'s (which have field names
and optionally types). The `.comment` may be passed a `str`.

The `.ttype` attribute holds the ``TClass``'s name. The `.fields` attribute
holds a list of fields of type [Field](#field-class). The `.comment` holds
an optional `str`.

The `.isfieldless` property returns `True` if there are no fields (which is
valid for a fieldless table); otherwise returns `False`.

The `.RecordClass` property returns an
[editabletuple](https://github.com/mark-summerfield/editabletuple), which
can be used to create a single record by calling it with each of the
record's fields' values (or with `*sequence` where `len(sequence)` equals
the number of fields).

Immutable after construction.

<a name="field-class"></a>
#### Field

This class is used to store a [TClass](#tclass-class)'s fields, i.e., the
fields for a [Table](#table-class).

Also provides support for `==` and `!=`.

##### Constructor

**`Field(name, vtype=None)`**

The `name` must start with a letter and be followed by
`0-uxf.MAX_IDENTIFIER_LEN-1` letters, digits, or underscores; it may not be
the same as a built-in type name or constant. A _vtype_ of `None` means that
the field may hold any valid UXF type (see [Python UXF
Types](#python-uxf-types)); otherwise it must be one of these ``str``s:
`'bool'`, `'bytes'`, `'date'`, `'datetime'`, `'int'`, `'real'`, `'list'`,
`'map'`, `'str'`, or `'table'`; or a _ttype_ name.

Immutable after construction.

<a name="format-class"></a>
#### Format

This class is used to specify aspects of the output format for the
[dump()](#dump-def) and [dumps()](#dumps-def) functions (and the equivalent
methods).

##### Constructor

**`Format(indent='  ', wrap_width=96, realdp=None, max_short_len=32)`**

The [dump()](#dump-def) and [dumps()](#dumps-def) functions use the default
`Format()` which uses the defaults shown (indent is two spaces). However, by
creating and passing your own `Format` object, you can change these to suit
your needs. For `realdp`, `None` signifies use however many digits after the
decimal point are needed for UXF ``real``'s (i.e., for Python ``float``'s);
otherwise specify a value 0-15.

For example, if you had a `Uxf` object with a table and wanted the output to
be one record per line, with ``real``s output with 3 decimal places, you
could pass a format of `Format(wrap_width=None, realdp=3)`.

<a name="error-class"></a>
#### Error

This class is used to propagate errors. It holds the `uxf.Event`, an error
code, an error message string, a filename (or `'-'`), and a line number
(`lino` which may be `0`).

(If this module produces an exception that isn't a `uxf.Error` or `OSError`
subclass then it is probably a bug that should be reported.)

### Functions

The functions are documented in convenience order. Here are alphabetically
ordered links:
[append\_to\_parent()](#append_to_parent-def),
[canonicalize()](#canonicalize-def),
[dump()](#dump-def),
[dumps()](#dumps-def),
[event\_text()](#event_text-def),
[isoformat()](#isoformat-def),
[isasciidigit()](#isasciidigit-def),
[is\_scalar()](#is_scalar-def),
[load()](#load-def),
[loads()](#loads-def),
[naturalize()](#naturalize-def),
[on\_event()](#on_event-def),
[table()](#table-def).

<a name="load-def"></a>
**`load(filename_or_filelike, *, on_event=on_event, drop_unused=False,
replace_imports=False)`**

Returns a [Uxf](uxf-class) object.

`filename_or_filelike` is `sys.stdin` or a filename (`str` or
`pathlib.Path`) or an open readable file (text mode UTF-8 encoded,
optionally gzipped).

`on_event` is a custom error handling function that defaults to
[on\_event](#on_event-def).

If `drop_unused` is `True`, then any _ttype_ definitions in the file that
aren't actually used in the data are dropped. So if the `Uxf` is later
dumped, the dropped _ttype_ definitions will not be present.

If `replace_imports` is `True`, then any imports are replaced by the _ttype_
definitions that they import.  So if the `Uxf` is later dumped, _no_ imports
will be present; instead any imported _ttypes_ will be dumped as _ttype_
definitions. Normally, if you use `replace_imports=True`, then you would
also use `drop_unused=True`.

<a name="loads-def"></a>
**`loads(uxt, filename='-', *, on_event=on_event, drop_unused=False,
replace_imports=False)`**

Returns a [Uxf](#uxf-class) object.

`uxt` must be a `str` of UXF data, e.g., `uxf 1.0\n[1 2 3]`.

If given, the `filename` is used for error messages.

For more on the other argument see [load()](#load-def).

<a name="dump-def"></a>
**`dump(filename_or_filelike, data, *, on_event=on_event,
format=Format())`**

`filename_or_filelike` is `sys.stdout` or a filename or an open writable
file (text mode UTF-8 encoded). If `filename_or_filelike` is a filename with
a `.gz` suffix then the output will be gzip-compressed.

`data` is a [Uxf](#uxf-class) object, or a list, [List](#list-class), dict,
[Map](#map-class), or [Table](#table-class), that this function will write
to the `filename_or_filelike` in UXF format.

`format` specifies aspects of the output (e.g., indent); see the
[Format](#format-class) class for details.

<a name="dumps-def"></a>
**`dumps(data, *, on_event=on_event, format=Format())`**

`data` is a [Uxf](#uxf-class) object, or a list, [List](#list-class), dict,
[Map](#map-class), or [Table](#table-class) that this function will write to
a `str` in UXF format which will then be returned.

For more on the other arguments see [dump()](#dump-def).

<a name="table-def"></a>
**`table(ttype, fields, *, comment=None)`**

Convenience function for creating empty tables with a new
[TClass](#tclass-class).

See also the [Table](#table-class) constructor.

<a name="on_event-def"></a>
**`on_event(event, code, message, *, filename='-', lino=0, verbose=True,
prefix='uxf')`**

This is the default error handler which you can replace by passing a custom
one to [load()](#load-def), [loads()](#loads-def), [dump()](#dump-def), or
[dumps()](#dumps-def).

This is called with an event of type `uxf.Event` (an `enum` of `WARNING`,
`REPAIR`, `ERROR`, and `FATAL`), an event code, a message, the filename (or
`'-'` if unknown or `stdin`), the line number (`0` if unknown or not
applicable), and verbosity. If `event` is `Event.FATAL` it means the event
is unrecoverable, so the normal action would be to raise. If verbose is
`True` the normal action is to print a textual version of the event data to
`stderr`.

To make `on_event()` quieter:

    on_event = functools.partial(uxf.on_event, verbose=False)

To make all errors fatal and use your own prefix (rather than the default of
`'uxf'`):

    def on_event(*args, **kwargs):
        kwargs['prefix'] = 'myapp'
        raise uxf.Error(uxf.event_text(*args, **kwargs))

There are four kinds of events:

- `Event.REPAIR` a fix was applied, e.g., an int was converted to a float, a
  string containing a date was converted to a date, etc.;
- `Event.WARNING` problem can't be automatically fixed but can be coped
  with;
- `Event.ERROR` problem can't be automatically fixed and means UXF may be
  invalid;
- `Event.FATAL` unrecoverable problem.
        
For further examples of custom `on_event()` functions, see
`t/test_errors.py`, `t/test_imports.py` `t/test_include.py`,
`t/test_merge.py`, and `t/test_sqlite.py`.

<a name="event_text-def"></a>
**`event_text(event, code, message, *, filename='-', lino=0, verbose=True,
prefix='uxf')`**

Returns a `str` containing the event in ``uxf``'s standard format. Useful
for `on_event()` reimplementations.

<a name="isasciidigit-def"></a>
**`isasciidigit(s)`**

Returns `True` if `s` matches ``/^[0-9]+$/``. (Python's `str.isdigit()` and
`str.isdecimal()` both match additional Unicode digit characters which is
why we use `isasciidigit()`.)

<a name="isoformat-def"></a>
**`isoformat(dt)`**

If `dt` is a `date`, returns the corresponding `str` with format
`'YYYY-MM-DD'`; if `dt` is a `datetime`, returns the corresponding `str`
with format `'YYYY-MM-DDTHH:MM:SS'`.

<a name="naturalize-def"></a>
**`naturalize(s)`**

Given `str` `s` returns `True` if the `str` is 't', 'true', 'y', 'yes', or
`False` if the `str` is 'f', 'false', 'n', 'no' (case-insensitive); or
returns an `int` if `s` holds a parsable int, or a `float` if `s` holds a
parsable `float`, or a `datetime.datetime` if `s` holds a parsable ISO8601
datetime, or a `datetime.date` if `s` holds a parsable ISO8601 date, or
failing these returns the original ``str``, ``s``, unchanged.

This is provided as a helper function (e.g., it is used by `uxfconvert.py`).

<a name="is_scalar-def"></a>
**`is_scalar(x)`**

Returns `True` if `x` is `None` or a `bool`, `bytes`, `bytearray`,
`datetime.date`, `datetime.datetime`, `int`, `float`, or `str`; otherwise
returns `False`. Its main use is as a helper for the [dump()](#dump-def) and
[dumps()](#dumps-def) functions.

<a name="canonicalize-def"></a>
**`canonicalize(name, is_table_name)`**

Utility for UXF processors. Given a `name`, returns a `name` that is a valid
table or field name. See `uxfconvert.py` for uses.

<a name="append_to_parent-def"></a>
**`append_to_parent(parent, value)`**

Utility for UXF processors; see `uxf.py` and `uxfconvert.py` for examples of
use.

### Constants

|**Constant**|**Description**|
|------------|---------------|
|`VERSION`|The UXF file format version.|
|`RESERVED_WORDS`|A set of names that cannot be used for table or field names.|
|`UTF8`|The string `'utf-8'`.|

### Command Line Usage

The uxf module can be used as an executable. To see the command line help
run:

    python3 -m uxf -h

or

    path/to/uxf.py -h

## Changes

- 2.2.0
  - Maps are now key-ordered as follows: when two keys are of different
    types they are ordered `bytes` `<` `date` `<` `datetime` `<` `int` `<`
    `str`, and when two keys have the same types they are ordered using `<`
    except for ``str``s which use case-insensitive `<`.
  - Added some new methods, in particular `Uxf.visit()` (and `List.visit()`,
    `Map.visit()`, and `Table.visit()`), and a new `eg/visit.py` example.
- 2.1.0 added `is_equivalent()` and support for `==` (and `!=`) for Lists,
  Maps, Tables, and TClasses.
- 2.0.1 `on_event()` now supports keyword argument `prefix`.
- 2.0.0 API changes
  - `on_error()` has been replaced by `on_event()` with a different API.
  - ``List``: after construction, the `vtype` and `comment` are immutable;
    `data` values remain mutable.
  - ``Map``: after construction, the `ktype`, `vtype`, and `comment` are
    immutable; `data` items remain mutable.
  - ``Table``: after construction, the `tclass` (i.e., the `ttype`) and
    `comment` are immutable; `records` remains mutable.
  - ``TClass``es are immutable after construction.
  - ``Field``s are immutable after construction.
  - Internal changes are refactorings and a slightly smarter lexer (and a
    correspondingly slightly simpler parser). And, of course, more tests.
- 1.1.2 Additional tests.
- 1.1.1 Internal improvements plus moved `.RecordClass` to `TClass` (without
  changing ``Table``'s API).
- 1.1.0 Dropped ``Format``'s `max_fields_in_line` and `max_list_in_line`
  settings since neither worked: use `wrap_width` instead.
- 1.0.2 Bug fix: I missed a corner case in the previous fix; now fixed.
- 1.0.1 Bug fix: when `dump()` and `dumps()` output _ttype_ definitions,
  they now respect the format's `wrap_width` and `indent` settings.
- 1.0.0 First stable release.

---
