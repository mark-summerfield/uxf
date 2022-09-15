// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::event::OnEventFn;
use crate::format::Format;
use anyhow::{bail, Result};

pub fn pprint(
    format: &Format,
    on_event: Option<OnEventFn>,
) -> Result<String> {
    bail!("TODO: pprint") // TODO
}
