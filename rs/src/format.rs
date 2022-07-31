// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

pub struct Format {
    pub indent: String,
    pub wrap_width: u8,
    pub realdp: Option<u8>,
    pub max_short_len: u8,
}

impl Default for Format {
    fn default() -> Self {
        Format {
            indent: "  ".to_string(),
            wrap_width: 96,
            realdp: None,
            max_short_len: 32,
        }
    }
}
