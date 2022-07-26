// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::list::List;
use crate::map::Map;
use crate::table::Table;
use crate::util::escape;
use anyhow::{bail, Result};
use chrono::prelude::*;
use std::fmt;
use std::fmt::Write as _;

pub type Row = Vec<Value>;

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Bytes(Vec<u8>),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Int(i64),
    List(List),
    Map(Map),
    Real(f64),
    Str(String),
    Table(Table),
}

impl Value {
    /// Returns `true` if `Value::Null`; otherwise returns `false`.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns `true` if this is a scalar (single-valued);
    /// otherwise returns `false`.
    pub fn is_scalar(&self) -> bool {
        matches!(
            self,
            Value::Bool(_)
                | Value::Bytes(_)
                | Value::Date(_)
                | Value::DateTime(_)
                | Value::Int(_)
                | Value::Real(_)
                | Value::Str(_)
        )
    }

    /// Returns `true` if this is a collection (a List, Map, or Table);
    /// otherwise returns `false`.
    pub fn is_collection(&self) -> bool {
        matches!(self, Value::List(_) | Value::Map(_) | Value::Table(_))
    }

    /// Returns `true` if this can be used as a Map key (i.e., Bytes, Date,
    /// Int, or Str); otherwise returns `false`.
    pub fn is_ktype(&self) -> bool {
        matches!(
            self,
            Value::Bytes(_)
                | Value::Date(_)
                | Value::Int(_)
                | Value::Str(_)
        )
    }

    /// Returns `true` if `Value::Bool`; otherwise returns `false`.
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// Returns `true` if `Value::Bytes`; otherwise returns `false`.
    pub fn is_bytes(&self) -> bool {
        matches!(self, Value::Bytes(_))
    }

    /// Returns `true` if `Value::Date`; otherwise returns `false`.
    pub fn is_date(&self) -> bool {
        matches!(self, Value::Date(_))
    }

    /// Returns `true` if `Value::DateTime`; otherwise returns `false`.
    pub fn is_datetime(&self) -> bool {
        matches!(self, Value::DateTime(_))
    }

    /// Returns `true` if `Value::Int`; otherwise returns `false`.
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    /// Returns `true` if `Value::List`; otherwise returns `false`.
    pub fn is_list(&self) -> bool {
        matches!(self, Value::List(_))
    }

    /// Returns `true` if `Value::Map`; otherwise returns `false`.
    pub fn is_map(&self) -> bool {
        matches!(self, Value::Map(_))
    }

    /// Returns `true` if `Value::Real`; otherwise returns `false`.
    pub fn is_real(&self) -> bool {
        matches!(self, Value::Real(_))
    }

    /// Returns `true` if `Value::Str`; otherwise returns `false`.
    pub fn is_str(&self) -> bool {
        matches!(self, Value::Str(_))
    }

    /// Returns `true` if `Value::Table`; otherwise returns `false`.
    pub fn is_table(&self) -> bool {
        matches!(self, Value::Table(_))
    }

    /// Returns `Ok(bool)` if `Value::Bool`; otherwise returns `Err`.
    pub fn as_bool(&self) -> Result<bool> {
        if let Value::Bool(value) = self {
            Ok(*value)
        } else {
            bail!("non-bool Value")
        }
    }

    /// Returns `Ok(&Vec<u8>)` if `Value::Bytes`; otherwise returns `Err`.
    pub fn as_bytes(&self) -> Result<&Vec<u8>> {
        if let Value::Bytes(value) = self {
            Ok(value)
        } else {
            bail!("non-bytes Value")
        }
    }

    /// Returns `Ok(NaiveDate)` if `Value::Date`; otherwise returns `Err`.
    pub fn as_date(&self) -> Result<NaiveDate> {
        if let Value::Date(value) = self {
            Ok(*value)
        } else {
            bail!("non-date Value")
        }
    }

    /// Returns `Ok(NaiveDateTime)` if `Value::DateTime`; otherwise returns
    /// `Err`.
    pub fn as_datetime(&self) -> Result<NaiveDateTime> {
        if let Value::DateTime(value) = self {
            Ok(*value)
        } else {
            bail!("non-datetime Value")
        }
    }

    /// Returns `Ok(i64)` if `Value::Int`; otherwise returns `Err`.
    pub fn as_int(&self) -> Result<i64> {
        if let Value::Int(value) = self {
            Ok(*value)
        } else {
            bail!("non-int Value")
        }
    }

    /// Returns `Ok(&List)` if `Value::List`; otherwise returns `Err`.
    pub fn as_list(&self) -> Result<&List> {
        if let Value::List(value) = self {
            Ok(value)
        } else {
            bail!("non-list Value")
        }
    }

    /// Returns `Ok(&mut List)` if `Value::List`; otherwise returns `Err`.
    pub fn as_list_mut(&mut self) -> Result<&mut List> {
        if let Value::List(value) = self {
            Ok(value)
        } else {
            bail!("non-list Value")
        }
    }

    /// Returns `Ok(&Map)` if `Value::Map`; otherwise returns `Err`.
    pub fn as_map(&self) -> Result<&Map> {
        if let Value::Map(value) = self {
            Ok(value)
        } else {
            bail!("non-map Value")
        }
    }

    /// Returns `Ok(&mut Map)` if `Value::Map`; otherwise returns `Err`.
    pub fn as_map_mut(&mut self) -> Result<&mut Map> {
        if let Value::Map(value) = self {
            Ok(value)
        } else {
            bail!("non-map Value")
        }
    }

    /// Returns `Ok(f64)` if `Value::Real`; otherwise returns `Err`.
    pub fn as_real(&self) -> Result<f64> {
        if let Value::Real(value) = self {
            Ok(*value)
        } else {
            bail!("non-real Value")
        }
    }

    /// Returns `Ok(&str)` if `Value::Str`; otherwise returns `Err`.
    pub fn as_str(&self) -> Result<&str> {
        if let Value::Str(value) = self {
            Ok(value)
        } else {
            bail!("non-str Value")
        }
    }

    /// Returns `Ok(&Table)` if `Value::Table`; otherwise returns `Err`.
    pub fn as_table(&self) -> Result<&Table> {
        if let Value::Table(value) = self {
            Ok(value)
        } else {
            bail!("non-table Value")
        }
    }

    /// Returns `Ok(&mut Table)` if `Value::Table`; otherwise returns `Err`.
    pub fn as_table_mut(&mut self) -> Result<&mut Table> {
        if let Value::Table(value) = self {
            Ok(value)
        } else {
            bail!("non-table Value")
        }
    }

    // Can't be vtype() because VALUE_NAME_NULL "null" is not a valid vtype
    /// Returns "null" if the Value is `Value::Null`; otherwise returns the
    /// Value's `vtype` (`bool`, `bytes', ... `table`).
    pub fn typename(&self) -> &'static str {
        match self {
            Value::Null => VALUE_NAME_NULL,
            Value::Bool(_) => VTYPE_NAME_BOOL,
            Value::Bytes(_) => VTYPE_NAME_BYTES,
            Value::Date(_) => VTYPE_NAME_DATE,
            Value::DateTime(_) => VTYPE_NAME_DATETIME,
            Value::Int(_) => VTYPE_NAME_INT,
            Value::List(_) => VTYPE_NAME_LIST,
            Value::Map(_) => VTYPE_NAME_MAP,
            Value::Real(_) => VTYPE_NAME_REAL,
            Value::Str(_) => VTYPE_NAME_STR,
            Value::Table(_) => VTYPE_NAME_TABLE,
        }
    }
}

impl fmt::Display for Value {
    /// Provides a .to_string() that returns a valid UXF fragment
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Null => "?".to_string(),
                Value::Bool(true) => "yes".to_string(),
                Value::Bool(false) => "no".to_string(),
                Value::Bytes(b) => bytes_to_uxf(b),
                Value::Date(d) => d.format(ISO8601_DATE).to_string(),
                Value::DateTime(dt) =>
                    dt.format(ISO8601_DATETIME).to_string(),
                Value::Int(i) => i.to_string(),
                Value::List(lst) => lst.to_string(),
                Value::Map(m) => m.to_string(),
                Value::Real(r) => {
                    // Must have . or e to it is parsed as real not int
                    let mut s = r.to_string();
                    if !s.contains(&['.', 'e', 'E']) {
                        s.push_str(".0");
                    }
                    s
                }
                Value::Str(s) => format!("<{}>", escape(s)),
                Value::Table(t) => t.to_string(),
            }
        )
    }
}

impl From<Key> for Value {
    fn from(key: Key) -> Self {
        match key {
            Key::Bytes(b) => Value::Bytes(b),
            Key::Date(d) => Value::Date(d),
            Key::Int(i) => Value::Int(i),
            Key::Str(s) => Value::Str(s),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum Key {
    Bytes(Vec<u8>),
    Date(NaiveDate),
    Int(i64),
    Str(String),
}

impl Key {
    /// Returns `true` if `Key::Bytes`; otherwise returns `false`.
    pub fn is_bytes(&self) -> bool {
        matches!(self, Key::Bytes(_))
    }

    /// Returns `true` if `Key::Date`; otherwise returns `false`.
    pub fn is_date(&self) -> bool {
        matches!(self, Key::Date(_))
    }

    /// Returns `true` if `Key::Int`; otherwise returns `false`.
    pub fn is_int(&self) -> bool {
        matches!(self, Key::Int(_))
    }

    /// Returns `true` if `Key::Str`; otherwise returns `false`.
    pub fn is_str(&self) -> bool {
        matches!(self, Key::Str(_))
    }

    /// Returns `Ok(&Vec<u8>)` if `Value::Bytes`; otherwise returns `Err`.
    pub fn as_bytes(&self) -> Result<&Vec<u8>> {
        if let Key::Bytes(value) = self {
            Ok(value)
        } else {
            bail!("non-bytes Key")
        }
    }

    /// Returns `Ok(NaiveDate)` if `Value::Date`; otherwise returns `Err`.
    pub fn as_date(&self) -> Result<NaiveDate> {
        if let Key::Date(value) = self {
            Ok(*value)
        } else {
            bail!("non-date Key")
        }
    }

    /// Returns `Ok(i64)` if `Key::Int`; otherwise returns `Err`.
    pub fn as_int(&self) -> Result<i64> {
        if let Key::Int(value) = self {
            Ok(*value)
        } else {
            bail!("non-int Key")
        }
    }

    /// Returns `Ok(&str)` if `Key::Str`; otherwise returns `Err`.
    pub fn as_str(&self) -> Result<&str> {
        if let Key::Str(value) = self {
            Ok(value)
        } else {
            bail!("non-str Key")
        }
    }
}

// NOTE: *must* match impl fmt::Display for Value
impl fmt::Display for Key {
    /// Provides a .to_string() that returns a valid UXF fragment
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Key::Bytes(b) => bytes_to_uxf(b),
                Key::Date(d) => d.format(ISO8601_DATE).to_string(),
                Key::Int(i) => i.to_string(),
                Key::Str(s) => format!("<{}>", escape(s)),
            }
        )
    }
}

fn bytes_to_uxf(b: &[u8]) -> String {
    let mut s = String::from("(:");
    for x in b {
        let _ = write!(s, "{:02X}", x);
    }
    s.push_str(":)");
    s
}
