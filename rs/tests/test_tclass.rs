// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use uxf::constants::*;
    use uxf::field::{make_fields, Field};
    use uxf::tclass::TClass;
    use uxf::test_utils::check_error_code;
    use uxf::value::Value;

    #[test]
    fn t_tclass_new() {
        let f2 = Field::new("selected", "bool").unwrap();
        let f5 = Field::new("timestamp", "datetime").unwrap();
        let f6 = Field::new_anyvtype("Kind").unwrap();
        let fields = valid_fields();
        let asize = fields.len();
        let a = TClass::new("AType", fields, Some("A row type")).unwrap();
        assert!(!a.is_fieldless());
        assert_eq!(a.len(), asize);
        assert!(!a.is_empty());
        assert_eq!(a.ttype(), "AType");
        assert_eq!(a.comment(), Some("A row type"));
        assert_eq!(a.fields().len(), asize);
        let fields = a.fields();
        assert_eq!(fields[2], f2);
        assert_eq!(fields[5], f5);
        assert_eq!(fields[6], f6);
        let mut rec = a.record_of_nulls().unwrap();
        assert_eq!(rec.len(), asize);
        rec[0] = Some(Value::Bool(true));
        rec[1] = Some(Value::Int(-17));
        rec[2] = Some(Value::Str("Test data".to_string()));
        assert_eq!(rec.len(), asize);
        assert_eq!(rec[0].as_ref().unwrap().as_bool().unwrap(), true);
        assert_eq!(rec[1].as_ref().unwrap().as_int().unwrap(), -17);
        assert_eq!(rec[2].as_ref().unwrap().as_str().unwrap(), "Test data");
        let bsize = 5;
        let b =
            TClass::new("BType", valid_fields()[..bsize].to_vec(), None)
                .unwrap();
        assert!(!b.is_fieldless());
        assert_eq!(b.len(), bsize);
        assert!(!b.is_empty());
        assert_eq!(b.ttype(), "BType");
        assert_eq!(b.comment(), None);
        assert_eq!(b.fields().len(), bsize);
        assert_eq!(b.fields()[2], f2);
        let rec = b.record_of_nulls().unwrap();
        assert_eq!(rec.len(), bsize);
    }

    #[test]
    fn t_tclass_new_fieldless() {
        let ready = TClass::new_fieldless("ReadyState", Some("Ready enum"))
            .unwrap();
        assert!(ready.is_fieldless());
        assert_eq!(ready.len(), 0);
        assert!(ready.is_empty());
        assert_eq!(ready.ttype(), "ReadyState");
        assert_eq!(ready.comment(), Some("Ready enum"));
        assert_eq!(ready.fields().len(), 0);
        let wait = TClass::new_fieldless("WaitState", None).unwrap();
        assert!(wait.is_fieldless());
        assert_eq!(wait.len(), 0);
        assert!(wait.is_empty());
        assert_eq!(wait.ttype(), "WaitState");
        assert!(wait.comment().is_none());
        let row = wait.record_of_nulls();
        assert!(row.is_err());
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
    fn t_tclass_eq_lt() {
        // For < we only care about the ttype and prefer case-insensitive
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
        assert!(a != b && b != c && c != d && d != e && e != f);
        let b2 = b.clone();
        let c2 = TClass::new("Charlie", valid_fields()[..3].to_vec(), None)
            .unwrap();
        assert!(b2 != c2 && b == b2 && c == c2);
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
