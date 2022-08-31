// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::event::OnEventFn;
use crate::lexer::Lexer;
use crate::tclass::TClass;
use crate::token::Tokens;
use crate::util::full_filename;
use crate::uxf::{ParserOptions, Uxf};
use anyhow::{bail, Result};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub(crate) fn parse(
    text: &str,
    filename: &str,
    options: ParserOptions,
    on_event: OnEventFn,
) -> Result<Uxf> {
    let data: Vec<char> = text.chars().collect();
    let mut lexer = Lexer::new(&data, filename, Rc::clone(&on_event));
    let (custom, tokens) = lexer.tokenize()?;
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
        None, // not an import and no imports carried over
    )?;
    parser.parse()?;
    Ok(uxo)
}

pub struct Parser<'a> {
    text: &'a str, // TODO do we need this?
    filename: &'a str,
    options: ParserOptions,
    on_event: OnEventFn,
    uxo: &'a mut Uxf,
    had_root: bool,
    is_import: bool,
    tokens: &'a Tokens<'a>,
    stack: Tokens<'a>,
    imports: HashMap<String, String>, // key=ttype value=import text
    imported: HashSet<String>, // ttype (to avoid reimports or self import)
    tclasses: HashMap<String, TClass>, // key=ttype value=TClass
    lino_for_tclass: HashMap<String, usize>, // key=ttype value=lino
    used_tclasses: HashSet<String>, // ttype (of ttypes actually used)
    pos: usize,
    lino: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(
        text: &'a str, // TODO do we need this?
        filename: &'a str,
        on_event: OnEventFn,
        uxo: &'a mut Uxf,
        options: ParserOptions,
        tokens: &'a Tokens,
        // None for not an import; empty for an import that has no ttypes
        imported: Option<HashSet<String>>,
    ) -> Result<Self> {
        let (is_import, mut imported) = if let Some(imported) = imported {
            (true, imported)
        } else {
            (false, HashSet::new())
        };
        if !filename.is_empty() && filename != "-" {
            let filename = full_filename(filename, ".");
            if imported.contains(&filename) {
                bail!("E400:{}:0:already imported this file", filename)
            }
            imported.insert(filename);
        }
        Ok(Parser {
            text, // TODO do we need this?
            filename,
            on_event: Rc::clone(&on_event),
            uxo,
            options,
            had_root: false,
            is_import,
            tokens,
            stack: vec![],
            imports: HashMap::new(),
            imported,
            tclasses: HashMap::new(),
            lino_for_tclass: HashMap::new(),
            used_tclasses: HashSet::new(),
            pos: INVALID_POS,
            lino: 0,
        })
    }

    fn parse(&mut self) -> Result<()> {
        bail!("TODO Parser::parse()") // TODO
    }
}
