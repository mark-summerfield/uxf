// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::event::{Event, EventKind, OnEventFn};
use crate::token::{Token, TokenKind, Tokens};
use crate::util::{
    check_ktype, check_ttype, check_vtype, count_bytes, realstr64,
    unescape_raw,
};
use crate::uxf::Uxf;
use crate::value::Value;
use anyhow::{bail, Result};
use std::{rc::Rc, str};

pub struct Lexer<'a> {
    pub raw: &'a Vec<u8>,
    pub filename: &'a str,
    on_event: OnEventFn,
    uxo: &'a mut Uxf,
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
        uxo: &'a mut Uxf,
    ) -> Self {
        Lexer {
            raw,
            filename,
            on_event: Rc::clone(&on_event),
            uxo,
            pos: 0,
            lino: 0,
            in_tclass: false,
            concatenate: false,
            tokens: vec![],
        }
    }

    pub fn tokenize(&mut self) -> Result<&Tokens> {
        self.scan_header()?;
        self.maybe_read_file_comment()?;
        while !self.at_end() {
            self.scan_next()?;
        }
        self.add_token(TokenKind::Eof, Value::Null)?;
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
            self.uxo.set_custom(parts[2].trim());
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
                let comment = unescape_raw(raw);
                self.uxo.set_comment(&comment);
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

    fn scan_next(&mut self) -> Result<()> {
        let c = self.getch();
        if c.is_ascii_whitespace() {
            // ignore insignificant whitespace
            if c == b'\n' {
                self.lino += 1;
            }
            Ok(())
        } else {
            match c {
                b'(' => bail!("TODO scan_next ("),  // TODO
                b')' => bail!("TODO scan_next )"),  // TODO
                b'[' => bail!("TODO scan_next ["),  // TODO
                b'=' => bail!("TODO scan_next ="),  // TODO
                b']' => bail!("TODO scan_next ]"),  // TODO
                b'{' => bail!("TODO scan_next {{"), // TODO
                b'}' => bail!("TODO scan_next }}"), // TODO
                b'?' => bail!("TODO scan_next ?"),  // TODO
                b'!' => bail!("TODO scan_next !"),  // TODO
                b'#' => bail!("TODO scan_next #"),  // TODO
                b'<' => bail!("TODO scan_next <"),  // TODO
                b'&' => bail!("TODO scan_next &"),  // TODO
                b':' => bail!("TODO scan_next :"),  // TODO
                _ => {
                    if c == b'-' && self.peek().is_ascii_digit() {
                        bail!("TODO scan_next -[0-9]") // TODO
                    } else if c.is_ascii_digit() {
                        bail!("TODO scan_next-[0-9]") // TODO
                    } else {
                        self.read_name()
                    }
                }
            }
        }
    }

    fn read_name(&mut self) -> Result<()> {
        if let Some(word) = self.match_any_of(&BARE_WORDS) {
            let word = String::from_utf8_lossy(word);
            if word == BOOL_FALSE {
                return self.add_token(TokenKind::Bool, Value::Bool(false));
            } else if word == BOOL_TRUE {
                return self.add_token(TokenKind::Bool, Value::Bool(true));
            }
            for vtype in VTYPES {
                if word == vtype {
                    return self.add_token(
                        TokenKind::Type,
                        Value::Str(word.to_string()),
                    );
                }
            }
        }
        // TODO if ! first char.is_alphabetic:
        bail!(
            "E170:{}:{}:invalid character encountered {}",
            self.filename,
            self.lino,
            "?" // TODO use first char found
        )
    }

    fn at_end(&self) -> bool {
        self.pos >= self.raw.len()
    }

    fn getch(&mut self) -> u8 {
        // advance
        let c = self.raw[self.pos];
        self.pos += 1;
        c
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

    fn match_any_of(&mut self, targets: &[&'a str]) -> Option<&'a [u8]> {
        let start = self.pos - 1; // rewind since we went one byte to far
        let mut targets: Vec<&[u8]> =
            targets.iter().map(|&s| s.as_bytes()).collect();
        targets.sort_by(|a, b| b.len().cmp(&a.len())); // long to short
        for target in targets {
            let end = self.pos + target.len();
            if end < self.raw.len() && &self.raw[start..end] == target {
                // TODO do I need the + 1 ???
                self.pos = end + 1; // skip past target
                return Some(target);
            }
        }
        None
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
        if top.vtype.is_empty() {
            if let Some(vtype) = value.as_str() {
                assert!(!vtype.is_empty());
                check_vtype(vtype)?;
                top.vtype = value.to_string();
            } else {
                bail!(
                    "E271:{}:{}:invalid vtype, got {}",
                    self.filename,
                    self.lino,
                    value
                )
            }
        } else {
            bail!(
                "E272:{}:{}:expected value, got type {}",
                self.filename,
                self.lino,
                value
            )
        }
        Ok(true)
    }

    fn subsume_map_type(
        &mut self,
        kind: TokenKind,
        value: &Value,
    ) -> Result<bool> {
        // safe because we only call when self.tokens is nonempty
        let top = self.tokens.last_mut().unwrap();
        if top.ktype.is_empty() {
            if kind == TokenKind::Identifier {
                bail!(
                    "E273:{}:{}:expected ktype, got {}",
                    self.filename,
                    self.lino,
                    value
                )
            }
            if let Some(ktype) = value.as_str() {
                assert!(!ktype.is_empty());
                check_ktype(ktype)?;
                top.ktype = ktype.to_string();
            } else {
                bail!(
                    "E275:{}:{}:invalid ktype, got {}",
                    self.filename,
                    self.lino,
                    value
                )
            }
        } else if top.vtype.is_empty() {
            if let Some(vtype) = value.as_str() {
                assert!(!vtype.is_empty());
                check_vtype(vtype)?;
                top.vtype = vtype.to_string();
            } else {
                bail!(
                    "E277:{}:{}:invalid vtype, got {}",
                    self.filename,
                    self.lino,
                    value
                )
            }
        } else {
            bail!(
                "E276:{}:{}:expected first map key, got type {}",
                self.filename,
                self.lino,
                value
            )
        }
        Ok(true)
    }

    fn subsume_table_ttype(
        &mut self,
        kind: TokenKind,
        value: &Value,
    ) -> Result<bool> {
        // safe because we only call when self.tokens is nonempty
        let top = self.tokens.last_mut().unwrap();
        if top.ttype.is_empty() {
            if let Some(ttype) = value.as_str() {
                assert!(!ttype.is_empty());
                check_ttype(ttype)?;
                top.ttype = value.to_string();
            } else {
                bail!(
                    "E278:{}:{}:invalid ttype, got {}",
                    self.filename,
                    self.lino,
                    value
                )
            }
        } else {
            bail!(
                "E274:{}:{}:expected value, got type {}",
                self.filename,
                self.lino,
                value
            )
        }
        Ok(true)
    }
}
