// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::{check_vtype, escape};
use crate::value::{Row, Value};
use anyhow::Result;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct List {
    vtype: String,
    comment: String,
    values: Row,
}

impl List {
    /// Returns a new List with the given `vtype` and `comment` and no
    /// values, _or_ returns an Err if the `vtype` is invalid.
    /// A `vtype` of `""` means that any `vtype` is acceptable; otherwise
    /// the `vtype` should be a built-in UXF type (e.g., `int`, `str`,
    /// `date`, etc), or a `ttype`.
    /// The `vtype` and `comment` are immutable after construction.
    pub fn new(vtype: &str, comment: &str) -> Result<Self> {
        if !vtype.is_empty() {
            check_vtype(vtype)?;
        }
        Ok(List {
            vtype: vtype.to_string(),
            comment: comment.to_string(),
            values: Row::new(),
        })
    }

    /// Returns the `vtype` which may be `""`.
    pub fn vtype(&self) -> &str {
        &self.vtype
    }

    /// Returns the `comment` which may be `""`.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Returns the number of values in the list.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` if the list is empty; otherwise returns `false`.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns `Some(&Value)` if `index` is in bounds; otherwise `None`.
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.values.get(index)
    }

    /// Returns `Some(&mut Value)` if `index` is in bounds; otherwise
    /// `None`.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.values.get_mut(index)
    }

    /// Returns `&values` to make the entire immutable Vec API available.
    pub fn inner(&self) -> &Row {
        &self.values
    }

    /// Returns `&mut values` to make the entire mutable Vec API available.
    pub fn inner_mut(&mut self) -> &mut Row {
        &mut self.values
    }

    /// Appends the given `Value` to the end of the list.
    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    /// Truncates the list to contain at most `size` values.
    pub fn truncate(&mut self, size: usize) {
        self.values.truncate(size);
    }

    /// Deletes every value in the list so that it is empty.
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Returns an iterator of the list's values as immutables.
    pub fn iter(&self) -> std::slice::Iter<Value> {
        self.values.iter()
    }

    /// Returns an iterator of the list's values as mutables.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Value> {
        self.values.iter_mut()
    }
}

impl Default for List {
    /// Returns a new List with an empty `vtype` meaning that any `vtype` is
    /// acceptable and an empty comment and no values.
    fn default() -> Self {
        List {
            vtype: "".to_string(),
            comment: "".to_string(),
            values: Row::new(),
        }
    }
}

impl Index<usize> for List {
    type Output = Value;

    /// Returns `&Value` if `index` is in bounds; otherwise panics.
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for List {
    /// Returns `&mut Value` if `index` is in bounds; otherwise panics.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl fmt::Display for List {
    /// Provides a .to_string() that returns a valid UXF fragment
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts = vec!["[".to_string()];
        if !self.comment().is_empty() {
            parts.push(format!("#<{}>", escape(self.comment())));
        }
        if !self.vtype().is_empty() {
            if !self.comment().is_empty() {
                parts.push(" ".to_string());
            }
            parts.push(self.vtype().to_string());
        }
        if !self.is_empty()
            && (!self.comment().is_empty() || !self.vtype().is_empty())
        {
            parts.push(" ".to_string());
        }
        let mut sep = "";
        for value in self.iter() {
            parts.push(sep.to_string());
            parts.push(value.to_string());
            sep = " ";
        }
        parts.push("]".to_string());
        write!(f, "{}", parts.join(""))
    }
}
