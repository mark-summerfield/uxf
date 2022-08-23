// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::event::{Event, OnEventFn};
use crate::lex_token::Tokens;
use crate::util::realstr64;
use anyhow::{bail, Result};
use std::{rc::Rc, str};

pub struct Lexer<'a> {
    pub raw: &'a Vec<u8>,
    pub filename: &'a str,
    pub custom: &'a str,
    on_event: OnEventFn,
    pos: usize,
    lino: usize,
    in_tclass: bool,
    concatenate: bool,
    tokens: Tokens,
}

impl<'a> Lexer<'a> {
    pub fn new(
        raw: &'a Vec<u8>,
        filename: &'a str,
        on_event: OnEventFn,
    ) -> Self {
        Lexer {
            raw,
            filename,
            on_event: Rc::clone(&on_event),
            pos: 0,
            lino: 0,
            custom: "",
            in_tclass: false,
            concatenate: false,
            tokens: vec![],
        }
    }

    pub fn tokenize(&mut self) -> Result<&Tokens> {
        self.scan_header()?;
        self.maybe_read_file_comment()?;
        // TODO
        Ok(&self.tokens)
    }

    fn scan_header(&mut self) -> Result<()> {
        self.lino = 1;
        self.pos =
            if let Some(i) = self.raw.iter().position(|&c| c == b'\n') {
                i
            } else {
                let event = Event::new_fatal(
                    110,
                    "missing UXF file header or missing data or empty file",
                );
                (self.on_event)(&event)?;
                bail!(event); // in case user on_event doesn't bail
            };
        let line = str::from_utf8(&self.raw[..self.pos]).unwrap();
        let parts: Vec<&str> = line.splitn(3, &[' ', '\t']).collect();
        if parts.len() < 2 {
            let event = Event::new_fatal(120, "invalid UXF file header");
            (self.on_event)(&event)?;
            bail!(event); // in case user on_event doesn't bail
        }
        if parts[0] != "uxf" {
            let event = Event::new_fatal(130, "not a UXF file");
            (self.on_event)(&event)?;
            bail!(event); // in case user on_event doesn't bail
        }
        if let Ok(version) = parts[1].trim().parse::<f64>() {
            if version > UXF_VERSION {
                let event = Event::new_warning(
                    141,
                    &format!(
                        "version {} > current {}",
                        realstr64(version),
                        realstr64(UXF_VERSION)
                    ),
                );
                (self.on_event)(&event)?;
            }
        } else {
            let event = Event::new_fatal(
                151,
                "failed to read UXF file version number",
            );
            (self.on_event)(&event)?;
            bail!(event); // in case user on_event doesn't bail
        }
        if parts.len() > 2 {
            self.custom = parts[2];
        }
        Ok(())
    }

    fn maybe_read_file_comment(&mut self) -> Result<()> {
        Ok(())
    }

    fn skip_ws(&mut self) {}
}
