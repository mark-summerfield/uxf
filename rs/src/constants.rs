// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

pub(crate) const MAX_IDENTIFIER_LEN: usize = 60;

pub static ISO8601_DATE: &str = "%Y-%m-%d";
pub static ISO8601_DATETIME: &str = "%Y-%m-%dT%H:%M:%S";

pub(crate) static VALUE_NAME_NULL: &str = "null";
pub(crate) static VTYPE_NAME_BOOL: &str = "bool";
pub(crate) static VTYPE_NAME_BYTES: &str = "bytes";
pub(crate) static VTYPE_NAME_DATE: &str = "date";
pub(crate) static VTYPE_NAME_DATETIME: &str = "datetime";
pub(crate) static VTYPE_NAME_INT: &str = "int";
pub(crate) static VTYPE_NAME_LIST: &str = "list";
pub(crate) static VTYPE_NAME_MAP: &str = "map";
pub(crate) static VTYPE_NAME_REAL: &str = "real";
pub(crate) static VTYPE_NAME_STR: &str = "str";
pub(crate) static VTYPE_NAME_TABLE: &str = "table";

pub(crate) static BOOL_FALSE: &str = "no";
pub(crate) static BOOL_TRUE: &str = "yes";

pub static RESERVED_WORDS:[&str; 13] = [
    VALUE_NAME_NULL, VTYPE_NAME_BOOL, VTYPE_NAME_BYTES, VTYPE_NAME_DATE,
    VTYPE_NAME_DATETIME, VTYPE_NAME_INT, VTYPE_NAME_LIST, VTYPE_NAME_MAP,
    VTYPE_NAME_REAL, VTYPE_NAME_STR, VTYPE_NAME_TABLE,
    BOOL_FALSE, BOOL_TRUE];

pub static KTYPES:[&str; 4] = [
    VTYPE_NAME_BYTES, VTYPE_NAME_DATE, VTYPE_NAME_INT, VTYPE_NAME_STR];

pub static VTYPES:[&str; 10] = [
    VTYPE_NAME_BOOL, VTYPE_NAME_BYTES, VTYPE_NAME_DATE, VTYPE_NAME_DATETIME,
    VTYPE_NAME_INT, VTYPE_NAME_LIST, VTYPE_NAME_MAP, VTYPE_NAME_REAL,
    VTYPE_NAME_STR, VTYPE_NAME_TABLE];

