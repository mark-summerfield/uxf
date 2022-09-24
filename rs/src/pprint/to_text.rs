// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::consts::UXF_VERSION;
use crate::format::Format;
use crate::pprint::{tokenizer::tokenize, writer};
use crate::uxf::Uxf;

pub(crate) fn to_text(uxo: &Uxf, format: &Format) -> String {
    let header = header(uxo.custom());
    let tokens = tokenize(
        uxo,
        format,
        uxo.tclass_for_ttype.clone(),
        uxo.import_for_ttype.clone(),
    );
    writer::to_text(&header, tokens, format)
}

fn header(custom: &str) -> String {
    let mut text = format!("uxf {}", UXF_VERSION);
    if !custom.is_empty() {
        text.push(' ');
        text.push_str(custom);
    }
    text.push('\n');
    text
}
