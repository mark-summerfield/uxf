// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::key::Key;
use crate::list::List;
use crate::map::Map;
use crate::table::Table;
use crate::tclass::TClass;
use crate::util::escape;
use chrono::prelude::*;
use std::fmt::Write as _;
use std::{cell::RefCell, fmt, rc::Rc};

pub type Values = Vec<Value>; // For Lists
pub type Record = Values; // For Tables
pub type Visitor = Rc<dyn Fn(&Value)>;

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

    /// Returns `Some(bool)` if `Value::Bool`; otherwise returns `None`.
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns `Some(&Vec<u8>)` if `Value::Bytes`; otherwise returns
    /// None`.
    pub fn as_bytes(&self) -> Option<&Vec<u8>> {
        if let Value::Bytes(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some(NaiveDate)` if `Value::Date`; otherwise returns
    /// `None`.
    pub fn as_date(&self) -> Option<NaiveDate> {
        if let Value::Date(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns `Some(NaiveDateTime)` if `Value::DateTime`; otherwise
    /// returns `None`.
    pub fn as_datetime(&self) -> Option<NaiveDateTime> {
        if let Value::DateTime(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns `Some(i64)` if `Value::Int`; otherwise returns `None`.
    pub fn as_int(&self) -> Option<i64> {
        if let Value::Int(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns `Some(&List)` if `Value::List`; otherwise returns `None`.
    pub fn as_list(&self) -> Option<&List> {
        if let Value::List(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some(&mut List)` if `Value::List`; otherwise returns
    /// `None`.
    pub fn as_list_mut(&mut self) -> Option<&mut List> {
        if let Value::List(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some(&Map)` if `Value::Map`; otherwise returns `None`.
    pub fn as_map(&self) -> Option<&Map> {
        if let Value::Map(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some(&mut Map)` if `Value::Map`; otherwise returns `None`.
    pub fn as_map_mut(&mut self) -> Option<&mut Map> {
        if let Value::Map(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some(f64)` if `Value::Real`; otherwise returns `None`.
    pub fn as_real(&self) -> Option<f64> {
        if let Value::Real(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns `Some(&str)` if `Value::Str`; otherwise returns `None`.
    pub fn as_str(&self) -> Option<&str> {
        if let Value::Str(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some(&Table)` if `Value::Table`; otherwise returns `None`.
    pub fn as_table(&self) -> Option<&Table> {
        if let Value::Table(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some(&mut Table)` if `Value::Table`; otherwise returns
    /// `None`.
    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        if let Value::Table(value) = self {
            Some(value)
        } else {
            None
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

    /// Iterates over this value and if it is a collection over every
    /// contained value, recursively, calling visitor() once for every
    /// value. List values and Table rows (and values within rows) are
    /// visited in order; Map items are visited in key order, key, then
    /// value, key, then value, etc.
    pub fn visit(&self, visitor: Visitor) {
        (Rc::clone(&visitor))(self);
        match self {
            Value::List(lst) => {
                for value in lst.iter() {
                    value.visit(Rc::clone(&visitor));
                }
            }
            Value::Map(m) => {
                let mut keys: Vec<&Key> = m.inner().keys().collect();
                keys.sort_unstable();
                for key in keys {
                    let key_value = Value::from(key.clone());
                    key_value.visit(Rc::clone(&visitor));
                    m.get(key).unwrap().visit(Rc::clone(&visitor));
                }
            }
            Value::Table(t) => {
                for record in t.iter() {
                    for value in record.iter() {
                        value.visit(Rc::clone(&visitor));
                    }
                }
            }
            _ => (), // already visited at the top
        }
    }

    /// Returns a (possibly empty) vec of all the TClasses in this value and
    /// of any values it contains (iterating recursively using `visit()`).
    pub fn tclasses(&self) -> Vec<TClass> {
        let tclasses = Rc::new(RefCell::new(Vec::<TClass>::new()));
        self.visit({
            let tclasses = Rc::clone(&tclasses);
            Rc::new(move |value: &Value| {
                if let Some(table) = value.as_table() {
                    let mut tclasses = tclasses.borrow_mut();
                    tclasses.push(table.tclass().clone());
                }
            })
        });
        tclasses.take()
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

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<Vec<u8>> for Value {
    fn from(b: Vec<u8>) -> Self {
        Value::Bytes(b)
    }
}

impl From<NaiveDate> for Value {
    fn from(d: NaiveDate) -> Self {
        Value::Date(d)
    }
}

impl From<NaiveDateTime> for Value {
    fn from(dt: NaiveDateTime) -> Self {
        Value::DateTime(dt)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<List> for Value {
    fn from(lst: List) -> Self {
        Value::List(lst)
    }
}

impl From<Map> for Value {
    fn from(m: Map) -> Self {
        Value::Map(m)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Real(f)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Str(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Str(s)
    }
}

impl From<Table> for Value {
    fn from(t: Table) -> Self {
        Value::Table(t)
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

pub(crate) fn bytes_to_uxf(b: &[u8]) -> String {
    let mut s = String::from("(:");
    for x in b {
        let _ = write!(s, "{:02X}", x);
    }
    s.push_str(":)");
    s
}
