// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::check_name;
use crate::value::{Row, Value};
use anyhow::Result;
use std::ops::{Index, IndexMut};

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

    pub fn get(&self, row: usize) -> Option<&Value> {
        self.values[row].as_ref()
    }

    pub fn get_mut(&mut self, row: usize) -> &mut Option<Value> {
        &mut self.values[row]
    }

    pub fn push(&mut self, value: Option<Value>) {
        self.values.push(value);
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
    type Output = Option<Value>;

    fn index(&self, row: usize) -> &Self::Output {
        &self.values[row]
    }
}

impl IndexMut<usize> for List {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.values[row]
    }
}
