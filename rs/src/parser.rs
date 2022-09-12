// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::event::{Event, OnEventFn};
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
    tclass_for_ttype: HashMap<String, TClass>, // key=ttype value=TClass
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
            tclass_for_ttype: HashMap::new(),
            lino_for_tclass: HashMap::new(),
            used_tclasses: HashSet::new(),
            lino: 0,
        })
    }

    fn parse(&mut self) -> Result<()> {
        self.parse_file_comment();
        self.parse_imports()?;
        self.parse_tclasses()?;
        self.parse_data()?;
        self.update_uxo();
        Ok(())
    }

    // rust forum's 2e71828's algorithm
    fn parse_data(&mut self) -> Result<()> {
        let mut value: Option<Value> = None;
        let mut stack: Values = vec![];
        let mut pos = 0;
        while pos < self.tokens.len() {
            let token = self.tokens[pos].clone();
            pos += 1;
            let kind = &token.kind;
            if kind == &TokenKind::Eof {
                break;
            }
            self.lino = token.lino;
            let expected_type = self.expected_type(&stack);
            if let Some(element) = value.take() {
                self.handle_collection_push(element, &mut stack, &token)?;
            }
            value = if kind.is_collection_start() {
                self.on_collection_start(
                    pos,
                    &mut stack,
                    &token,
                    &expected_type,
                )?
            } else if kind.is_collection_end() {
                self.on_collection_end(&mut stack, &token)?
            } else if kind == &TokenKind::Str {
                self.handle_str(&token, &expected_type, stack.len())?
            } else if kind.is_scalar() {
                self.handle_scalar(&token, &expected_type, stack.len())?
            } else if kind == &TokenKind::Identifier {
                bail!(self.handle_invalid_identifier(&token));
            } else {
                bail!(self.error_t(410, "unexpected token", &token));
            };
        }
        if !self.is_import {
            self.cleanup_tclasses();
        }
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
                        &mut self.tclass_for_ttype,
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

    fn expected_type(&self, stack: &Values) -> String {
        if let Some(value) = stack.last() {
            if let Some(lst) = value.as_list() {
                lst.expected_type()
            } else if let Some(m) = value.as_map() {
                m.expected_type()
            } else if let Some(t) = value.as_table() {
                t.expected_type()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
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
        &mut self,
        pos: usize,
        stack: &mut Values,
        token: &Token,
        expected_type: &str,
    ) -> Result<Option<Value>> {
        let next_value = if pos < self.tokens.len() {
            Some(self.tokens[pos].clone())
        } else {
            None
        };
        stack.push(self.handle_collection_start(
            &token.clone(),
            next_value,
            expected_type,
        )?);
        Ok(None)
    }

    fn handle_collection_start(
        &mut self,
        token: &Token,
        next_value: Option<Token>,
        expected_type: &str,
    ) -> Result<Value> {
        match token.kind {
            TokenKind::ListBegin => self.handle_list_start(token),
            TokenKind::MapBegin => self.handle_map_start(token),
            TokenKind::TableBegin => {
                self.handle_table_start(token, next_value, expected_type)
            }
            _ => bail!(self.error_t(
                504,
                "expected to create a map, list, or table",
                token
            )),
        }
    }

    fn handle_list_start(&mut self, token: &Token) -> Result<Value> {
        self.verify_type_identifier(&token.vtype)?;
        Ok(Value::from(List::new(&token.vtype, &token.comment)?))
    }

    fn handle_map_start(&mut self, token: &Token) -> Result<Value> {
        if !token.ktype.is_empty()
            && !KTYPES.contains(&token.ktype.as_str())
        {
            bail!(self.error_s(440, "expected map ktype", &token.ktype))
        }
        self.verify_type_identifier(&token.vtype)?;
        Ok(Value::from(Map::new(
            &token.ktype,
            &token.vtype,
            &token.comment,
        )?))
    }

    fn handle_table_start(
        &mut self,
        token: &Token,
        next_value: Option<Token>,
        expected_type: &str,
    ) -> Result<Value> {
        if let Some(tclass) = self.tclass_for_ttype.get(&token.vtype) {
            let ttype = tclass.ttype();
            self.used_tclasses.insert(ttype.to_string());
            self.verify_ttype_identifier(ttype, expected_type)?;
            Ok(Value::from(Table::new(tclass.clone(), &token.comment)))
        } else {
            let next_value = if let Some(next_value) = next_value {
                next_value.to_string()
            } else {
                "nothing".to_string()
            };
            bail!(self.error_s(
                450,
                "expected table ttype, got {:?}",
                &next_value
            ))
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

    fn handle_str(
        &mut self,
        token: &Token,
        expected_type: &str,
        size: usize,
    ) -> Result<Option<Value>> {
        if size == 0 {
            bail!(self.error(590, "invalid UXF data"));
        }
        let mut value = token.value.clone();
        let message = self.verify_type(&value, expected_type);
        if value != Value::Null
            && !message.is_empty()
            && ["bool", "int", "real", "date", "datetime"]
                .contains(&expected_type)
        {
            let new_value = value.naturalize();
            if new_value != value {
                (self.on_event)(&Event::new_repair(
                    486,
                    &format!(
                        "converted str {:?} to {:?}",
                        value, new_value
                    ),
                    self.filename,
                    self.lino,
                ));
                value = new_value;
            } else {
                bail!(self.error(488, &message));
            }
        }
        Ok(Some(value))
    }

    fn handle_scalar(
        &mut self,
        token: &Token,
        expected_type: &str,
        size: usize,
    ) -> Result<Option<Value>> {
        if size == 0 {
            bail!(self.error(501, "invalid UXF data"));
        }
        let mut value = token.value.clone();
        let message = self.verify_type(&value, expected_type);
        if value != Value::Null && !message.is_empty() {
            let new_value = if expected_type == "real" && value.is_int() {
                Value::Real(value.as_int().unwrap() as f64) // safe
            } else if expected_type == "int" && value.is_real() {
                Value::Int(value.as_real().unwrap().round() as i64) // safe
            } else {
                bail!(self.error(500, &message));
            };
            (self.on_event)(&Event::new_repair(
                486,
                &format!("converted str {:?} to {:?}", value, new_value),
                self.filename,
                self.lino,
            ));
            value = new_value;
        }
        Ok(Some(value))
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

    fn verify_type_identifier(&mut self, vtype: &str) -> Result<()> {
        if !vtype.is_empty() {
            if VTYPES.contains(&vtype) {
                return Ok(()); // built-in type
            }
            if let Some(tclass) = self.tclass_for_ttype.get(vtype) {
                self.used_tclasses.insert(tclass.ttype().to_string());
            } else {
                bail!(self.error_s(446, "expected vtype", vtype));
            }
        }
        Ok(())
    }

    fn verify_ttype_identifier(
        &self,
        ttype: &str,
        expected_type: &str,
    ) -> Result<()> {
        if !expected_type.is_empty()
            && expected_type != "table"
            && expected_type != ttype
        {
            bail!(
                "E456:{}:{}:expected table value of type {}, got value \
                 of type {}",
                self.filename,
                self.lino,
                expected_type,
                ttype
            );
        }
        Ok(())
    }

    // uxf.py: typecheck()
    fn verify_type(&self, value: &Value, expected_type: &str) -> String {
        if value != &Value::Null && !expected_type.is_empty() {
            if VTYPES.contains(&expected_type) {
                if value.typename() != expected_type {
                    return format!(
                        "expected {}, got {:?}",
                        expected_type, &value
                    );
                }
            } else if !self.tclass_for_ttype.contains_key(expected_type) {
                return format!(
                    "expected {}, got {:?}",
                    expected_type, &value
                );
            }
        }
        "".to_string()
    }

    fn update_uxo(&mut self) {
        std::mem::swap(
            &mut self.uxo.tclass_for_ttype,
            &mut self.tclass_for_ttype,
        );
    }

    fn cleanup_tclasses(&mut self) {
        let mut imported: HashSet<String> =
            self.imports.keys().cloned().collect();
        if self.options.contains(ParserOptions::REPLACE_IMPORTS) {
            for ttype in &imported {
                if !self.used_tclasses.contains(ttype) {
                    self.tclass_for_ttype.remove(ttype); // unused ttype
                }
            }
            self.imports.clear();
            imported.clear();
        }
        let mut defined: HashSet<String> =
            self.tclass_for_ttype.keys().cloned().collect();
        if self.options.contains(ParserOptions::DROP_UNUSED_TTYPES) {
            let mut ttypes_for_filename = self.get_ttypes_for_filename();
            for ttype in &defined.clone() {
                if !self.used_tclasses.contains(ttype) {
                    self.tclass_for_ttype.remove(ttype); // unused ttype def
                    defined.remove(ttype);
                    for (_, ttypes) in ttypes_for_filename.iter_mut() {
                        ttypes.remove(ttype.as_str());
                    }
                }
            }
            for (filename, ttypes) in ttypes_for_filename {
                if ttypes.is_empty() {
                    let mut ttypes: Vec<String> = vec![];
                    for (ttype, ifilename) in &self.imports {
                        if filename == *ifilename {
                            ttypes.push(ttype.to_string());
                        }
                    }
                    for ttype in &ttypes {
                        self.imports.remove(ttype); // del unused import
                    }
                }
            }
        }
        let mut unused: HashSet<String> = defined
            .difference(&self.used_tclasses)
            .map(|s| s.to_string())
            .collect();
        // don't warn unused imports
        unused =
            unused.difference(&imported).map(|s| s.to_string()).collect();
        // TODO ####################################
    }

    fn get_ttypes_for_filename(&self) -> HashMap<String, HashSet<String>> {
        let mut ttypes_for_filename: HashMap<String, HashSet<String>> =
            HashMap::new();
        for (ttype, filename) in &self.imports {
            ttypes_for_filename
                .entry(ttype.to_string())
                .and_modify(|v| {
                    v.insert(filename.to_string());
                })
                .or_insert({
                    let mut set = HashSet::new();
                    set.insert(filename.to_string());
                    set
                });
        }
        ttypes_for_filename
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
    tclass_for_ttype: &mut HashMap<String, TClass>,
    tclass: TClass,
    filename: &str,
    lino: usize,
    code: u16,
) -> Result<bool> {
    let first_tclass = if let Some(first_tclass) =
        tclass_for_ttype.get_mut(tclass.ttype())
    {
        first_tclass
    } else {
        tclass_for_ttype.insert(tclass.ttype().to_string(), tclass);
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
