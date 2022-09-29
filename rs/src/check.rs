// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::consts::*;
use anyhow::{bail, Result};

/// Returns `Ok(())` if `ktype` is a valid ktype; otherwise `Err`.
pub(crate) fn check_ktype(ktype: &str) -> Result<()> {
    check_ktype_x(ktype, "-", 0)
}

/// Returns `Ok(())` if `ktype` is a valid ktype; otherwise `Err`.
pub(crate) fn check_ktype_x(
    ktype: &str,
    filename: &str,
    lino: usize,
) -> Result<()> {
    if KTYPES.contains(&ktype) {
        Ok(())
    } else {
        bail!(
            "E308:{}:{}:a ktype must be one of {:?}, got {}",
            filename,
            lino,
            KTYPES,
            ktype
        )
    }
}

/// Returns `Ok(())` if `ttype` is a valid table name (ttype); otherwise
/// `Err`.
/// Note that this does not (cannot) check whether a ttype of this name has
/// been defined.
pub(crate) fn check_ttype(ttype: &str) -> Result<()> {
    check_name(ttype)
}

/// Returns `Ok(())` if `ttype` is a valid table name (ttype); otherwise
/// `Err`.
/// Note that this does not (cannot) check whether a ttype of this name has
/// been defined.
pub(crate) fn check_ttype_x(
    ttype: &str,
    filename: &str,
    lino: usize,
) -> Result<()> {
    check_name_x(ttype, filename, lino)
}

/// Returns `Ok(())` if `name` is a valid field name; otherwise `Err`.
pub(crate) fn check_name(name: &str) -> Result<()> {
    check_name_x(name, "-", 0)
}

/// Returns `Ok(())` if `name` is a valid field name; otherwise `Err`.
pub(crate) fn check_name_x(
    name: &str,
    filename: &str,
    lino: usize,
) -> Result<()> {
    if RESERVED_WORDS.contains(&name) {
        bail!(
            "E304:{}:{}:table names (ttypes) and fieldnames cannot be the \
            same as built-in type names or constants, got {}",
            filename,
            lino,
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
    check_vtype_x(name, "-", 0)
}

/// Returns `Ok(())` if `name` is a valid vtype; otherwise `Err`.
/// Note that for lists and maps an empty vtype means any vtype is
/// acceptable; so for these, only check the vtype if it is nonempty.
pub(crate) fn check_vtype_x(
    name: &str,
    filename: &str,
    lino: usize,
) -> Result<()> {
    if name.is_empty() {
        bail!("E298:{}:{}:names must be nonempty", filename, lino)
    }
    let first = name.chars().next().unwrap(); // safe because nonempty
    if !(first == '_' || first.is_alphabetic()) {
        bail!(
            "E300:{}:{}:names must start with a letter or underscore, got {}", filename, lino,
            name
        )
    }
    if name == BOOL_TRUE || name == BOOL_FALSE {
        bail!(
            "E302:{}:{}:names may not be yes or no got {}",
            filename,
            lino,
            name
        )
    }
    for (i, c) in name.chars().enumerate() {
        if i == MAX_IDENTIFIER_LEN {
            bail!(
                "E306:{}:{}:names may be at most {} characters long, \
                  got {} ({} characters)",
                filename,
                lino,
                MAX_IDENTIFIER_LEN,
                name,
                i + 1
            )
        }
        if !(c == '_' || c.is_alphanumeric()) {
            bail!(
                "E310:{}:{}:names may only contain letters, digits, or \
                  underscores, got {}",
                filename,
                lino,
                name
            )
        }
    }
    Ok(())
}
