// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::check::check_vtype;
use crate::util::escape;
use crate::uxf::Compare;
use crate::value::{Value, Values};
use anyhow::Result;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct List {
    vtype: String,
    comment: String,
    values: Values,
}

impl List {
    /// Returns a new List with the given `vtype` and `comment` and no
    /// values, _or_ returns an Err if the `vtype` is invalid.
    /// A `vtype` of `""` means that any `vtype` is acceptable; otherwise
    /// the `vtype` should be a built-in UXF type (e.g., `int`, `str`,
    /// `date`, etc), or a `ttype`.
    /// The `vtype` and `comment` are immutable after construction.
    /// The List does _not_ enforce the `vtype` if it is specified.
    pub fn new(vtype: &str, comment: &str) -> Result<Self> {
        if !vtype.is_empty() {
            check_vtype(vtype)?;
        }
        Ok(List {
            vtype: vtype.to_string(),
            comment: comment.to_string(),
            values: Values::new(),
        })
    }

    /// Returns the `vtype` which may be `""`.
    pub fn vtype(&self) -> &str {
        &self.vtype
    }

    /// To support type checking during parsing
    pub(crate) fn expected_type(&self) -> String {
        self.vtype.to_string()
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

    /// Returns `Some(&Value)` if there's at least one value;
    /// otherwise `None`.
    pub fn first(&self) -> Option<&Value> {
        self.values.get(0)
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

    /// Returns the list's last value if there is one
    pub fn last(&self) -> Option<&Value> {
        self.values.last()
    }

    /// Returns the list's last value if there is one
    pub fn last_mut(&mut self) -> Option<&mut Value> {
        self.values.last_mut()
    }

    /// Appends the given `Value` to the end of the list.
    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    /// Appends the given `values` to the end of the list.
    pub fn push_many(&mut self, values: &[Value]) {
        self.values.extend_from_slice(values);
    }

    /// `push_t(value)` is convenience for `push(value.into())`
    pub fn push_t<T: Into<Value>>(&mut self, value: T) {
        self.values.push(value.into());
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

    /// Returns `&values` to make the entire immutable Vec API available.
    pub fn inner(&self) -> &Values {
        &self.values
    }

    /// Returns `&mut values` to make the entire mutable Vec API available.
    pub fn inner_mut(&mut self) -> &mut Values {
        &mut self.values
    }

    /// Returns `true` if this `List` and the `other` `List` are the same.
    /// Set `compare` to `EQUIVALENT` or `IGNORE_COMMENTS` if comment
    /// differences don't matter.
    /// See also `==` and `Uxf::is_equivalent()`.
    pub fn is_equivalent(&self, other: &List, compare: Compare) -> bool {
        if !compare.contains(Compare::IGNORE_COMMENTS)
            && self.comment != other.comment
        {
            return false;
        }
        self == other
    }
}

impl Default for List {
    /// Returns a new List with an empty `vtype` meaning that any `vtype` is
    /// acceptable and an empty comment and no values.
    fn default() -> Self {
        List {
            vtype: "".to_string(),
            comment: "".to_string(),
            values: Values::new(),
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

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        if self.vtype != other.vtype {
            return false;
        }
        if self.comment != other.comment {
            return false;
        }
        if self.values.len() != other.values.len() {
            return false;
        }
        for (avalue, bvalue) in self.values.iter().zip(other.values.iter())
        {
            if avalue != bvalue {
                return false;
            }
        }
        true
    }
}

impl Eq for List {}

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
            sep = "\n";
        }
        parts.push("]".to_string());
        write!(f, "{}", parts.join(""))
    }
}
