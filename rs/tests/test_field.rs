// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

mod utils;

use utils::check_error;
use uxf::consts::*;
use uxf::field::Field;

#[test]
fn t_field() {
    // Tests new() name() vtype() == != clone()
    for (i, vtype) in [
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
        "MyType",
        "Custom_",
        "_special",
        "_99",
        "a_y_47ĕặæ_",
    ]
    .iter()
    .enumerate()
    {
        // With Some vtype
        let name = format!("{vtype}{}", i + 1);
        let f = Field::new(&name, vtype).unwrap();
        assert_eq!(f.to_string(), format!("{name}:{vtype}"));
        assert_eq!(f.name(), name);
        assert_eq!(&f.vtype().unwrap(), vtype);
        let g = Field::new(&name, vtype).unwrap();
        assert!(f == g);

        // With vtype None
        let name = format!("{vtype}{}", i + 1);
        let h = Field::new(&name, "").unwrap();
        assert_eq!(h.to_string(), format!("{name}"));
        assert_eq!(h.name(), name);
        assert!(&h.vtype().is_none());
        let i = Field::new(&name, "").unwrap();
        assert!(h == i);
        assert!(f != h);
        assert!(g != i);
        let j = f.clone();
        assert!(j == f);
        assert!(j != h);
        let k = h.clone();
        assert!(k == h);
        assert!(k != g);
    }
}

#[test]
fn t_field_lt() {
    // We only care about the name and prefer case-insensitive
    let a = Field::new("Alpha", "").unwrap();
    let b = Field::new("bravo", "int").unwrap();
    let c = Field::new("Charlie", "MyType4").unwrap();
    let d = Field::new("Delta", "").unwrap();
    let e = Field::new("ECHO", "").unwrap();
    let f = Field::new("echo", "").unwrap();
    assert!(a < b && b < c && c < d && d < e && e < f);
}

#[test]
fn t_field_new_invalid_name() {
    for (code, name) in [
        (300, "*abc"),
        (300, "1int"),
        (300, "€200"),
        (304, BOOL_FALSE),
        (304, BOOL_TRUE),
        (304, VALUE_NAME_NULL),
        (304, VTYPE_NAME_BOOL),
        (304, VTYPE_NAME_BYTES),
        (304, VTYPE_NAME_DATE),
        (304, VTYPE_NAME_DATETIME),
        (304, VTYPE_NAME_INT),
        (304, VTYPE_NAME_LIST),
        (304, VTYPE_NAME_MAP),
        (304, VTYPE_NAME_REAL),
        (304, VTYPE_NAME_STR),
        (304, VTYPE_NAME_TABLE),
        (306, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxy"),
        (310, "e—"),
        (310, "_almost#1"),
        (310, "f1:"),
    ] {
        // With Some vtype
        let f = Field::new(name, if code == 304 { name } else { "str" });
        assert!(f.is_err(), "expected err of #{} on {}", code, name);
        let e = f.unwrap_err();
        check_error(&e.to_string(), code, name);

        // With vtype None
        let f = Field::new(name, "");
        assert!(f.is_err(), "expected err of #{} on {}", code, name);
        let e = f.unwrap_err();
        check_error(&e.to_string(), code, name);
    }
}

#[test]
fn t_field_new_invalid_vtype() {
    // A vtype of "" is valid since it is taken to be None i.e., any
    // vtype is accepted
    for (code, vtype) in [
        (300, "*abc"),
        (300, ".Custom_"),
        (300, "1int"),
        (300, "€200"),
        (302, BOOL_FALSE),
        (302, BOOL_TRUE),
        (306, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxy"),
        (310, "My.Type"),
        (310, "_9.9"),
        (310, "_almost#1"),
        (310, "_special."),
        (310, "a_y_47ĕặæ_."),
        (310, "e—"),
        (310, "f1:"),
    ] {
        let f = Field::new("test", vtype);
        assert!(f.is_err(), "expected err of #{} on {}", code, vtype);
        let e = f.unwrap_err();
        check_error(&e.to_string(), code, vtype);
    }
}
