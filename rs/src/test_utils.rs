// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::constants::*;
use crate::value::Value;

pub fn opt_value_to_str(v: Option<Value>) -> String {
    match v {
        None => "?".to_string(),
        Some(v) => value_to_str(v),
    }
}

pub fn value_to_str(v: Value) -> String {
    match v {
        // TODO better output for List, Map, Table: once I've implemented
        // Display for them change "{:?}" to "{}".
        Value::Bool(true) => "yes".to_string(),
        Value::Bool(false) => "no".to_string(),
        Value::Bytes(b) => format!("{:?}", b),
        Value::Date(d) => d.format(ISO8601_DATE).to_string(),
        Value::DateTime(dt) => dt.format(ISO8601_DATETIME).to_string(),
        Value::Int(i) => format!("{}", i),
        Value::List(lst) => format!("{:?}", lst),
        Value::Map(m) => format!("{:?}", m),
        Value::Real(r) => format!("{}", r),
        Value::Str(s) => s,
        Value::Table(t) => format!("{:?}", t),
    }
}

pub fn check_error_code(error: &str, code: i32, name: &str) {
    match code {
        304 => {
            assert_eq!(code, 304, "code={} name={}", code, name);
            assert_eq!(
                error,
                format!(
                    "#304:names cannot be the same \
                               as built-in type names or constants, got {}",
                    name
                )
            );
        }
        336 => assert_eq!(
            error,
            format!(
                "#336:can't have duplicate table tclass field names, \
                got {:?} twice",
                name
            )
        ),
        600 => assert_eq!(error, "#600:type names must be nonempty"),
        602 => assert_eq!(
            error,
            format!(
                "#602:type names must start \
                                  with a letter or underscore, got {}",
                name
            )
        ),
        604 => assert_eq!(
            error,
            format!("#604:type names may not be yes or no got {}", name)
        ),
        606 => {
            let n = name.len(); // byte count is fine: all ASCII
            assert_eq!(
                error,
                format!(
                    "#606:type names may be at most \
                               {} characters long, got {} ({} characters)",
                    MAX_IDENTIFIER_LEN, name, n
                )
            );
        }
        608 => assert_eq!(
            error,
            format!(
                "#608:type names may only \
                                  contain letters, digits, or \
                                  underscores, got {}",
                name
            )
        ),
        _ => assert!(false, "unexpected error code {} ({:?})", code, name),
    }
}
