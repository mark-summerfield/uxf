// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::consts::*;

#[derive(Clone, Debug)]
pub struct Format {
    pub indent: String,
    pub wrapwidth: u8,
    pub realdp: Option<u8>,
}

impl Format {
    /// Invalid values are silently replaced with appropriate defaults.
    /// indent should be 0-9 with 0 meaning no indent, 1-8 meaning that many
    /// spaces, and 9 meaning use one tab. The default is 2 (i.e., 2
    /// spaces).
    /// wrapwidth should be 40-240. The default is 96.
    /// realdp should either be None (i.e., use "natural" number of decimal
    /// places — and for no decimals always use .0 unless e or E is present,
    /// or use Some(u8) where the u8 is 0-15 for that many decimal places.)
    pub fn new(indent: u8, wrapwidth: u8, realdp: Option<u8>) -> Self {
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
            realdp: if let Some(realdp) = realdp {
                if realdp <= 15 {
                    Some(realdp)
                } else {
                    None
                }
            } else {
                None
            },
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format { indent: "  ".to_string(), wrapwidth: 96, realdp: None }
    }
}
