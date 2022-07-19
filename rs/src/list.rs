// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::util::check_name;
use crate::value::Row;
use anyhow::Result;

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
