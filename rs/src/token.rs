// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

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
}

impl fmt::Display for Token {
    /// Purely for debugging
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = if self.kind == TokenKind::TClassBegin {
            format!("{} ttype={}", &self.kind, &self.value)
        } else if matches!(
            &self.kind,
            TokenKind::Field | TokenKind::FileComment | TokenKind::Import
        ) {
            format!("{} ", &self.kind)
        } else if !matches!(
            self.kind,
            TokenKind::TClassBegin
                | TokenKind::TClassEnd
                | TokenKind::TableBegin
                | TokenKind::TableEnd
                | TokenKind::ListBegin
                | TokenKind::ListEnd
                | TokenKind::MapBegin
                | TokenKind::MapEnd
                | TokenKind::Eof
        ) {
            "".to_string()
        } else {
            format!("{}", &self.kind)
        };
        let comment = if !self.comment.is_empty() {
            format!(" # {}", self.comment)
        } else {
            "".to_string()
        };
        let value = if matches!(
            self.kind,
            TokenKind::TClassBegin
                | TokenKind::TClassEnd
                | TokenKind::TableBegin
                | TokenKind::TableEnd
                | TokenKind::ListBegin
                | TokenKind::ListEnd
                | TokenKind::MapBegin
                | TokenKind::MapEnd
                | TokenKind::Eof
        ) {
            "".to_string()
        } else if self.value == Value::Null {
            "?".to_string()
        } else if self.kind == TokenKind::Bytes {
            format!("Bytes( {} )", self.value)
        } else {
            format!("{:?}", self.value)
        };
        let xtype = if !self.ktype.is_empty() && !self.vtype.is_empty() {
            format!(" ktype={} vtype={}", self.ktype, self.vtype)
        } else if !self.ktype.is_empty() {
            format!(" ktype={}", self.ktype)
        } else if !self.vtype.is_empty() {
            format!(
                " {}type={}",
                if self.kind == TokenKind::TableBegin { 't' } else { 'v' },
                self.vtype
            )
        } else {
            "".to_string()
        };
        write!(f, "{}{}{}{}", kind, value, xtype, comment)
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
        write!(f, "{:?}", self)
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
