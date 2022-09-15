// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/*!
Uniform eXchange Format (UXF) is a plain text human readable optionally
typed storage format. UXF is designed to make life easier for software
developers and data designers. It directly competes with csv, ini, json,
toml, and yaml formats. One key advantage of UXF is that it supports custom
(i.e., user-defined) types. This can result in more compact, more readable,
and easier to parse data. And in some contexts it may prove to be a
convenient alternative to sqlite or xml.

For details of the Uniform eXchange Format (UXF) supported by this library,
see the [UXF Overview](https://github.com/mark-summerfield/uxf/blob/main/README.md).

**Note that this is WIP — currently the parser is complete, along with
human unfriendly output. Aim is for 1.0.0 to include human-friendly output
(pretty printing).**

# Reading and Writing UXF Files

To read a UXF file into a `Uxf` object use `parse()` (or `parse_options()` for finer control), e.g.:

```rust,ignore
let uxo = uxf::parse(&uxt_or_filename)?;
```

These functions can accept a filename (which may be gzip-compressed if it ends with `.gz`) or the _text_ of a UXF file.

It is also possible to create `Uxf` objects programmatically by creating and
populating a `List`, `Map`, or `Table`; see the corresponding test files for
some basic examples.

To write a `Uxf` object to a string (e.g., to write to a file) using
canonical human-readable output, use `pretty()` (or `pretty_format()` for
more control, or `pretty_options()` for even more control). Or use
`to_string() for bare bones not very human friendly output.

```rust,ignore
let uxt = uxo.pretty()?;
// write uxt of type String to the target...
```

# Dependencies

To use uxf, add this line your `Cargo.toml` file's `[dependencies]`
section:

```toml,ignore
uxf = "1"
```
TODO

Comments, ktypes, vtypes, are all strings. If a ktype or vtype is empty this
means that any valid ktype or vtype respectively is acceptable.
For TClasses the ttype is also a string, and this may not be empty.

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
pub mod test_utils;
pub mod util;
pub mod uxf;
pub mod value;

pub use crate::event::{ignore_event, on_event};
pub use crate::format::Format;
pub use crate::uxf::{parse, parse_options, Compare, ParserOptions, Uxf};
