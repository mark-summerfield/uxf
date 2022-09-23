// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::event::{self, Event, OnEventFn};
use crate::format::Format;
use crate::pprint::token::{Token, TokenKind, Tokens};
use crate::tclass::TClass;
use crate::util::{escape, rindex_of_char, str_for_chars, VecExt};
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

    debug_tokens(&tokens);

    Ok(tokens)
}

fn debug_tokens(tokens: &Tokens) {
    for t in tokens {
        eprintln!("{:?}", t);
    }
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

    pub fn get_tokens(&mut self) -> Tokens {
        let mut tokens = Tokens::new();
        std::mem::swap(&mut tokens, &mut self.tokens);
        tokens
    }

    pub fn visit(&mut self, visit: Visit, value: &Value) -> Result<()> {
        match visit {
            Visit::UxfBegin => self.handle_uxf_begin(value),
            Visit::UxfEnd => self.eof(),
            Visit::ListBegin => self.handle_list_begin(value),
            Visit::ListEnd => self.handle_list_end(),
            Visit::ListValueBegin => (),
            Visit::ListValueEnd => self.handle_list_value_end(),
            Visit::MapBegin => self.handle_map_begin(value),
            Visit::MapEnd => self.handle_map_end(),
            Visit::MapItemBegin => self.begin(),
            Visit::MapItemEnd => self.handle_item_end(),
            Visit::TableBegin => self.handle_table_begin(value),
            Visit::TableEnd => self.handle_table_end(),
            Visit::TableRecordBegin => self.begin(),
            Visit::TableRecordEnd => self.handle_record_end(),
            Visit::Value => self.handle_scalar(value),
        }
        Ok(())
    }

    fn handle_uxf_begin(&mut self, value: &Value) {
        if let Some(comment) = value.as_str() {
            if !comment.is_empty() {
                self.handle_str(comment, "#", "\n");
            }
        }
        self.handle_imports();
        self.handle_tclasses();
    }

    fn handle_imports(&mut self) {
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
    }

    fn handle_tclasses(&mut self) {
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
    }

    fn handle_list_begin(&mut self, value: &Value) {
        // Value is a List or there's a bug
        let lst = value.as_list().unwrap();
        self.list_value_counts.push(lst.len());
        self.begin();
        self.puts("[");
        let has_comment = !lst.comment().is_empty();
        if has_comment {
            self.handle_comment(lst.comment());
        }
        if !lst.vtype().is_empty() {
            if has_comment {
                self.rws();
            }
            self.puts(lst.vtype());
            if lst.len() == 1 {
                self.rws();
            }
        }
        if lst.len() > 1 {
            self.rnl();
        } else if has_comment && lst.len() == 1 {
            self.rws();
        }
        self.depth += 1;
    }

    fn handle_list_end(&mut self) {
        self.chop_if_redundant_rws();
        self.depth -= 1;
        self.puts("]");
        self.end();
        self.rws();
        self.list_value_counts.chop();
    }

    fn handle_list_value_end(&mut self) {
        // Safe index because we can only get here within a list
        if self.list_value_counts[self.list_value_counts.len() - 1] > 1 {
            self.rnl();
        }
    }

    fn handle_map_begin(&mut self, value: &Value) {
        // Value is a Map or there's a bug
        let m = value.as_map().unwrap();
        self.map_item_counts.push(m.len());
        self.begin();
        self.puts("{");
        let has_comment = !m.comment().is_empty();
        if has_comment {
            self.handle_comment(m.comment());
        }
        if !m.ktype().is_empty() {
            if has_comment {
                self.rws();
            }
            let mut text = String::from(m.ktype());
            if !m.vtype().is_empty() {
                text.push(' ');
                text.push_str(m.vtype());
            }
            self.puts(&text);
            if m.len() == 1 {
                self.rws();
            }
        }
        if m.len() > 1 {
            self.rnl();
        } else if has_comment && m.len() == 1 {
            self.rws();
        }
        self.depth += 1;
    }

    fn handle_map_end(&mut self) {
        self.chop_if_redundant_rws();
        self.depth -= 1;
        self.puts("}");
        self.end();
        self.map_item_counts.chop();
    }

    fn handle_item_end(&mut self) {
        self.end();
        // Safe index because we can only get here within a list
        if self.map_item_counts[self.map_item_counts.len() - 1] > 1 {
            self.rnl();
        }
    }

    fn handle_table_begin(&mut self, value: &Value) {
        // Value is a Table or there's a bug
        let t = value.as_table().unwrap();
        self.table_record_counts.push(t.len());
        self.begin();
        self.puts("(");
        if !t.comment().is_empty() {
            self.handle_comment(t.comment());
            self.rws();
        }
        self.puts_num(t.ttype(), Some(t.len()));
        match t.len() {
            0 => (),
            1 => self.rws(),
            _ => {
                self.rnl();
                self.depth += 1;
            }
        }
    }

    fn handle_table_end(&mut self) {
        self.chop_if_redundant_rws();
        // Safe because only called inside a table
        let count =
            self.table_record_counts[self.table_record_counts.len() - 1];
        if count > 1 {
            self.depth -= 1;
        }
        self.puts(")");
        self.end();
        if count > 1 {
            self.rnl();
        } else {
            self.rws()
        }
        self.table_record_counts.chop();
    }

    fn handle_record_end(&mut self) {
        self.end();
        // Safe index because we can only get here within a list
        if self.table_record_counts[self.table_record_counts.len() - 1] > 1
        {
            self.rnl();
        }
    }

    fn handle_scalar(&mut self, value: &Value) {
        match value {
            Value::Null => self.puts("?"),
            Value::Bool(b) => self.puts(if *b { "yes" } else { "no" }),
            Value::Bytes(d) => (), // TODO
            Value::Date(d) => (), // TODO
            Value::DateTime(d) => (), // TODO
            Value::Int(i) => self.puts(&format!("{}", i)),
            Value::Real(r) => (), // TODO Real (use util::realstr64 on top
            // of py algo) or add Option<realdp> to realstr()
            Value::Str(s) => (), // TODO
            _ => panic!("expected scalar, got {:?}", value),
        }
    }

    fn handle_str(&mut self, s: &str, prefix: &str, suffix: &str) {
        let text = escape(s);
        let prefix_len = prefix.chars().count();
        let span = self.wrapwidth - prefix_len;
        let mut too_wide = false;
        for line in text.lines() {
            if line.chars().count() > span {
                too_wide = true;
                break;
            }
        }
        if !too_wide {
            self.puts(&format!("{}<{}>{}", prefix, text, suffix));
        } else {
            // Assumes there is no suffix
            self.handle_long_str(&text, prefix, prefix_len);
        }
    }

    // Assumes there is no suffix and that text is already escaped
    fn handle_long_str(
        &mut self,
        text: &str,
        prefix: &str,
        prefix_len: usize,
    ) {
        let span = self.wrapwidth - (4 + prefix_len);
        let mut chars: Vec<char> = text.chars().collect();
        let mut prefix = String::from(prefix);
        while !chars.is_empty() {
            // find last space within span; if no space, split at span
            let i = rindex_of_char(' ', &chars[..span]);
            let i = if let Some(i) = i { i + 1 } else { span };
            let chunk = str_for_chars(&chars[..i]);
            chars.drain(..i);
            if !chunk.is_empty() {
                let end = if chars.is_empty() { "" } else { " &" };
                self.put_line(
                    &format!("{}<{}>{}", prefix, chunk, end),
                    self.depth,
                );
                prefix.clear();
                self.rnl();
            }
        }
    }

    fn handle_comment(&mut self, comment: &str) {
        self.handle_str(comment, "#", "")
    }

    fn begin(&mut self) {
        if !self.tokens.is_empty()
            && self.tokens[self.tokens.len() - 1].kind == TokenKind::End
        {
            self.rws();
        }
        self.append_bare(TokenKind::Begin, self.depth);
    }

    fn end(&mut self) {
        if !self.tokens.is_empty()
            && self.tokens[self.tokens.len() - 1].kind == TokenKind::Rws
        {
            self.tokens.chop();
        }
        self.append_bare(TokenKind::End, self.depth);
    }

    // Don't need duplicate RWS; don't need RWS if RNL present
    fn rws(&mut self) {
        if !self.tokens.is_empty() {
            let mut pos = self.tokens.len() - 1; // last
            if self.tokens[pos].kind == TokenKind::End
                && self.tokens.len() > 1
            {
                pos -= 1;
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
                self.tokens.chop();
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

    // Don't need RWS before closer
    fn chop_if_redundant_rws(&mut self) {
        if !self.tokens.is_empty()
            && self.tokens[self.tokens.len() - 1].kind == TokenKind::Rws
        {
            self.tokens.chop();
        }
    }
}
