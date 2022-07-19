// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/*!

uxf is a library that supports the UXF format. UXF is a plain text human
readable optionally typed storage format. UXF may serve as a convenient
alternative to csv, ini, json, sqlite, toml, xml, or yaml.

TODO

*/

pub mod constants;
pub mod event;
pub mod field;
pub mod list;
pub mod map;
pub mod parser;
pub mod table;
pub mod tclass;
pub mod test_utils;
pub mod util;
pub mod value;

pub use crate::value::Value;
// pub use crate::parser::parser; // etc
