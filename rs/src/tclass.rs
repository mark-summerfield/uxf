// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/*! A TClass represents a Table's type: its ttype (i.e., table type name),
and its fields.

A TClass may have no fields (for a fieldless table), and may have a
comment.

The easiest way to programmatically create a TClass is to use the
make_tclass() function.
*/
use crate::check::check_ttype;
use crate::field::{check_fields, make_field, Field};
use crate::util::escape;
use crate::value::{Record, Value};
use anyhow::{bail, Result};
use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    fmt,
    fmt::Write as _,
};

/// Convenience method for making a TClass from a UXF ttype definition,
/// e.g., `let tclass = make_tclass("=Point x:real y:real").unwrap();`.
/// The leading `=` is optional, but the rest must be a valid UXF ttype
/// definition (but may not contain a comment).
///
/// BNF:
///     TCLASS ::= '=' OWS NAME (OWS NAME (OWS ':' OWS NAME))*
///     NAME ::=/\p{L}\w+/
///     OWS ::= /\s*/
pub fn make_tclass(ttype_definition: &str) -> Result<TClass> {
    let ttype_definition =
        if let Some(stripped) = ttype_definition.strip_prefix('=') {
            stripped
        } else {
            ttype_definition
        };
    let re = regex::Regex::new(r"\s*:\s*").unwrap();
    let normalized = re.replace_all(ttype_definition, ":");
    let mut parts: VecDeque<&str> = normalized.split_whitespace().collect();
    if let Some(ttype) = parts.pop_front() {
        if parts.is_empty() {
            TClass::new_fieldless(ttype, "")
        } else {
            let mut fields = vec![];
            for part in parts {
                fields.push(make_field(part)?);
            }
            TClass::new(ttype, fields, "")
        }
    } else {
        bail!("failed to create a TClass from {ttype_definition:?}");
    }
}

/// Provides a definition of a tclass (`name`, `fields`, and `comment`)
/// for use in ``Table``s.
///
/// ``TClass``es are immutable.
#[derive(Clone, Debug, Eq)]
pub struct TClass {
    ttype: String,
    fields: Vec<Field>,
    comment: String,
    columns_for_names_cache: Option<HashMap<String, usize>>,
}

impl TClass {
    /// Creates a new `TClass` with the given `name`, `fields`, and
    /// `commment` _or_ returns an Err if the `name` is invalid or if
    /// there are duplicate field names.
    /// See `Field::make_fields()` for a function that can generate a
    /// suitable vector of fields.
    /// `TClass` instances are immutable.
    pub fn new(
        ttype: &str,
        fields: Vec<Field>,
        comment: &str,
    ) -> Result<Self> {
        check_ttype(ttype)?;
        check_fields(&fields)?;
        Ok(TClass {
            ttype: ttype.to_string(),
            fields,
            comment: comment.to_string(),
            columns_for_names_cache: None,
        })
    }

    /// Creates a new `TClass` with the given `name`, no `fields`, and
    /// `commment` _or_ returns an Err if the `name` is invalid.
    /// `TClass` instances are immutable.
    pub fn new_fieldless(ttype: &str, comment: &str) -> Result<Self> {
        check_ttype(ttype)?;
        Ok(TClass {
            ttype: ttype.to_string(),
            fields: vec![],
            comment: comment.to_string(),
            columns_for_names_cache: None,
        })
    }

    /// Returns `true` fieldless; otherwise returns `false`.
    pub fn is_fieldless(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns the `ttype`.
    pub fn ttype(&self) -> &str {
        &self.ttype
    }

    /// Returns the optional `comment`.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Returns the optional `comment`.
    pub fn set_comment(&mut self, comment: &str) {
        self.comment = comment.to_string()
    }

    /// Returns the `fields` (which will be empty if `is_fieldless()`).
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    /// Returns the field names (which will be empty if `is_fieldless()`).
    pub fn fieldnames(&self) -> Vec<&str> {
        self.fields.iter().map(|f| f.name()).collect()
    }

    /// Returns how many fields; this will be `0` if `is_fieldless()`.
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns `true` if `is_fieldless()`; otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns a record with `TClass.len()` (i.e., `fields.len()`) fields,
    /// each holding a `Value::Null`.
    /// This is a helper for adding new rows to ``Table``s.
    pub fn record_of_nulls(&self) -> Result<Record> {
        self.record_of_nulls_x("-", 0)
    }

    /// Returns a record with `TClass.len()` (i.e., `fields.len()`) fields,
    /// each holding a `Value::Null`.
    /// This is a helper for adding new rows to ``Table``s.
    pub fn record_of_nulls_x(
        &self,
        filename: &str,
        lino: usize,
    ) -> Result<Record> {
        if self.is_fieldless() {
            bail!(
                "E732:{}:{}:can't create a record of nulls for a \
                fieldless table's tclass",
                filename,
                lino
            )
        }
        let mut record = Record::new();
        record.resize(self.len(), Value::Null);
        Ok(record)
    }

    /// Returns the column for the given fieldname.
    /// More robust in the face of change than using column indexes
    /// directly.
    pub fn column_for_fieldname(
        &mut self,
        fieldname: &str,
    ) -> Option<usize> {
        if self.columns_for_names_cache.is_none() {
            let mut named = HashMap::new();
            for (i, field) in self.fields.iter().enumerate() {
                named.insert(field.name().to_string(), i);
            }
            self.columns_for_names_cache = Some(named);
        }
        if let Some(named) = &self.columns_for_names_cache {
            named.get(fieldname).copied()
        } else {
            None
        }
    }
}

impl Ord for TClass {
    fn cmp(&self, other: &Self) -> Ordering {
        let attype = self.ttype.to_lowercase();
        let bttype = other.ttype.to_lowercase();
        if attype != bttype {
            // prefer case-insensitive ordering
            attype.cmp(&bttype)
        } else if self.ttype != other.ttype {
            self.ttype.cmp(&other.ttype)
        } else {
            // identical names names so use fields to tie-break
            self.fields.cmp(&other.fields)
        }
    }
}

impl PartialOrd for TClass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TClass {
    fn eq(&self, other: &Self) -> bool {
        self.ttype == other.ttype && self.fields == other.fields
    }
}

impl fmt::Display for TClass {
    /// Provides a .to_string() that returns a valid UXF fragment
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("=");
        if !self.comment().is_empty() {
            let _ = write!(s, "#<{}> ", escape(self.comment()));
        }
        s.push_str(&self.ttype);
        if !self.is_fieldless() {
            for field in &self.fields {
                let _ = write!(s, " {}", &field);
            }
        }
        write!(f, "{}", s)
    }
}

/// This allows us to build up a TClass incrementally since a real
/// TClass is immutable.
pub struct TClassBuilder {
    pub ttype: String,
    fields: Vec<Field>,
    comment: String,
}

impl TClassBuilder {
    pub fn new(ttype: &str, comment: &str) -> Self {
        TClassBuilder {
            ttype: ttype.to_string(),
            fields: vec![],
            comment: comment.to_string(),
        }
    }

    pub fn initialize(&mut self, ttype: &str, comment: &str) {
        self.ttype = ttype.to_string();
        self.comment = comment.to_string();
    }

    pub fn clear(&mut self) {
        self.ttype = "".to_string();
        self.comment = "".to_string();
        self.fields.clear();
    }

    pub fn is_valid(&self) -> bool {
        !self.ttype.is_empty()
    }

    pub fn append(&mut self, field: &Field) {
        self.fields.push(field.clone());
    }

    pub fn append_field(
        &mut self,
        ttype: &str,
        comment: &str,
    ) -> Result<()> {
        self.fields.push(Field::new(ttype, comment)?);
        Ok(())
    }

    pub fn build(&self) -> Result<TClass> {
        check_fields(&self.fields)?;
        TClass::new(&self.ttype, self.fields.clone(), &self.comment)
    }
}

impl Default for TClassBuilder {
    fn default() -> Self {
        TClassBuilder {
            ttype: "".to_string(),
            fields: vec![],
            comment: "".to_string(),
        }
    }
}
