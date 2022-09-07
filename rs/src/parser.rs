// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::event::OnEventFn;
use crate::lexer::Lexer;
use crate::list::List;
use crate::map::Map;
use crate::table::Table;
use crate::tclass::{TClass, TClassBuilder};
use crate::token::{Token, TokenKind, Tokens};
use crate::util::full_filename;
use crate::uxf::{ParserOptions, Uxf};
use crate::value::{Value, Values};
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
    is_import: bool,
    tokens: &'a mut Tokens<'a>,
    root: Option<Value>,
    stack: Values,
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
            is_import,
            tokens,
            root: None,
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
        self.parse_file_comment();
        self.parse_imports()?;
        self.parse_tclasses()?;
        let mut pos = 0;
        while pos < self.tokens.len() {
            let token = &self.tokens[pos];
            self.lino = token.lino;
            let kind = &token.kind;
            let is_collection_start = kind.is_collection_start();
            if let Some(element) = self.root.take() {
                if let Some(collection) = self.stack.last_mut() {
                    if collection.is_list() {
                        if let Some(lst) = collection.as_list_mut() {
                            lst.push(element);
                        }
                    } else if collection.is_map() {
                        if let Some(m) = collection.as_map_mut() {
                            m.push(element)?;
                        }
                    } else if collection.is_table() {
                        if let Some(t) = collection.as_table_mut() {
                            t.push(element)?;
                        }
                    }
                }
            }
            if self.root.is_none() && !is_collection_start {
                bail!(
                    "E402:{}:{}:expected a map, list, or table, got {:?}",
                    self.filename,
                    self.lino,
                    token
                );
            }
            self.root = if is_collection_start {
                let next_value = if pos + 1 < self.tokens.len() {
                    Some(self.tokens[pos + 1].clone())
                } else {
                    None
                };
                self.handle_collection_start(&token.clone(), next_value)?;
                None
            } else if kind.is_collection_end() {
                Some(self.stack.pop().unwrap())
            } else {
                // Some(token.into())
                None // TODO ############# MUST RETURN a Value
            };
            pos += 1;
        }
        let root = self.root.replace(Value::Null);
        if let Some(root) = root {
            if root.is_collection() {
                self.uxo.set_value(root)?
            }
        }
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
        bail!("TODO parser::handle_import: {:?}", value); // TODO (last)
    }

    fn parse_tclasses(&mut self) -> Result<()> {
        let mut tclass_builder = TClassBuilder::default();
        let mut offset = 0;
        let mut lino = 0;
        for (index, token) in self.tokens.iter().enumerate() {
            self.lino = token.lino;
            match token.kind {
                TokenKind::TClassBegin => {
                    tclass_builder.initialize(
                        token.value.as_str().unwrap(),
                        &token.comment,
                    );
                    lino = self.lino;
                }
                TokenKind::Field => {
                    if tclass_builder.is_valid() {
                        tclass_builder.append_field(
                            token.value.as_str().unwrap(),
                            &token.vtype,
                        )?;
                    } else {
                        bail!(
                            "E524:{}:{}:Field outside TClass",
                            self.filename,
                            self.lino
                        );
                    }
                }
                TokenKind::TClassEnd => {
                    let tclass = if tclass_builder.is_valid() {
                        tclass_builder.build()?
                    } else {
                        bail!(
                            "E526:{}:{}:TClass without ttype",
                            self.filename,
                            self.lino
                        );
                    };
                    add_to_tclasses(
                        &mut self.tclasses,
                        tclass,
                        self.filename,
                        self.lino,
                        528,
                    )?;
                    self.lino_for_tclass
                        .insert(tclass_builder.ttype.to_string(), lino);
                    offset = index + 1;
                    tclass_builder.clear();
                    lino = 0;
                }
                _ => break, // no TClasses at all
            }
        }
        self.tokens.drain(..offset);
        Ok(())
    }

    fn handle_collection_start(
        &mut self,
        token: &Token,
        next_value: Option<Token>,
    ) -> Result<()> {
        self.stack.push(match token.kind {
            TokenKind::ListBegin => {
                // self.verify_type_identifier(&token.vtype)?; // TODO
                Value::from(List::new(&token.vtype, &token.comment)?)
            }
            TokenKind::MapBegin => {
                if !token.ktype.is_empty()
                    && !KTYPES.contains(&token.ktype.as_str())
                {
                    bail!(
                        "E440:{}:{}:expected map ktype, got {:?}",
                        self.filename,
                        self.lino,
                        token.ktype
                    )
                }
                // self.verify_type_identifier(&token.vtype)?; // TODO
                Value::from(Map::new(
                    &token.ktype,
                    &token.vtype,
                    &token.comment,
                )?)
            }
            TokenKind::TableBegin => Value::Null, // TODO
            _ => bail!(
                "E504:{}:{}:expected to create a map, list, or table, \
                got {:?}",
                self.filename,
                self.lino,
                token
            ),
        });
        Ok(())
    }
}

fn add_to_tclasses(
    tclasses: &mut HashMap<String, TClass>,
    tclass: TClass,
    filename: &str,
    lino: usize,
    code: u16,
) -> Result<bool> {
    let first_tclass =
        if let Some(first_tclass) = tclasses.get_mut(tclass.ttype()) {
            first_tclass
        } else {
            tclasses.insert(tclass.ttype().to_string(), tclass);
            return Ok(true); // this is the first definition of this ttype
        };
    if first_tclass == &tclass {
        if !tclass.comment().is_empty()
            && tclass.comment() != first_tclass.comment()
        {
            first_tclass.set_comment(tclass.comment()); // last one wins
        }
        return Ok(true); // harmless duplicate
    }
    bail!(
        "E{}:{}:{}:conflicting ttype definitions for {}",
        code,
        filename,
        lino,
        tclass.ttype()
    )
}
