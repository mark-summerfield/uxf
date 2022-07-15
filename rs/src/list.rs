// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::value::{Row, Value};

#[derive(Clone, Debug)]
pub struct List {
    vtype: Option<String>,
    comment: Option<String>,
    values: Row,
}
