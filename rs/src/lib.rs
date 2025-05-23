// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/*!
Uniform eXchange Format (UXF) is a plain text human readable optionally
typed storage format that supports custom types.

UXF is designed to make life easier for software developers and data
designers. It directly competes with csv, ini, json, toml, and yaml formats.
One key advantage of UXF is that it supports custom (i.e., user-defined)
types. This can result in more compact, more readable, and easier to parse
data. And in some contexts it may prove to be a convenient alternative to
sqlite or xml.

For details of the Uniform eXchange Format (UXF) supported by this library,
see the [UXF Overview](https://github.com/mark-summerfield/uxf/blob/main/README.md).

# Reading and Writing UXF Files

To read a UXF file into a `Uxf` object use `parse()` (or `parse_options()` for finer control), e.g.:

```rust
let uxt = "uxf 1\n#<File comment>\n{<alpha> 1\n<bravo> 2}\n";
let uxo = uxf::parse(uxt).unwrap(); // -or- pass a filename
assert!(uxt == uxo.to_string());
```

These functions can accept a filename (which may be gzip-compressed if it ends with `.gz`) or the _text_ of a UXF file.

It is also possible to create `Uxf` objects programmatically by creating and
populating a `List`, `Map`, or `Table`; see the corresponding test files for
some basic examples.

To write a `Uxf` object to a string (e.g., to write to a file) using
canonical human-readable output, use `to_text()` (or `to_text_format()` for
more control)). Or use `to_string() for bare bones not very human friendly
output.

```rust
let uxt = "uxf 1\n=Point x:real y:real\n(Point\n  3.4 -7.4\n  8.0 4.2\n)\n";
let uxo1 = uxf::parse(uxt).unwrap(); // -or- pass a filename
assert!(uxt == uxo1.to_text());
let uxo2 = uxf::parse(&uxo1.to_text()).unwrap(); // round-trip tests:
assert!(uxo1 == uxo2);
assert!(uxo1.to_string() == uxo2.to_string());
assert!(uxo1.to_text() == uxo2.to_text());
```

# Dependencies

To use uxf, add this line your `Cargo.toml` file's `[dependencies]`
section: `uxf = "1"`.

# Command Line Interface

The library comes with an application, `uxf` which provides UXF comparisons,
linting, and formatting functionality. Run `uxf -h` for details.

# API Notes

Comments, ktypes, vtypes, are all strings. If a ktype or vtype is empty this
means that any valid ktype or vtype respectively is acceptable.
For TClasses the ttype is also a string, and this may not be empty.

# Application Examples

- The [amp](https://github.com/mark-summerfield/amp-rs) application uses UXF
to store its configuration (see its `src/config.rs` file).
- The [tlm](https://github.com/mark-summerfield/tlm) application uses UXF
for two purposes: to store its configuration (see its `src/config.rs` file),
and for its application data files (see its `src/model.rs` file—in
particular the functions whose names end with `_uxf`).

# Tests

The whitebox unit tests (in the `tests` folder) provide some simple examples
of how to create and modify `Uxf`, `Table`, `Map`, `List`, and `Value`
objects.

Most of the tests are blackbox regression tests from
`../testdata/regression.dat.gz` (itself a UXF file) and using the
`../regression.py` test runner. The test runner will start
`../misc/test_server.py` if necessary, for tests that depend on HTTP
imports.

*/

pub mod check;
pub mod consts;
pub mod event;
pub mod field;
pub mod format;
pub mod key;
pub mod list;
pub mod map;
pub mod parser;
pub mod pprint;
pub mod table;
pub mod tclass;
pub mod util;
pub mod uxf;
pub mod value;

// Public API
pub use crate::consts::{UXF_VERSION, VERSION};
pub use crate::event::{ignore_event, on_event, Event};
pub use crate::field::{make_field, make_fields, Field};
pub use crate::format::Format;
pub use crate::list::List;
pub use crate::map::Map;
pub use crate::table::{NamedRecord, Table};
pub use crate::tclass::{make_tclass, TClass};
pub use crate::uxf::{parse, parse_options, Compare, ParserOptions, Uxf};
pub use crate::value::{Value, Visit, naturalize};
