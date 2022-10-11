// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/// Constants used throughout the uxf module.
pub const UXF_VERSION: u16 = 1;
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub const MAX_IDENTIFIER_LEN: usize = 32;

pub const NL: char = '\n';
pub const NUL: char = '\0';

pub static ISO8601_DATE: &str = "%Y-%m-%d"; //YYYY-MM-DD
pub static ISO8601_DATETIME: &str = "%Y-%m-%dT%H:%M:%S"; // YYYY-MM-DDTHH:MM:SS
pub static ISO8601_DATETIME_M: &str = "%Y-%m-%dT%H:%M"; // YYYY-MM-DDTHH:MM
pub static ISO8601_DATETIME_H: &str = "%Y-%m-%dT%H"; // YYYY-MM-DDTHH

pub static VALUE_NAME_NULL: &str = "null";
pub static VTYPE_NAME_BOOL: &str = "bool";
pub static VTYPE_NAME_BYTES: &str = "bytes";
pub static VTYPE_NAME_DATE: &str = "date";
pub static VTYPE_NAME_DATETIME: &str = "datetime";
pub static VTYPE_NAME_INT: &str = "int";
pub static VTYPE_NAME_LIST: &str = "list";
pub static VTYPE_NAME_MAP: &str = "map";
pub static VTYPE_NAME_REAL: &str = "real";
pub static VTYPE_NAME_STR: &str = "str";
pub static VTYPE_NAME_TABLE: &str = "table";

pub static BOOL_FALSE: &str = "no";
pub static BOOL_TRUE: &str = "yes";

pub static BARE_WORDS: [&str; 12] = [
    VTYPE_NAME_BOOL,
    VTYPE_NAME_BYTES,
    VTYPE_NAME_DATE,
    VTYPE_NAME_DATETIME,
    VTYPE_NAME_INT,
    VTYPE_NAME_LIST,
    VTYPE_NAME_MAP,
    VTYPE_NAME_REAL,
    VTYPE_NAME_STR,
    VTYPE_NAME_TABLE,
    BOOL_FALSE,
    BOOL_TRUE,
];

pub static RESERVED_WORDS: [&str; 13] = [
    VALUE_NAME_NULL,
    VTYPE_NAME_BOOL,
    VTYPE_NAME_BYTES,
    VTYPE_NAME_DATE,
    VTYPE_NAME_DATETIME,
    VTYPE_NAME_INT,
    VTYPE_NAME_LIST,
    VTYPE_NAME_MAP,
    VTYPE_NAME_REAL,
    VTYPE_NAME_STR,
    VTYPE_NAME_TABLE,
    BOOL_FALSE,
    BOOL_TRUE,
];

pub static KTYPES: [&str; 4] =
    [VTYPE_NAME_BYTES, VTYPE_NAME_DATE, VTYPE_NAME_INT, VTYPE_NAME_STR];

pub static VTYPES: [&str; 10] = [
    VTYPE_NAME_BOOL,
    VTYPE_NAME_BYTES,
    VTYPE_NAME_DATE,
    VTYPE_NAME_DATETIME,
    VTYPE_NAME_INT,
    VTYPE_NAME_LIST,
    VTYPE_NAME_MAP,
    VTYPE_NAME_REAL,
    VTYPE_NAME_STR,
    VTYPE_NAME_TABLE,
];
