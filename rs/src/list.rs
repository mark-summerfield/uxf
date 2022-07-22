// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::check_name;
use crate::value::{Row, Value};
use anyhow::Result;
use std::fmt;
use std::ops::{Index, IndexMut};

// TODO docs for every fn
// TODO tests

#[derive(Clone, Debug)]
pub struct List {
    vtype: String,
    comment: String,
    values: Row,
}

impl List {
    pub fn new(vtype: &str, comment: &str) -> Result<Self> {
        if !vtype.is_empty() {
            check_name(vtype)?;
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
        if index < self.values.len() {
            Some(&self.values[index])
        } else {
            None
        }
    }

    /// Returns `mut Some(&Value)` if `index` is in bounds;
    /// otherwise `None`.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        if index < self.values.len() {
            Some(&mut self.values[index])
        } else {
            None
        }
    }

    /// Returns `&Value` if `index` is in bounds; otherwise panics.
    pub fn get_unchecked(&self, index: usize) -> &Value {
        &self.values[index]
    }

    /// Returns `mut &Value` if `index` is in bounds; otherwise panics
    pub fn get_unchecked_mut(&mut self, index: usize) -> &mut Value {
        &mut self.values[index]
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
