// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::format::Format;
use crate::pprint::token::Tokens;

pub(crate) fn to_text(
    header: &str,
    tokens: Tokens,
    format: &Format,
) -> String {
    let mut writer = Writer::new(header, tokens, format);
    writer.pprint()
}

pub struct Writer {
    pub tokens: Tokens,
    pub uxt: String,
    pub indent: String,
    pub wrapwidth: usize,
    pub realdp: Option<u8>,
    pub pos: usize,
    pub tp: usize,
    pub end_nl: bool,
    pub pending_rws: bool,
}

impl Writer {
    pub fn new(header: &str, tokens: Tokens, format: &Format) -> Self {
        Self {
            tokens,
            uxt: String::from(header),
            indent: format.indent.clone(),
            realdp: format.realdp,
            wrapwidth: format.wrapwidth as usize,
            pos: 0,
            tp: 0,
            end_nl: false,
            pending_rws: false,
        }
    }

    pub fn pprint(&mut self) -> String {
        // TODO

        let mut uxt = String::new();
        std::mem::swap(&mut uxt, &mut self.uxt);
        uxt
    }
}
