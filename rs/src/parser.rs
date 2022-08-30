// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::event::OnEventFn;
use crate::lexer::Lexer;
use crate::token::{debug_tokens, Tokens};
use crate::uxf::{ParserOptions, Uxf};
use anyhow::{bail, Result};
use std::rc::Rc;

pub(crate) fn parse(
    text: &str,
    filename: &str,
    options: ParserOptions,
    on_event: OnEventFn,
) -> Result<Uxf> {
    let data: Vec<char> = text.chars().collect();
    let mut lexer = Lexer::new(&data, filename, Rc::clone(&on_event));
    let (custom, tokens) = lexer.tokenize()?;
    debug_tokens(tokens); // TODO delete
    let mut uxo = Uxf::new_on_event(Rc::clone(&on_event));
    if !custom.is_empty() {
        uxo.set_custom(&custom);
    }
    let mut parser = Parser::new(
        text,
        filename,
        Rc::clone(&on_event),
        &mut uxo,
        options,
        tokens,
    );
    parser.parse()?;
    Ok(uxo)
}

pub struct Parser<'a> {
    text: &'a str,
    filename: &'a str,
    options: ParserOptions,
    on_event: OnEventFn,
    uxo: &'a mut Uxf,
    had_root: bool,
    tokens: &'a Tokens<'a>,
    // TODO see uxf.py Parser clear()
}

impl<'a> Parser<'a> {
    pub(crate) fn new(
        text: &'a str,
        filename: &'a str,
        on_event: OnEventFn,
        uxo: &'a mut Uxf,
        options: ParserOptions,
        tokens: &'a Tokens,
    ) -> Self {
        Parser {
            text,
            filename,
            on_event: Rc::clone(&on_event),
            uxo,
            options,
            tokens,
            had_root: false,
        }
    }

    pub(crate) fn parse(&mut self) -> Result<()> {
        bail!("TODO Parser::parse()") // TODO parse tokens and populate rest of uxo
    }
}
