// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::list::List;
use crate::map::Map;
use crate::table::Table;
use anyhow::{bail, Result};
use chrono::prelude::*;
use std::fmt;

// TODO impl Display for to_string()
// TODO docs for every fn
// TODO tests

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
    pub fn is_null(&self) -> bool {
        if let Value::Null = self {
            true
        } else {
            false
        }
    }

    pub fn is_bool(&self) -> bool {
        if let Value::Bool(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_bytes(&self) -> bool {
        if let Value::Bytes(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_date(&self) -> bool {
        if let Value::Date(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_datetime(&self) -> bool {
        if let Value::DateTime(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_int(&self) -> bool {
        if let Value::Int(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_list(&self) -> bool {
        if let Value::List(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_map(&self) -> bool {
        if let Value::Map(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_real(&self) -> bool {
        if let Value::Real(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_str(&self) -> bool {
        if let Value::Str(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_table(&self) -> bool {
        if let Value::Table(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        if let Value::Bool(value) = self {
            Ok(*value)
        } else {
            bail!("non-bool Value")
        }
    }

    pub fn as_bytes(&self) -> Result<&Vec<u8>> {
        if let Value::Bytes(value) = self {
            Ok(value)
        } else {
            bail!("non-bytes Value")
        }
    }

    pub fn as_date(&self) -> Result<NaiveDate> {
        if let Value::Date(value) = self {
            Ok(*value)
        } else {
            bail!("non-date Value")
        }
    }

    pub fn as_datetime(&self) -> Result<NaiveDateTime> {
        if let Value::DateTime(value) = self {
            Ok(*value)
        } else {
            bail!("non-datetime Value")
        }
    }

    pub fn as_int(&self) -> Result<i64> {
        if let Value::Int(value) = self {
            Ok(*value)
        } else {
            bail!("non-int Value")
        }
    }

    pub fn as_list(&self) -> Result<&List> {
        if let Value::List(value) = self {
            Ok(value)
        } else {
            bail!("non-list Value")
        }
    }

    pub fn as_map(&self) -> Result<&Map> {
        if let Value::Map(value) = self {
            Ok(value)
        } else {
            bail!("non-map Value")
        }
    }

    pub fn as_real(&self) -> Result<f64> {
        if let Value::Real(value) = self {
            Ok(*value)
        } else {
            bail!("non-real Value")
        }
    }

    pub fn as_str(&self) -> Result<&str> {
        if let Value::Str(value) = self {
            Ok(value)
        } else {
            bail!("non-str Value")
        }
    }

    pub fn as_table(&self) -> Result<&Table> {
        if let Value::Table(value) = self {
            Ok(value)
        } else {
            bail!("non-table Value")
        }
    }

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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Null => "?".to_string(),
                Value::Bool(true) => "yes".to_string(),
                Value::Bool(false) => "no".to_string(),
                Value::Bytes(b) => format!("{:?}", b), // TODO?
                Value::Date(d) => d.format(ISO8601_DATE).to_string(),
                Value::DateTime(dt) =>
                    dt.format(ISO8601_DATETIME).to_string(),
                Value::Int(i) => i.to_string(),
                Value::List(lst) => lst.to_string(),
                Value::Map(m) => m.to_string(),
                Value::Real(r) => r.to_string(),
                Value::Str(s) => s.to_string(),
                Value::Table(t) => t.to_string(),
            }
        )
    }
}

impl From<Scalar> for Value {
    fn from(scalar: Scalar) -> Self {
        match scalar {
            Scalar::Bool(b) => Value::Bool(b),
            Scalar::DateTime(dt) => Value::DateTime(dt),
            Scalar::Real(r) => Value::Real(r),
        }
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

impl From<Collection> for Value {
    fn from(collection: Collection) -> Self {
        match collection {
            Collection::List(lst) => Value::List(lst),
            Collection::Map(m) => Value::Map(m),
            Collection::Table(t) => Value::Table(t),
        }
    }
}

#[derive(Debug)]
pub enum Scalar {
    Bool(bool),
    DateTime(NaiveDateTime),
    Real(f64),
}

#[derive(Clone, Debug)]
pub enum Key {
    Bytes(Vec<u8>),
    Date(NaiveDate),
    Int(i64),
    Str(String),
}

#[derive(Debug)]
pub enum Collection {
    List(List),
    Map(Map),
    Table(Table),
}
