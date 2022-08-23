// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::event::{Event, EventKind, OnEventFn};
use crate::token::{Token, TokenKind, Tokens};
use crate::util::{count_bytes, realstr64, unescape_raw};
use crate::value::Value;
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
    tokens: Tokens<'a>,
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
                bail!(
                    "E110:{}:{}:missing UXF file header or missing data \
                    or empty file",
                    self.filename,
                    self.lino,
                )
            };
        let line = str::from_utf8(&self.raw[..self.pos]).unwrap();
        let parts: Vec<&str> = line.splitn(3, &[' ', '\t']).collect();
        if parts.len() < 2 {
            bail!(
                "E120:{}:{}:invalid UXF file header",
                self.filename,
                self.lino,
            )
        }
        if parts[0] != "uxf" {
            bail!("E130:{}:{}:not a UXF file", self.filename, self.lino)
        }
        if let Ok(version) = parts[1].trim().parse::<f64>() {
            if version > UXF_VERSION {
                (self.on_event)(&Event::new(
                    EventKind::Warning,
                    141,
                    &format!(
                        "version {} > current {}",
                        realstr64(version),
                        realstr64(UXF_VERSION)
                    ),
                    self.filename,
                    self.lino,
                ));
            }
        } else {
            bail!(
                "E151:{}:{}:failed to read UXF file version number",
                self.filename,
                self.lino,
            )
        }
        if parts.len() > 2 {
            self.custom = parts[2].trim();
        }
        Ok(())
    }

    fn maybe_read_file_comment(&mut self) -> Result<()> {
        self.skip_ws();
        if !self.at_end() && self.raw[self.pos] == b'#' {
            self.pos += 1; // skip the #
            if self.peek() == b'<' {
                self.pos += 1; // skip the leading <
                let raw =
                    self.match_to_byte(b'>', "file comment string")?;
                let value = Value::Str(unescape_raw(raw));
                self.add_token(TokenKind::FileComment, value)?;
            } else {
                let c = if let Some(c) = char::from_u32(self.peek() as u32)
                {
                    c
                } else {
                    '\u{FFFD}'
                };
                bail!(
                    "E160:{}:{}:invalid comment syntax: expected '<', \
                    got '{}'",
                    self.filename,
                    self.lino,
                    c.to_string()
                )
            }
        }
        Ok(())
    }

    fn at_end(&self) -> bool {
        self.pos >= self.raw.len()
    }

    fn peek(&self) -> u8 {
        if self.at_end() {
            0
        } else {
            self.raw[self.pos]
        }
    }

    fn skip_ws(&mut self) {
        while self.pos < self.raw.len()
            && self.raw[self.pos].is_ascii_whitespace()
        {
            if self.raw[self.pos] == b'\n' {
                self.lino += 1;
            }
            self.pos += 1;
        }
    }

    fn match_to_byte(&mut self, b: u8, what: &str) -> Result<&[u8]> {
        if !self.at_end() {
            if let Some(i) =
                self.raw[self.pos..].iter().position(|&c| c == b)
            {
                let raw = &self.raw[self.pos..i];
                self.lino += count_bytes(b, raw);
                self.pos = i + 1; // skip past byte b
                return Ok(raw);
            }
        }
        bail!("E270:{}:{}:unterminated {}", self.filename, self.lino, what)
    }

    fn add_token(&mut self, kind: TokenKind, value: Value) -> Result<()> {
        if !self.in_tclass
            && !self.tokens.is_empty()
            && self.subsumed(kind.clone(), &value)?
        {
            return Ok(());
        }
        self.tokens.push(Token::new(kind, value, self.filename, self.lino));
        Ok(())
    }

    fn subsumed(&mut self, kind: TokenKind, value: &Value) -> Result<bool> {
        if matches!(kind, TokenKind::Identifier | TokenKind::Type) {
            // safe because we only call when self.tokens is nonempty
            let top = self.tokens.last().unwrap();
            return match top.kind {
                TokenKind::ListBegin => {
                    self.subsume_list_vtype(kind, value)
                }
                TokenKind::MapBegin => self.subsume_map_type(kind, value),
                TokenKind::TableBegin if kind == TokenKind::Identifier => {
                    self.subsume_table_ttype(kind, value)
                }
                _ => Ok(false),
            };
        }
        Ok(false)
    }

    fn subsume_list_vtype(
        &mut self,
        kind: TokenKind,
        value: &Value,
    ) -> Result<bool> {
        // safe because we only call when self.tokens is nonempty
        let top = self.tokens.last_mut().unwrap();
        // TODO maybe subsume
        Ok(false)
    }

    fn subsume_map_type(
        &mut self,
        kind: TokenKind,
        value: &Value,
    ) -> Result<bool> {
        // safe because we only call when self.tokens is nonempty
        let top = self.tokens.last_mut().unwrap();
        // TODO maybe subsume
        Ok(false)
    }

    fn subsume_table_ttype(
        &mut self,
        kind: TokenKind,
        value: &Value,
    ) -> Result<bool> {
        // safe because we only call when self.tokens is nonempty
        let top = self.tokens.last_mut().unwrap();
        // TODO maybe subsume
        Ok(false)
    }
}
