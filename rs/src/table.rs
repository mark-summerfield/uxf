// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::tclass::TClass;
use crate::value::Row;
use std::fmt;

// TODO ttype() comment()
// TODO len() is_empty()
// TODO get() get_mut()
// TODO truncate() clear()
// TODO impl Iter
// TODO impl Display
// TODO docs for every fn
// TODO tests

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

    pub fn ttype(&self) -> &str {
        self.tclass.ttype()
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Table ttype:{} comment:{} records:{:?}",
            self.ttype(),
            self.comment,
            self.records
        )
    }
}
