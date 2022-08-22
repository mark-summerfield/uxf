// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::event::OnEventFn;
use crate::lex_token::{Token, Tokens};
use anyhow::Result;
use std::rc::Rc;

pub struct Lexer<'a> {
    uxt: &'a str,
    filename: &'a str,
    on_event: OnEventFn,
    tokens: Tokens,
}

impl<'a> Lexer<'a> {
    pub fn new(
        uxt: &'a str,
        filename: &'a str,
        on_event: OnEventFn,
    ) -> Self {
        Lexer {
            uxt,
            filename,
            on_event: Rc::clone(&on_event),
            tokens: vec![],
        }
    }

    pub fn lex(&mut self) -> Result<&Tokens> {
        // TODO
        Ok(&self.tokens)
    }
}
