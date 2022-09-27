// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::format::Format;
use crate::pprint::token::{Token, TokenKind, Tokens};

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
    pub wrapwidth: usize,
    pub indent: String,
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
            wrapwidth: format.wrapwidth as usize,
            indent: format.indent.clone(),
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
            match &token.kind {
                TokenKind::Begin => self.begin(token.depth),
                TokenKind::Str => self.string(token.clone()),
                TokenKind::Rws => self.rws(),
                TokenKind::Rnl => self.rnl(),
                TokenKind::End => (),
                TokenKind::Eof => break,
            }
        }
        if !self.end_nl {
            self.rnl();
        }
    }

    fn begin(&mut self, depth: usize) {
        let tab = self.indent.repeat(depth);
        let needed = if self.pos > 0 { self.pos } else { tab.len() };
        if let Some(tp) = self.find_matching_end(needed, self.tp, depth) {
            // found & fits on line
            if self.pos == 0 {
                self.write(&tab);
            }
            self.write_tokens_to(tp);
        } else if self.pos > 0 {
            // try to fit begin…end on its own wrapped line
            if let Some(tp) =
                self.find_matching_end(tab.len(), self.tp, depth)
            {
                self.pending_rws = false;
                self.rnl();
                self.write(&tab);
                self.write_tokens_to(tp);
            }
        }
        // else skip this begin and continue from the next token
    }

    fn find_matching_end(
        &self,
        needed: usize,
        tp: usize,
        depth: usize,
    ) -> Option<usize> {
        let mut needed = if self.pending_rws { needed + 1 } else { needed };
        let mut tp = tp;
        while needed <= self.wrapwidth && tp < self.tokens.len() {
            let token = &self.tokens[tp];
            tp += 1;
            match token.kind {
                TokenKind::End => {
                    if token.depth == depth {
                        return Some(tp); // matching end
                    }
                }
                // de-facto: forced onto newline anyway or EOF:
                TokenKind::Rnl | TokenKind::Eof => return Some(tp),
                TokenKind::Rws => needed += 1,
                TokenKind::Str => {
                    if token.is_multiline() {
                        return Some(tp); // de-facto: forced onto newline
                    }
                    needed += token.text.chars().count();
                }
                TokenKind::Begin => (), // irrelevant
            }
        }
        None
    }

    fn write_tokens_to(&mut self, tp: usize) {
        while self.tp < tp {
            // room for more
            let token = self.tokens[self.tp].clone();
            self.tp += 1;
            match token.kind {
                TokenKind::Str => {
                    self.write(&token.text);
                    if token.is_multiline() {
                        break;
                    }
                }
                TokenKind::Rws => self.rws(),
                TokenKind::Rnl => self.rnl(),
                TokenKind::Begin | TokenKind::End => (),
                TokenKind::Eof => break,
            }
        }
    }

    fn string(&mut self, token: Token) {
        if token.is_multiline() {
            self.multiline(token);
        } else {
            let width = token.text.chars().count();
            if self.pos > 0 {
                // in a line
                let n = if self.pending_rws { 1 } else { 0 };
                if self.pos + width + n <= self.wrapwidth {
                    self.write(&token.text);
                    return;
                } else {
                    self.write("\n");
                }
            }
            let tab = self.indent.repeat(token.depth);
            if tab.len() + width <= self.wrapwidth {
                self.write(&tab); // fits after indent
            }
            self.write(&token.text);
        }
    }

    fn multiline(&mut self, token: Token) {
        // This method writes direct to the uxt
        if self.pos > 0 {
            // in a line
            let n = if self.pending_rws { 1 } else { 0 };
            // Should always succeed because we know text contains \n
            let (first, rest) = token.text.split_once('\n').unwrap();
            if self.pos + first.chars().count() + n <= self.wrapwidth {
                if self.pending_rws {
                    self.pending_rws = false;
                    self.uxt.push(' ');
                }
                self.uxt.push_str(first);
                self.uxt.push('\n');
                self.uxt.push_str(rest);
                self.set_pos(rest);
            } else {
                self.pending_rws = false;
                self.uxt.push('\n');
                self.uxt.push_str(&token.text);
                self.set_pos(&token.text);
            }
        } else {
            // newline
            self.pending_rws = false;
            self.uxt.push_str(&token.text);
            self.set_pos(&token.text);
        }
    }

    fn rws(&mut self) {
        if self.pos > 0 {
            // safe to ignore RWS at start of line
            if self.pos + self.peek_len(self.tp + 1) <= self.wrapwidth {
                self.pending_rws = true;
            } else {
                self.rnl();
            }
        }
    }

    fn peek_len(&self, tp: usize) -> usize {
        if tp < self.tokens.len() {
            self.tokens[tp].text.chars().count()
        } else {
            0
        }
    }

    fn rnl(&mut self) {
        self.pending_rws = false;
        self.write("\n");
    }

    fn write(&mut self, s: &str) {
        // This method is bypassed in multiline()
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
