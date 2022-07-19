// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::value::Row;

#[derive(Clone, Debug)]
pub struct List {
    vtype: String,
    comment: String,
    values: Row,
}
