// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/*! A Uxf object represents a UXF file in memory.

A Uxf object holds a single value of type Value. This single value is
always a List, Map, or Table, each of which can hold any number of other
Values, both collections and scalars. In addition a Uxf object holds a
custom string (which may be empty), a comment string (which may be
empty), a HashMap each of whose keys is a ttype name and whose
corresponding value is a TClass, and an IndexMap each of whose keys is a
ttype name and whose corresponding value is an import string.

The easiest way to obtain a Uxf object is to use one of the module level
parse() or parse_options() methods. These accept a UXF file as a string
literal, or a filename (in which case they read the file).

To save a Uxf object use to_text() to obtain a string in human-friendly
form, or to_string() for faster but not particularly friendly
formatting, or write() or write_format() to wite to a file.

Uxf objects can be created entirely programmatically. And all the data
in a Uxf object can be visited using the visit() method.
*/
use crate::consts::*;
use crate::event::{self, OnEventFn};
use crate::format::Format;
use crate::list::List;
use crate::parser;
use crate::pprint;
use crate::tclass::TClass;
use crate::util::{escape, read_file};
use crate::value::{Value, Visit, Visitor};
use anyhow::{bail, Context, Result};
use bitflags::bitflags;
use flate2::{write::GzEncoder, Compression};
use indexmap::map::IndexMap;
use std::{
    collections::{HashMap, HashSet},
    fmt,
    fs::File,
    io::Write,
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

    /// A mutable reference to the collection value. This defaults to an
    /// empty List.
    pub fn value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    /// Sets the collection value which must be a List, Map, or Table.
    /// (Normally a Uxf is created using parse() or parse_options().)
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

    /// Adds a new TClass
    pub fn add_tclass(&mut self, tclass: TClass) {
        self.tclass_for_ttype.insert(tclass.ttype().to_string(), tclass);
    }

    /// Iterates over every value in this Uxf's value; see Value::visit().
    ///
    /// For a very short and simple example see the `Value::tclasses()`
    /// method. For a full example, see the `pprint::tokenizer::tokenize()`
    /// function.
    pub fn visit(&self, visitor: Visitor) -> Result<()> {
        (Rc::clone(&visitor))(Visit::UxfBegin, &self.comment().into())?;
        self.value.visit(Rc::clone(&visitor))?;
        (Rc::clone(&visitor))(Visit::UxfEnd, &Value::Null)
    }

    /// Returns the text of a valid UXF file using the default human
    /// readable `Format` options.
    /// Use `to_string()` for compact output if human readability isn't
    /// needed.
    /// This is a convenience wrapper for
    /// `to_text_format(&Format::default())`
    pub fn to_text(&self) -> String {
        self.to_text_format(&Format::default())
    }

    /// Returns the text of a valid UXF file using the given `Format`
    /// options (or use the `to_text()` convenience method or pass
    /// `Format::default()` for the human readable defaults).
    /// Use `to_string()` for compact output if human readability isn't
    /// needed.
    pub fn to_text_format(&self, format: &Format) -> String {
        pprint::to_text(self, format)
    }

    /// Writes the Uxf's data to the specified filename (gzip-compressing if
    /// the filename ends with `.gz`) using the default human readable
    /// `Format` options and ignoring repair and warning events.
    /// This is a convenience wrapper for
    /// `write_format(&Format::default())`
    pub fn write(&self, filename: &str) -> Result<()> {
        self.write_format(filename, &Format::default())
    }

    /// Writes the Uxf's data to the specified filename (gzip-compressing if
    /// the filename ends with `.gz`) using the given `Format`
    /// options and ignoring repair and warning events.
    ///
    /// (For the most compact output without human friendly formatting, use
    /// `to_string()` and write the text returned to a file ending `.gz`
    /// and using gzip compression.)
    pub fn write_format(
        &self,
        filename: &str,
        format: &Format,
    ) -> Result<()> {
        let mut file = File::create(filename)?;
        let text = self.to_text_format(format);
        if filename.ends_with(".gz") {
            let mut out = GzEncoder::new(file, Compression::best());
            out.write_all(text.as_bytes()).with_context(|| {
                format!("E900:{filename}:0:failed to write gzipped")
            })?;
            out.finish().with_context(|| {
                format!("E901:{filename}:0:failed to gzip")
            })?;
        } else {
            file.write_all(text.as_bytes()).with_context(|| {
                format!("E902:{filename}:0:failed to write")
            })?
        }
        Ok(())
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
        let mut parts = vec![format!("uxf {UXF_VERSION}")];
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
                parts.push(format!("!{import}\n"));
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
