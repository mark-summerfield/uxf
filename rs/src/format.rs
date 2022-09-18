// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[derive(Clone, Debug)]
pub struct Format {
    pub indent: String,
    pub wrapwidth: u8,
    pub realdp: Option<u8>,
}

impl Format {
    pub fn new(indent: u8, wrapwidth: u8, realdp: Option<u8>) -> Self {
        Format {
            indent: match indent {
                0 => "".to_string(),
                1..=8 => " ".repeat(indent as usize),
                9 => "\t".to_string(),
                _ => "  ".to_string(),
            },
            wrapwidth,
            realdp,
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format { indent: "  ".to_string(), wrapwidth: 96, realdp: None }
    }
}
