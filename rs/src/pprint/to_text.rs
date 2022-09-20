// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::consts::UXF_VERSION;
use crate::event::OnEventFn;
use crate::format::Format;
use crate::pprint::tokenizer::tokenize;
use crate::uxf::Uxf;
use anyhow::Result;

pub(crate) fn to_text(
    uxo: &Uxf,
    format: &Format,
    on_event: Option<OnEventFn>,
) -> Result<String> {
    let tokens = tokenize(
        uxo,
        format,
        on_event,
        uxo.tclass_for_ttype.clone(),
        uxo.import_for_ttype.clone(),
    )?;
    let mut text = initialize(uxo.custom());
    // TODO create pprint/writer.rs
    // let mut writer = Writer::new(tokens, format, &mut text);
    // writer.pprint();
    Ok(text)
}

fn initialize(custom: &str) -> String {
    let mut text = format!("uxf {}", UXF_VERSION);
    if !custom.is_empty() {
        text.push(' ');
        text.push_str(custom);
    }
    text.push('\n');
    text
}
