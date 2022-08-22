// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::value::Value;
use std::fmt;

pub type Tokens = Vec<Token>;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Value,
    pub lino: i32,
    pub comment: String,
    pub ttype: String,
    pub ktype: String,
    pub vtype: String,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Token {
            kind,
            value: Value::Null,
            lino: 0,
            comment: "".to_string(),
            ttype: "".to_string(),
            ktype: "".to_string(),
            vtype: "".to_string(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let comment = if !self.comment.is_empty() { " #" } else { "" };
        let lino = if self.lino > 0 {
            format!(" {}", self.lino)
        } else {
            "".to_string()
        };
        write!(
            f,
            "Token={} value={} ttype={} ktype={} vtype={}{}{}",
            &self.kind,
            &self.value,
            &self.ttype,
            &self.ktype,
            &self.vtype,
            comment,
            lino
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
