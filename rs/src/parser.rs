// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::event::OnEventFn;
use crate::lexer::Lexer;
use crate::tclass::TClass;
use crate::token::{Token, TokenKind, Tokens};
use crate::util::full_filename;
use crate::uxf::{ParserOptions, Uxf};
use crate::value::Value;
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
    let (custom, mut tokens) = lexer.tokenize()?;
    let mut uxo = Uxf::new_on_event(Rc::clone(&on_event));
    if !custom.is_empty() {
        uxo.set_custom(&custom);
    }
    if !tokens.is_empty() {
        let mut parser = Parser::new(
            text,
            filename,
            Rc::clone(&on_event),
            &mut uxo,
            options,
            &mut tokens,
            None, // not an import and no imports carried over
        )?;
        parser.parse()?;
    }
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
    tokens: &'a mut Tokens<'a>,
    stack: Vec<Token<'a>>,
    imports: HashMap<String, String>, // key=ttype value=import text
    imported: HashSet<String>, // ttype (to avoid reimports or self import)
    tclasses: HashMap<String, TClass>, // key=ttype value=TClass
    lino_for_tclass: HashMap<String, usize>, // key=ttype value=lino
    used_tclasses: HashSet<String>, // ttype (of ttypes actually used)
    lino: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(
        text: &'a str, // TODO do we need this?
        filename: &'a str,
        on_event: OnEventFn,
        uxo: &'a mut Uxf,
        options: ParserOptions,
        tokens: &'a mut Tokens<'a>,
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
            lino: 0,
        })
    }

    fn parse(&mut self) -> Result<()> {
        let mut value: Option<Value> = None;
        self.parse_file_comment();
        self.parse_imports()?;
        self.parse_tclasses()?;
        Ok(())
    }

    fn parse_file_comment(&mut self) {
        if !self.tokens.is_empty()
            && self.tokens[0].kind == TokenKind::FileComment
        {
            let token = self.tokens.pop_front().unwrap(); // safe
            self.lino = token.lino;
            self.uxo.set_comment(token.value.as_str().unwrap());
        }
    }

    fn parse_imports(&mut self) -> Result<()> {
        while !self.tokens.is_empty()
            && self.tokens[0].kind == TokenKind::Import
        {
            let token = self.tokens.pop_front().unwrap(); // safe
            self.lino = token.lino;
            self.handle_import(token.value.as_str().unwrap())?;
        }
        Ok(())
    }

    fn handle_import(&mut self, value: &str) -> Result<()> {
        bail!("TODO parser::handle_import"); // TODO (last)
    }

    fn parse_tclasses(&mut self) -> Result<()> {
        // TODO
        Ok(())
    }
}
