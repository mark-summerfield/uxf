// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use uxf::constants::*;
    use uxf::field::{make_fields, Field};
    use uxf::tclass::{TClass, TClassBuilder};
    use uxf::test_utils::check_error_code;
    use uxf::value::Value;

    #[test]
    fn t_tclass_new() {
        let f2 = Field::new("selected", "bool").unwrap();
        let f5 = Field::new("timestamp", "datetime").unwrap();
        let f6 = Field::new("Kind", "").unwrap();
        let fields = valid_fields();
        let asize = fields.len();
        let a = TClass::new("AType", fields, "A row type").unwrap();
        assert!(!a.is_fieldless());
        assert_eq!(a.len(), asize);
        assert!(!a.is_empty());
        assert_eq!(a.ttype(), "AType");
        assert_eq!(a.comment(), "A row type");
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
        let b = TClass::new("BType", valid_fields()[..bsize].to_vec(), "")
            .unwrap();
        assert!(!b.is_fieldless());
        assert_eq!(b.len(), bsize);
        assert!(!b.is_empty());
        assert_eq!(b.ttype(), "BType");
        assert!(b.comment().is_empty());
        assert_eq!(b.fields().len(), bsize);
        assert_eq!(b.fields()[2], f2);
        let rec = b.record_of_nulls().unwrap();
        assert_eq!(rec.len(), bsize);
    }

    #[test]
    fn t_tclass_new_fieldless() {
        let ready =
            TClass::new_fieldless("ReadyState", "Ready enum").unwrap();
        assert!(ready.is_fieldless());
        assert_eq!(ready.len(), 0);
        assert!(ready.is_empty());
        assert_eq!(ready.ttype(), "ReadyState");
        assert_eq!(ready.comment(), "Ready enum");
        assert_eq!(ready.fields().len(), 0);
        let wait = TClass::new_fieldless("WaitState", "").unwrap();
        assert!(wait.is_fieldless());
        assert_eq!(wait.len(), 0);
        assert!(wait.is_empty());
        assert_eq!(wait.ttype(), "WaitState");
        assert!(wait.comment().is_empty());
        let row = wait.record_of_nulls();
        assert!(row.is_err());
    }

    #[test]
    fn t_tclass_display() {
        let fields = valid_fields();
        let tclass = TClass::new("General", fields, "first test").unwrap();
        assert_eq!(
            tclass.to_string(),
            "TClass::new(\"General\", vec![Field::new(\"CID\", \"int\"), \
            Field::new(\"title\", \"str\"), \
            Field::new(\"selected\", \"bool\"), \
            Field::new(\"when\", \"date\"), \
            Field::new(\"size\", \"real\"), \
            Field::new(\"timestamp\", \"datetime\"), \
            Field::new(\"Kind\", \"\"), \
            Field::new(\"Filename\", \"\"), \
            Field::new(\"Categories\", \"Categories\"), \
            Field::new(\"Extra\", \"Point\")], \"first test\")"
        );
        let tclass = TClass::new_fieldless("StateReady", "enum").unwrap();
        assert_eq!(
            tclass.to_string(),
            "TClass::new_fieldless(\"StateReady\", \"enum\")"
        );
        let tclass = TClass::new_fieldless("StateWait", "").unwrap();
        assert_eq!(
            tclass.to_string(),
            "TClass::new_fieldless(\"StateWait\", \"\")"
        );
    }

    #[test]
    fn t_tclass_eq_lt() {
        // For < we only care about the ttype and prefer case-insensitive
        let fields = valid_fields();
        let a = TClass::new("Alpha", fields, "").unwrap();
        let fields = valid_fields()[..6].to_vec();
        let b = TClass::new("bravo", fields, "").unwrap();
        let fields = valid_fields()[..3].to_vec();
        let c = TClass::new("Charlie", fields, "").unwrap();
        let d = TClass::new_fieldless("Delta", "").unwrap();
        let e = TClass::new_fieldless("ECHO", "").unwrap();
        let f = TClass::new_fieldless("echo", "").unwrap();
        assert!(a < b && b < c && c < d && d < e && e < f);
        assert!(a != b && b != c && c != d && d != e && e != f);
        let b2 = b.clone();
        let c2 = TClass::new("Charlie", valid_fields()[..3].to_vec(), "")
            .unwrap();
        assert!(b2 != c2 && b == b2 && c == c2);
    }

    #[test]
    fn t_tclass_duplicate_field() {
        let mut fields = valid_fields();
        fields.push(Field::new("size", "").unwrap());
        let e = TClass::new("General", fields, "").unwrap_err();
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
            let tclass = TClass::new(name, fields, "");
            assert!(
                tclass.is_err(),
                "expected err of #{} on {}",
                code,
                name
            );
            let e = tclass.unwrap_err();
            check_error_code(&e.to_string(), code, name);
            // Fieldless
            let tclass = TClass::new_fieldless(name, "");
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

    #[test]
    fn t_tclassbuilder_valid() {
        let fields = valid_fields();
        let asize = fields.len();
        let mut b = TClassBuilder::new("Builder", "");
        for field in &fields {
            b.append(field);
        }
        let a = b.build().unwrap();
        assert_eq!(a.len(), asize);
        assert!(!a.is_empty());
        assert_eq!(a.ttype(), "Builder");
        assert_eq!(a.comment(), "");
        assert_eq!(a.fields().len(), asize);
        let asize = 3;
        let mut c = TClassBuilder::new("BuilderX", "new build");
        for field in &fields[..3] {
            c.append(field);
        }
        let d = c.build().unwrap();
        assert_eq!(d.len(), asize);
        assert!(!d.is_empty());
        assert_eq!(d.ttype(), "BuilderX");
        assert_eq!(d.comment(), "new build");
        assert_eq!(d.fields().len(), asize);
    }

    #[test]
    fn t_tclassbuilder_invalid() {
        let b = TClassBuilder::new("New Builder", "");
        let e = b.build().unwrap_err();
        check_error_code(&e.to_string(), 310, "New Builder");
        let mut b = TClassBuilder::new("New_Builder", "");
        let fields = valid_fields();
        for field in &fields {
            b.append(field);
        }
        b.append(&Field::new("Filename", "str").unwrap());
        let e = b.build().unwrap_err();
        check_error_code(&e.to_string(), 336, "Filename");
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
