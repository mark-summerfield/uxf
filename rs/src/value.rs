// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/// A Value object represents a UXF datum in memory.
///
/// A Value may be a scalar or a collection (which in turn may hold scalars
/// or collections, recursively). So a single Value may hold an arbitrarily
/// complex data structure.
use crate::consts::*;
use crate::key::Key;
use crate::list::List;
use crate::map::Map;
use crate::table::Table;
use crate::tclass::TClass;
use crate::util::{escape, isclose64, realstr64};
use crate::uxf::Compare;
use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use std::fmt::Write as _;
use std::{cell::RefCell, fmt, rc::Rc};

pub type Values = Vec<Value>; // For Lists
pub type Record = Values; // For Tables
pub type Visitor = Rc<dyn Fn(Visit, &Value) -> Result<()>>;

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
    /// Convenience for Value::List(List::default())
    pub fn new_list() -> Value {
        Value::List(List::default())
    }

    /// Convenience for Value::Map(Map::default())
    pub fn new_map() -> Value {
        Value::Map(Map::default())
    }

    /// Convenience for Value::Table(Table::new(tclass, ""))
    pub fn new_table(tclass: TClass) -> Value {
        Value::Table(Table::new(tclass, ""))
    }

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

    pub fn naturalize(&self) -> Self {
        // date/times are ASCII so we can use str.len()
        if let Value::Str(value) = self {
            naturalize(value)
        } else {
            self.clone()
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

    /// Add a single Value to a collection Value.
    /// For a List this appends one item.
    /// For a Map this adds a pending key (or returns Err if the Value
    /// isn't a valid key type) or if there's already one, adds an item
    /// with them pending key and the Value.
    /// For a Table this adds a pending field or if this would be the last
    /// one of a record, adds the pending record with this as the last
    /// field.
    /// Returns Ok if successful; otherwise Err: or panics if this isn't a
    /// collection.
    pub fn push(&mut self, value: Value) -> Result<()> {
        match self {
            Value::List(lst) => {
                lst.push(value);
                Ok(())
            }
            Value::Map(m) => m.push(value),
            Value::Table(t) => t.push(value),
            _ => panic!("can't push a value onto a scalar"),
        }
    }

    /// Add multiple Values to a List or Table.
    /// Same behavior as `push()`.
    pub fn push_many(&mut self, values: &[Value]) -> Result<()> {
        match self {
            Value::List(lst) => {
                lst.push_many(values);
                Ok(())
            }
            Value::Table(t) => t.push_many(values),
            Value::Map(_) => {
                panic!("can't push multiple values onto a map")
            }
            _ => panic!("can't push values onto a scalar"),
        }
    }

    /// Add value as a Value to a collection Value.
    /// `push_t(value)` is convenience for `push(value.into())`.
    pub fn push_t<T: Into<Value>>(&mut self, value: T) -> Result<()> {
        self.push(value.into())
    }

    /// Add a single Value to a collection Value or if this is a
    /// nonempty Value::List whose last item is a table and the new value
    /// is a Value::Table with the same ttype as the list's last table item,
    /// combines the new value with the last table item.
    /// Otherwise, behaves the same as for ``push()``.
    pub fn push_or_combine(&mut self, value: Value) -> Result<()> {
        match self {
            Value::List(lst) => {
                if let Some(tbl) = value.as_table() {
                    let tbl_ttype = tbl.ttype();
                    if let Some(end) = lst.last_mut() {
                        let end_ttype =
                            if let Some(end_tbl) = end.as_table() {
                                end_tbl.ttype()
                            } else {
                                ""
                            };
                        if end_ttype == tbl_ttype {
                            let end_tbl = end.as_table_mut().unwrap();
                            for record in tbl.iter() {
                                end_tbl.append(record.clone())?;
                            }
                            return Ok(());
                        }
                    }
                }
                lst.push(value);
                Ok(())
            }
            Value::Map(m) => m.push(value),
            Value::Table(t) => t.push(value),
            _ => panic!("can't push or combine a value onto a scalar"),
        }
    }

    /// `push_or_combine_t(value)` is convenience for
    /// `push_or_combine(value.into())`.
    pub fn push_or_combine_t<T: Into<Value>>(
        &mut self,
        value: T,
    ) -> Result<()> {
        self.push_or_combine(value.into())
    }

    /// Returns how many values, items, or records are in this Value if
    /// it is a collection; otherwise returns None.
    pub fn len(&self) -> Option<usize> {
        match self {
            Value::List(lst) => Some(lst.len()),
            Value::Map(m) => Some(m.len()),
            Value::Table(t) => Some(t.len()),
            _ => None,
        }
    }

    /// Returns true if Value is an empty collection or a scalar;
    /// otherwise returns false.
    pub fn is_empty(&self) -> bool {
        match self {
            Value::List(lst) => lst.is_empty(),
            Value::Map(m) => m.is_empty(),
            Value::Table(t) => t.is_empty(),
            _ => true,
        }
    }

    /// Returns `Some(&Value)` if this is a `Value::List` and `index` is
    /// in bounds; otherwise `None`.
    pub fn get(&self, index: usize) -> Option<&Value> {
        if let Some(lst) = self.as_list() {
            lst.get(index)
        } else {
            None
        }
    }

    /// Returns `Some(&mut Value)` if this is a `Value::List` and `index`
    /// is in bounds; otherwise `None`.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        if let Some(lst) = self.as_list_mut() {
            lst.get_mut(index)
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
    /// value, key, then value, etc. See also `Uxf::visit()`.
    ///
    /// For a very short and simple example see the `Value::tclasses()`
    /// method. For a full example, see the `pprint::tokenizer::tokenize()`
    /// function.
    pub fn visit(&self, visitor: Visitor) -> Result<()> {
        match self {
            Value::List(lst) => {
                (Rc::clone(&visitor))(Visit::ListBegin, self)?;
                for value in lst.iter() {
                    (Rc::clone(&visitor))(
                        Visit::ListValueBegin,
                        &Value::Null,
                    )?;
                    value.visit(Rc::clone(&visitor))?;
                    (Rc::clone(&visitor))(
                        Visit::ListValueEnd,
                        &Value::Null,
                    )?;
                }
                (Rc::clone(&visitor))(Visit::ListEnd, &Value::Null)?;
            }
            Value::Map(m) => {
                (Rc::clone(&visitor))(Visit::MapBegin, self)?;
                for key in m.sorted_keys() {
                    (Rc::clone(&visitor))(
                        Visit::MapItemBegin,
                        &Value::Null,
                    )?;
                    // A key is never a collection
                    let key_value = Value::from(key.clone());
                    (Rc::clone(&visitor))(Visit::Value, &key_value)?;
                    m.get(key).unwrap().visit(Rc::clone(&visitor))?;
                    (Rc::clone(&visitor))(Visit::MapItemEnd, &Value::Null)?;
                }
                (Rc::clone(&visitor))(Visit::MapEnd, &Value::Null)?;
            }
            Value::Table(t) => {
                (Rc::clone(&visitor))(Visit::TableBegin, self)?;
                for record in t.iter() {
                    (Rc::clone(&visitor))(
                        Visit::TableRecordBegin,
                        &Value::Null,
                    )?;
                    for value in record.iter() {
                        value.visit(Rc::clone(&visitor))?;
                    }
                    (Rc::clone(&visitor))(
                        Visit::TableRecordEnd,
                        &Value::Null,
                    )?;
                }
                (Rc::clone(&visitor))(Visit::TableEnd, &Value::Null)?;
            }
            _ => (Rc::clone(&visitor))(Visit::Value, self)?,
        }
        Ok(())
    }

    /// Returns a (possibly empty) vec of all the TClasses in this value and
    /// of any values it contains (iterating recursively using `visit()`).
    pub fn tclasses(&self) -> Vec<TClass> {
        let tclasses = Rc::new(RefCell::new(Vec::<TClass>::new()));
        let _ = self.visit({
            // Should only return Ok
            let tclasses = Rc::clone(&tclasses);
            Rc::new(move |_: Visit, value: &Value| {
                if let Some(table) = value.as_table() {
                    let mut tclasses = tclasses.borrow_mut();
                    tclasses.push(table.tclass().clone());
                }
                Ok(())
            })
        });
        tclasses.take()
    }

    /// Returns `true` if this `Value` and the `other` `Value` are the same
    /// (or contain the same maps, lists, or tables, in the same order),
    /// Set `compare` to `EQUIVALENT` or `IGNORE_COMMENTS` if comment
    /// differences don't matter.
    /// See also `==` and `Uxf::is_equivalent()`.
    pub fn is_equivalent(&self, other: &Value, compare: Compare) -> bool {
        if self.is_collection() && other.is_collection() {
            if let Some(alst) = self.as_list() {
                if let Some(blst) = other.as_list() {
                    return alst.is_equivalent(blst, compare);
                } else {
                    return false;
                }
            } else if let Some(am) = self.as_map() {
                if let Some(bm) = other.as_map() {
                    return am.is_equivalent(bm, compare);
                } else {
                    return false;
                }
            } else if let Some(at) = self.as_table() {
                if let Some(bt) = other.as_table() {
                    return at.is_equivalent(bt, compare);
                } else {
                    return false;
                }
            }
        }
        self == other
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
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
                Value::Real(r) => realstr64(*r),
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

impl From<usize> for Value {
    fn from(i: usize) -> Self {
        Value::Int(i as i64)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Int(i as i64)
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

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::Real(f as f64)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Real(f)
    }
}

impl From<&str> for Value {
    /// Converts a &str to a Value::Str.
    /// Use naturalize(str) to _parse_ a str into a Value::Bool or
    /// Value::Int or ...
    fn from(s: &str) -> Self {
        Value::Str(s.to_string())
    }
}

impl From<String> for Value {
    /// Converts a String to a Value::Str.
    /// Use naturalize(str) to _parse_ a str into a Value::Bool or
    /// Value::Int or ...
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
    /// Converts a Map's Key into a Value::Bytes or Value::Date or
    /// Value::DateTime or Value::Int or Value::Str as appropriate.
    fn from(key: Key) -> Self {
        match key {
            Key::Bytes(b) => Value::Bytes(b),
            Key::Date(d) => Value::Date(d),
            Key::DateTime(d) => Value::DateTime(d),
            Key::Int(i) => Value::Int(i),
            Key::Str(s) => Value::Str(s),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Null => other.is_null(),
            Value::Bool(b) => {
                if let Some(other) = other.as_bool() {
                    *b == other
                } else {
                    false
                }
            }
            Value::Bytes(b) => {
                if let Some(other) = other.as_bytes() {
                    b == other
                } else {
                    false
                }
            }
            Value::Date(d) => {
                if let Some(other) = other.as_date() {
                    *d == other
                } else {
                    false
                }
            }
            Value::DateTime(dt) => {
                if let Some(other) = other.as_datetime() {
                    *dt == other
                } else {
                    false
                }
            }
            Value::Int(i) => {
                if let Some(other) = other.as_int() {
                    *i == other
                } else {
                    false
                }
            }
            Value::Str(s) => {
                if let Some(other) = other.as_str() {
                    s == other
                } else {
                    false
                }
            }
            Value::Real(r) => {
                if let Some(other) = other.as_real() {
                    isclose64(*r, other)
                } else {
                    false
                }
            }
            Value::List(lst) => {
                if let Some(other) = other.as_list() {
                    lst == other
                } else {
                    false
                }
            }
            Value::Map(m) => {
                if let Some(other) = other.as_map() {
                    m == other
                } else {
                    false
                }
            }
            Value::Table(t) => {
                if let Some(other) = other.as_table() {
                    t == other
                } else {
                    false
                }
            }
        }
    }
}

impl Eq for Value {}

#[derive(Clone, Debug)]
pub enum Visit {
    UxfBegin,
    UxfEnd,
    ListBegin,
    ListEnd,
    ListValueBegin,
    ListValueEnd,
    MapBegin,
    MapEnd,
    MapItemBegin,
    MapItemEnd,
    TableBegin,
    TableEnd,
    TableRecordBegin,
    TableRecordEnd,
    Value,
}

pub(crate) fn bytes_to_uxf(b: &[u8]) -> String {
    let mut s = String::from("(:");
    for x in b {
        let _ = write!(s, "{:02X}", x);
    }
    s.push_str(":)");
    s
}

/// Convert a string into a Value of the most appropriate type (Value::Bool,
/// Value::Int, Value::Real, Value::Date, or Value::DateTime); otherwise as
/// a Value::Str.
/// If you just want to convert a &str or String as-is to a Value::Str, use
/// .into() or .from().
pub fn naturalize(value: &str) -> Value {
    // date/times are ASCII so we can use str.len()
    let uvalue = value.to_uppercase();
    if ["T", "TRUE", "Y", "YES"].contains(&uvalue.as_str()) {
        return Value::Bool(true);
    }
    if ["F", "FALSE", "N", "NO"].contains(&uvalue.as_str()) {
        return Value::Bool(false);
    }
    if let Ok(i) = value.parse::<i64>() {
        return Value::Int(i);
    } else if let Ok(r) = value.parse::<f64>() {
        return Value::Real(r);
    } else if value.len() == 10 {
        if let Ok(d) = NaiveDate::parse_from_str(value, ISO8601_DATE) {
            return Value::Date(d);
        }
    } else if value.len() == 13 {
        if let Ok(dt) =
            NaiveDateTime::parse_from_str(value, ISO8601_DATETIME_H)
        {
            return Value::DateTime(dt);
        }
    } else if value.len() == 16 {
        if let Ok(dt) =
            NaiveDateTime::parse_from_str(value, ISO8601_DATETIME_M)
        {
            return Value::DateTime(dt);
        }
    } else if value.len() == 19 {
        if let Ok(dt) =
            NaiveDateTime::parse_from_str(value, ISO8601_DATETIME)
        {
            return Value::DateTime(dt);
        }
    }
    Value::Str(value.to_string())
}
