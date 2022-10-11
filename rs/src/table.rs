// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/// The `Table` type is used to store zero or more records (always zero in
/// the case of a fieldless table).
///
/// A Table also has a TClass a ttype (the TClass's name), and a (possibly
/// empty) comment.
///
/// The safest way to access a record is using one of the `*_named()`
/// methods, e.g., `first_named(), `last_named()`, or `get_named(row)`.
/// These return a `NamedRecord` (a `HashMap<&str, &Value>`) whose keys are
/// field names and whose values are the corresponding field values. Using
/// these methods is more robust in the face of change since they are not
/// field-index dependent as the `first()`, `last()`, and `get(row)` methods
/// are.
///
/// The easiest way to create a Table is to use Table::new() with the TClass
/// provided by the make_tclass() function.
use crate::tclass::TClass;
use crate::util::escape;
use crate::uxf::Compare;
use crate::value::{Record, Value, Values};
use anyhow::{bail, Result};
use std::{
    collections::HashMap,
    fmt,
    ops::{Index, IndexMut},
};

/// The keys are field names and the values are the corresponding field
/// values.
pub type NamedRecord<'a> = HashMap<&'a str, &'a Value>;

#[derive(Clone, Debug)]
pub struct Table {
    tclass: TClass,
    comment: String,
    pending_record: Record,
    records: Vec<Record>,
}

impl Table {
    /// Returns a new Table with the given `TClass` and `comment` and no
    /// records.
    /// The `TClass` and `comment` are immutable after construction.
    pub fn new(tclass: TClass, comment: &str) -> Self {
        Table {
            tclass,
            comment: comment.to_string(),
            pending_record: Values::new(),
            records: vec![],
        }
    }

    /// Returns a new Table with a fieldless`TClass` (with the given
    /// `ttype` and and no `TClass` comment), and the given `comment`
    /// and no records or `Err` if the `ttype` is invalid.
    /// The `TClass` and `comment` are immutable after construction.
    pub fn new_fieldless(ttype: &str, comment: &str) -> Result<Self> {
        Ok(Table {
            tclass: TClass::new_fieldless(ttype, "")?,
            comment: comment.to_string(),
            pending_record: Values::new(),
            records: vec![],
        })
    }

    /// Returns the `TClass`.
    pub fn tclass(&self) -> &TClass {
        &self.tclass
    }

    /// Returns the `ttype`.
    pub fn ttype(&self) -> &str {
        self.tclass.ttype()
    }

    /// Returns the `ttype`'s number of fields.
    pub fn ttype_len(&self) -> usize {
        self.tclass.len()
    }

    /// Returns whether the `ttype` is fieldless.
    pub fn is_fieldless(&self) -> bool {
        self.tclass.is_fieldless()
    }

    /// Returns the `comment` which may be `""`.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Returns the number of records in the table.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns `true` if the table is empty; otherwise returns `false`.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns the record as a `Some(NamedRecord)` whose keys are
    /// field names and whose values are field values at `row` 0 if any;
    /// otherwise `None`.
    pub fn first_named(&self) -> Option<NamedRecord> {
        self.get_named(0)
    }

    /// Returns `Some(&Record)` at row 0 if any; otherwise `None`.
    pub fn first(&self) -> Option<&Record> {
        self.records.get(0)
    }

    /// Returns the record as a `Some(NamedRecord)` whose keys are
    /// field names and whose values are field values at `row` 1 if any;
    /// otherwise `None`.
    pub fn second_named(&self) -> Option<NamedRecord> {
        self.get_named(1)
    }

    /// Returns `Some(&Record)` at row 1 if any; otherwise `None`.
    pub fn second(&self) -> Option<&Record> {
        self.records.get(1)
    }

    /// Returns the record as a `Some(NamedRecord)` whose keys are
    /// field names and whose values are field values at `row` 2 if any;
    /// otherwise `None`.
    pub fn third_named(&self) -> Option<NamedRecord> {
        self.get_named(2)
    }

    /// Returns `Some(&Record)` at row 2 if any; otherwise `None`.
    pub fn third(&self) -> Option<&Record> {
        self.records.get(2)
    }

    /// Returns the record as a `Some(NamedRecord)` whose keys are
    /// field names and whose values are field values at `row` 3 if any;
    /// otherwise `None`.
    pub fn fourth_named(&self) -> Option<NamedRecord> {
        self.get_named(3)
    }

    /// Returns `Some(&Record)` at row 3 if any; otherwise `None`.
    pub fn fourth(&self) -> Option<&Record> {
        self.records.get(3)
    }

    /// Returns the last record as a `Some(NamedRecord)` whose
    /// keys are field names and whose values are field values if the table
    /// isn't empty; otherwise `None`.
    pub fn last_named(&self) -> Option<NamedRecord> {
        self.get_named(self.len() - 1)
    }

    /// Returns the last `Some(&Record)` if the table isn't empty;
    /// otherwise `None`.
    pub fn last(&self) -> Option<&Record> {
        self.records.get(self.len() - 1)
    }

    /// Returns a record as a `Some(NamedRecord)` whose keys are
    /// field names and whose values are field values if `row` is in
    /// bounds; otherwise `None`.
    pub fn get_named(&self, row: usize) -> Option<NamedRecord> {
        if let Some(record) = self.records.get(row) {
            let mut named = HashMap::new();
            for (i, name) in self.tclass.fieldnames().iter().enumerate() {
                named.insert(*name, &record[i]);
            }
            Some(named)
        } else {
            None
        }
    }

    /// Returns `Some(&Record)` if `row` is in bounds; otherwise `None`.
    pub fn get(&self, row: usize) -> Option<&Record> {
        self.records.get(row)
    }

    /// Returns `Some(&mut Record)` if `row` is in bounds; otherwise
    /// `None`.
    pub fn get_mut(&mut self, row: usize) -> Option<&mut Record> {
        self.records.get_mut(row)
    }

    /// Returns `Some(&Value)` if `row` is in bounds and `fieldname`
    /// is valid; otherwise `None`.
    pub fn get_field(
        &mut self,
        row: usize,
        fieldname: &str,
    ) -> Option<&Value> {
        if let Some(column) = self.tclass.column_for_fieldname(fieldname) {
            if let Some(record) = self.records.get(row) {
                record.get(column)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns `Some(&mut Value)` if `row` is in bounds and `fieldname`
    /// is valid; otherwise `None`.
    ///
    /// ```
    /// let fields = uxf::make_fields(&[("x", "real"),
    ///                                 ("y", "real")]).unwrap();
    /// let point_tclass = uxf::TClass::new("Point", fields, "").unwrap();
    /// let mut table = uxf::Table::new(point_tclass, "");
    /// table.append(vec![6.7.into(), 11.1.into()]);
    /// if let Some(record) = table.first() {
    ///     assert_eq!("11.1", &format!("{}", &record[1])); // fragile index
    /// }
    /// if let Some(record) = table.get_mut(0) {
    ///     record[1] = 22.2.into();
    /// }
    /// if let Some(record) = table.first() {
    ///     assert_eq!("22.2", &format!("{}", &record[1])); // fragile index
    /// }
    /// // More robust:
    /// if let Some(value) = table.get_field_mut(0, "y") { // robust field name
    ///     *value = 44.4.into();
    /// }
    /// if let Some(record) = table.first_named() {
    ///     if let Some(value) = record.get("y") { // robust field name
    ///         assert_eq!("44.4", &format!("{}",
    ///                    value.as_real().unwrap()));
    ///     }
    /// }
    /// ```
    pub fn get_field_mut(
        &mut self,
        row: usize,
        fieldname: &str,
    ) -> Option<&mut Value> {
        if let Some(column) = self.tclass.column_for_fieldname(fieldname) {
            if let Some(record) = self.records.get_mut(row) {
                record.get_mut(column)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Appends the given `record` of `Value`s to the end of the table or
    /// returns `Err` if `record` doesn't have `Table::ttype_len()` values
    /// or if this is a fieldless table.
    pub fn append(&mut self, record: Record) -> Result<()> {
        self.append_x(record, "-", 0)
    }

    /// Appends the given `record` of `Value`s to the end of the table or
    /// returns `Err` if `record` doesn't have `Table::ttype_len()` values
    /// or if this is a fieldless table.
    pub fn append_x(
        &mut self,
        record: Record,
        filename: &str,
        lino: usize,
    ) -> Result<()> {
        if record.len() != self.tclass.len() {
            bail!(
                "E736:{}:{}:rows for table of ttype {} must have exactly \
                {} values, got {}",
                self.ttype(),
                self.tclass.len(),
                record.len(),
                filename,
                lino
            )
        }
        self.records.push(record);
        Ok(())
    }

    /// Appends a `record` of `Value::Null`s to the end of the table or
    /// returns `Err` if this is a fieldless table.
    pub fn append_empty(&mut self) -> Result<()> {
        self.append_empty_x("-", 0)
    }

    /// Appends a `record` of `Value::Null`s to the end of the table or
    /// returns `Err` if this is a fieldless table.
    pub fn append_empty_x(
        &mut self,
        filename: &str,
        lino: usize,
    ) -> Result<()> {
        let record = self.tclass.record_of_nulls_x(filename, lino)?;
        self.records.push(record);
        Ok(())
    }

    /// Allows records to be added one value at a time
    #[allow(dead_code)]
    pub(crate) fn push(&mut self, value: Value) -> Result<()> {
        self.push_x(value, "-", 0)
    }

    /// Appends the given `values` to the table
    pub fn push_many(&mut self, values: &[Value]) -> Result<()> {
        for value in values {
            self.push_x(value.clone(), "-", 0)?;
        }
        Ok(())
    }

    /// `push_t(value)` is convenience for `push(value.into())`
    pub fn push_t<T: Into<Value>>(&mut self, value: T) -> Result<()> {
        self.push_x(value.into(), "-", 0)
    }

    /// Allows records to be added one value at a time
    pub(crate) fn push_x(
        &mut self,
        value: Value,
        filename: &str,
        lino: usize,
    ) -> Result<()> {
        if self.is_fieldless() {
            bail!(
                "E334:{}:{}:can't append to a fieldless table",
                filename,
                lino
            )
        }
        self.pending_record.push(value);
        if self.pending_record.len() == self.tclass.len() {
            let mut record = Values::new();
            std::mem::swap(&mut self.pending_record, &mut record);
            self.append_x(record, filename, lino)?;
        }
        Ok(())
    }

    /// To support type checking during parsing
    pub(crate) fn expected_type(&self) -> String {
        if self.is_fieldless() {
            "".to_string()
        } else if self.pending_record.is_empty()
            || self.pending_record.len() == self.tclass.len()
        {
            self.tclass.fields()[0].vtype().unwrap_or("").to_string()
        } else {
            self.tclass.fields()[self.pending_record.len()]
                .vtype()
                .unwrap_or("")
                .to_string()
        }
    }

    /// Truncates the table to contain at most `size` records.
    pub fn truncate(&mut self, size: usize) {
        self.records.truncate(size);
    }

    /// Deletes every value in the table so that it is empty.
    pub fn clear(&mut self) {
        self.records.clear();
    }

    /// Returns an iterator of the table's records as immutables.
    pub fn iter(&self) -> std::slice::Iter<Record> {
        self.records.iter()
    }

    /// Returns an iterator of the table's records as mutables.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Record> {
        self.records.iter_mut()
    }

    /// Returns `&records` to make the entire immutable Vec API available.
    pub fn inner(&self) -> &Vec<Record> {
        &self.records
    }

    /// Returns `&mut records` to make the entire mutable Vec API available.
    pub fn inner_mut(&mut self) -> &mut Vec<Record> {
        &mut self.records
    }

    /// Returns `true` if this `Table` and the `other` `Table` are the same.
    /// Set `compare` to `EQUIVALENT` or `IGNORE_COMMENTS` if comment
    /// differences don't matter.
    /// See also `==` and `Uxf::is_equivalent()`.
    pub fn is_equivalent(&self, other: &Table, compare: Compare) -> bool {
        if !compare.contains(Compare::IGNORE_COMMENTS)
            && self.comment != other.comment
        {
            return false;
        }
        self == other
    }
}

impl Index<usize> for Table {
    type Output = Record;

    /// Returns `&Record` if `row` is in bounds; otherwise panics.
    fn index(&self, row: usize) -> &Self::Output {
        &self.records[row]
    }
}

impl IndexMut<usize> for Table {
    /// Returns `&mut Record` if `row` is in bounds; otherwise panics.
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.records[row]
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        if self.tclass != other.tclass {
            return false;
        }
        if self.comment != other.comment {
            return false;
        }
        if self.records.len() != other.records.len() {
            return false;
        }
        for (arecord, brecord) in
            self.records.iter().zip(other.records.iter())
        {
            for (avalue, bvalue) in arecord.iter().zip(brecord.iter()) {
                if avalue != bvalue {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Table {}

impl fmt::Display for Table {
    /// Provides a .to_string() that returns a valid UXF fragment
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts = vec!["(".to_string()];
        if !self.comment().is_empty() {
            parts.push(format!("#<{}> ", escape(self.comment())));
        }
        parts.push(self.ttype().to_string());
        if !self.is_empty() {
            parts.push(" ".to_string());
        }
        let mut nl = "";
        for record in self.iter() {
            parts.push(nl.to_string());
            let mut sep = "";
            for value in record.iter() {
                parts.push(sep.to_string());
                parts.push(value.to_string());
                sep = " ";
            }
            nl = "\n";
        }
        parts.push(")".to_string());
        write!(f, "{}", parts.join(""))
    }
}
