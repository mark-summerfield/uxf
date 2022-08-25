// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::event::OnEventFn;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use crate::uxf::{ParseOptions, Uxf};
use anyhow::Result;
use std::rc::Rc;

pub(crate) fn parse(
    text: &str,
    filename: &str,
    options: ParseOptions,
    on_event: OnEventFn,
) -> Result<Uxf> {
    let mut uxo = Uxf::new_on_event(Rc::clone(&on_event));
    let data: Vec<char> = text.chars().collect();
    let mut lexer =
        Lexer::new(&data, filename, Rc::clone(&on_event), &mut uxo);
    let tokens = lexer.tokenize()?;
    debug_tokens(tokens); // TODO delete
                          // TODO parse tokens and populate rest of uxo
    Ok(uxo)
}

fn debug_tokens(tokens: &[Token]) {
    let mut indent = 0;
    for token in tokens.iter() {
        if matches!(
            &token.kind,
            TokenKind::ListEnd | TokenKind::MapEnd | TokenKind::TableEnd
        ) {
            indent -= 1;
        }
        if indent > 0 {
            print!("{}", "  ".repeat(indent));
        }
        println!("{}", token);
        if matches!(
            &token.kind,
            TokenKind::ListBegin
                | TokenKind::MapBegin
                | TokenKind::TableBegin
        ) {
            indent += 1;
        }
    }
    println!("----------------------------------------");
}
