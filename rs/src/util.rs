// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::value::Value;
use anyhow::{bail, Result};

pub(crate) fn check_name(name: &str) -> Result<()> {
    check_type_name(name)?;
    if RESERVED_WORDS.contains(&name) {
        bail!(
            "#304:names cannot be the same as built-in type names or \
              constants, got {}",
            name
        );
    }
    Ok(())
}

pub(crate) fn check_type_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("#298:names must be nonempty");
    }
    let first = name.chars().next().unwrap(); // safe because nonempty
    if !(first == '_' || first.is_alphabetic()) {
        bail!(
            "#300:type names must start with a letter or underscore, \
              got {}",
            name
        );
    }
    if name == BOOL_TRUE || name == BOOL_FALSE {
        bail!("#302:names may not be yes or no got {}", name);
    }
    for (i, c) in name.chars().enumerate() {
        if i == MAX_IDENTIFIER_LEN {
            bail!(
                "#306:type names may be at most {} characters long, \
                  got {} ({} characters)",
                MAX_IDENTIFIER_LEN,
                name,
                i + 1
            );
        }
        if !(c == '_' || c.is_alphanumeric()) {
            bail!(
                "#310:names may only contain letters, digits, or \
                  underscores, got {}",
                name
            );
        }
    }
    Ok(())
}
