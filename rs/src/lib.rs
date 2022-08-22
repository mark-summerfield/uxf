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

TODO

Comments, ktypes, vtypes, are all strings. If a ktype or vtype is empty this
means that any valid ktype or vtype respectively is acceptable.
For TClasses the ttype is also a string, and this may not be empty.

*/

pub mod constants;
pub mod event;
pub mod field;
pub mod format;
pub mod key;
pub mod lex_token;
pub mod lexer;
pub mod list;
pub mod map;
pub mod parser;
pub mod prettyprint;
pub mod table;
pub mod tclass;
pub mod test_utils;
pub mod util;
pub mod uxf;
pub mod value;

pub use crate::uxf::{parse, parse_options, Uxf};
