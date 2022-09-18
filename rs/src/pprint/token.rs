// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

pub type Tokens = Vec<Token>;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub depth: u16,
    pub num_records: u32,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        text: &str,
        depth: u16,
        num_records: u32,
    ) -> Self {
        Token { kind, text: text.to_string(), depth, num_records }
    }

    pub fn new_empty(kind: TokenKind) -> Self {
        Token { kind, text: "".to_string(), depth: 0, num_records: 0 }
    }

    pub fn is_multiline(&self) -> bool {
        self.text.contains('\n')
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Begin,
    End,
    Str,
    Rws, // required whitespace: output either ' ' or '\n'
    Rnl, // required newline: output '\n'
    Eof,
}
