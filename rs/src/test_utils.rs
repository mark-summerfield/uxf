// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;

pub fn check_error_code(error: &str, code: i32, name: &str) {
    match code {
        304 => {
            assert_eq!(code, 304, "code={} name={}", code, name);
            assert_eq!(
                error,
                format!(
                    "uxf:F304:-:0:names cannot be the same \
                               as built-in type names or constants, got {}",
                    name
                )
            );
        }
        298 => assert_eq!(error, "uxf:F298:-:0:names must be nonempty"),
        300 => assert_eq!(
            error,
            format!(
                "uxf:F300:-:0:names must start \
                                  with a letter or underscore, got {}",
                name
            )
        ),
        302 => assert_eq!(
            error,
            format!("uxf:F302:-:0:names may not be yes or no got {}", name)
        ),
        306 => {
            let n = name.len(); // byte count is fine: all ASCII
            assert_eq!(
                error,
                format!(
                    "uxf:F306:-:0:names may be at most \
                               {} characters long, got {} ({} characters)",
                    MAX_IDENTIFIER_LEN, name, n
                )
            );
        }
        310 => assert_eq!(
            error,
            format!(
                "uxf:F310:-:0:names may only contain letters, digits, or \
                                  underscores, got {}",
                name
            )
        ),
        336 => assert_eq!(
            error,
            format!(
                "uxf:F336:-:0:can't have duplicate table tclass field \
                names, got {:?} twice",
                name
            )
        ),
        _ => panic!("unexpected error code {} ({:?})", code, name),
    }
}
