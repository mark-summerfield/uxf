// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::check_type_name;
use crate::value::{Row, Value};
use anyhow::Result;
use std::fmt;
use std::ops::{Index, IndexMut};

// TODO docs for every fn

#[derive(Clone, Debug)]
pub struct List {
    vtype: String,
    comment: String,
    values: Row,
}

impl List {
    pub fn new(vtype: &str, comment: &str) -> Result<Self> {
        if !vtype.is_empty() {
            check_type_name(vtype)?;
        }
        Ok(List {
            vtype: vtype.to_string(),
            comment: comment.to_string(),
            values: Row::new(),
        })
    }

    pub fn vtype(&self) -> &str {
        &self.vtype
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

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

    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn truncate(&mut self, size: usize) {
        self.values.truncate(size);
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn iter(&self) -> std::slice::Iter<Value> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Value> {
        self.values.iter_mut()
    }
}

impl Default for List {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "List vtype:{} comment:{} values:{:?}",
            self.vtype, self.comment, self.values
        )
    }
}
