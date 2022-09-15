// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::consts::*;
use crate::event::{self, OnEventFn};
use crate::format::Format;
use crate::list::List;
use crate::parser;
use crate::pprint;
use crate::tclass::TClass;
use crate::util::{escape, read_file};
use crate::value::{Value, Visit, Visitor};
use anyhow::{bail, Result};
use bitflags::bitflags;
use indexmap::map::IndexMap;
use std::{
    collections::{HashMap, HashSet},
    fmt,
    rc::Rc,
};

#[derive(Clone)]
pub struct Uxf {
    custom: String,
    comment: String,
    value: Value, // NOTE must be Value::List | Value::Map | Value::Table
    pub(crate) tclass_for_ttype: HashMap<String, TClass>, // ttype x TClass
    pub(crate) import_for_ttype: IndexMap<String, String>, // ttype x import
}

impl Uxf {
    /// Returns a `Uxf` with the given `custom` and `comment` strings and
    /// containing an empty list.
    pub fn new(custom: &str, comment: &str) -> Self {
        Uxf {
            custom: custom.to_string(),
            comment: comment.to_string(),
            value: Value::List(List::default()),
            tclass_for_ttype: HashMap::new(),
            import_for_ttype: IndexMap::new(),
        }
    }

    /// Returns the `custom` which may be `""`.
    pub fn custom(&self) -> &str {
        &self.custom
    }

    /// Use to change the custom text
    pub fn set_custom(&mut self, custom: &str) {
        self.custom = custom.to_string();
    }

    /// Returns the `comment` which may be `""`.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Use to change the comment text
    pub fn set_comment(&mut self, comment: &str) {
        self.comment = comment.to_string();
    }

    /// The collection value. This defaults to an empty List.
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Sets the collection value which must be a List, Map, or Table.
    /// Normally the Uxf is populated using from_str(), parse(), or
    /// parse_options().
    pub fn set_value(&mut self, value: Value) -> Result<()> {
        if !value.is_collection() {
            bail!(
                "E100:-:0:Uxf value must be a List, Map, or Table, got {}",
                value.typename()
            )
        }
        self.import_for_ttype.clear();
        self.tclass_for_ttype.clear();
        for tclass in value.tclasses() {
            self.tclass_for_ttype
                .insert(tclass.ttype().to_string(), tclass);
        }
        self.value = value;
        Ok(())
    }

    /// Returns a &TClass for the given ttype or None
    pub fn tclass(&self, ttype: &str) -> Option<&TClass> {
        self.tclass_for_ttype.get(ttype)
    }

    /// Iterates over every value in this Uxf's value; see Value::visit().
    pub fn visit(&self, visitor: Visitor) {
        (Rc::clone(&visitor))(Visit::UxfBegin, &Value::Null);
        self.value.visit(Rc::clone(&visitor));
        (Rc::clone(&visitor))(Visit::UxfEnd, &Value::Null);
    }

    /// Returns the text of a valid UXF file using the default human
    /// readable `Format` options and using the default `on_event` event
    /// handler.
    /// Use `to_string()` for compact output if human readability isn't
    /// needed.
    /// This is a convenience wrapper for
    /// `to_string_options(&Format::default(), None)`
    pub fn to_str(&self) -> Result<String> {
        self.to_string_options(&Format::default(), None)
    }

    /// Returns the text of a valid UXF file using the given `Format`
    /// options (or use `Format::default()` for the human readable defaults)
    /// and using the default `on_event` event handler.
    /// Use `to_string()` for compact output if human readability isn't
    /// needed.
    /// This is a convenience wrapper for `to_string_options(&format, None)`
    pub fn to_string_format(&self, format: &Format) -> Result<String> {
        self.to_string_options(format, None)
    }

    /// Returns the text of a valid UXF file using the given `Format`
    /// options (or use `Format::default()` for the human readable defaults)
    /// and using the given `on_event` event handler (or the default
    /// handler if `None`).
    /// Use `to_string()` for compact output if neither human readability
    /// nor custom event handling is needed.
    pub fn to_string_options(
        &self,
        format: &Format,
        on_event: Option<OnEventFn>,
    ) -> Result<String> {
        pprint::pprint(format, on_event)
    }

    /// Returns `true` if this `Uxf` and the `other` `Uxf` have the same
    /// values (and for any contained lists or tables, in the same order),
    /// and with the same imports and _ttypes_ if `compare` is `default()`
    /// (although in such cases simply use `==` or `!=`).
    /// Set `compare` to `EQUIVALENT` if comment differences don't matter
    /// and if imports and _ttype_ definitions don't matter except that both
    /// define or import and use the same _ttypes_.
    /// See also `==`.
    pub fn is_equivalent(&self, other: &Uxf, compare: Compare) -> bool {
        if self.custom != other.custom {
            return false;
        }
        if !compare.contains(Compare::IGNORE_COMMENTS)
            && self.comment != other.comment
        {
            return false;
        }
        if !compare.contains(Compare::IGNORE_IMPORTS)
            && self.import_for_ttype != other.import_for_ttype
        {
            return false;
        }
        if !compare.contains(Compare::IGNORE_UNUSED_TTYPES)
            && self.tclass_for_ttype != other.tclass_for_ttype
        {
            // This means that we only compare actually used ttypes when
            // comparing any tables.
            return false;
        }
        self.value.is_equivalent(&other.value, compare)
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Compare: u8 {
        const IGNORE_COMMENTS = 0b001;
        const IGNORE_UNUSED_TTYPES = 0b010;
        const IGNORE_IMPORTS = 0b100;
        const EQUIVALENT = Self::IGNORE_COMMENTS.bits |
            Self::IGNORE_UNUSED_TTYPES.bits | Self::IGNORE_IMPORTS.bits;
    }
}

impl Default for Uxf {
    /// Returns a new `Uxf` with empty custom and comment strings
    /// and containing an empty list
    fn default() -> Self {
        Uxf {
            custom: "".to_string(),
            comment: "".to_string(),
            value: Value::List(List::default()),
            tclass_for_ttype: HashMap::new(),
            import_for_ttype: IndexMap::new(),
        }
    }
}

impl PartialEq for Uxf {
    /// Returns `true` if this `Uxf` and the `other` `Uxf` have the same
    /// values (and for any contained lists or tables, in the same order),
    /// with the same imports and the same _ttypes_.
    /// See also `is_equivalent()`.
    fn eq(&self, other: &Self) -> bool {
        if self.custom != other.custom {
            return false;
        }
        if self.comment != other.comment {
            return false;
        }
        if self.import_for_ttype != other.import_for_ttype {
            return false;
        }
        if self.tclass_for_ttype != other.tclass_for_ttype {
            return false;
        }
        self.value == other.value
    }
}

impl Eq for Uxf {}

impl fmt::Debug for Uxf {
    fn fmt<'a>(&'a self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Uxf")
            .field("custom", &self.custom)
            .field("comment", &self.comment)
            .field("value", &self.value)
            .finish()
    }
}

impl fmt::Display for Uxf {
    /// Provides a .to_string() that returns the text of a valid UXF file.
    /// Use `to_string_options(&Format, Option<OnEventFn>)` to control
    /// output formatting and event handling.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const NL: &str = "\n";
        let mut parts = vec![format!("uxf {}", UXF_VERSION)];
        if !self.custom().is_empty() {
            parts.push(" ".to_string());
            parts.push(self.custom().to_string());
        }
        parts.push(NL.to_string());
        if !self.comment().is_empty() {
            parts.push(format!("#<{}>", escape(self.comment())));
            parts.push(NL.to_string());
        }
        // Output unique imports in order of insertion
        let mut seen: HashSet<&str> = HashSet::new();
        for (_, import) in &self.import_for_ttype {
            if !seen.contains(import.as_str()) {
                parts.push(format!("!{}\n", &import));
                seen.insert(import);
            }
        }
        let mut tclasses: Vec<TClass> =
            self.tclass_for_ttype.values().cloned().collect();
        tclasses.sort_unstable(); // Use alphabetical order
        for tclass in tclasses.iter() {
            if !self.import_for_ttype.contains_key(tclass.ttype()) {
                parts.push(tclass.to_string());
                parts.push(NL.to_string());
            }
        }
        parts.push(self.value.clone().to_string());
        parts.push(NL.to_string());
        write!(f, "{}", parts.join(""))
    }
}

/// If `uxt_or_filename`' contains '\n` it is taken to be a UXF file
/// in a string; otherwise it is taken to be the name of file (which
/// may be gzipped if the filename ends `.gz`). In the latter case,
/// the file's text is read.
/// Then in either case the UXF text is parsed into a `Uxf` object if
/// possible, using the default `on_event` event handler.
/// This is just a convenience wrapper for
/// `parse_options(uxt_or_filename, ParserOptions::default(), None)`
pub fn parse(uxt_or_filename: &str) -> Result<Uxf> {
    parse_options(uxt_or_filename, ParserOptions::default(), None)
}

/// If `uxt_or_filename`' contains '\n` it is taken to be a UXF file
/// in a string; otherwise it is taken to be the name of file (which
/// may be gzipped if the filename ends `.gz`). In the latter case,
/// the file's text is read.
/// Then in either case the UXF text is parsed into a `Uxf` object if
/// possible, dropping unused _ttypes_ if `options` is `DROP_UNUSED_TTYPES`
/// or `AS_STANDALONE` and replacing imports with the _ttypes_ they import
/// if `options` is `REPLACE_IMPORTS` or `AS_STANDALONE` and using the given
/// `on_event` event handler (or the default handler if `None`).
pub fn parse_options(
    uxt_or_filename: &str,
    options: ParserOptions,
    on_event: Option<OnEventFn>,
) -> Result<Uxf> {
    let on_event = on_event.unwrap_or_else(|| Rc::new(event::on_event));
    let filename: &str;
    let text: String;
    if !uxt_or_filename.contains(NL) {
        text = read_file(uxt_or_filename)?;
        filename = uxt_or_filename;
    } else {
        text = uxt_or_filename.to_string();
        filename = "-";
    }
    parser::parse(&text, filename, options, Rc::clone(&on_event))
}

bitflags! {
    #[derive(Default)]
    pub struct ParserOptions: u8 {
        const DEFAULT = 0b00;
        const DROP_UNUSED_TTYPES = 0b01;
        const REPLACE_IMPORTS = 0b10;
        const AS_STANDALONE = Self::DROP_UNUSED_TTYPES.bits |
            Self::REPLACE_IMPORTS.bits;
    }
}
