// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::event::OnEventFn;
use crate::lexer::Lexer;
use crate::uxf::{ParseOptions, Uxf};
use anyhow::Result;
use std::rc::Rc;

pub(crate) fn parse(
    raw: &Vec<u8>,
    filename: &str,
    options: ParseOptions,
    on_event: OnEventFn,
) -> Result<Uxf> {
    let mut lexer = Lexer::new(raw, filename, Rc::clone(&on_event));
    let tokens = lexer.tokenize()?;
    let mut uxo = Uxf::new_on_event(Rc::clone(&on_event));
    uxo.set_custom(lexer.custom);
    // TODO parse tokens and populate uxo
    Ok(uxo)
}
