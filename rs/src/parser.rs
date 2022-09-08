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
    filename: &'a str,
    options: ParserOptions,
    on_event: OnEventFn,
    uxo: &'a mut Uxf,
    is_import: bool,
    tokens: &'a mut Tokens,
    imports: HashMap<String, String>, // key=ttype value=import text
    imported: HashSet<String>, // ttype (to avoid reimports or self import)
    tclasses: HashMap<String, TClass>, // key=ttype value=TClass
    lino_for_tclass: HashMap<String, usize>, // key=ttype value=lino
    used_tclasses: HashSet<String>, // ttype (of ttypes actually used)
    lino: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(
        filename: &'a str,
        on_event: OnEventFn,
        uxo: &'a mut Uxf,
        options: ParserOptions,
        tokens: &'a mut Tokens,
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
            filename,
            on_event: Rc::clone(&on_event),
            uxo,
            options,
            is_import,
            tokens,
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
        self.parse_root()
    }

    // rust forum's 2e71828's algorithm
    fn parse_root(&mut self) -> Result<()> {
        let mut value: Option<Value> = None;
        let mut stack: Values = vec![];
        let mut pos = 0;
        while pos < self.tokens.len() {
            let token = &self.tokens[pos];
            pos += 1;
            let kind = &token.kind;
            if kind == &TokenKind::Eof {
                break;
            }
            self.lino = token.lino;
            if let Some(element) = value.take() {
                self.handle_collection_push(element, &mut stack, token)?;
            }
            value = if kind.is_collection_start() {
                self.on_collection_start(pos, &mut stack, token)?
            } else if kind.is_collection_end() {
                self.on_collection_end(&mut stack, token)?
            } else if kind == &TokenKind::Str {
                Some(Value::Null) // TODO MUST RETURN a Value
            } else if kind.is_scalar() {
                Some(Value::Null) // TODO MUST RETURN a Value
            } else if kind == &TokenKind::Identifier {
                bail!(self.handle_invalid_identifier(token));
            } else {
                bail!(self.error_t(410, "unexpected token", token));
            };
        }
        // TODO if not is_import: check_tclasses
        if let Some(value) = value {
            if value.is_collection() {
                self.uxo.set_value(value)?
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
                    self.handle_tclass_begin(&mut tclass_builder, token);
                    lino = self.lino;
                }
                TokenKind::Field => {
                    self.handle_tclass_field(&mut tclass_builder, token)?;
                }
                TokenKind::TClassEnd => {
                    let tclass = if tclass_builder.is_valid() {
                        tclass_builder.build()?
                    } else {
                        bail!(self.error(526, "TClass without ttype"));
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

    fn handle_tclass_begin(
        &self,
        tclass_builder: &mut TClassBuilder,
        token: &Token,
    ) {
        tclass_builder
            .initialize(token.value.as_str().unwrap(), &token.comment);
    }

    fn handle_tclass_field(
        &self,
        tclass_builder: &mut TClassBuilder,
        token: &Token,
    ) -> Result<()> {
        if tclass_builder.is_valid() {
            tclass_builder
                .append_field(token.value.as_str().unwrap(), &token.vtype)
        } else {
            bail!(self.error(524, "Field outside TClass"));
        }
    }

    fn handle_collection_push(
        &self,
        element: Value,
        stack: &mut Values,
        token: &Token,
    ) -> Result<()> {
        if let Some(collection) = stack.last_mut() {
            if let Some(lst) = collection.as_list_mut() {
                lst.push(element);
            } else if let Some(m) = collection.as_map_mut() {
                m.push(element)?;
            } else if let Some(t) = collection.as_table_mut() {
                t.push(element)?;
            } else {
                bail!(self.error_t(
                    402,
                    "expected a map, list, or table",
                    token
                ));
            }
        }
        Ok(())
    }

    fn on_collection_start(
        &self,
        pos: usize,
        stack: &mut Values,
        token: &Token,
    ) -> Result<Option<Value>> {
        let next_value = if pos < self.tokens.len() {
            Some(self.tokens[pos].clone())
        } else {
            None
        };
        stack.push(
            self.handle_collection_start(&token.clone(), next_value)?,
        );
        Ok(None)
    }

    fn handle_collection_start(
        &self,
        token: &Token,
        next_value: Option<Token>,
    ) -> Result<Value> {
        match token.kind {
            TokenKind::ListBegin => self.handle_list_start(token),
            TokenKind::MapBegin => self.handle_map_start(token),
            TokenKind::TableBegin => self.handle_table_start(token),
            _ => bail!(self.error_t(
                504,
                "expected to create a map, list, or table",
                token
            )),
        }
    }

    fn handle_list_start(&self, token: &Token) -> Result<Value> {
        // self.verify_type_identifier(&token.vtype)?; // TODO
        Ok(Value::from(List::new(&token.vtype, &token.comment)?))
    }

    fn handle_map_start(&self, token: &Token) -> Result<Value> {
        if !token.ktype.is_empty()
            && !KTYPES.contains(&token.ktype.as_str())
        {
            bail!(self.error_s(440, "expected map ktype", &token.ktype))
        }
        // self.verify_type_identifier(&token.vtype)?; // TODO
        Ok(Value::from(Map::new(
            &token.ktype,
            &token.vtype,
            &token.comment,
        )?))
    }

    fn handle_table_start(&self, token: &Token) -> Result<Value> {
        if let Some(tclass) = self.tclasses.get(&token.vtype) {
            // self.verify_ttype_identifier(tclass, next_value) // TODO
            Ok(Value::from(Table::new(tclass.clone(), &token.comment)))
        } else {
            bail!(self.error_s(503, "undefined ttype", &token.vtype))
        }
    }

    fn on_collection_end(
        &self,
        stack: &mut Values,
        token: &Token,
    ) -> Result<Option<Value>> {
        if let Some(value) = stack.pop() {
            Ok(Some(value))
        } else {
            bail!(self.error_t(
                403,
                "missing a map, list, or table",
                token
            ));
        }
    }

    fn handle_invalid_identifier(&self, token: &Token) -> String {
        // All valid identifiers have already been handled
        if let Some(s) = token.value.as_str() {
            if ["true", "false"].contains(&s.to_lowercase().as_str()) {
                return self.error(
                    458,
                    "boolean values are represented by yes or no",
                );
            }
        }
        self.error_t(
            460,
            "ttypes may only appear at the start of a map (as the \
            value type), list, or table",
            token,
        )
    }

    fn error(&self, code: u16, message: &str) -> String {
        format!("E{}:{}:{}:{}", code, self.filename, self.lino, message)
    }

    fn error_s(&self, code: u16, message: &str, s: &str) -> String {
        format!(
            "E{}:{}:{}:{}, got {:?}",
            code, self.filename, self.lino, message, s
        )
    }

    fn error_t(&self, code: u16, message: &str, t: &Token) -> String {
        format!(
            "E{}:{}:{}:{}, got {:?}",
            code, self.filename, self.lino, message, t
        )
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
