// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::format::Format;
use crate::pprint::token::{TokenKind, Tokens};

pub(crate) fn to_text(
    header: &str,
    tokens: Tokens,
    format: &Format,
) -> String {
    let mut writer = Writer::new(header, tokens, format);
    writer.pprint();
    let mut uxt = String::new();
    std::mem::swap(&mut uxt, &mut writer.uxt);
    uxt
}

struct Writer {
    pub tokens: Tokens,
    pub uxt: String,
    pub indent: String,
    pub wrapwidth: usize,
    pub realdp: Option<u8>,
    pub pos: usize, // line position
    pub tp: usize,  // token position
    pub end_nl: bool,
    pub pending_rws: bool,
}

impl Writer {
    fn new(header: &str, tokens: Tokens, format: &Format) -> Self {
        Self {
            tokens,
            uxt: String::from(header),
            indent: format.indent.clone(),
            realdp: format.realdp,
            wrapwidth: format.wrapwidth as usize,
            pos: 0,
            tp: 0,
            end_nl: false,
            pending_rws: false,
        }
    }

    fn pprint(&mut self) {
        if self.tokens.is_empty() {
            return;
        }
        self.pos = 0;
        self.tp = 0;
        while self.tp < self.tokens.len() {
            let token = &self.tokens[self.tp];
            self.tp += 1;
            match token.kind {
                TokenKind::Begin => (), // TODO
                TokenKind::Str => (),   // TODO
                TokenKind::Rws => (),   // TODO
                TokenKind::Rnl => (),   // TODO
                TokenKind::End => (),   // TODO
                TokenKind::Eof => (),   // TODO
            }
        }
        if !self.end_nl {
            self.rnl();
        }
    }

    fn rnl(&mut self) {
        self.pending_rws = false;
        self.write("\n");
    }

    fn write(&mut self, s: &str) {
        let s = if self.pending_rws {
            self.pending_rws = false;
            format!(" {}", s)
        } else {
            s.to_string()
        };
        self.uxt.push_str(&s);
        self.set_pos(&s);
    }

    fn set_pos(&mut self, s: &str) {
        if s.ends_with('\n') {
            self.pos = 0;
            self.end_nl = true;
        } else {
            self.end_nl = false;
            self.pos += if let Some(i) = s.rfind('\n') {
                s[i + 1..].chars().count()
            } else {
                s.chars().count()
            }
        }
    }
}
