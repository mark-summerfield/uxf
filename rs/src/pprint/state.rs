// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::event::{self, OnEventFn};
use crate::format::Format;
use crate::pprint::token::Tokens;
use std::rc::Rc;

pub struct State {
    pub on_event: OnEventFn,
    pub indent: String,
    pub wrapwidth: usize,
    pub realdp: Option<u8>,
    pub depth: usize,
    pub tokens: Tokens,
    pub list_value_counts: Vec<usize>,
    pub map_item_counts: Vec<usize>,
    pub table_record_counts: Vec<usize>,
}

impl State {
    pub fn new(on_event: Option<OnEventFn>, format: &Format) -> Self {
        Self {
            on_event: if let Some(on_event) = on_event {
                Rc::clone(&on_event)
            } else {
                Rc::new(event::on_event)
            },
            indent: format.indent.clone(),
            realdp: format.realdp,
            wrapwidth: format.wrapwidth as usize,
            depth: 0,
            tokens: Tokens::new(),
            list_value_counts: vec![],
            map_item_counts: vec![],
            table_record_counts: vec![],
        }
    }

    pub fn puts(&mut self, s: &str) {
        if !self.tokens.is_empty() {}
        // TODO
    }

    pub fn get_tokens(&mut self) -> Tokens {
        let mut tokens = Tokens::new();
        std::mem::swap(&mut tokens, &mut self.tokens);
        tokens
    }
}
