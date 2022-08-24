// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

// To access the Event details from a Result::Err see test_list.rs's
// t_list_err() test.

use std::{fmt, rc::Rc};

pub type OnEventFn = Rc<dyn Fn(&Event)>;

/// Used to output warning and repair events
pub fn on_event(event: &Event) {
    eprintln!("{}", event);
}

#[derive(Clone, Debug)]
pub struct Event {
    pub kind: EventKind,
    pub code: i16,
    pub message: String,
    pub filename: String,
    pub lino: usize,
}

impl Event {
    pub fn new(
        kind: EventKind,
        code: i16,
        message: &str,
        filename: &str,
        lino: usize,
    ) -> Self {
        Event {
            kind,
            code,
            message: message.to_string(),
            filename: filename.to_string(),
            lino,
        }
    }
    pub fn new_warning(
        code: i16,
        message: &str,
        filename: &str,
        lino: usize,
    ) -> Self {
        Event {
            kind: EventKind::Warning,
            code,
            message: message.to_string(),
            filename: filename.to_string(),
            lino,
        }
    }

    pub fn new_repair(
        code: i16,
        message: &str,
        filename: &str,
        lino: usize,
    ) -> Self {
        Event {
            kind: EventKind::Repair,
            code,
            message: message.to_string(),
            filename: filename.to_string(),
            lino,
        }
    }

    fn letter(&self) -> char {
        match self.kind {
            EventKind::Warning => 'W',
            EventKind::Repair => 'R',
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}:{}:{}:{}",
            self.letter(),
            self.code,
            self.filename,
            self.lino,
            self.message
        )
    }
}

impl std::error::Error for Event {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventKind {
    Warning,
    Repair,
}
