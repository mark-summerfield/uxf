// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use anyhow::{bail, Result};
use flate2::read::GzDecoder;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

/// Returns a clone of `s` with replacements & → &amp; < → &lt; > → &gt;
pub fn escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Returns a clone of `s` with replacements &amp; → & &lt; → < &gt; → >
pub fn unescape(s: &str) -> String {
    s.replace("&gt;", ">").replace("&lt;", "<").replace("&amp;", "&")
}

/// Returns a String for the given Vec<char>
pub fn str_for_chars(data: &[char]) -> String {
    data.iter().collect::<String>()
}
///

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

/// Returns the entire text of the given file which is either plain text
/// or gzipped plain text (UTF-8 encoded).
pub(crate) fn read_file(filename: &str) -> Result<String> {
    let compressed = is_compressed(filename)?;
    let mut text = String::new();
    let file = File::open(&filename)?;
    if compressed {
        let mut gz = GzDecoder::new(file);
        gz.read_to_string(&mut text)?;
    } else {
        let mut buffer = BufReader::new(file);
        buffer.read_to_string(&mut text)?;
    }
    Ok(text)
}

/// Returns true if the given file is gzip compressed; otherwise false.
pub(crate) fn is_compressed(filename: &str) -> Result<bool> {
    let mut file = File::open(&filename)?;
    let mut buffer = [0; 2]; // 0x1F 0x8B gzip magic
    file.read_exact(&mut buffer)?;
    Ok(buffer[0] == 0x1F && buffer[1] == 0x8B)
}

/// Returns the bytes for the given slices of chars.
/// Each char may be 0-9A-Fa-f or ASCII whitespace (which is ignored) and
/// non-whitespace chars must come in pairs (even if separated by
/// whitespace).
pub(crate) fn hex_as_bytes(
    h: &str,
    filename: &str,
    lino: usize,
) -> Result<Vec<u8>> {
    let mut raw = vec![];
    let mut b = NUL;
    for c in h.chars() {
        if c.is_ascii_hexdigit() {
            if b == NUL {
                b = c;
            } else {
                // safe to unwrap because of is_ascii_hexdigit()
                let x = b.to_digit(16).unwrap() * 16;
                let y = c.to_digit(16).unwrap();
                raw.push((x | y) as u8);
                b = NUL;
            }
        } else if !c.is_ascii_whitespace() {
            bail!("E600:{}:{}:invalid hex char: {:?}", filename, lino, c)
        }
    }
    Ok(raw)
}
