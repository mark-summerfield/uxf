// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/*! A Map holds a ktype (a possibly empty string), vtype (also possibly
empty), a comment (also possibly empty), and a (possibly empty) map
of Key-Value pairs.
*/
use crate::check::{check_ktype_x, check_vtype_x};
use crate::key::Key;
use crate::util::escape;
use crate::uxf::Compare;
use crate::value::Value;
use anyhow::{bail, Result};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Map {
    ktype: String,
    vtype: String,
    comment: String,
    pending_key: Option<Key>,
    items: HashMap<Key, Value>,
}

impl Map {
    /// Returns a new Map with the given `ktype`, `vtype`, and `comment` and
    /// no items, _or_ returns an Err if the `ktype` or `vtype` is invalid.
    /// A `ktype` of `""` means that any valid `ktype` is acceptable;
    /// otherwise the `ktype` must be a specific `ktype` (e.g., `bytes`,
    /// `date`, `int`, or `str`).
    /// A `vtype` of `""` means that any `vtype` is acceptable; otherwise
    /// the `vtype` should be a built-in UXF type (e.g., `int`, `str`,
    /// `date`, etc), or a `ttype`.
    /// The `vtype` and `comment` are immutable after construction.
    /// The Map does _not_ enforce the `ktype` or `vtype` if either or both
    /// is specified (although keys must be _a_ valid `ktype`).
    pub fn new(ktype: &str, vtype: &str, comment: &str) -> Result<Self> {
        Map::new_x(ktype, vtype, comment, "-", 0)
    }

    /// Returns a new Map with the given `ktype`, `vtype`, and `comment` and
    /// no items, _or_ returns an Err if the `ktype` or `vtype` is invalid.
    /// A `ktype` of `""` means that any valid `ktype` is acceptable;
    /// otherwise the `ktype` must be a specific `ktype` (e.g., `bytes`,
    /// `date`, `int`, or `str`).
    /// A `vtype` of `""` means that any `vtype` is acceptable; otherwise
    /// the `vtype` should be a built-in UXF type (e.g., `int`, `str`,
    /// `date`, etc), or a `ttype`.
    /// The `vtype` and `comment` are immutable after construction.
    /// The Map does _not_ enforce the `ktype` or `vtype` if either or both
    /// is specified (although keys must be _a_ valid `ktype`).
    pub fn new_x(
        ktype: &str,
        vtype: &str,
        comment: &str,
        filename: &str,
        lino: usize,
    ) -> Result<Self> {
        if !ktype.is_empty() {
            check_ktype_x(ktype, filename, lino)?;
        }
        if !vtype.is_empty() {
            if ktype.is_empty() {
                bail!(
                    "E299:{}:{}:a map may only have a vtype if it has \
                      a ktype",
                    filename,
                    lino
                )
            }
            check_vtype_x(vtype, filename, lino)?;
        }
        Ok(Map {
            ktype: ktype.to_string(),
            vtype: vtype.to_string(),
            comment: comment.to_string(),
            pending_key: None,
            items: HashMap::new(),
        })
    }

    /// Returns the `ktype` which may be `""`.
    pub fn ktype(&self) -> &str {
        &self.ktype
    }

    /// Returns the `vtype` which may be `""`.
    pub fn vtype(&self) -> &str {
        &self.vtype
    }

    /// Returns the `comment` which may be `""`.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Returns the number of values in the map.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` if the map is empty; otherwise returns `false`.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns `Some(&Value)` if `key` is in the map; otherwise `None`.
    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.items.get(key)
    }

    /// Returns `Some(&mut Value)` if `key` is in the map; otherwise
    /// `None`.
    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        self.items.get_mut(key)
    }

    /// Allows key-value items to be added one part at a time
    #[allow(dead_code)]
    pub(crate) fn push(&mut self, value: Value) -> Result<()> {
        self.push_x(value, "-", 0)
    }

    /// `push_t(value)` is convenience for `push(value.into())`
    pub fn push_t<T: Into<Value>>(&mut self, value: T) -> Result<()> {
        self.push_x(value.into(), "-", 0)
    }

    /// Allows key-value items to be added one part at a time
    pub(crate) fn push_x(
        &mut self,
        value: Value,
        filename: &str,
        lino: usize,
    ) -> Result<()> {
        if let Some(key) = &self.pending_key {
            self.insert(key.clone(), value);
            self.pending_key = None;
        } else {
            self.pending_key = Some(Key::from_x(value, filename, lino)?);
        }
        Ok(())
    }

    /// To support type checking during parsing
    pub(crate) fn expected_type(&self) -> String {
        if self.pending_key.is_none() {
            self.ktype.to_string() // expecting a key
        } else {
            self.vtype.to_string() // expecting a value
        }
    }

    /// Inserts the given `key` and `value` into the map.
    /// If the `key` was already present, returns the previous value;
    /// otherwise returns `None`.
    /// This ignores any pending key.
    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.items.insert(key, value)
    }

    /// If there's an item with the given `key`, removes the item and
    /// returns its corresponding value; otherwise leaves the map unchanged
    /// and returns `None`.
    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.items.remove(key)
    }

    /// Deletes every item in the map so that it is empty.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns the map's keys in sorted order.
    /// It is recommended to always iterate using these keys.
    /// The order works as follows: when two keys are of different types
    /// they are ordered `bytes` `<` `date` `<` `datetime` `<` `int` `<`
    /// `str`, and when two keys have the same types they are ordered using
    /// `<` except for ``str``s which use case-insensitive `<`.
    pub fn sorted_keys(&self) -> Vec<&Key> {
        let mut keys: Vec<&Key> = self.items.keys().collect();
        keys.sort_unstable();
        keys
    }

    /// Returns `&items` to make the entire immutable HashMap API available.
    pub fn inner(&self) -> &HashMap<Key, Value> {
        &self.items
    }

    /// Returns `&mut items` to make the entire mutable HashMap API
    /// available.
    pub fn inner_mut(&mut self) -> &mut HashMap<Key, Value> {
        &mut self.items
    }

    /// Returns `true` if this `Map` and the `other` `Map` are the same.
    /// Set `compare` to `EQUIVALENT` or `IGNORE_COMMENTS` if comment
    /// differences don't matter.
    /// See also `==` and `Uxf::is_equivalent()`.
    pub fn is_equivalent(&self, other: &Map, compare: Compare) -> bool {
        if !compare.contains(Compare::IGNORE_COMMENTS)
            && self.comment != other.comment
        {
            return false;
        }
        self == other
    }
}

impl Default for Map {
    /// Returns a new Map with an empty `ktype` meaning that any valid
    /// `ktype` may be used for keys, and an empty `vtype` meaning that any
    /// `vtype` is acceptable, and an empty comment and no items.
    fn default() -> Self {
        Map {
            ktype: "".to_string(),
            vtype: "".to_string(),
            comment: "".to_string(),
            pending_key: None,
            items: HashMap::new(),
        }
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        if self.ktype != other.ktype {
            return false;
        }
        if self.vtype != other.vtype {
            return false;
        }
        if self.comment != other.comment {
            return false;
        }
        if self.items.len() != other.items.len() {
            return false;
        }
        let akeys = self.sorted_keys();
        let bkeys = other.sorted_keys();
        for (akey, bkey) in akeys.iter().zip(bkeys.iter()) {
            if akey != bkey {
                return false;
            }
            let avalue = self.items.get(akey);
            let bvalue = other.items.get(akey);
            if avalue != bvalue {
                return false;
            }
        }
        true
    }
}

impl Eq for Map {}

impl fmt::Display for Map {
    /// Provides a .to_string() that returns a valid UXF fragment
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts = vec!["{".to_string()];
        let mut need_space = false;
        if !self.comment().is_empty() {
            parts.push(format!("#<{}>", escape(self.comment())));
            need_space = true;
        }
        if !self.ktype().is_empty() {
            if need_space {
                parts.push(" ".to_string());
            }
            parts.push(self.ktype().to_string());
            need_space = true;
        }
        if !self.vtype().is_empty() {
            if need_space {
                parts.push(" ".to_string());
            }
            parts.push(self.vtype().to_string());
            need_space = true;
        }
        if need_space && !self.is_empty() {
            parts.push(" ".to_string());
        }
        let mut sep = "";
        for key in self.sorted_keys() {
            parts.push(sep.to_string());
            parts.push(key.to_string());
            parts.push(" ".to_string());
            parts.push(self.items.get(key).unwrap().to_string());
            sep = "\n";
        }
        parts.push("}".to_string());
        write!(f, "{}", parts.join(""))
    }
}
