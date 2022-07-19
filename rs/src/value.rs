// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::list::List;
use crate::map::Map;
use crate::table::Table;
use anyhow::{bail, Result};
use chrono::prelude::*;

pub type Row = Vec<Option<Value>>;

#[derive(Clone, Debug)]
pub enum Value {
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
