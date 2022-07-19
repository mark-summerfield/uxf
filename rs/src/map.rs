// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::value::{Key, Value};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Map {
    ktype: String,
    vtype: String,
    comment: String,
    items: HashMap<Key, Option<Value>>,
}
