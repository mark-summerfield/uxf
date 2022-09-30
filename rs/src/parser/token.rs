// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::consts::*;
use crate::value::Value;
use std::{collections::VecDeque, fmt};

pub type Tokens = VecDeque<Token>;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Value,
    pub lino: usize,
    pub comment: String,
    pub ktype: String,
    pub vtype: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: Value, lino: usize) -> Self {
        Token {
            kind,
            value, // may store vtype or ttype
            lino,
            comment: "".to_string(),
            ktype: "".to_string(),
            vtype: "".to_string(), // stores vtype _or_ ttype
        }
    }

    pub(crate) fn typename(&self) -> &str {
        match self.kind {
            TokenKind::TableBegin => VTYPE_NAME_TABLE,
            TokenKind::ListBegin => VTYPE_NAME_LIST,
            TokenKind::MapBegin => VTYPE_NAME_MAP,
            TokenKind::Null => VALUE_NAME_NULL,
            TokenKind::Bool => VTYPE_NAME_BOOL,
            TokenKind::Int => VTYPE_NAME_INT,
            TokenKind::Real => VTYPE_NAME_REAL,
            TokenKind::Date => VTYPE_NAME_DATE,
            TokenKind::DateTime => VTYPE_NAME_DATETIME,
            TokenKind::Str => VTYPE_NAME_STR,
            TokenKind::Bytes => VTYPE_NAME_BYTES,
            _ => "",
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.kind == TokenKind::Eof {
            write!(f, "{}", self.kind)
        } else {
            write!(f, "{} {}", self.kind, self.value)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    FileComment,
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

impl TokenKind {
    pub fn is_collection_start(&self) -> bool {
        matches!(
            self,
            TokenKind::ListBegin
                | TokenKind::MapBegin
                | TokenKind::TableBegin
        )
    }

    pub fn is_collection_end(&self) -> bool {
        matches!(
            self,
            TokenKind::ListEnd | TokenKind::MapEnd | TokenKind::TableEnd
        )
    }

    pub fn is_scalar(&self) -> bool {
        matches!(
            self,
            TokenKind::Null
                | TokenKind::Bool
                | TokenKind::Int
                | TokenKind::Real
                | TokenKind::Date
                | TokenKind::DateTime
                | TokenKind::Str
                | TokenKind::Bytes
        )
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self == &TokenKind::Eof {
            write!(f, "EOF")
        } else {
            write!(f, "{:?}", self)
        }
    }
}

/* DEBUG
pub(crate) fn debug_tokens(tokens: &[Token]) {
    let mut indent = 0;
    for token in tokens.iter() {
        if matches!(
            &token.kind,
            TokenKind::TClassEnd
                | TokenKind::ListEnd
                | TokenKind::MapEnd
                | TokenKind::TableEnd
        ) {
            indent -= 1;
        }
        if indent > 0 {
            print!("{}", "  ".repeat(indent));
        }
        println!("{}", token);
        if matches!(
            &token.kind,
            TokenKind::TClassBegin
                | TokenKind::ListBegin
                | TokenKind::MapBegin
                | TokenKind::TableBegin
        ) {
            indent += 1;
        }
    }
    println!("----------------------------------------");
}
*/
