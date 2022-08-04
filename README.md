# UXF Overview

Uniform eXchange Format (UXF) is a plain text human readable optionally
typed storage format. UXF may serve as a convenient alternative to csv, ini,
json, sqlite, toml, xml, or yaml.

UXF is an open standard. The UXF software linked from this page is all free
open source software.

The primary purpose of UXF is to make developers lives easier. It does this
by providing a convenient, scalable, easy-to-use file format that can be
used for most purposes, from configuration files to application data. And
UXF-based formats are very easy to adapt to future requirements

- [Datatypes](#datatypes)
    - [Table of Built-in Types](#table-of-built-in-types)
    - [Terminology](#terminology)
    - [Minimal empty UXF](#minimal-empty-uxf)
    - [Built-in Types](#built-in-types)
    - [Custom Types](#custom-types)
- [Examples](#examples)
- [Libraries](#libraries) [[Python](py/README.md)]
    - [Implementation Notes](#implementation-notes)
- [Imports](#imports)
- [BNF](#bnf)
- [Supplementary](#supplementary)
    - [Vim Support](#vim-support)
    - [UXF Logo](#uxf-logo)

## Datatypes

UXF supports the following eleven built-in datatypes.


|**Type**<a name="table-of-built-in-types"></a>|**Example(s)**|**Notes**|
|-----------|----------------------|--|
|`null`     |`?`|`?` is the UXF _null_ type's literal representation.|
|`bool`     |`no` `yes`|Use `no` for false and `yes` for true.|
|`bytes`    |`(:20AC 65 66 48:)`|There must be an even number of case-insensitive hex digits; whitespace (spaces, newlines, etc.) optional.|
|`date`     |`2022-04-01`|Basic ISO8601 YYYY-MM-DD format.|
|`datetime` |`2022-04-01T16:11:51`|ISO8601 YYYY-MM-DDTHH[:MM[:SS]] format; 1-sec resolution no timezone support (see also [Custom Types](#custom-types)).|
|`int`      |`-192` `+234` `7891409`|Standard integers with optional sign.|
|`real`     |`0.15` `0.7e-9` `2245.389`|Standard and scientific notation.|
|`str`      |`<Some text which may include newlines>`|For &, <, >, use \&amp;, \&lt;, \&gt; respectively.|
|`list`     |`[value1 value2 ... valueN]`|A list of values of any type.|
|`list`     |`[vtype value1 value2 ... valueN]`|A list of values of type _vtype_.|
|`map`      |`{key1 value1 key2 value2 ... keyN valueN}`|A map with keys of any valid key type and values of any type.|
|`map`      |`{ktype key1 value1 key2 value2 ... keyN valueN}`|A map with keys of type _ktype_ and values of any type.|
|`map`      |`{ktype vtype key1 value1 key2 value2 ... keyN valueN}`|A map with keys of type _ktype_ and values of type _vtype_.|
|`table`    |`(ttype <value0_0> ... <value0_N> ... <valueM_0> ... <valueM_N>)`|A table of values. Each value's type must be of the corresponding type specified in the _ttype_, or any value type where no type has been specified.|

Note that it is also possible to represent [Custom Types](#custom-types).

### Terminology

- A `map` _key-value_ is collectively called an _item_.
- A “single” valued type (`bool`, `bytes`, `date`, `datetime`, `int`,
  `str`), is called a _scalar_.
- A “multi-” valued type (`list`, `map`, `table`) is called a _collection_.
- A `list`, `map`, or `table` which contains only scalar values is called a
  scalar `list`, scalar `map`, or scalar `table`, respectively.
- A _`ttype`_ is the name of a table's user-defined type.

### Minimal empty UXF

    uxf 1.0
    []

Every UXF file consists of a single header line (starting `uxf 1.0`,
optionally followed by custom text). This may be followed by an optional
file-level comment, then any _ttype_ (table type) imports, then any _ttype_
definitions. After this comes the data in the form of a single `list`,
`map`, or `table` in which all the values are stored. The data must be
present even if it is merely an empty list (as here), an empty map (e.g.,
`{}`), or an empty table. Since ``list``s, ``map``s, and ``table``s can be
nested inside each other, the UXF format is extremely flexible.

### Built-in Types

Map keys (i.e., _ktype_) may only be of types `bytes`, `date`, `datetime`,
`int`, and `str`.

List, map, and table values may be of _any_ type (including nested ``map``s,
``list``s, and ``table``s), unless constrained to a specific type. If
constrained to a specific _vtype_, the _vtype_ may be any built-in type (as
listed above, except `null`), or any user-defined _ttype_, and the
corresponding value or values must be any valid value for the specified
type, or `?` (null).

Lists and tables preserve the order in which values are appended. So the
first value is at index/row 0, the second at index/row 1, etc. Maps are
unordered. (However, many libraries output Maps in a predictable order to
aid testing.)

A `table` starts with a _ttype_. Next comes the table's values. The number
of values in any given row is equal to the number of field names in the
_ttype_.

Lists, maps, tables, and _ttype_ definitions may begin with a comment. And
lists, maps, and tables may optionally by typed as indicated above. (See
also the examples below and the BNF near the end).

Strings may not include `&`, `<` or `>`, so if they are needed, they must be
replaced by the XML/HTML escapes `&amp;`, `&lt;`, and `&gt;` respectively.

Where whitespace is allowed (or required) it may consist of one or more
spaces, tabs, or newlines in any combination.

If you don't want to be committed to a particular UXF type, just use a `str`
and do whatever conversion you want, or use a [Custom Type](#custom-types).

### Custom Types

There are two common approaches to handling custom types in UXF. Both
allow for UXFs to remain round-trip readable and writeable even by UXF
processors that aren't aware of the use of custom types as such.

Here, we'll look at both approaches for two different custom types, a
point and an enumeration.

    uxf 1.0
    [
      {<Point> [1.4 9.8]} {<Point> [-0.7 3.0]} {<Point> [2.1 -6.3]}
      <TrafficLightGreen> <TrafficLightAmber> <TrafficLightRed>
    ]

This first approach shows three points, each represented by a `map` with a
`str` indicating the custom type (“Point”), and using ``list``s of two
``real``s for the _x_ and _y_ coordinates. The example also shows a traffic
light enumeration each represented by a `str`.

    uxf 1.0
    [
      {<Point> [1.4 9.8 -0.7 3.0 2.1 -6.3]}
      <TrafficLightGreen> <TrafficLightAmber> <TrafficLightRed>
    ]

Since we have multiple points we've changed to a single `map` with a `list`
of point values. This is more compact but assumes that the reading
application knows that points come in pairs.

A UXF processor has no knowledge of these representations of points or
enumerations, but will handle both seamlessly since they are both
represented in terms of built-in UXF types. Nonetheless, an application that
reads such UXF data can recognize and convert to and from these
representations to and from the actual types.

    uxf 1.0
    =Point x:real y:real
    =TrafficLightGreen
    =TrafficLightAmber
    =TrafficLightRed
    [
      (Point 1.4 9.8 -0.7 3.0 2.1 -6.3)
      (TrafficLightGreen) (TrafficLightAmber) (TrafficLightRed)
    ]

This second approach uses four _ttypes_ (custom table types). For the Point
we specify it as having two real fields (so the processor now knows that
Points have two `real` values). And for the enumeration we used three
separate fieldless tables.

Using tables has the advantage that we can represent any number of values of
a particular _ttype_ in a single table (including just one, or even none),
thus cutting down on repetitive text. Here, the Point table has three Points
(rows). And some UXF processor libraries will be able to return table values
as custom types. (For example, the [Python UXF library](py/README.md) would
return these as custom class instances—as “editable tuples”.)

If many applications need to use the same _ttypes_, it _may_ make sense to
create some shared _ttype_ definitions. See [Imports](#imports) for how to
do this.

## Examples

### Minimal UXFs

    uxf 1.0
    {}

We saw earlier an example of a minimal UXF file with an empty list; here we
have one with an empty map.

    uxf 1.0
    =Pair first second
    (Pair)

Here is a UXF with a _ttype_ specifying a Pair that has two fields each of
which can hold _any_ UXF value (including nested collections). In this case
the data is a single _empty_ Pair table.

    uxf 1.0
    =Pair first second
    (Pair (Pair 1 2) (Pair 3 (Pair 4 5)))

And here is a UXF with a single Pair table that contains two nested Pair
tables, the second of which itself contains a nested pair.

### CSV to UXF

Although widely used, the CSV format is not standardized and has a number of
problems. UXF is a standardized alternative that can distinguish fieldnames
from data rows, can handle multiline text (including text with commas and
quotes) without formality, and can store one—or more—tables in a single UXF
file.

#### CSV

    Date,Price,Quantity,ID,Description
    "2022-09-21",3.99,2,"CH1-A2","Chisels (pair), 1in & 1¼in"
    "2022-10-02",4.49,1,"HV2-K9","Hammer, 2lb"
    "2022-10-02",5.89,1,"SX4-D1","Eversure Sealant, 13-floz"

#### UXF equivalents

The most obvious translation would be to a `list` of ``list``s:

    uxf 1.0
    [
      [<Price List> <Date> <Price> <Quantity> <ID> <Description>]
      [2022-09-21 3.99 2 <CH1-A2> <Chisels (pair), 1in &amp; 1¼in>]
      [2022-10-02 4.49 1 <HV2-K9> <Hammer, 2lb>]
      [2022-10-02 5.89 1 <SX4-D1> <Eversure Sealant, 13-floz>]
    ]

This is perfectly valid. However, it has the same problem as `.csv` files:
is the first row data values or column titles? (For software this isn't
always obvious, for example, if all the values are strings.) Not to mention
the fact that we have to use a nested `list` of ``list``s. Nonetheless it is
an improvement, since unlike the `.csv` representation, every value has a
concrete type (all ``str``s for the first row, and `date`, `real`, `int`,
`str`, `str`, for the subsequent rows).

The most _appropriate_ UXF equivalent is to use a UXF `table`:

    uxf 1.0
    =PriceList Date Price Quantity ID Description
    (PriceList
      2022-09-21 3.99 2 <CH1-A2> <Chisels (pair), 1in &amp; 1¼in> 
      2022-10-02 4.49 1 <HV2-K9> <Hammer, 2lb> 
      2022-10-02 5.89 1 <SX4-D1> <Eversure Sealant, 13-floz> 
    )

When one or more tables are used each one's _ttype_ (table type) must be
defined at the start of the `.uxf` file. A _ttype_ definition begins with an
`=` sign followed by the _ttype_ (i.e., the table name), followed by one or
more fields. A field consists of a name optionally followed by a `:` and
then a type (here only names are given).

Both table and field names are user chosen and consist of 1-60 letters,
digits, or underscores, starting with a letter or underscore. No table or
field name may be the same as any built-in type name, so no table or field
can be called `bool`, `bytes`, `date`, `datetime`, `int`, `list`, `map`,
`null`, `real`, `str`, or `table`. (But `Date`, `DateTime`, and `Real` or
`real_` are fine, since names are case-sensitive and none of the built-in
types contains an underscore or uses uppercase letters.) If whitespace is
wanted one convention is to use underscores in their place.

Once we have defined a _ttype_ we can use it.

Here, we've created a single table whose _ttype_ is “PriceList”. There's no
need to group rows into lines as we've done here (although doing so is
common and easier for human readability), since the UXF processor knows how
many values go into each row based on the number of field names. In this
example, the UXF processor will treat every five values as a single record
(row) since the _ttype_ has five fields.

This is already an improvement on `.csv`—we know the table's name and field
names, and could easily store two or more tables (as we'll see later).
Although the UXF processor will correctly determine the field types, what if
we want to constrain each field's value to a particular type?

    uxf 1.0 Price List
    =PriceList Date:date Price:real Quantity:int ID:str Description:str
    (PriceList
      2022-09-21 3.99 2 <CH1-A2> <Chisels (pair), 1in &amp; 1¼in> 
      2022-10-02 4.49 1 <HV2-K9> <Hammer, 2lb> 
      2022-10-02 5.89 1 <SX4-D1> <Eversure Sealant, 13-floz> 
    )

Here we've added a custom file description in the header, and we've also
added field types to the _ttype_ definition. When types are specified, the
UXF processor is expected to be able to check that each value is of the
correct type. Omit the type altogether (as in the earliler examples) to
indicate _any_ valid table type.

    uxf 1.0 Price List
    =PriceList Date:date Price:real Quantity:int ID:str Description:str
    (PriceList)

Just for completeness, here's an example of an empty price list table.

### INI to UXF

Windows `.ini` format (and Unix's often similar `.conf` format) are commonly
used but unstandardized formats. UXF can be used as a more reliable and
easier to use alternative.

#### INI

    shapename = Hexagon
    zoom = 150
    showtoolbar = False
    [Window1]
    x=615
    y=252
    width=592
    height=636
    scale=1.1
    [Window2]
    x=28
    y=42
    width=140
    height=81
    scale=1.0
    [Window3]
    x=57
    y=98
    width=89
    height=22
    scale=0.5
    [Files]
    current=test1.uxf
    recent1=/tmp/test2.uxf
    recent2=C:\Users\mark\test3.uxf

#### UXF equivalents

This first equivalent is a simplistic conversion that we'll improve in
stages.

    uxf 1.0 MyApp 1.2.0 Config
    =Files Kind Filename
    {
      <General> {
        <shapename> <Hexagon>
        <zoom> 150
        <showtoolbar> no
      }
      <Window1> {
        <x> 615
        <y> 252
        <width> 592
        <height> 636
        <scale> 1.1
      }
      <Window2> {
        <x> 28
        <y> 42
        <width> 140
        <height> 81
        <scale> 1.0
      }
      <Window3> {
        <x> 57
        <y> 98
        <width> 89
        <height> 22
        <scale> 0.5
      }
      <Files> (Files
        <current> <test1.uxf> 
        <recent1> </tmp/test2.uxf> 
        <recent2> <C:\Users\mark\test3.uxf> 
      )
    }

UXF accepts `no` and `yes` for `bool` false and true repectively. (`0` and
`1` cannot be used as ``bool``s since the UXF processor would interpret them
as ``int``s.)

For configuration data it is often convenient to use ``map``s with name
keys and data values. In this case the overall data is a `map` which
contains each configuration section. The values of each of the first two of
the ``map``'s keys are themselves ``map``s. But for the third key's value
we use a `table`. The table's _ttype_ is defined at the start and consists
of two untyped fields.

Of course, we can nest as deep as we like and mix ``list``s and ``map``s.
For example, here's an alternative:

    uxf 1.0 MyApp 1.2.0 Config
    =pos x:int y:int
    =size width:int height:int
    {
      <General> {#<Miscellaneous settings>
        <shapename> <Hexagon> <zoom> 150 <showtoolbar> no <Files> {
          <current> <test1.uxf>
          <recent> [#<From most to least recent>
          </tmp/test2.uxf> <C:\Users\mark\test3.uxf>]
        }
      }
      <Window1> {#<Window dimensions and scales> str
        <pos> (pos 615 252)
        <size> (size 592 636)
        <scale> 1.1
      }
      <Window2> {
        <pos> (pos 28 42)
        <size> (size 140 81)
        <scale> 1.0
      }
      <Window3> {
        <pos> (pos 57 98)
        <size> (size 89 22)
        <scale> 0.5
      }
    }

Here, we've laid out the _General_ and _Window_ maps more compactly. We've
also moved the _Files_ into _General_ and changed the _Files_ from a `table`
to a two-item `map` with the second item's value being a `list` of
filenames. We've also changed the _x_, _y_ coordinates and the _width_ and
_height_ into “pos” and “size” tables. Notice that for each of these tables
we've defined their _ttype_ to include both field names and types.

We've also added some example comments to two of the ``map``s. A comment is
a `#` immediately followed by a `str`. In the data, a comment may only be
placed at the start of a list before the optional _vtype_ or the first
value, or at the start of a map before the optional _ktype_ or the first
key, or at the start of a table before the _ttype_ name.

    uxf 1.0 MyApp 1.2.0 Config
    =pos x:int y:int
    =size width:int height:int
    {#<We want str keys and map values> str map
      <General> {#<We want str keys and any values> str
        <shapename> <Hexagon> <zoom> 150 <showtoolbar> no <Files> {str
          <current> <test1.uxf>
          <recent> [#<From most to least recent> str
          </tmp/test2.uxf> <C:\Users\mark\test3.uxf>]
        }
      }
      <Windows> {#<Window dimensions and scales>
        <pos> (pos 615 252 28 42 57 98)
        <size> (size 592 636 140 81 89 22)
        <scale> [1.1 1.0 0.5]
      }
    }

Here we've added some types. The outermost map must have `str` keys and
`map` values, and the _General_, _Files_, and _Window_ maps must all have
`str` keys and _any_ values. For ``map``s we may specify the key and
value types, or just the key type, or neither. We've also specified that the
_recent_ files ``list``'s values must be ``str``s.

Notice that instead of individual “Windows” entries we've just used one.
Since “pos” and “size” are tables they can have as many rows as we like, in
this case three (since each row has two fields based on each table's
_ttype_).

    uxf 1.0 MyApp 1.2.0 Config
    =#<Window dimensions> Geometry x:int y:int width:int height:int scale:real
    {#<Notes on this configuration file format> str map
      <General> {#<Miscellaneous settings> str
        <shapename> <Hexagon> <zoom> 150 <showtoolbar> no <Files> {str
          <current> <test1.uxf>
          <recent> [#<From most to least recent> str
          </tmp/test2.uxf> <C:\Users\mark\test3.uxf>]
        }
      }
      <Windows> (#<Window dimensions and scales> Geometry
         615 252 592 636 1.1
         28 42 140 81 1.0
         57 98 89 22 0.5
      )
    }

For this variation we've gathered all the window data into a single table
type and laid it out for human readability. We could, of course, have just
written it as `(Geometry 615 252 592 636 1.1 28 42 140 81 1.0 57 98 89 22
0.5)`.

### Database to UXF

Database files aren't normally human readable and usually require
specialized tools to read and modify their contents. Yet many databases are
relatively small (both in size and number of tables), and would be more
convenient to work with if human readable. For these, UXF format provides a
viable alternative.

A UXF equivalent to a database of tables can easily be created using a
`list` of ``table``s:

    uxf 1.0 MyApp Data
    =Customers CID Company Address Contact Email
    =Invoices INUM CID Raised_Date Due_Date Paid Description
    =Items IID INUM Delivery_Date Unit_Price Quantity Description
    [#<There is a 1:M relationship between the Invoices and Items tables>
      (Customers
        50 <Best People> <123 Somewhere> <John Doe> <j@doe.com> 
        19 <Supersuppliers> ? <Jane Doe> <jane@super.com> 
      )
      (Invoices
        152 50 2022-01-17 2022-02-17 no <COD> 
        153 19 2022-01-19 2022-02-19 yes <> 
      )
      (Items
        1839 152 2022-01-16 29.99 2 <Bales of hay> 
        1840 152 2022-01-16 5.98 3 <Straps> 
        1620 153 2022-01-19 11.5 1 <Washers (1-in)> 
      )
    ]

Here we have a `list` of ``table``s representing three database tables.
The `list` begins with a comment.

Notice that the second customer has a null (`?`) address and the second
invoice has an empty description.

    uxf 1.0 MyApp Data
    #<It is also possible to have one overall comment at the beginning,
    after the uxf header and before any ttype definitions or the data.>
    =Customers CID:int Company:str Address:str Contact:str Email:str
    =Invoices INUM:int CID:int Raised_Date:date Due_Date:date Paid:bool Description:str
    =Items IID:int INUM:int Delivery_Date:date Unit_Price:real Quantity:int Description:str
    [#<There is a 1:M relationship between the Invoices and Items tables>
      (Customers
        50 <Best People> <123 Somewhere> <John Doe> <j@doe.com> 
        19 <Supersuppliers> ? <Jane Doe> <jane@super.com> 
      )
      (Invoices
        152 50 2022-01-17 2022-02-17 no <COD> 
        153 19 2022-01-19 2022-02-19 yes <> 
      )
      (Items
        1839 152 2022-01-16 29.99 2 <Bales of hay> 
        1840 152 2022-01-16 5.98 3 <Straps> 
        1620 153 2022-01-19 11.5 1 <Washers (1-in)> 
      )
    ]

Here, we've added types to each table's _ttype_.

It is conventional in a database to have IDs and foreign keys. But these can
often be avoided by using hierarchical data. For example:

    uxf 1.0 MyApp Data
    #<There is a 1:M relationship between the Invoices and Items tables>
    =Database customers:Customers invoices:Invoices
    =Customers CID:int Company:str Address:str Contact:str Email:str
    =Invoices INUM:int CID:int Raised_Date:date Due_Date:date Paid:bool
    Description:str Items:Items
    =Items IID:int Delivery_Date:date Unit_Price:real Quantity:int Description:str
    (Database
        (Customers
        50 <Best People> <123 Somewhere> <John Doe> <j@doe.com> 
        19 <Supersuppliers> ? <Jane Doe> <jane@super.com> 
        )
        (Invoices
        152 50 2022-01-17 2022-02-17 no <COD> (Items
            1839 2022-01-16 29.99 2 <Bales of hay> 
            1840 2022-01-16 5.98 3 <Straps> 
            )
        153 19 2022-01-19 2022-02-19 yes <> (Items
            1620 2022-01-19 11.5 1 <Washers (1-in)> 
            )
        )
    )

Notice that Items no longer need an INUM to identify the Invoice they belong
to because they are nested inside their Invoice. However, the relational
approach has been retained for Customers since more than one Invoice could
be for the same Customer.

In addition, rather than using a simple `list` of tables, we've created a
“Database” _ttype_ and specified it as containing two tables.

What if we wanted to add some extra configuration data to the database? One
solution would be to add a third field to the “Database” _ttype_ (e.g., 
`=Database customers:Customers invoices:Invoices config:map`). Or we could
go further and specify a “Config” _ttype_ and specify the third field as
`config:Config`.

### Additional Examples

See the `testdata` folder for more examples of `.uxf` files (some with other
suffixes). See also the `t` and `eg` folders in each language-specific
library (e.g., `py/t` and `py/eg`) for additional examples.

## Libraries

_Implementations in additional languages are planned._

|**Library**|**Language**|**Notes**                    |
|-----------|------------|-----------------------------|
|uxf        | Python 3   | See the [Python UXF library](py/README.md).|

### Implementation Notes

UXF ``Map``s are unordered, i.e., make no guarantee as to their order. If a
library implementer chooses to make their `Map` representation ordered
(e.g., to simplify regression testing), they are free to do so, since this
doesn't break any guarantees. However, if a `Map` is to be ordered it is
recommended that the ordering be: ``bytes``, ``date``s, ``datetime``s,
``int``s, ``str``s (case-sensitively). For example, a `Map` item with a
`date` key will always follow all items with ``bytes`` keys and precede all
items with `datetime` keys. Similarly all items with `datetime` keys will
precede all items with `int` keys, and all items with `int` keys will
precede all items with `str` keys.

## Imports

UXF files are normally completely self-contained. However, in some cases it
may be desirable to share a set of _ttype_ definitions amongst many UXF
files.

The _disadvantages_ of doing this are: first, that the relevant UXF files
become dependent on one or more external dependencies; second, it is
possible to have import conflicts (i.e., two _ttypes_ with the same name but
different definitions; and third, if URL imports are used, load times will
be affected by network availability and latency. (However, the first and
third disadvantages don't apply if all the dependencies are provided by the
UXF processor itself, i.e., are system imports.)

The _advantage_ of importing _ttype_ definitions is that for UXF's that have
lots of _ttypes_, only the import(s) and the data need be in the file,
without having to repeat all the _ttype_ definitions.

Imports go at the start of the file _after_ the header and _after_ any
file-level comment, and _before_ any _ttype_ definitions. Each import must
be on its own line and may not span lines, nor have comments.

If a filename import has no path or a relative path, the import attempt will
be made relative to the importing `.uxf` file, and failing that, relative to
the current folder, and failing those, relative to each path in the
`UXF_PATH` environment variable (if it exists and is nonempty).

Any _ttype_ definition that follows an import will redefine any imported
defintion of the same name.

|**Import**|**Notes**|
|----------|---------|
|`! complex`|System import of _ttype_ `Complex`|
|`! fraction`|System import of _ttype_ `Fraction`|
|`! numeric`|System import of _ttypes_ `Complex` and `Fraction`|
|`! mydefs.uxi`|Import the _ttypes_ from `mydefs.uxi` in the importing `.uxf` file's folder, or from the current folder, or from a folder in the `UXF_PATH`|
|`! /path/to/shared.uxf`|Import the _ttypes_ from the given file|
|`! http://www.qtrac.eu/ttype-eg.uxf`|Import from the given URL|

Imports with no suffix (e.g., `complex`, `fraction`, `numeric`), are
provided by the UXF processor itself.

The imported file must be a valid UXF file. It need not have a `.uxf` suffix
(e.g., you might prefer `.uxt` or `.uxi`), but must have _a_ suffix (to
distinguish it from a system import), and must have a `.gz` suffix if gzip
compressed. Any custom string, comments, or data the imported file may
contain are ignored: only the _ttype_ definitions are used.

    uxf 1.0
    !complex
    !fraction
    [(Complex 5.1 7.2 8e-2 -9.1e6 0.1 -11.2) <a string> (Fraction 22 7 355 113)]

Here we've used the official system ``complex``'s `Complex` and
``fraction``'s `Fraction` _ttypes_ without having to specify them
explicitly. The data represented is a list consisting of three Complex
numbers each holding two ``real``s each, a `str`, and two Fractions holding
two ``int``s each.

    uxf 1.0
    !numeric
    [(Complex 5.1 7.2 8e-2 -9.1e6 0.1 -11.2) <a string> (Fraction 22 7 355 113)]

This is the same as the previous example, but using the system convenience
`numeric` import to pull in both the `Complex` and `Fraction` _ttypes_.

If you choose to use imports we recommed that UXF files intended for import
_either_ contain a single _ttype_ definition _or_ two or more imports.

We recommend avoiding imports and using stand-alone UXF files wherever
possible. Some UXF processors can do UXF to UXF conversions that will
replace imports with (actually used) _ttype_ definitions. (For example, the
[Python UXF library](py/README.md)'s `uxf.py` module can do this.)

## BNF

A UXF file consists of a mandatory header followed by an optional file-level
comment, optional imports, optional _ttype_ definitions, and then a single
mandatory `list`, `map`, or `table` (which may be empty).

    UXF          ::= 'uxf' RWS VERSION CUSTOM? '\n' CONTENT
    VERSION      ::= /\d+\.\d+/
    CUSTOM       ::= RWS [^\n]+ # user-defined data e.g. filetype and version
    CONTENT      ::= COMMENT? IMPORT* TTYPEDEF* (MAP | LIST | TABLE)
    IMPORT       ::= '!' /\s*/ IMPORT_FILE '\n' # See below for IMPORT_FILE
    TTYPEDEF     ::= '=' COMMENT? OWS IDENFIFIER (RWS FIELD)* # IDENFIFIER is the ttype (i.e., the table name)
    FIELD        ::= IDENFIFIER (OWS ':' OWS VALUETYPE)? # IDENFIFIER is the field name (see note below)
    MAP          ::= '{' COMMENT? MAPTYPES? OWS (KEY RWS VALUE)? (RWS KEY RWS VALUE)* OWS '}'
    MAPTYPES     ::= OWS KEYTYPE (RWS VALUETYPE)?
    KEYTYPE      ::=  'bytes' | 'date' | 'datetime' | 'int' | 'str'
    VALUETYPE    ::= KEYTYPE | 'bool' | 'real' | 'list' | 'map' | 'table' | IDENFIFIER # IDENFIFIER is table name
    LIST         ::= '[' COMMENT? LISTTYPE? OWS VALUE? (RWS VALUE)* OWS ']'
    LISTTYPE     ::= OWS VALUETYPE
    TABLE        ::= '(' COMMENT? OWS IDENFIFIER (RWS VALUE)* ')' # IDENFIFIER is the ttype (i.e., the table name)
    COMMENT      ::= OWS '#' STR
    KEY          ::= BYTES | DATE | DATETIME | INT | STR
    VALUE        ::= KEY | NULL | BOOL | REAL | LIST | MAP | TABLE
    NULL         ::= '?'
    BOOL         ::= 'no' | 'yes'
    INT          ::= /[-+]?\d+/
    REAL         ::= # standard or scientific notation
    DATE         ::= /\d\d\d\d-\d\d-\d\d/ # basic ISO8601 YYYY-MM-DD format
    DATETIME     ::= /\d\d\d\d-\d\d-\d\dT\d\d(:\d\d(:\d\d)?)?/ # see note below
    STR          ::= /[<][^<>]*?[>]/ # newlines allowed, and &amp; &lt; &gt; supported i.e., XML
    BYTES        ::= '(:' (OWS [A-Fa-f0-9]{2})* OWS ':)'
    IDENFIFIER   ::= /[_\p{L}]\w{0,59}/ # Must start with a letter or underscore; may not be a built-in typename or constant
    OWS          ::= /[\s\n]*/
    RWS          ::= /[\s\n]+/ # in some cases RWS is actually optional

Note that a UXF file _must_ contain a single list, map, or table, even if
it is empty.

An `IMPORT_FILE` may be a filename which does _not_  have a file suffix, in
which case it is assumed to be a “system” UXF provided by the UXF processor
itself. (Currently there are just three system UXFs: `complex`, `fraction`,
and `numeric`.) Or it may be a filename with an absolute or relative path.
In the latter case the import is searched for in the importing `.uxf` file's
folder, or the current folder, or a folder in the `UXF_PATH` until it is
found—or not). Or it may be a URL referring to an external UXF file. (See
[Imports](#imports).)

To indicate any type valid for the context, simply omit the type name.

As the BNF shows, `list`, `map`, and `table` values may be of _any_ type
including nested ``list``s, ``map``s, and ``table``s.

For a `table`, after the optional comment, there must be an identifier which
is the table's _ttype_. This is followed by the table's values. There's no
need to distinguish between one row and the next (although it is common to
start new rows on new lines) since the number of fields indicate how many
values each row has. It is possible to create tables that have no fields;
these might be used for representing enumerations or states.

Note that for any given table each field name must be unique.

If a list value, map key, or table value's type is specified, then the UXF
processor is expected to be able to check for (and if requested and
possible, correct) any mistyped values. UXF readers and writers are expected
to preserve map items in the original reading order (first to last, i.e.,
in insertion order).

For ``datetime``'s, only 1-second resolution is supported and no timezones.
If microsecond resolution or timezones are required, consider using custom
_ttypes_, e.g.,

    =Timestamp when:datetime microseconds:real
    =DateTime when:datetime zone:str

Note that a UXF reader (writer) _must_ be able to read (write) a plain text
`.uxf` file containing UTF-8 encoded text, and _ought_ to be able to read
and write gzipped plain text `.uxf.gz` files.

Note also that UXF readers and writers should not care about the actual file
extension (apart from the `.gz` needed for gzipped files), since users are
free to use their own. For example, `data.myapp` and `data.myapp.gz`.

## Supplementary

### Vim Support

If you use the vim editor, simple color syntax highlighting is available.
Copy `uxf.vim` into your `$VIM/syntax/` folder and add these lines (or
similar) to your `.vimrc` or `.gvimrc` file:

    au BufRead,BufNewFile,BufEnter * if getline(1) =~ '^uxf ' | setlocal ft=uxf | endif
    au BufRead,BufNewFile,BufEnter *.uxf set ft=uxf|set expandtab|set tabstop=2|set softtabstop=2|set shiftwidth=2

### UXF Logo

![uxf logo](uxf.svg)

---
