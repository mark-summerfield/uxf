// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::value::Value;
use std::fmt;

pub type Tokens<'a> = Vec<Token<'a>>;

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub value: Value,
    pub filename: &'a str,
    pub lino: usize,
    pub comment: String,
    pub ttype: String,
    pub ktype: String,
    pub vtype: String,
}

impl<'a> Token<'a> {
    pub fn new(
        kind: TokenKind,
        value: Value,
        filename: &'a str,
        lino: usize,
    ) -> Self {
        Token {
            kind,
            value,
            filename,
            lino,
            comment: "".to_string(),
            ttype: "".to_string(),
            ktype: "".to_string(),
            vtype: "".to_string(),
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let comment = if !self.comment.is_empty() { " #" } else { "" };
        let filename = if self.filename.is_empty() {
            "".to_string()
        } else {
            format!(" {}", self.filename)
        };
        let lino = if self.lino > 0 {
            format!(" {}", self.lino)
        } else {
            "".to_string()
        };
        write!(
            f,
            "Token={} value={} ttype={} ktype={} vtype={}{}{}{}",
            &self.kind,
            &self.value,
            &self.ttype,
            &self.ktype,
            &self.vtype,
            comment,
            self.filename,
            lino,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Import,
    TClassBegin,
    TClassEnd,
    Field,
    TableBegin,
    TableEnd,
    ListBegin,
    ListEnd,
    MapBegin,
    MapEnd,
    FileComment,
    Null,
    Bool,
    Int,
    Real,
    Date,
    DateTime,
    Str,
    Bytes,
    Type,
    Identifier,
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
