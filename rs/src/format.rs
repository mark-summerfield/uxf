// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/// A Format is used to specify how to format a UXF file written using a
/// pretty printing method (Uxf::write(), Uxf::write_format(),
/// Uxf::to_text(), or Uxf::to_text_format()).
///
/// A Format holds an indent, a wrapwidth, and realdp which controls how
/// real numbers are formatted.
use crate::consts::*;

#[derive(Clone, Debug)]
pub struct Format {
    pub indent: String,
    pub wrapwidth: u8,
    pub realdp: u8,
}

impl Format {
    /// Invalid values are silently replaced with appropriate defaults.
    /// indent should be 0-9 with 0 meaning no indent, 1-8 meaning that many
    /// spaces, and 9 meaning use one tab. The default is 2 (i.e., 2
    /// spaces).
    /// wrapwidth should be 40-240. The default is 96.
    /// realdp should be 0-15. The default is 0 which means use at least one
    /// decimal digit (even if .0) and as many as needed; 1-15 mean use that
    /// fixed number of decimal digits.
    pub fn new(indent: u8, wrapwidth: u8, realdp: u8) -> Self {
        Format {
            indent: match indent {
                0 => "".to_string(),
                1..=8 => " ".repeat(indent as usize),
                9 => "\t".to_string(),
                _ => "  ".to_string(),
            },
            wrapwidth: if (MAX_IDENTIFIER_LEN as u8 + 8) <= wrapwidth
                && wrapwidth <= 240
            {
                wrapwidth
            } else {
                96
            },
            realdp: if realdp <= 15 { realdp } else { 0 },
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format { indent: "  ".to_string(), wrapwidth: 96, realdp: 0 }
    }
}
