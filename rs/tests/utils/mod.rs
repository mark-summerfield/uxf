// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use uxf::consts::*;
use uxf::event::{Event, EventKind};

#[allow(dead_code)]
pub fn assert_warning(event: &Event, code: u16, message: &str) {
    assert_event(event, EventKind::Warning, code, "-", 0, message);
}

#[allow(dead_code)]
pub fn assert_repair(event: &Event, code: u16, message: &str) {
    assert_event(event, EventKind::Repair, code, "-", 0, message);
}

#[allow(dead_code)]
pub fn assert_event(
    event: &Event,
    kind: EventKind,
    code: u16,
    filename: &str,
    lino: usize,
    message: &str,
) {
    assert_eq!(event.kind, kind);
    assert_eq!(event.code, code);
    assert_eq!(event.filename, filename);
    assert_eq!(event.lino, lino);
    assert_eq!(event.message, message);
}

pub fn check_error(err: &str, code: i32, name: &str) {
    match code {
        110 => assert_eq!(
            err,
            "E110:-:0:missing UXF file header or missing data \
            or empty file"
        ),
        120 => assert_eq!(err, "E120:-:1:invalid UXF file header"),
        130 => assert_eq!(err, "E130:-:1:not a UXF file"),
        151 => assert_eq!(
            err,
            "E151:-:1:failed to read UXF file version number"
        ),
        160 => assert_eq!(
            err,
            format!(
                "E160:-:2:invalid comment syntax: expected '<', \
                got '{name}'",
            )
        ),
        304 => {
            assert_eq!(
                err,
                format!(
                    "E304:-:0:table names (ttypes) and fieldnames \
                    cannot be the same as built-in type names or \
                    constants, got {name}",
                )
            );
        }
        298 => assert_eq!(err, "E298:-:0:names must be nonempty"),
        300 => assert_eq!(
            err,
            format!(
                "E300:-:0:names must start with a letter or underscore, \
                got {name}",
            )
        ),
        302 => assert_eq!(
            err,
            format!("E302:-:0:names may not be yes or no got {name}")
        ),
        306 => {
            let n = name.len(); // byte count is fine: all ASCII
            assert_eq!(
                err,
                format!(
                    "E306:-:0:names may be at most {MAX_IDENTIFIER_LEN} \
                    characters long, got {name} ({n} characters)"
                )
            );
        }
        310 => assert_eq!(
            err,
            format!(
                "E310:-:0:names may only contain letters, digits, or \
                underscores, got {name}",
            )
        ),
        336 => assert_eq!(
            err,
            format!(
                "E336:-:0:can't have duplicate table tclass field \
                names, got {name:?} twice",
            )
        ),
        _ => panic!("unexpected error code {code} ({name:?})"),
    }
}
