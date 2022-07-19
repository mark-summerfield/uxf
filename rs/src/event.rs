// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use anyhow::{bail, Result};

pub fn on_event(event: &Event) -> Result<()> {
    let text = event.text();
    if event.kind == EventKind::Fatal {
        bail!(text);
    }
    if event.verbose {
        eprintln!("{}", &text);
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

    pub fn text(&self) -> String {
        format!(
            "{}:{}{}:{}:{}:{}",
            self.prefix,
            self.letter(),
            self.code,
            self.filename,
            self.lino,
            self.message
        )
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

#[derive(Clone, Debug, PartialEq)]
pub enum EventKind {
    Warning,
    Repair,
    Error,
    Fatal,
}
