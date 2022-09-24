// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::format::Format;
use crate::pprint::token::Tokens;

pub(crate) fn to_text(
    header: &str,
    tokens: &Tokens,
    format: &Format,
) -> String {
    // let mut writer = Writer::new(header, tokens, format);
    // let text = writer.pprint();
    // text;
    "".to_string() // TODO
}

pub struct Writer {
    pub tokens: &Tokens,
    pub uxt: String,
    pub indent: String,
    pub wrapwidth: usize,
    pub realdp: Option<u8>,
    pub pos: usize,
    pub tp: usize,
    pub end_nl: bool,
    pub pending_rws: bool,
}
