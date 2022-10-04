// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::consts::*;
use crate::event::{Event, OnEventFn};
use crate::field::make_fields_x;
use crate::list::List;
use crate::map::Map;
use crate::parser::{
    lexer::Lexer,
    token::{Token, TokenKind, Tokens},
};
use crate::table::Table;
use crate::tclass::{TClass, TClassBuilder};
use crate::util::{dirname, full_filename, read_file};
use crate::uxf::{ParserOptions, Uxf};
use crate::value::{Value, Values};
use anyhow::{bail, Result};
use indexmap::map::IndexMap;
use std::{
    collections::{HashMap, HashSet},
    env,
    path::Path,
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
    let mut uxo = Uxf::default();
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

fn parse_import(
    text: &str,
    filename: &str,
    on_event: OnEventFn,
    imported: HashSet<String>,
) -> Result<Uxf> {
    let data: Vec<char> = if text.is_empty() {
        read_file(filename)?.chars().collect()
    } else {
        text.chars().collect()
    };
    let mut lexer = Lexer::new(&data, filename, Rc::clone(&on_event));
    let (_, mut tokens) = lexer.tokenize()?; // ignore comment
    let mut uxo = Uxf::default();
    if !tokens.is_empty() {
        let mut parser = Parser::new(
            filename,
            Rc::clone(&on_event),
            &mut uxo,
            ParserOptions::DEFAULT, // ignore options
            &mut tokens,
            Some(imported),
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
    tokens: &'a mut Tokens,
    is_import: bool,
    imported: HashSet<String>,
    import_for_ttype: IndexMap<String, String>, // ttype x import text
    tclass_for_ttype: HashMap<String, TClass>,  // ttype x TClass
    lino_for_tclass: HashMap<String, usize>,    // key=ttype value=lino
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
                bail!("E400:{}:0:already imported", filename)
            }
            imported.insert(filename);
        }
        Ok(Parser {
            filename,
            on_event: Rc::clone(&on_event),
            uxo,
            options,
            tokens,
            is_import,
            imported,
            import_for_ttype: IndexMap::new(),
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
            if let Some(element) = value.take() {
                self.handle_collection_push(element, &mut stack, &token)?;
            }
            value = if kind.is_collection_start() {
                self.on_collection_start(pos, &mut stack, &token)?
            } else if kind.is_collection_end() {
                self.on_collection_end(&mut stack, &token)?
            } else if kind == &TokenKind::Str {
                let expected_type = self.expected_type(&stack);
                self.handle_str(&token, &expected_type, stack.len())?
            } else if kind.is_scalar() {
                let expected_type = self.expected_type(&stack);
                self.handle_scalar(&token, &expected_type, stack.len())?
            } else if kind == &TokenKind::Identifier {
                bail!(self.handle_invalid_identifier(&token));
            } else {
                bail!(self.error_t(410, "unexpected token", &token));
            };
        }
        if !self.is_import {
            self.cleanup_tclasses()?;
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
        let (filename, text) = if value.starts_with("http://")
            || value.starts_with("https://")
        {
            let (text, already_imported) = self.url_import(value)?;
            if already_imported {
                return Ok(()); // don't reimport & errors already handled
            }
            ("".to_string(), text)
        } else if !value.contains('.') {
            return self.system_import(value);
        } else {
            (value.to_string(), "".to_string())
        };
        let (uxo, already_imported) = if !filename.is_empty() {
            self.load_import(&filename)?
        } else if !text.is_empty() {
            self.parse_import(&text, value)?
        } else {
            bail!(self.error(
                540,
                &format!(
                    "there are no ttype definitions to import \
                    {value} ({filename:?})"
                )
            ));
        };
        if already_imported {
            return Ok(()); // don't reimport & errors already handled
        }
        if let Some(uxo) = uxo {
            for (ttype, tclass) in &uxo.tclass_for_ttype {
                if add_to_tclasses(
                    &mut self.tclass_for_ttype,
                    tclass.clone(),
                    self.filename,
                    self.lino,
                    544,
                )? {
                    self.import_for_ttype
                        .insert(ttype.to_string(), value.to_string());
                }
            }
        } else {
            bail!(self.error(541, "invalid UXF data"));
        }
        Ok(())
    }

    fn url_import(&mut self, url: &str) -> Result<(String, bool)> {
        if self.imported.contains(url) {
            return Ok(("".to_string(), true));
        }
        self.imported.insert(url.to_string()); // don't want to retry
        match reqwest::blocking::get(url) {
            Ok(reply) => match reply.text() {
                Ok(text) => Ok((text, false)),
                Err(err) => bail!(self.error(
                    551,
                    &format!("failed to read import's text: {err}")
                )),
            },
            Err(err) => bail!(self
                .error(550, &format!("failed to download import: {err}"))),
        }
    }

    fn parse_import(
        &mut self,
        text: &str,
        filename: &str,
    ) -> Result<(Option<Uxf>, bool)> {
        match parse_import(
            text,
            filename,
            self.on_event.clone(),
            self.imported.clone(),
        ) {
            Ok(uxo) => Ok((Some(uxo), false)),
            Err(err) => bail!(self.error(
                530,
                &format!("failed to import {filename:?}: {err}")
            )),
        }
    }

    fn system_import(&mut self, import: &str) -> Result<()> {
        match import {
            "complex" => self.system_import_tclass("Complex", import),
            "fraction" => self.system_import_tclass("Fraction", import),
            "numeric" => {
                self.system_import_tclass("Complex", import)?;
                self.system_import_tclass("Fraction", import)
            }
            _ => bail!(self.error(
                560,
                &format!(
                    "there is no system ttype import called {import:?}",
                )
            )),
        }
    }

    fn system_import_tclass(
        &mut self,
        ttype: &str,
        import: &str,
    ) -> Result<()> {
        let fields = if ttype == "Complex" {
            make_fields_x(
                &[("Real", "real"), ("Imag", "real")],
                self.filename,
                self.lino,
            )?
        } else {
            // Fraction
            make_fields_x(
                &[("numerator", "int"), ("denominator", "int")],
                self.filename,
                self.lino,
            )?
        };
        let tclass = TClass::new(ttype, fields, "")?;
        if add_to_tclasses(
            &mut self.tclass_for_ttype,
            tclass,
            self.filename,
            self.lino,
            570,
        )? {
            self.import_for_ttype
                .insert(ttype.to_string(), import.to_string());
        }
        Ok(())
    }

    fn load_import(
        &mut self,
        filename: &str,
    ) -> Result<(Option<Uxf>, bool)> {
        let (filename, already_imported) = self.find_import(filename);
        if already_imported {
            return Ok((None, true)); // don't reimport
        }
        match self.parse_import("", &filename) {
            Ok(reply) => {
                self.imported.insert(filename.clone()); // don't reimport
                Ok(reply)
            }
            Err(err) => {
                let err = err.to_string();
                if err.contains("E530:") && err.contains("E450:") {
                    bail!(self.error_f(
                        580,
                        "cannot do circular imports",
                        &filename
                    ))
                }
                self.imported.insert(filename.clone()); // don't retry
                bail!(self.error_f(586, "failed to import", &filename))
            }
        }
    }

    fn parse_tclasses(&mut self) -> Result<()> {
        let mut tclass_builder = TClassBuilder::default();
        let mut offset = 0;
        let mut lino = 0;
        for (index, token) in self.tokens.iter().enumerate() {
            self.lino = token.lino;
            match token.kind {
                TokenKind::TClassBegin => {
                    self.handle_tclass_begin(&mut tclass_builder, token)?;
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
    ) -> Result<()> {
        if let Some(s) = token.value.as_str() {
            tclass_builder.initialize(s, &token.comment);
            Ok(())
        } else {
            bail!(self.error(523, "invalid or missing ttype name"))
        }
    }

    fn handle_tclass_field(
        &self,
        tclass_builder: &mut TClassBuilder,
        token: &Token,
    ) -> Result<()> {
        if tclass_builder.is_valid() {
            if let Some(s) = token.value.as_str() {
                tclass_builder.append_field(s, &token.vtype)
            } else {
                bail!(self.error(522, "invalid or missing field name"))
            }
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
                m.push_x(element, self.filename, self.lino)?;
            } else if let Some(t) = collection.as_table_mut() {
                t.push_x(element, self.filename, self.lino)?;
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
    ) -> Result<Option<Value>> {
        let expected_type = self.expected_type(stack);
        self.check_contained_collection_type(token, &expected_type)?;
        let next_value = if pos < self.tokens.len() {
            Some(self.tokens[pos].clone())
        } else {
            None
        };
        stack.push(self.handle_collection_start(
            &token.clone(),
            next_value,
            &expected_type,
        )?);
        Ok(None)
    }

    fn check_contained_collection_type(
        &self,
        token: &Token,
        expected_type: &str,
    ) -> Result<()> {
        let typename = token.typename();
        if expected_type.is_empty()
            || expected_type == typename
            || expected_type == token.vtype /* can't be ktype */
            || expected_type == token.value.as_str().unwrap_or("")
        {
            Ok(())
        } else {
            let xtype = if !token.vtype.is_empty() {
                format!(" of type {}", &token.vtype)
            } else if let Some(value) = token.value.as_str() {
                format!(" {value}")
            } else {
                "".to_string()
            };
            bail!(self.error(
                506,
                &format!("expected {expected_type}, got {typename}{xtype}",)
            ))
        }
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
        self.verify_type_identifier(&token.vtype, "list")?;
        Ok(Value::from(List::new(&token.vtype, &token.comment)?))
    }

    fn handle_map_start(&mut self, token: &Token) -> Result<Value> {
        if !token.ktype.is_empty()
            && !KTYPES.contains(&token.ktype.as_str())
        {
            bail!(self.error_s(440, "expected map ktype", &token.ktype))
        }
        self.verify_type_identifier(&token.vtype, "map")?;
        Ok(Value::from(Map::new_x(
            &token.ktype,
            &token.vtype,
            &token.comment,
            self.filename,
            self.lino,
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
            bail!(self.error_s(450, "expected table ttype", &next_value))
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
                        "converted {} {value} to {} {new_value}",
                        value.typename(),
                        new_value.typename(),
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
                &format!(
                    "converted {} {value} to {} {new_value}",
                    value.typename(),
                    new_value.typename(),
                ),
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

    fn verify_type_identifier(
        &mut self,
        vtype: &str,
        what: &str,
    ) -> Result<()> {
        if !vtype.is_empty() {
            if VTYPES.contains(&vtype) {
                return Ok(()); // built-in type
            }
            if let Some(tclass) = self.tclass_for_ttype.get(vtype) {
                self.used_tclasses.insert(tclass.ttype().to_string());
            } else {
                bail!(self.error(
                    446,
                    &format!("expected {what} vtype, got {vtype}")
                ));
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
            bail!(self.error(
                456,
                &format!(
                    "expected table value of type {expected_type}, \
                    got value of type {ttype}",
                )
            ));
        }
        Ok(())
    }

    // uxf.py: typecheck()
    fn verify_type(&self, value: &Value, expected_type: &str) -> String {
        if value != &Value::Null && !expected_type.is_empty() {
            if VTYPES.contains(&expected_type) {
                if value.typename() != expected_type {
                    return format!(
                        "expected {expected_type}, got {value}"
                    );
                }
            } else if !self.tclass_for_ttype.contains_key(expected_type) {
                return format!("expected {expected_type}, got {value}",);
            }
        }
        "".to_string()
    }

    fn update_uxo(&mut self) {
        std::mem::swap(
            &mut self.uxo.tclass_for_ttype,
            &mut self.tclass_for_ttype,
        );
        std::mem::swap(
            &mut self.uxo.import_for_ttype,
            &mut self.import_for_ttype,
        );
    }

    fn cleanup_tclasses(&mut self) -> Result<()> {
        let mut imported: HashSet<String> =
            self.import_for_ttype.keys().cloned().collect();
        // replace imports
        if self.options.contains(ParserOptions::REPLACE_IMPORTS) {
            for ttype in &imported {
                if !self.used_tclasses.contains(ttype) {
                    self.tclass_for_ttype.remove(ttype); // unused ttype
                }
            }
            self.import_for_ttype.clear();
            imported.clear();
        }
        let mut defined: HashSet<String> =
            self.tclass_for_ttype.keys().cloned().collect();
        // drop unused
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
            let mut import_for_ttype =
                self.get_updated_import_for_ttype(&ttypes_for_filename);
            std::mem::swap(
                &mut import_for_ttype,
                &mut self.import_for_ttype,
            );
        }
        self.cleanup_maybe_warn(&imported, &defined)
    }

    fn get_deleted_ttypes(
        &self,
        ttypes_for_filename: &HashMap<String, HashSet<String>>,
    ) -> HashSet<String> {
        let mut deleted_ttypes: HashSet<String> = HashSet::new();
        for (filename, ttypes) in ttypes_for_filename {
            if ttypes.is_empty() {
                let mut ttypes: Vec<String> = vec![];
                for (ttype, ifilename) in &self.import_for_ttype {
                    if filename == ifilename {
                        ttypes.push(ttype.to_string());
                    }
                }
                for ttype in &ttypes {
                    deleted_ttypes.insert(ttype.to_string());
                }
            }
        }
        deleted_ttypes
    }

    fn get_updated_import_for_ttype(
        &self,
        ttypes_for_filename: &HashMap<String, HashSet<String>>,
    ) -> IndexMap<String, String> {
        let deleted_ttypes = self.get_deleted_ttypes(ttypes_for_filename);
        // We can't delete from an IndexMap since that ruins the
        // insertion order, so we must copy to a new one just those
        // items we want to keep.
        let mut import_for_ttype = IndexMap::new();
        for (ttype, filename) in &self.import_for_ttype {
            if !deleted_ttypes.contains(ttype.as_str()) {
                import_for_ttype
                    .insert(ttype.to_string(), filename.to_string());
            }
        }
        import_for_ttype
    }

    fn cleanup_maybe_warn(
        &self,
        imported: &HashSet<String>,
        defined: &HashSet<String>,
    ) -> Result<()> {
        let set: HashSet<String> = defined
            .difference(&self.used_tclasses)
            .map(|s| s.to_string())
            .collect();
        // don't warn on unused imports or on fieldless ttypes
        let set: HashSet<String> =
            set.difference(imported).map(|s| s.to_string()).collect();
        let mut unused: HashSet<String> = HashSet::new();
        for ttype in set {
            if !self.tclass_for_ttype[&ttype].is_fieldless() {
                unused.insert(ttype);
            }
        }
        if !unused.is_empty() {
            self.cleanup_report_problem(&unused, 422, "unused")?;
        }
        let undefined: HashSet<String> = self
            .used_tclasses
            .difference(defined)
            .map(|s| s.to_string())
            .collect();
        if !undefined.is_empty() {
            self.cleanup_report_problem(&unused, 424, "undefined")?;
        }
        Ok(())
    }

    fn cleanup_report_problem(
        &self,
        diff: &HashSet<String>,
        code: u16,
        what: &str,
    ) -> Result<()> {
        let s = if diff.len() == 1 { "" } else { "s" };
        let mut diff: Vec<String> =
            diff.iter().map(|s| s.to_string()).collect();
        diff.sort_by_key(|x| x.to_lowercase());
        let message = format!("{what} ttype{s}: {}", diff.join(" "));
        if code == 422 {
            (self.on_event)(&Event::new_warning(
                code,
                &message,
                self.filename,
                self.lino,
            ));
        } else {
            bail!(self.error(code, &message));
        }
        Ok(())
    }

    fn get_ttypes_for_filename(&self) -> HashMap<String, HashSet<String>> {
        let mut ttypes_for_filename: HashMap<String, HashSet<String>> =
            HashMap::new();
        for (ttype, filename) in &self.import_for_ttype {
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

    // Searches in order: filename's path, cwd, UXF_PATH
    fn find_import(&self, filename: &str) -> (String, bool) {
        let mut paths = vec![];
        if !self.filename.is_empty() && self.filename != "-" {
            paths.push(dirname(self.filename));
        }
        if !paths.is_empty() && paths[0] != "." {
            paths.push(".".to_string());
        }
        if let Ok(uxf_paths) = env::var("UXF_PATH") {
            for path in env::split_paths(&uxf_paths) {
                paths.push(path.to_string_lossy().to_string());
            }
        }
        for path in &paths {
            let fullname = full_filename(filename, path);
            if self.imported.contains(&fullname) {
                return (fullname, true); // already imported
            }
            if Path::new(&fullname).is_file() {
                return (fullname, false); // stop as soon as we find one
            }
        }
        (full_filename(filename, "."), false)
    }

    fn error(&self, code: u16, message: &str) -> String {
        format!("E{code}:{}:{}:{message}", self.filename, self.lino)
    }

    fn error_f(&self, code: u16, message: &str, filename: &str) -> String {
        format!(
            "E{code}:{}:{}:{message} {filename:?}",
            self.filename, self.lino
        )
    }

    fn error_s(&self, code: u16, message: &str, s: &str) -> String {
        format!(
            "E{code}:{}:{}:{message}, got {s:?}",
            self.filename, self.lino
        )
    }

    fn error_t(&self, code: u16, message: &str, t: &Token) -> String {
        format!(
            "E{code}:{}:{}:{message}, got {t}",
            self.filename, self.lino
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
