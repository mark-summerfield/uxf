// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::{check_ktype, check_vtype};
use crate::value::{Key, Value};
use anyhow::Result;
use std::collections::HashMap;
use std::fmt;

// TODO ktype() vtype() comment()
// TODO len() is_empty()
// TODO get() get_mut() v = [] [] = v
// TODO insert() remove() clear()
// TODO inner() inner_mut()
// TODO impl Iter
// TODO impl Display
// TODO docs for every fn
// TODO tests

#[derive(Clone, Debug)]
pub struct Map {
    ktype: String,
    vtype: String,
    comment: String,
    items: HashMap<Key, Value>,
}

impl Map {
    pub fn new(ktype: &str, vtype: &str, comment: &str) -> Result<Self> {
        if !ktype.is_empty() {
            check_ktype(ktype)?;
        }
        if !vtype.is_empty() {
            check_vtype(vtype)?;
        }
        Ok(Map {
            ktype: ktype.to_string(),
            vtype: vtype.to_string(),
            comment: comment.to_string(),
            items: HashMap::new(),
        })
    }
}

impl Default for Map {
    fn default() -> Self {
        Map {
            ktype: "".to_string(),
            vtype: "".to_string(),
            comment: "".to_string(),
            items: HashMap::new(),
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Map ktype:{} vtype:{} comment:{} items:{:?}",
            self.ktype, self.vtype, self.comment, self.items
        )
    }
}
