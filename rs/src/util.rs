// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::event::fatal;
use anyhow::Result;
use flate2::read::GzDecoder;
use std::{fs::File, io::prelude::*};

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

/// Returns a string of an f64 which is guaranteed to contain an 'e' or 'E'
/// or to end with ".0".
pub fn realstr64(x: f64) -> String {
    let mut s = x.to_string();
    if !s.contains(&['.', 'e', 'E']) {
        s.push_str(".0");
    }
    s
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

/// Returns the raw bytes of the given file which is either plain text or
/// gzipped plain text (UTF-8 encoded).
pub(crate) fn read_raw_file(filename: &str) -> Result<Vec<u8>> {
    let compressed = is_compressed(filename)?;
    let mut raw: Vec<u8>;
    let file = File::open(&filename)?;
    if compressed {
        let mut gz = GzDecoder::new(file);
        gz.read_to_end(&mut raw)?;
    } else {
        raw = vec![];
        file.read_to_end(&mut raw)?;
    }
    Ok(raw)
}

/// Returns true if the given file is gzip compressed; otherwise false.
pub(crate) fn is_compressed(filename: &str) -> Result<bool> {
    let mut file = File::open(&filename)?;
    let mut buffer = [0; 2]; // 0x1F 0x8B gzip magic
    file.read_exact(&mut buffer)?;
    Ok(buffer[0] == 0x1F && buffer[1] == 0x8B)
}
