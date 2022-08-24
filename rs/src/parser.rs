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
    let mut uxo = Uxf::new_on_event(Rc::clone(&on_event));
    let mut lexer =
        Lexer::new(raw, filename, Rc::clone(&on_event), &mut uxo);
    let tokens = lexer.tokenize()?;
    // TODO parse tokens and populate rest of uxo
    Ok(uxo)
}
