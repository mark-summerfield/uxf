// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use std::fmt;

pub type Tokens = Vec<Token>;

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub depth: usize,
    pub num_records: Option<usize>,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        text: &str,
        depth: usize,
        num_records: Option<usize>,
    ) -> Self {
        Token { kind, text: text.to_string(), depth, num_records }
    }

    pub fn is_multiline(&self) -> bool {
        self.text.contains('\n')
    }
}

impl fmt::Debug for Token {
    fn fmt<'a>(&'a self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = if let Some(num_records) = self.num_records {
            format!(" #{} ", num_records)
        } else {
            "".to_string()
        };
        let text = if self.text.is_empty() {
            "".to_string()
        } else {
            format!(" {:?}", self.text)
        };
        write!(
            f,
            "{}{:?}{}{}",
            "    ".repeat(self.depth),
            self.kind,
            n,
            text
        )
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum TokenKind {
    Begin,
    End,
    Str,
    Rws, // required whitespace: output either ' ' or '\n'
    Rnl, // required newline: output '\n'
    Eof,
}

impl fmt::Debug for TokenKind {
    fn fmt<'a>(&'a self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenKind::Begin => "BEG",
                TokenKind::End => "END",
                TokenKind::Str => "STR",
                TokenKind::Rws => "RWS",
                TokenKind::Rnl => "RNL",
                TokenKind::Eof => "EOF",
            }
        )
    }
}
