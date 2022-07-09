# Python UXF Library Examples

- [Tlm.py](#tlm-py)
- [include.py](#include-py)
- [merge.py](#merge-py)
- [Slides](#slides)
    - [slides1.py](#slides1-py)
    - [slides2.py](#slides2-py)
- [compare.py](#compare-py)
- [eq.py](#eq-py)
- [Config.py](#config-py)
- [t/ Files](#t--files)
    - [gen.py](#gen-py)
    - [benchmark.py](#benchmark-py)


## Tlm.py

This example shows UXF being used as both a “native” format and an exchange
format for importing and exporting. The main class, `Tlm` holds a track list
and a list of history strings. The `Tlm` can load and save in its own TLM
format, and also seamlessly, a TLM UXF format.

Code-wise, loading needs ~60 lines for TLM and ~23 lines for TLM UXF. This
is because the uxf module does most of the parsing. Saving needs ~18 lines
for TLM and for TLM UXF.

## include.py

This example shows how you might implement an “include” facility in a UXF
file. For example, given:

    uxf 1.0 UXF Include
    #<This is main.uxf>
    =include filename:str
    (include
    <file1.uxf>
    <file2.uxf>
    <file3.uxf>
    )

if you run `include.py main.uxf outfile.uxf` the `outfile.uxf` will have as
    its value a list of three values, the first containing ``file1.uxf``'s
    value, and so on.

This example imports the [merge.py](#merge-py) example.

## merge.py

This example is a little utility for merging two or more UXF files into a
single UXF file.

`usage: merge.py [-l|--list] [-|[-o|--outfile] <outfile>] <infile1> <infile2> [... <infileN>]`

If `-l` or `--list` is specified, the `outfile` will contain a list where
each value is the corresponding ``infile``'s value. The default is for the
`outfile` to contain a `map` where each key is the name of an `infile` and
each value the corresponding ``infile``'s value. The `outfile` will be in
UXF format. If no `outfile` is specified, output is to `stdout`. Regardless
of suffix, all infiles are assumed to be UXF format.

This module can be imported and its `merge()` function used; this is done by
the [include.py](#include-py) example.

## Slides

The `py/eg/slides.sld` file is a very basic UXF format file which defines
some custom _ttypes_ and includes some example slides using this format.

Two examples can read files of this format and output HTML pages as
“slides”; their key difference being the way they handle the UXF `.sld`
file.

### slides1.py

This example uses `Uxf.load()` and then uses the `visit.py` module's
`visit()` function to iterate over the returned `Uxf` object's value to
produce HTML output.

### slides2.py

This example uses `Uxf.load()` and then manually iterates over the returned
`Uxf` object's value to produce HTML output.

## compare.py

This example can be used stand-alone or as an import. It is used to compare
two UXF files for equality (or for equivalence) using the `py/eg/eq.py`
module.

## eq.py

This module provides a single function `eq(a, b)` which compares two UXF
values (i.e., two `Uxf` objects, or two ``List``s, ``Map``s, ``Table``'s or
UXF scalars (``int``s, ``str``s, etc).

## Config.py

This example shows a practical use case of saving and loading application
configuration data, preserving comments, providing defaults, and validating.

The UXF file format used here is very short but also quite complex. It
includes an enumeration with two valid values, and three other custom
_ttypes_. The data is held in a `map` with `str` keys, with one value being
an `int`, another a `list` of ``table``s, and another a `map` with `str`
keys and values.

The `Config` class hides the complexity to present a very simple
property-based API. (Of course there's no free lunch—the API's simplicity is
won at the cost of the `Config` class itself being quite large.)

## t/ Files

These files are in the `py/t` (test) folder but might be interesting or
useful as examples.

### gen.py

This is used to generate a mock UXF file of a size proportional to a given
scale. The default scale of 7 produces a file of around 1 MB. This is
imported by [benchmarks.py](#benchmark-py) but can also be used stand-alone
to create test files for performance testing.

### benchmark.py

This does some load/dump benchmarks and saves previous results in UXF format
in `py/t/benchmark.uxf.gz`.

---
