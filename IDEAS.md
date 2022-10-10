# Ideas

Ideas under considerations (and [rejected ideas](#rejected-ideas)).

## More UXF library implementations

- C++
- Java?
- JS: use Dart or TypeScript or similar that can output JS?
- ...?

## UXF Improvements

- Add three new pseudo-types:
    - `key` which will accept any ``bytes``, ``date``, ``datetime``,
      ``int``, or ``str``;
    - `scalar` which will accept any ``bool``, ``bytes``, ``date``,
      ``datetime``, ``int``, ``real``, or ``str``;
    - `number` which will accept either ``int`` or ``real``.

- Language: allow '.' in identifiers (excl. first char)?

- Python library: load(), loads(), etc., accept listclass=List,
  mapclass=Map, tableclass=Table, & uses these rather than List, Map, and
  Table, so the user can use their own subclasses

- Silent repairs: rs & py: uxf 1\n{1} → uxf 1\n{} — silent repair; ought to
  error?

## More and better Documentation:

- Complete manual with egs and use cases
    - Part I Preliminaries
    - Part II Practicalities
        Scalars / Collections / Replacing CSV / Replacing INI /
        Replacing JSON / Replacing SQLite / Creating Custom UXF Formats
    - Part III Technicalities
        Railroad Diagrams / BNF / Limits (e.g., date/time/numeric;
        str and bytes lengths; etc) / Lists / Maps / Tables /
        Implementations // Python // Rust? // JS? // ???
- Uniform eXchange Format - a 7"x9" PDF book? (see paper notes)

## Verification suite

Create files (valid & invalid UXF etc) & language/library-neutral scripts
for validating a UXF processor's conformance.

## UXF as Data Storage

Experiment with using UXF format to store various kinds of data, e.g.,
typesetting language, spreadsheet, graphics, etc., & equivalents to other
formats, e.g., geojson, etc.

## UXF Languages

All these using UXF syntax:

- A UXF query language
- A UXF schema language
- A UXF transformation (XSLT-like) language

## GeoUXF

create geo.uxi based on GeoJSON and if successful create:

   uxfgeo.py <infile.{json,geojson,uxf,uxg}> <outfile.{json,geojson,uxf,uxg}>

## Articles

Article(s) on replacing csv with uxf & ini/json/toml with uxf.

## Applications

uxfedit (GUI) application (fltk-rs?)

## Rejected Ideas

### Aliases

**This considerably complicates the type system — is it worth it?**

uxf branch alias

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

**Motivating examples: if there aren't any then this isn't worth doing!**

    uxf 1
    !aliases
    =Point x:number y:number
    [
      [number 1 2.3 4 5.6 7 8.9]
      (Point  1 2.3 4 5.6 7 8.9)
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

A twelfth type) e.g., `MONEY ::= '$' REAL`.

Instead, use, say ``=Money currency:str amount``; or, ``=USD amount``, or
similar, (where amount could be str or int or real

### Datetimes with timezones

Use, say ``=DateTime when:datetime tz:str``, or use say ``=TimeZone
tz:str``, e.g., ``(TimeZone <+00:30> 2021-11-17 ...)``

### Elide ttypes

For example, given ``=Pair a b``

    [Pair (Pair 1 2) (Pair 3 4)] → [Pair (1 2) (3 4)]

redundant because we can rewrite it as this:

    (Pair 1 2 3 4)
