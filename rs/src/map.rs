// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::value::{Key, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    ktype: Option<String>,
    vtype: Option<String>,
    comment: Option<String>,
    items: HashMap<Key, Option<Value>>,
}
