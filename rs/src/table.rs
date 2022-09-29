// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::tclass::TClass;
use crate::util::escape;
use crate::uxf::Compare;
use crate::value::{Record, Value, Values};
use anyhow::{bail, Result};
use std::fmt;
use std::ops::{Index, IndexMut};

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

    /// Returns `Some(&Record)` if `index` is in bounds; otherwise `None`.
    pub fn get(&self, index: usize) -> Option<&Record> {
        self.records.get(index)
    }

    /// Returns `Some(&mut Record)` if `index` is in bounds; otherwise
    /// `None`.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Record> {
        self.records.get_mut(index)
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

    /// Returns `&Record` if `index` is in bounds; otherwise panics.
    fn index(&self, index: usize) -> &Self::Output {
        &self.records[index]
    }
}

impl IndexMut<usize> for Table {
    /// Returns `&mut Record` if `index` is in bounds; otherwise panics.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.records[index]
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
