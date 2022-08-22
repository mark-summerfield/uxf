// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

// To access the Event details from a Result::Err see test_list.rs's
// t_list_err() test.

use anyhow::{bail, Result};
use std::{fmt, rc::Rc};

pub type OnEventFn = Rc<dyn Fn(&Event) -> Result<()>>;

pub fn fatal(code: i16, message: &str) -> Result<()> {
    bail!(Event::new(EventKind::Fatal, code, message))
}

pub fn on_event(event: &Event) -> Result<()> {
    if event.kind == EventKind::Fatal {
        bail!(event.clone());
    }
    if event.verbose {
        eprintln!("{}", event);
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Event {
    pub kind: EventKind,
    pub code: i16,
    pub message: String,
    pub filename: String,
    pub lino: u32,
    pub verbose: bool,
    pub prefix: String,
}

impl Event {
    pub fn new_warning(code: i16, message: &str) -> Self {
        Event {
            kind: EventKind::Warning,
            code,
            message: message.to_string(),
            filename: "-".to_string(),
            lino: 0,
            verbose: true,
            prefix: "uxf".to_string(),
        }
    }

    pub fn new_fatal(code: i16, message: &str) -> Self {
        Event {
            kind: EventKind::Fatal,
            code,
            message: message.to_string(),
            filename: "-".to_string(),
            lino: 0,
            verbose: true,
            prefix: "uxf".to_string(),
        }
    }

    pub fn new(kind: EventKind, code: i16, message: &str) -> Self {
        Event {
            kind,
            code,
            message: message.to_string(),
            filename: "-".to_string(),
            lino: 0,
            verbose: true,
            prefix: "uxf".to_string(),
        }
    }

    pub fn new_all(
        kind: EventKind,
        code: i16,
        message: &str,
        filename: Option<&str>,
        lino: u32,
        verbose: bool,
        prefix: Option<&str>,
    ) -> Self {
        Event {
            kind,
            code,
            message: message.to_string(),
            filename: (if let Some(filename) = filename {
                filename
            } else {
                "-"
            })
            .to_string(),
            lino,
            verbose,
            prefix: (if let Some(prefix) = prefix {
                prefix
            } else {
                "uxf"
            }
            .to_string()),
        }
    }

    fn letter(&self) -> char {
        match self.kind {
            EventKind::Warning => 'W',
            EventKind::Repair => 'R',
            EventKind::Error => 'E',
            EventKind::Fatal => 'F',
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}{}:{}:{}:{}",
            self.prefix,
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
    Error,
    Fatal,
}
