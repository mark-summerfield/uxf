// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/// When parsing a UXF file, errors produce an Err and halt the parse.
///
/// To access the Event details from a Result::Err see test_list.rs's
/// t_list_err() test.
///
/// However, warnings and repairs result in a call to on_event() which
/// prints the details to stderr.
///
/// If you want to ignore warnings and repairs, use parse_options() passing
/// ignore_event() as the on_event handler.
///
/// If you want to take over the handling of warnings and repairs, again use
/// parse_options(), but this time pass your own custom function as the
/// on_event handler.

use std::{fmt, rc::Rc};

pub type OnEventFn = Rc<dyn Fn(&Event)>;

/// Used to output warning and repair events
pub fn on_event(event: &Event) {
    eprintln!("{event}");
}

/// Used to ignore warning and repair events
pub fn ignore_event(_: &Event) {}

#[derive(Clone, Debug)]
pub struct Event {
    pub kind: EventKind,
    pub code: u16,
    pub message: String,
    pub filename: String,
    pub lino: usize,
}

impl Event {
    pub fn new(
        kind: EventKind,
        code: u16,
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
        code: u16,
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

    pub fn bare_warning(code: u16, message: &str) -> Self {
        Event {
            kind: EventKind::Warning,
            code,
            message: message.to_string(),
            filename: "".to_string(),
            lino: 0,
        }
    }

    pub fn new_repair(
        code: u16,
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
