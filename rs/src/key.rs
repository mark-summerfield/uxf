// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

// NOTE Keep in harmony with Value

use crate::constants::*;
use crate::util::escape;
use crate::value::{bytes_to_uxf, Value};
use anyhow::{bail, Result};
use chrono::prelude::*;
use std::{cmp::Ordering, fmt};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Key {
    Bytes(Vec<u8>),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
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

    /// Returns `true` if `Key::DateTime`; otherwise returns `false`.
    pub fn is_datetime(&self) -> bool {
        matches!(self, Key::DateTime(_))
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

    /// Returns `Some(NaiveDateTime)` if `Value::DateTime`; otherwise
    /// returns `None`.
    pub fn as_datetime(&self) -> Option<NaiveDateTime> {
        if let Key::DateTime(value) = self {
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

    pub(crate) fn from(value: Value) -> Result<Key> {
        match value {
            Value::Bytes(b) => Ok(Key::Bytes(b)),
            Value::Date(d) => Ok(Key::Date(d)),
            Value::DateTime(d) => Ok(Key::DateTime(d)),
            Value::Int(i) => Ok(Key::Int(i)),
            Value::Str(s) => Ok(Key::Str(s)),
            _ => bail!(
                "E600:-:0:can only convert bytes, date, datetime, \
                int, str from Value to Key, got {:?}",
                value
            ),
        }
    }
}

impl From<Vec<u8>> for Key {
    fn from(b: Vec<u8>) -> Self {
        Key::Bytes(b)
    }
}

impl From<&[u8]> for Key {
    fn from(b: &[u8]) -> Self {
        Key::Bytes(b.to_vec())
    }
}

impl From<NaiveDate> for Key {
    fn from(d: NaiveDate) -> Self {
        Key::Date(d)
    }
}

impl From<NaiveDateTime> for Key {
    fn from(d: NaiveDateTime) -> Self {
        Key::DateTime(d)
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
                Key::DateTime(d) => d.format(ISO8601_DATETIME).to_string(),
                Key::Int(i) => i.to_string(),
                Key::Str(s) => format!("<{}>", escape(s)),
            }
        )
    }
}

impl Ord for Key {
    /// ordering: bytes < date < datetime < int < str (case-insensitive)
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Key::Bytes(a) => match other {
                Key::Bytes(b) => a.cmp(b),
                _ => Ordering::Less,
            },
            Key::Date(a) => match other {
                Key::Date(b) => a.cmp(b),
                Key::Bytes(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            Key::DateTime(a) => match other {
                Key::DateTime(b) => a.cmp(b),
                Key::Bytes(_) | Key::Date(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            Key::Int(a) => match other {
                Key::Int(b) => a.cmp(b),
                Key::Str(_) => Ordering::Less,
                _ => Ordering::Greater,
            },
            Key::Str(a) => match other {
                Key::Str(b) => a.to_lowercase().cmp(&b.to_lowercase()),
                _ => Ordering::Greater,
            },
        }
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
