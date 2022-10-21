# Ideas

Ideas under considerations (and [rejected ideas](#rejected-ideas)).

## More UXF library implementations

- C, Go, JavaScript, ...
- C++ — I'm too rusty; hopefully someone else will
- Java — ditto

## UXF Uses

Experiment with using UXF format to store various kinds of data, e.g.,
typesetting language, spreadsheet, graphics, etc., & equivalents to other
formats, e.g., geojson, etc.

### GeoUXF

create geo.uxi based on GeoJSON and if successful create:

```
uxfgeo.py <infile.{json,geojson,uxf,uxg}> <outfile.{json,geojson,uxf,uxg}>
```

## UXF Improvements

- Add three new pseudo-types:
    - `key` which will accept any ``bytes``, ``date``, ``datetime``,
      ``int``, or ``str``;
    - `scalar` which will accept any ``bool``, ``bytes``, ``date``,
      ``datetime``, ``int``, ``real``, or ``str``;
    - `number` which will accept either ``int`` or ``real``.

- Support enums, e.g.,
    `|State Pending Active Finished`
  This would mean we had three fieldless tables `(Pending)` `(Active)` and
  `(Finished)` and could specify them in a _ttype_ using `State`, e.g.,

    ```
    uxf 1
    =Task name:str state:State args:list
    (Task <t1> (Active) [] <t2> (Pending) [1 2 3])
    ```

- Language: allow '.' in identifiers (excl. first char)?

- Python library: ``load()``, ``loads()``, etc., accept ``listclass=List``,
  ``mapclass=Map``, ``tableclass=Table``, & uses these rather than ``List``,
  ``Map``, and ``Table``, so the user can use their own subclasses

- Silent repairs: rs & py: `uxf 1\n{1}` → `uxf 1\n{}` — silent repair; ought
  to error?

## More and better Documentation:

- Complete manual with egs and use cases
    - Part I Preliminaries
    - Part II Practicalities
        Scalars / Collections / Replacing CSV / Replacing INI /
        Replacing JSON / Replacing SQLite / Creating Custom UXF Formats
    - Part III Technicalities
        Railroad Diagrams / BNF / Limits (e.g., date/time/numeric;
        str and bytes lengths; etc) / Lists / Maps / Tables /
        Implementations // Python // Rust // ...
- Uniform eXchange Format - a 7"x9" PDF book? (see paper notes)

## Verification suite

Create files (valid & invalid UXF etc) & language/library-neutral scripts
for validating a UXF processor's conformance.

## UXF Languages

All these using UXF syntax:

### A UXF query language

For example, system import `!query` is the equivalent of `query.uxq`:

    =EQ
    =NEQ
    =LT
    =LTE
    =GT
    =GTE
    =Match op value
    =Table name fields:map
    =List match
    =Map keymatch valuematch
    =Int match
    =Str match
    ...

This would allow queries like:

    uxf 1
    (#<match any table called Point> Table <Point> ?)

    uxf 1
    (#<match any table called Point2D or Point3D>
     Table [<Point2D> <Point3d>] ?)

    uxf 1
    (#<match any table which has x and y fields where the x field's value is
     an int &gt;= 0 and the y field's value is &lt; 0>
     Table ? {<x> (Match (GTE 0)) <y> (Match (LT 0))})

### A UXF schema language

### A UXF transformation (XSLT-like) language

## Articles

Article(s) on replacing csv with uxf & ini/json/toml with uxf.

## Supporting Applications

uxfedit (GUI) application (fltk-rs?)

## Rejected Ideas

### Aliases

_This considerably complicates the type system — is it worth it?_

uxf branch alias

Aliases may be used only within ttype definitions.

#### BNF

    VALUETYPE    ::= KEYTYPE | 'bool' | 'real' | 'list' | 'map' | 'table' | IDENFIFIER # IDENFIFIER is an alias or a table name
    ALIAS        ::= '@' COMMENT? OWS IDENFIFIER (RWS (VALUETYPE | ALIASTYPE))+ # IDENFIFIER is the alias
    ALIASTYPE    ::= '[' OWS (VALUETYPE | ALIASTYPE) OWS ']' '{' OWS KEYTYPE (RWS (VALUETYPE | ALIASTYPE))? '}'

#### Additional system import

    !aliases

which is equivalent to:

    uxf 1
    @key bytes date datetime int str'
    @number int real
    @scalar bool key real

#### Examples

    uxf 1
    !aliases
    @ismap {int str}
    =Point x:number y:number
    =Data name ismap
    [
      [number 1 2.3 4 5.6 7 8.9]
      (Point  1 2.3 4 5.6 7 8.9)
      (Data <thing> {1 <one> 2 <two>})
    ]

### Union Types

Could be done with aliases: too conceptually dificult for broad range of
target users & introduces too much syntactic complexity.

### Notnullability

Could be done with aliases: too conceptually dificult for broad range of
target users & introduces too much syntactic complexity.

### Support for Table max\_records

	TTYPEDEF ::= '=' COMMENT? OWS IDENFIFIER (RWS INT)? (RWS FIELD)*

If present the INT is max\_records: this is not enforced but _is_ warned
when linting; typical value would be 1 for a single record table. For
fieldless tables this _must_ be 0 if present and is automatically 0 if not
present. For all other tables this is None for no limit or an int.

Adds a little complexity for very little benefit.

### Use fieldless tables instead of bools

Replace yes and no bools with built-in fieldless tables, (Y) (N)

### Money type

A twelfth type, e.g., `MONEY ::= '$' REAL`.

Instead, use, say ``=Money currency:str amount``; or, ``=USD amount``, or
similar, (where amount could be str or int or real).

### Datetimes with timezones

Use, say ``=DateTime when:datetime tz:str``, or use say ``=TimeZone
tz:str``, e.g., ``(TimeZone <+00:30> 2021-11-17 ...)``

### Elide ttypes

For example, given ``=Pair a b``

    [Pair (Pair 1 2) (Pair 3 4)] → [Pair (1 2) (3 4)]

redundant because we can rewrite it as this:

    (Pair 1 2 3 4)
