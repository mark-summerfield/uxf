// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::event::fatal;
use crate::key::Key;
use crate::util::{check_ktype, check_vtype, escape};
use crate::uxf::Compare;
use crate::value::Value;
use anyhow::Result;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Map {
    ktype: String,
    vtype: String,
    comment: String,
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
        if !ktype.is_empty() {
            check_ktype(ktype)?;
        }
        if !vtype.is_empty() {
            if ktype.is_empty() {
                fatal(
                    299,
                    "a map may only have a vtype if it has a ktype",
                )?;
            }
            check_vtype(vtype)?;
        }
        Ok(Map {
            ktype: ktype.to_string(),
            vtype: vtype.to_string(),
            comment: comment.to_string(),
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

    /// Inserts the given `key` and `value` into the map.
    /// If the `key` was already present, returns the previous value;
    /// otherwise returns `None`.
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
    pub fn sorted_keys(&self) -> Vec<&Key> {
        let mut keys: Vec<&Key> = self.items.keys().collect();
        keys.sort();
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
            sep = " ";
        }
        parts.push("}".to_string());
        write!(f, "{}", parts.join(""))
    }
}
