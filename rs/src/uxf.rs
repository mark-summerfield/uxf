// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::event::{self, OnEventFn};
use crate::list::List;
use crate::tclass::TClass;
use crate::util::escape;
use crate::value::Value;
use std::{collections::HashMap, fmt};

pub struct Uxf {
    custom: String,
    comment: String,
    value: Value, // NOTE must be Value::List | Value::Map | Value::Table
    on_event: OnEventFn,
    tclass_for_ttype: HashMap<String, TClass>, // ttype x TClass
    import_index_for_ttype: HashMap<String, usize>, // imports index
    imports: Vec<String>, // import text NOTE preserve order & no duplicates
}

impl Uxf {
    // TODO doc
    pub fn new(
        custom: &str,
        comment: &str,
        on_event: Option<OnEventFn>,
    ) -> Self {
        Uxf {
            custom: custom.to_string(),
            comment: comment.to_string(),
            value: Value::List(List::default()),
            tclass_for_ttype: HashMap::new(),
            import_index_for_ttype: HashMap::new(),
            imports: vec![],
            on_event: if let Some(on_event) = on_event {
                on_event // user's
            } else {
                event::on_event // default
            },
        }
    }

    /// Returns the `custom` which may be `""`.
    pub fn custom(&self) -> &str {
        &self.custom
    }

    /// Use to change the custom text
    pub fn set_custom(&mut self, custom: &str) {
        self.custom = custom.to_string();
    }

    /// Returns the `comment` which may be `""`.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Use to change the comment text
    pub fn set_comment(&mut self, comment: &str) {
        self.comment = comment.to_string();
    }

    /// The collection value. This is immutable and defaults to an empty
    /// List. Normally populated by a load function.
    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl Default for Uxf {
    /// Returns a new empty Uxf that uses the default `event::on_event()`
    /// event handler.
    fn default() -> Self {
        Uxf {
            custom: "".to_string(),
            comment: "".to_string(),
            value: Value::List(List::default()),
            tclass_for_ttype: HashMap::new(),
            import_index_for_ttype: HashMap::new(),
            imports: vec![],
            on_event: event::on_event,
        }
    }
}

impl fmt::Debug for Uxf {
    fn fmt<'a>(&'a self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Uxf")
            .field("custom", &self.custom)
            .field("comment", &self.comment)
            .field("value", &self.value)
            .finish()
    }
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
            // Preserve original order
            parts.push(format!("!{}\n", import));
        }
        let mut tclasses: Vec<TClass> =
            self.tclass_for_ttype.values().cloned().collect();
        tclasses.sort_unstable(); // Use alphabetical order
        for tclass in tclasses.iter() {
            if !self.import_index_for_ttype.contains_key(tclass.ttype()) {
                parts.push(tclass.to_string());
                parts.push(NL.to_string());
            }
        }
        parts.push(self.value.clone().to_string());
        parts.push(NL.to_string());
        write!(f, "{}", parts.join(""))
    }
}
