// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::{check_ktype, check_name};
use crate::value::{Key, Value};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Map {
    ktype: String,
    vtype: String,
    comment: String,
    items: HashMap<Key, Option<Value>>,
}

impl Map {
    pub fn new(ktype: &str, vtype: &str, comment: &str) -> Result<Self> {
        if !ktype.is_empty() {
            check_ktype(ktype)?;
        }
        if !vtype.is_empty() {
            check_name(vtype)?;
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
