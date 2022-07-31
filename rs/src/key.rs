// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

// NOTE Keep in harmony with Value

use crate::constants::*;
use crate::util::escape;
use crate::value::bytes_to_uxf;
use chrono::prelude::*;
use std::fmt;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum Key {
    Bytes(Vec<u8>),
    Date(NaiveDate),
    Int(i64),
    Str(String),
}

impl Key {
    /// Returns `true` if `Key::Bytes`; otherwise returns `false`.
    pub fn is_bytes(&self) -> bool {
        matches!(self, Key::Bytes(_))
    }

    /// Returns `true` if `Key::Date`; otherwise returns `false`.
    pub fn is_date(&self) -> bool {
        matches!(self, Key::Date(_))
    }

    /// Returns `true` if `Key::Int`; otherwise returns `false`.
    pub fn is_int(&self) -> bool {
        matches!(self, Key::Int(_))
    }

    /// Returns `true` if `Key::Str`; otherwise returns `false`.
    pub fn is_str(&self) -> bool {
        matches!(self, Key::Str(_))
    }

    /// Returns `Some(&Vec<u8>)` if `Value::Bytes`; otherwise returns
    /// `None`.
    pub fn as_bytes(&self) -> Option<&Vec<u8>> {
        if let Key::Bytes(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// Returns `Some(NaiveDate)` if `Value::Date`; otherwise returns
    /// `None`.
    pub fn as_date(&self) -> Option<NaiveDate> {
        if let Key::Date(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns `Some(i64)` if `Key::Int`; otherwise returns `None`.
    pub fn as_int(&self) -> Option<i64> {
        if let Key::Int(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns `Some(&str)` if `Key::Str`; otherwise returns `None`.
    pub fn as_str(&self) -> Option<&str> {
        if let Key::Str(value) = self {
            Some(value)
        } else {
            None
        }
    }
}

impl From<Vec<u8>> for Key {
    fn from(b: Vec<u8>) -> Self {
        Key::Bytes(b)
    }
}

impl From<NaiveDate> for Key {
    fn from(d: NaiveDate) -> Self {
        Key::Date(d)
    }
}

impl From<i64> for Key {
    fn from(i: i64) -> Self {
        Key::Int(i)
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Key::Str(s.to_string())
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Key::Str(s)
    }
}

// NOTE: *must* match impl fmt::Display for Value
impl fmt::Display for Key {
    /// Provides a .to_string() that returns a valid UXF fragment
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Key::Bytes(b) => bytes_to_uxf(b),
                Key::Date(d) => d.format(ISO8601_DATE).to_string(),
                Key::Int(i) => i.to_string(),
                Key::Str(s) => format!("<{}>", escape(s)),
            }
        )
    }
}
