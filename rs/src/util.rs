// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::event::fatal;
use anyhow::Result;

/// Returns a clone of `s` with replacements & → &amp; < → &lt; > → &gt;
pub fn escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Returns a clone of `s` with replacements &amp; → & &lt; → < &gt; → >
pub fn unescape(s: &str) -> String {
    s.replace("&gt;", ">").replace("&lt;", "<").replace("&amp;", "&")
}

/// Returns `true` if `a` and `b` are close enough to be considered equal
/// for all practical purposes; otherwise returns `false`.
pub fn isclose64(a: f64, b: f64) -> bool {
    (a..=(a + f64::EPSILON)).contains(&b)
}


/// Returns `Ok(())` if `ktype` is a valid ktype; otherwise `Err`.
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
        fatal(
            304,
            &format!(
                "table names (ttypes) and fieldnames cannot be the same \
                as built-in type names or constants, got {}",
                name
            ),
        )?;
    }
    check_vtype(name)?;
    Ok(())
}

/// Returns `Ok(())` if `name` is a valid vtype; otherwise `Err`.
/// Note that for lists and maps an empty vtype means any vtype is
/// acceptable; so for these, only check the vtype if it is nonempty.
pub(crate) fn check_vtype(name: &str) -> Result<()> {
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
