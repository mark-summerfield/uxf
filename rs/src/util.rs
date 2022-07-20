// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::event::fatal;
use anyhow::Result;

pub(crate) fn check_ktype(ktype: &str) -> Result<()> {
    if KTYPES.contains(&ktype) {
        Ok(())
    } else {
        fatal(
            308,
            &format!("a ktype must be one of {:?}, got {}", KTYPES, ktype),
        )
    }
}

pub(crate) fn check_name(name: &str) -> Result<()> {
    if RESERVED_WORDS.contains(&name) {
        fatal(
            304,
            &format!(
                "names cannot be the same as built-in type names or \
              constants, got {}",
                name
            ),
        )?;
    }
    check_type_name(name)?;
    Ok(())
}

pub(crate) fn check_type_name(name: &str) -> Result<()> {
    if name.is_empty() {
        fatal(298, "names must be nonempty")?;
    }
    let first = name.chars().next().unwrap(); // safe because nonempty
    if !(first == '_' || first.is_alphabetic()) {
        fatal(
            300,
            &format!(
                "names must start with a letter or underscore, got {}",
                name
            ),
        )?;
    }
    if name == BOOL_TRUE || name == BOOL_FALSE {
        fatal(302, &format!("names may not be yes or no got {}", name))?;
    }
    for (i, c) in name.chars().enumerate() {
        if i == MAX_IDENTIFIER_LEN {
            fatal(
                306,
                &format!(
                    "names may be at most {} characters long, \
                  got {} ({} characters)",
                    MAX_IDENTIFIER_LEN,
                    name,
                    i + 1
                ),
            )?;
        }
        if !(c == '_' || c.is_alphanumeric()) {
            fatal(
                310,
                &format!(
                    "names may only contain letters, digits, or \
                  underscores, got {}",
                    name
                ),
            )?;
        }
    }
    Ok(())
}
