// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use anyhow::{bail, Result};

/// Returns `Ok(())` if `ktype` is a valid ktype; otherwise `Err`.
pub(crate) fn check_ktype(ktype: &str) -> Result<()> {
    if KTYPES.contains(&ktype) {
        Ok(())
    } else {
        bail!("E308:-:0:a ktype must be one of {:?}, got {}", KTYPES, ktype)
    }
}

/// Returns `Ok(())` if `ttype` is a valid table name (ttype); otherwise
/// `Err`.
/// Note that this does not (cannot) check whether a ttype of this name has
/// been defined.
pub(crate) fn check_ttype(ttype: &str) -> Result<()> {
    check_name(ttype)
}

/// Returns `Ok(())` if `name` is a valid field name; otherwise `Err`.
pub(crate) fn check_fieldname(fieldname: &str) -> Result<()> {
    check_name(fieldname)
}

fn check_name(name: &str) -> Result<()> {
    if RESERVED_WORDS.contains(&name) {
        bail!(
            "E304:-:0:table names (ttypes) and fieldnames cannot be the \
            same as built-in type names or constants, got {}",
            name
        )
    }
    check_vtype(name)?;
    Ok(())
}

/// Returns `Ok(())` if `name` is a valid vtype; otherwise `Err`.
/// Note that for lists and maps an empty vtype means any vtype is
/// acceptable; so for these, only check the vtype if it is nonempty.
pub(crate) fn check_vtype(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("E298:-:0:names must be nonempty")
    }
    let first = name.chars().next().unwrap(); // safe because nonempty
    if !(first == '_' || first.is_alphabetic()) {
        bail!(
            "E300:-:0:names must start with a letter or underscore, got {}",
            name
        )
    }
    if name == BOOL_TRUE || name == BOOL_FALSE {
        bail!("E302:-:0:names may not be yes or no got {}", name)
    }
    for (i, c) in name.chars().enumerate() {
        if i == MAX_IDENTIFIER_LEN {
            bail!(
                "E306:-:0:names may be at most {} characters long, \
                  got {} ({} characters)",
                MAX_IDENTIFIER_LEN,
                name,
                i + 1
            )
        }
        if !(c == '_' || c.is_alphanumeric()) {
            bail!(
                "E310:-:0:names may only contain letters, digits, or \
                  underscores, got {}",
                name
            )
        }
    }
    Ok(())
}
