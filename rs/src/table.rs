// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::tclass::TClass;
use crate::value::Row;

#[derive(Clone, Debug)]
pub struct Table {
    tclass: TClass,
    comment: String,
    records: Vec<Row>,
}

impl Table {
    pub fn new(tclass: TClass, comment: &str) -> Self {
        Table { tclass, comment: comment.to_string(), records: vec![] }
    }
}
