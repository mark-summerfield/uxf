// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::event::{self, Event, OnEventFn};
use crate::format::Format;
use crate::pprint::token::{Token, TokenKind, Tokens};
use crate::tclass::TClass;
use crate::uxf::Uxf;
use crate::value::{Value, Visit};
use anyhow::{bail, Result};
use indexmap::map::IndexMap;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub(crate) fn tokenize(
    uxo: &Uxf,
    format: &Format,
    on_event: Option<OnEventFn>,
    tclass_for_ttype: HashMap<String, TClass>,
    import_for_ttype: IndexMap<String, String>,
) -> Result<Tokens> {
    let tokenizer = Rc::new(RefCell::new(Tokenizer::new(
        on_event,
        format,
        tclass_for_ttype,
        import_for_ttype,
    )));
    uxo.visit(Rc::new({
        let tokenizer = Rc::clone(&tokenizer);
        move |visit: Visit, value: &Value| {
            let mut tokenizer = tokenizer.borrow_mut();
            tokenizer.visit(visit, value)
        }
    }))?;
    let tokens = tokenizer.borrow_mut().get_tokens();

    // DEBUG
    for t in &tokens {
        eprintln!("{:?}", t);
    }
    // END DEBUG
    
    Ok(tokens)
}

pub struct Tokenizer {
    pub on_event: OnEventFn,
    pub indent: String,
    pub wrapwidth: usize,
    pub realdp: Option<u8>,
    pub tclass_for_ttype: HashMap<String, TClass>, // ttype x TClass
    pub import_for_ttype: IndexMap<String, String>, // ttype x import
    pub depth: usize,
    pub tokens: Tokens,
    pub list_value_counts: Vec<usize>,
    pub map_item_counts: Vec<usize>,
    pub table_record_counts: Vec<usize>,
}

impl Tokenizer {
    pub fn new(
        on_event: Option<OnEventFn>,
        format: &Format,
        tclass_for_ttype: HashMap<String, TClass>,
        import_for_ttype: IndexMap<String, String>,
    ) -> Self {
        Self {
            on_event: if let Some(on_event) = on_event {
                Rc::clone(&on_event)
            } else {
                Rc::new(event::on_event)
            },
            indent: format.indent.clone(),
            realdp: format.realdp,
            wrapwidth: format.wrapwidth as usize,
            depth: 0,
            tokens: Tokens::new(),
            tclass_for_ttype,
            import_for_ttype,
            list_value_counts: vec![],
            map_item_counts: vec![],
            table_record_counts: vec![],
        }
    }

    pub fn visit(&mut self, visit: Visit, value: &Value) -> Result<()> {
        match visit {
            Visit::UxfBegin => self.handle_uxf_begin(value)?,
            Visit::UxfEnd => self.eof(),
            Visit::ListBegin => (),        // TODO
            Visit::ListEnd => (),          // TODO
            Visit::ListValueBegin => (),   // TODO
            Visit::ListValueEnd => (),     // TODO
            Visit::MapBegin => (),         // TODO
            Visit::MapEnd => (),           // TODO
            Visit::MapItemBegin => (),     // TODO
            Visit::MapItemEnd => (),       // TODO
            Visit::TableBegin => (),       // TODO
            Visit::TableEnd => (),         // TODO
            Visit::TableRecordBegin => (), // TODO
            Visit::TableRecordEnd => (),   // TODO
            Visit::Value => (),            // TODO
        }
        Ok(())
    }

    pub fn get_tokens(&mut self) -> Tokens {
        let mut tokens = Tokens::new();
        std::mem::swap(&mut tokens, &mut self.tokens);
        tokens
    }

    pub fn handle_imports(&mut self) -> Result<()> {
        let mut widest = 0;
        let mut seen = HashSet::new();
        let imports: Vec<String> =
            self.import_for_ttype.values().map(|i| i.to_string()).collect();
        for import in imports {
            if !seen.contains(&import) {
                self.puts(&format!("!{}\n", &import));
                let width = import.chars().count() + 1; // +1 for !
                if width > widest {
                    widest = width;
                }
                seen.insert(import);
            }
        }
        if widest > self.wrapwidth {
            self.wrapwidth = widest;
            (self.on_event)(&Event::bare_warning(
                563,
                &format!(
                    "imports forced wrapwidth to be increased to {}",
                    widest
                ),
            ));
        }
        Ok(())
    }

    pub fn handle_tclasses(&mut self) -> Result<()> {
        let mut widest = 0;
        let mut ttype_tclass_pairs: Vec<(String, TClass)> = self
            .tclass_for_ttype
            .iter()
            .filter(|pair| !self.import_for_ttype.contains_key(pair.0))
            .map(|pair| (pair.0.to_lowercase(), pair.1.clone()))
            .collect();
        ttype_tclass_pairs.sort();
        for (ttype, tclass) in ttype_tclass_pairs {
            self.puts("=");
            if !tclass.comment().is_empty() {
                self.handle_comment(tclass.comment());
                self.rws();
            }
            self.puts(&ttype);
            let width = ttype.chars().count() + 1; // +1 for =
            if width > widest {
                widest = width;
            }
            self.depth = 1; // to indent any wrapped fields
            for field in tclass.fields() {
                self.rws();
                self.puts(field.name());
                if let Some(vtype) = field.vtype() {
                    self.puts(&format!(":{}", vtype));
                }
                let width = field.name().chars().count();
                if width > widest {
                    widest = width;
                }
            }
            self.rnl();
        }
        self.depth = 0;
        if widest > self.wrapwidth {
            self.wrapwidth = widest;
            (self.on_event)(&Event::bare_warning(
                564,
                &format!(
                    "ttype forced wrapwidth to be increased to {}",
                    widest
                ),
            ));
        }
        Ok(())
    }

    pub fn handle_uxf_begin(&mut self, value: &Value) -> Result<()> {
        if let Some(comment) = value.as_str() {
            self.handle_str(comment, "#", "\n");
        }
        self.handle_imports()?;
        self.handle_tclasses()
    }

    fn handle_str(&mut self, comment: &str, prefix: &str, suffix: &str) {
        // TODO
    }

    fn handle_comment(&mut self, comment: &str) {
        self.handle_str(comment, "#", "")
    }

    // Don't need duplicate RWS; don't need RWS if RNL present
    fn rws(&mut self) {
        if !self.tokens.is_empty() {
            let mut pos = self.tokens.len() - 1; // last
            if self.tokens[pos].kind == TokenKind::End
                && self.tokens.len() > 1
            {
                pos += 1;
            }
            if self.tokens[pos].kind == TokenKind::Rws
                || self.tokens[pos].kind == TokenKind::Rnl
            {
                return;
            }
        }
        self.append_bare(TokenKind::Rws, self.depth);
    }

    // Don't need RWS before newline; don't need dup RNL
    fn rnl(&mut self) {
        if !self.tokens.is_empty() {
            let last = self.tokens.len() - 1;
            if self.tokens[last].kind == TokenKind::Rws {
                self.tokens.truncate(self.tokens.len() - 1);
            }
            let last = self.tokens.len() - 1;
            if self.tokens[last].kind == TokenKind::Rnl
                || (self.tokens.len() > 1
                    && self.tokens[last].kind == TokenKind::End
                    && self.tokens[last - 1].kind == TokenKind::Rnl)
            {
                return;
            }
        }
        self.append_bare(TokenKind::Rnl, self.depth);
    }

    fn eof(&mut self) {
        self.append_bare(TokenKind::Eof, self.depth);
    }

    fn put_line(&mut self, s: &str, depth: usize) {
        self.append(TokenKind::Str, s, depth, None);
    }

    fn puts(&mut self, s: &str) {
        self.puts_num(s, None);
    }

    fn puts_num(&mut self, s: &str, num_records: Option<usize>) {
        if !self.tokens.is_empty() {
            if let Some(token) = self.tokens.last_mut() {
                if token.kind == TokenKind::Str
                    && !token.is_multiline()
                    && !token.text.ends_with('\n')
                {
                    token.text.push_str(s); // absorb s into the prev one
                    if let Some(num_records) = num_records {
                        if token.num_records.is_none() {
                            token.num_records = Some(num_records);
                        }
                    }
                    return;
                }
            }
        }
        self.append(TokenKind::Str, s, self.depth, num_records);
    }

    fn append_bare(&mut self, kind: TokenKind, depth: usize) {
        self.append(kind, "", depth, None);
    }

    fn append(
        &mut self,
        kind: TokenKind,
        text: &str,
        depth: usize,
        num_records: Option<usize>,
    ) {
        self.tokens.push(Token::new(kind, text, depth, num_records));
    }
}
