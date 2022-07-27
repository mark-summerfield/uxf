// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::tclass::TClass;
use crate::util::escape;
use crate::value::{Collection, Value};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug)]
pub struct Uxf {
    custom: String,
    comment: String,
    value: Collection,
    // on_event // callback
    // NOTE TClasses must be output in alphabetical order
    tclass_for_ttype: HashMap<String, TClass>, // ttype x TClass
    // NOTE imports must be output in original insertion-order
    import_index_for_ttype: HashMap<String, usize>, // imports index
    // NOTE import must not be duplicated
    imports: Vec<String>,                           // import text
}

impl Uxf {
    // TODO new(custom: &str, comment: &str, on_event: ???)

    /// Returns the `custom` which may be `""`.
    pub fn custom(&self) -> &str {
        &self.custom
    }

    // TODO set_custom()

    /// Returns the `comment` which may be `""`.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    // TODO set_comment()
}

impl fmt::Display for Uxf {
    /// Provides a .to_string() that returns the text of a valid UXF file
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const NL: &str = "\n";
        let mut parts = vec![format!("uxf {}", UXF_VERSION)];
        if !self.custom().is_empty() {
            parts.push(" ".to_string());
            parts.push(self.custom().to_string());
        }
        parts.push(NL.to_string());
        if !self.comment().is_empty() {
            parts.push(format!("#<{}>", escape(self.comment())));
            parts.push(NL.to_string());
        }
        for import in self.imports.iter() {
            parts.push(format!("!{}\n", import));
        }
        let mut tclasses: Vec<TClass> =
            self.tclass_for_ttype.values().cloned().collect();
        tclasses.sort_unstable();
        for tclass in tclasses.iter() {
            if !self.import_index_for_ttype.contains_key(tclass.ttype()) {
                parts.push(tclass.to_string());
                parts.push(NL.to_string());
            }
        }
        parts.push(Value::from(self.value.clone()).to_string());
        parts.push(NL.to_string());
        write!(f, "{}", parts.join(""))
    }
}
