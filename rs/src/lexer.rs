// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::event::{Event, EventKind, OnEventFn};
use crate::token::{Token, TokenKind, Tokens};
use crate::util::{
    check_ktype, check_ttype, check_vtype, realstr64, str_for_chars,
    unescape,
};
use crate::uxf::Uxf;
use crate::value::Value;
use anyhow::{bail, Result};
use std::{rc::Rc, str};

pub struct Lexer<'a> {
    pub text: &'a Vec<char>,
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
        text: &'a Vec<char>,
        filename: &'a str,
        on_event: OnEventFn,
        uxo: &'a mut Uxf,
    ) -> Self {
        Lexer {
            text,
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
            if let Some(i) = self.text.iter().position(|&c| c == '\n') {
                i
            } else {
                bail!(
                    "E110:{}:{}:missing UXF file header or missing data \
                    or empty file",
                    self.filename,
                    self.lino,
                )
            };
        let line = str_for_chars(&self.text[..self.pos]);
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
        if !self.at_end() && self.text[self.pos] == '#' {
            self.pos += 1; // skip the #
            if self.peek() == '<' {
                self.pos += 1; // skip the leading <
                let text =
                    self.match_to_char('>', "file comment string")?;
                let comment = unescape(&text);
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
            if c == '\n' {
                self.lino += 1;
            }
            Ok(())
        } else {
            match c {
                '(' => bail!("TODO scan_next ("),  // TODO
                ')' => bail!("TODO scan_next )"),  // TODO
                '[' => bail!("TODO scan_next ["),  // TODO
                '=' => bail!("TODO scan_next ="),  // TODO
                ']' => bail!("TODO scan_next ]"),  // TODO
                '{' => bail!("TODO scan_next {{"), // TODO
                '}' => bail!("TODO scan_next }}"), // TODO
                '?' => bail!("TODO scan_next ?"),  // TODO
                '!' => bail!("TODO scan_next !"),  // TODO
                '#' => bail!("TODO scan_next #"),  // TODO
                '<' => bail!("TODO scan_next <"),  // TODO
                '&' => bail!("TODO scan_next &"),  // TODO
                ':' => bail!("TODO scan_next :"),  // TODO
                _ => {
                    if c == '-' && self.peek().is_ascii_digit() {
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
        self.pos >= self.text.len()
    }

    fn getch(&mut self) -> char {
        // advance
        let c = self.text[self.pos];
        self.pos += 1;
        c
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.text[self.pos]
        }
    }

    fn skip_ws(&mut self) {
        while self.pos < self.text.len()
            && self.text[self.pos].is_ascii_whitespace()
        {
            if self.text[self.pos] == '\n' {
                self.lino += 1;
            }
            self.pos += 1;
        }
    }

    fn match_any_of(&mut self, targets: &[&'a str]) -> Option<&'a str> {
        let start = self.pos - 1; // rewind since we went one byte to far
        let mut targets = targets.to_vec();
        targets.sort_by(|a, b| b.len().cmp(&a.len())); // long to short
        for target in targets {
            let end = self.pos + target.len();
            let chars: Vec<char> = target.chars().collect();
            if end < self.text.len() && &self.text[start..end] == chars {
                self.pos = end + 1; // skip past target
                return Some(target);
            }
        }
        None
    }

    fn match_to_char(&mut self, c: char, what: &str) -> Result<String> {
        if !self.at_end() {
            if let Some(i) =
                self.text[self.pos..].iter().position(|&x| x == c)
            {
                let text = &self.text[self.pos..i];
                self.lino += text.iter().filter(|&c| *c == '\n').count();
                self.pos = i + 1; // skip past char c
                return Ok(str_for_chars(text));
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
