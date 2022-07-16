// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use uxf::constants::*;
    use uxf::field::{make_fields, Field};
    use uxf::tclass::TClass;
    use uxf::test_utils::check_error_code;

    // TODO new() new_fieldless() is_fieldless() ttype() comment() len()
    // record_of_nulls() == != < clone()
    #[test]
    fn t_tclass() {
        // TODO with & without comment
    }

    #[test]
    fn t_tclass_display() {
        let fields = valid_fields();
        let tclass =
            TClass::new("General", fields, Some("first test")).unwrap();
        assert_eq!(
            tclass.to_string(),
            "TClass::new(\"General\", vec![Field::new(\"CID\", \"int\"), \
            Field::new(\"title\", \"str\"), \
            Field::new(\"selected\", \"bool\"), \
            Field::new(\"when\", \"date\"), \
            Field::new(\"size\", \"real\"), \
            Field::new(\"timestamp\", \"datetime\"), \
            Field::new_anyvtype(\"Kind\"), \
            Field::new_anyvtype(\"Filename\"), \
            Field::new(\"Categories\", \"Categories\"), \
            Field::new(\"Extra\", \"Point\")], Some(\"first test\"))"
        );
        let tclass =
            TClass::new_fieldless("StateReady", Some("enum")).unwrap();
        assert_eq!(
            tclass.to_string(),
            "TClass::new_fieldless(\"StateReady\", Some(\"enum\"))"
        );
    }

    #[test]
    fn t_tclass_new_fieldless() {
        // TODO with & without comment
    }

    #[test]
    fn t_tclass_lt() {
        // We only care about the ttype and prefer case-insensitive
        let fields = valid_fields();
        let a = TClass::new("Alpha", fields, None).unwrap();
        let fields = valid_fields()[..6].to_vec();
        let b = TClass::new("bravo", fields, None).unwrap();
        let fields = valid_fields()[..3].to_vec();
        let c = TClass::new("Charlie", fields, None).unwrap();
        let d = TClass::new_fieldless("Delta", None).unwrap();
        let e = TClass::new_fieldless("ECHO", None).unwrap();
        let f = TClass::new_fieldless("echo", None).unwrap();
        assert!(a < b && b < c && c < d && d < e && e < f);
    }

    #[test]
    fn t_tclass_duplicate_field() {
        let mut fields = valid_fields();
        fields.push(Field::new_anyvtype("size").unwrap());
        let e = TClass::new("General", fields, None).unwrap_err();
        check_error_code(&e.to_string(), 336, "size");
    }

    #[test]
    fn t_tclass_invalid_ttype() {
        for (code, name) in [
            (300, "*abc"),
            (300, "1int"),
            (300, "€200"),
            (302, BOOL_FALSE),
            (302, BOOL_TRUE),
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
            (
                306,
                "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\
                   xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxy",
            ),
            (310, "e—"),
            (310, "_almost#1"),
            (310, "f1:"),
        ] {
            // With fields
            let fields = valid_fields();
            let tclass = TClass::new(name, fields, None);
            assert!(
                tclass.is_err(),
                "expected err of #{} on {}",
                code,
                name
            );
            let e = tclass.unwrap_err();
            check_error_code(&e.to_string(), code, name);
            // Fieldless
            let tclass = TClass::new_fieldless(name, None);
            assert!(
                tclass.is_err(),
                "expected err of #{} on {}",
                code,
                name
            );
            let e = tclass.unwrap_err();
            check_error_code(&e.to_string(), code, name);
        }
    }

    fn valid_fields() -> Vec<Field> {
        make_fields(&[
            ("CID", "int"),
            ("title", "str"),
            ("selected", "bool"),
            ("when", "date"),
            ("size", "real"),
            ("timestamp", "datetime"),
            ("Kind", ""),
            ("Filename", ""),
            ("Categories", "Categories"),
            ("Extra", "Point"),
        ])
        .unwrap()
    }
}
