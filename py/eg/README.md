# Python UXF Library Examples

- [visit.py](#visit-py)
- [include.py](#include-py)
- [merge.py](#merge-py)
- [slides.py](#slides-py)
- [Tlm.py](#tlm-py)
- [Config.py](#config-py)
- [t/ Files](#t--files)
    - [gen.py](#gen-py)
    - [benchmark.py](#benchmark-py)


## visit.py

This example shows how to use the `Uxf.visit()` method to iterate over a UXF
file's contents. In this case it is used to operate on a file which has
three tables associated with IDs (e.g., like a database) and outputs them as
a hierarchy.

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

## slides.py

The `py/eg/slides.sld` file is a very basic UXF format file which defines
some custom _ttypes_ and includes some example slides using this format.

Two examples can read files of this format and output HTML pages as
“slides”; their key difference being the way they handle the UXF `.sld`
file.

This example uses `Uxf.load()` and then manually iterates over the returned
`Uxf` object's value to produce HTML output.

Using UXF as a markup format isn't ideal, but as this example shows, it can
be done.

## Tlm.py

This example shows UXF being used as both a “native” format and an exchange
format for importing and exporting. The main class, `Tlm` holds a track list
and a list of history strings. The `Tlm` can load and save in its own TLM
format, and also seamlessly, a TLM UXF format.

TLM UXF format is quite complex. It consists of a list which contains Group,
Track, or History tables. Each Group table consists of a name and a list.
This list contains Group or Track tables. (So you can have Groups within
Groups etc.) A Track table's records are filenames and seconds durations. A
History table is just a list of Group names and a track name.

## Config.py

This example shows a practical use case of saving and loading application
configuration data, preserving comments, providing defaults, and validating.

The UXF file format used here is very short but also the most complex of the
examples. It includes an enumeration with two valid values, and three other
custom _ttypes_. The data is held in a `map` with `str` keys, with one value
being an `int`, another a `list` of ``table``s, and another a `map` with
`str` keys and values.

The `Config` class hides the complexity to present a very simple
property-based API. (Of course there's no free lunch—the API's simplicity is
won at the cost of the `Config` class itself being quite large.)

Of course the same data could be expressed more simply as, say, nested maps:
it is for the software developer or data designer to choose the balance
between format complexity, human readability, and programming convenience.

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
