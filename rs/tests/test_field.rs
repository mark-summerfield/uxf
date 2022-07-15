// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use uxf::constants::*;
    use uxf::field::Field;
    use uxf::test_utils::check_error_code;

    #[test]
    fn t_field() {
        // Tests new() new_anyvtype() name() vtype() == != clone()
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
            let name = format!("{}{}", vtype, i + 1);
            let f = Field::new(&name, vtype).unwrap();
            assert_eq!(
                f.to_string(),
                format!("Field::new({:?}, {:?})", name, vtype)
            );
            assert_eq!(f.name(), name);
            assert_eq!(&f.vtype().unwrap(), vtype);
            let g = Field::new(&name, vtype).unwrap();
            assert!(f == g);

            // With vtype None
            let name = format!("{}{}", vtype, i + 1);
            let h = Field::new_anyvtype(&name).unwrap();
            assert_eq!(
                h.to_string(),
                format!("Field::new_anyvtype({:?})", name)
            );
            assert_eq!(h.name(), name);
            assert!(&h.vtype().is_none());
            let i = Field::new_anyvtype(&name).unwrap();
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
        let a = Field::new_anyvtype("Alpha").unwrap();
        let b = Field::new("bravo", "int").unwrap();
        let c = Field::new("Charlie", "MyType4").unwrap();
        let d = Field::new_anyvtype("Delta").unwrap();
        let e = Field::new_anyvtype("ECHO").unwrap();
        let f = Field::new_anyvtype("echo").unwrap();
        assert!(a < b && b < c && c < d && d < e && e < f);
    }

    #[test]
    fn t_field_new_invalid_name() {
        for (code, name) in [
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
            (602, "*abc"),
            (602, "1int"),
            (602, "€200"),
            (604, BOOL_FALSE),
            (604, BOOL_TRUE),
            (
                606,
                "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\
                   xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxy",
            ),
            (608, "e—"),
            (608, "_almost#1"),
            (608, "f1:"),
        ] {
            // With Some vtype
            let f =
                Field::new(name, if code == 304 { name } else { "str" });
            assert!(f.is_err(), "expected err of #{} on {}", code, name);
            let e = f.unwrap_err();
            check_error_code(&e.to_string(), code, name);

            // With vtype None
            let f = Field::new_anyvtype(name);
            assert!(f.is_err(), "expected err of #{} on {}", code, name);
            let e = f.unwrap_err();
            check_error_code(&e.to_string(), code, name);
        }
    }

    #[test]
    fn t_field_new_invalid_vtype() {
        for (code, vtype) in [
            (600, ""),
            (602, "*abc"),
            (602, ".Custom_"),
            (602, "1int"),
            (602, "€200"),
            (604, BOOL_FALSE),
            (604, BOOL_TRUE),
            (
                606,
                "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\
                   xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxy",
            ),
            (608, "My.Type"),
            (608, "_9.9"),
            (608, "_almost#1"),
            (608, "_special."),
            (608, "a_y_47ĕặæ_."),
            (608, "e—"),
            (608, "f1:"),
        ] {
            let f = Field::new("test", vtype);
            assert!(f.is_err(), "expected err of #{} on {}", code, vtype);
            let e = f.unwrap_err();
            check_error_code(&e.to_string(), code, vtype);
        }
    }
}
