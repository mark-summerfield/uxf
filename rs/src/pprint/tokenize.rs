// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::event::{Event, OnEventFn};
use crate::format::Format;
use crate::pprint::{state::State, token::Tokens};
use crate::uxf::Uxf;
use crate::value::{Value, Visit};
use anyhow::{bail, Result};
use std::{cell::RefCell, rc::Rc};

pub(crate) fn tokenize(
    uxo: &Uxf,
    format: &Format,
    on_event: Option<OnEventFn>,
) -> Result<Tokens> {
    let state = Rc::new(RefCell::new(State::new(on_event, format)));
    uxo.visit(Rc::new({
        let state = Rc::clone(&state);
        move |visit: Visit, value: &Value| {
            match visit {
                Visit::UxfBegin => {
                    handle_uxf_begin(Rc::clone(&state), value)?
                }
                Visit::UxfEnd => eprintln!("TODO to_text visit UxfEnd"),
                Visit::Import => handle_import(Rc::clone(&state), value)?,
                Visit::Ttype => handle_ttype(Rc::clone(&state), value)?,
                Visit::ListBegin => {
                    eprintln!("TODO to_text visit ListBegin")
                }
                Visit::ListEnd => eprintln!("TODO to_text visit ListEnd"),
                Visit::ListValueBegin => {
                    eprintln!("TODO to_text visit ListValueBegin")
                }
                Visit::ListValueEnd => {
                    eprintln!("TODO to_text visit ListValueEnd")
                }
                Visit::MapBegin => eprintln!("TODO to_text visit MapBegin"),
                Visit::MapEnd => eprintln!("TODO to_text visit MapEnd"),
                Visit::MapItemBegin => {
                    eprintln!("TODO to_text visit MapItemBegin")
                }
                Visit::MapItemEnd => {
                    eprintln!("TODO to_text visit MapItemEnd")
                }
                Visit::TableBegin => {
                    eprintln!("TODO to_text visit TableBegin")
                }
                Visit::TableEnd => eprintln!("TODO to_text visit TableEnd"),
                Visit::TableRecordBegin => {
                    eprintln!("TODO to_text visit TableRecordBegin")
                }
                Visit::TableRecordEnd => {
                    eprintln!("TODO to_text visit TableRecordEnd")
                }
                Visit::Value => eprintln!("TODO to_text visit Value"),
            }
            Ok(())
        }
    }))?;
    let tokens = state.borrow_mut().get_tokens();
    Ok(tokens)
}

fn handle_uxf_begin(
    state: Rc<RefCell<State>>,
    value: &Value,
) -> Result<()> {
    if let Some(comment) = value.as_str() {
        handle_str(Rc::clone(&state), comment, "#", "\n")?;
    }
    Ok(())
}

fn handle_import(state: Rc<RefCell<State>>, value: &Value) -> Result<()> {
    if let Some(import) = value.as_str() {
        let mut state = state.borrow_mut();
        state.puts(&format!("!{}\n", &import));
        let width = import.chars().count() + 1; // +1 for !
        if width > state.wrapwidth {
            state.wrapwidth = width;
            (state.on_event)(&Event::bare_warning(
                563,
                &format!(
                    "import {:?} forced wrapwidth to be increased to {}",
                    import, width
                ),
            ));
        }
        Ok(())
    } else {
        bail!("can't write invalid import {:?}", value)
    }
}

// Each tclass is encoded as:
// [#<tclass comment> ttype [[#<fieldname1> vtype1] ... ]]
fn handle_ttype(state: Rc<RefCell<State>>, value: &Value) -> Result<()> {
    eprintln!("TODO to_text handle_ttype {:?}", value);
    Ok(())
}

fn handle_str(
    state: Rc<RefCell<State>>,
    comment: &str,
    prefix: &str,
    suffix: &str,
) -> Result<()> {
    Ok(())
}
