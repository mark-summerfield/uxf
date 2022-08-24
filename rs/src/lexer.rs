// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::event::{Event, OnEventFn};
use crate::token::{Token, TokenKind, Tokens};
use crate::util::{
    check_ktype, check_ttype, check_vtype, realstr64, str_for_chars,
    unescape,
};
use crate::uxf::Uxf;
use crate::value::Value;
use anyhow::{bail, Result};
use hex;
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
                (self.on_event)(&Event::new_warning(
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
                bail!(
                    "E160:{}:{}:invalid comment syntax: expected '<', \
                    got '{}'",
                    self.filename,
                    self.lino,
                    self.peek()
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
                '(' => self.handle_table_begin(),
                ')' => self.add_token(TokenKind::TableEnd, Value::Null),
                '[' => self.handle_list_begin(),
                '=' => self.handle_tclass_begin(),
                ']' => self.add_token(TokenKind::ListEnd, Value::Null),
                '{' => self.handle_map_begin(),
                '}' => self.handle_map_end(),
                '?' => self.add_token(TokenKind::Null, Value::Null),
                '!' => self.read_imports(),
                '#' => bail!("TODO scan_next #"), // TODO
                '<' => bail!("TODO scan_next <"), // TODO
                '&' => bail!("TODO scan_next &"), // TODO
                ':' => bail!("TODO scan_next :"), // TODO
                'c' if self.peek().is_ascii_digit() => {
                    bail!("TODO scan_next -[0-9]") // TODO
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8'
                | '9' => bail!("TODO scan_next [0-9]"), // TODO
                _ => {
                    if c.is_alphabetic() {
                        self.read_name()
                    } else {
                        bail!(
                            "E170:{}:{}:invalid character encountered {}",
                            self.filename,
                            self.lino,
                            "?" // TODO use first char found
                        )
                    }
                }
            }
        }
    }

    fn handle_table_begin(&mut self) -> Result<()> {
        if self.peek() == ':' {
            self.pos += 1;
            self.read_bytes()
        } else {
            self.check_in_tclass()?;
            self.add_token(TokenKind::TableBegin, Value::Null)
        }
    }

    fn handle_list_begin(&mut self) -> Result<()> {
        self.check_in_tclass()?;
        self.add_token(TokenKind::ListBegin, Value::Null)
    }

    fn handle_tclass_begin(&mut self) -> Result<()> {
        self.check_in_tclass()?; // allow for fieldless TClasses
        self.add_token(TokenKind::TClassBegin, Value::Null)?;
        self.in_tclass = true;
        Ok(())
    }

    fn handle_map_begin(&mut self) -> Result<()> {
        self.check_in_tclass()?;
        self.add_token(TokenKind::MapBegin, Value::Null)
    }

    fn handle_map_end(&mut self) -> Result<()> {
        self.in_tclass = false;
        self.add_token(TokenKind::MapEnd, Value::Null)
    }

    fn read_imports(&mut self) -> Result<()> {
        bail!("TODO read_imports") // TODO
    }

    fn read_bytes(&mut self) -> Result<()> {
        let i = self.match_to_chars_index(&[':', ')'], "bytes")?;
        let text = self.text[self.pos..i]
            .iter()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        let raw = hex::decode(text)?;
        self.add_token(TokenKind::Bytes, Value::Bytes(raw))
    }

    fn check_in_tclass(&mut self) -> Result<()> {
        if self.in_tclass {
            self.in_tclass = false;
            self.add_token(TokenKind::TClassEnd, Value::Null)
        } else {
            Ok(())
        }
    }

    fn read_name(&mut self) -> Result<()> {
        if let Some(word) = self.match_any_of(&BARE_WORDS) {
            if word == BOOL_FALSE {
                return self.add_token(TokenKind::Bool, Value::Bool(false));
            } else if word == BOOL_TRUE {
                return self.add_token(TokenKind::Bool, Value::Bool(true));
            }
            if VTYPES.contains(&word) {
                return self.add_token(
                    TokenKind::Type,
                    Value::Str(word.to_string()),
                );
            }
        }
        let start = self.pos - 1; // rewind since we went one byte to far
        if self.text[start] == '_' || self.text[start].is_alphabetic() {
            return self.read_ttype_or_identifier(start);
        }
        bail!(
            "E250:{}:{}:expected const or identifier, got {:?}",
            self.filename,
            self.lino,
            &self.peek_chunk(start)
        )
    }

    fn read_ttype_or_identifier(&mut self, start: usize) -> Result<()> {
        let identifier = self.match_identifier(start, "identifier")?;
        let identifier = Value::Str(identifier);
        if self.in_tclass {
            // safe because if in TClass there must have been a prev token
            let top = self.tokens.last_mut().unwrap();
            if top.kind == TokenKind::TClassBegin
                && top.value == Value::Null
            {
                top.value = identifier;
                Ok(())
            } else {
                self.add_token(TokenKind::Field, identifier)
            }
        } else {
            self.add_token(TokenKind::Identifier, identifier)
        }
    }

    fn match_identifier(
        &mut self,
        start: usize,
        what: &str,
    ) -> Result<String> {
        while self.pos < self.text.len() {
            if self.text[self.pos] != '_'
                || !self.text[self.pos].is_alphanumeric()
            {
                break;
            }
            self.pos += 1;
        }
        let identifier = &self.text[start..self.pos];
        let end = std::cmp::min(identifier.len(), MAX_IDENTIFIER_LEN + 1);
        let identifier = &identifier[..end];
        if !identifier.is_empty() {
            Ok(str_for_chars(identifier))
        } else {
            bail!(
                "E260:{}:{}:expected {}, got {:?}…",
                self.filename,
                self.lino,
                what,
                &self.peek_chunk(start)
            )
        }
    }

    fn peek_chunk(&self, start: usize) -> String {
        let i = if let Some(i) =
            self.text[start..].iter().position(|&x| x == '\n')
        {
            i
        } else if start + 8 < self.text.len() {
            8
        } else {
            1
        };
        str_for_chars(&self.text[start..start + i])
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
        targets.sort_by_key(|x| std::cmp::Reverse(x.len())); // long → short
        for target in targets {
            let end = start + target.len();
            let chars: Vec<char> = target.chars().collect();
            if end < self.text.len() && self.text[start..end] == chars {
                self.pos = end - 1; // skip past target
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

    fn match_to_chars_index(
        &mut self,
        cs: &[char],
        what: &str,
    ) -> Result<usize> {
        if !self.at_end() {
            for (i, x) in
                self.text[self.pos..].windows(cs.len()).enumerate()
            {
                if x == cs {
                    let text = &self.text[self.pos..i];
                    self.lino +=
                        text.iter().filter(|&c| *c == '\n').count();
                    self.pos = i + cs.len(); // skip past terminator
                    return Ok(i);
                }
            }
        }
        bail!("E280:{}:{}:unterminated {}", self.filename, self.lino, what)
    }

    fn match_to_chars(
        &mut self,
        cs: &[char],
        what: &str,
    ) -> Result<String> {
        if !self.at_end() {
            for (i, x) in
                self.text[self.pos..].windows(cs.len()).enumerate()
            {
                if x == cs {
                    let text = &self.text[self.pos..i];
                    self.lino +=
                        text.iter().filter(|&c| *c == '\n').count();
                    self.pos = i + cs.len(); // skip past terminator
                    return Ok(str_for_chars(text));
                }
            }
        }
        bail!("E279:{}:{}:unterminated {}", self.filename, self.lino, what)
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
