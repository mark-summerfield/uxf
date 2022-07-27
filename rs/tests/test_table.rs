// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use uxf::field::make_fields;
    use uxf::table::Table;
    use uxf::tclass::TClass;
    use uxf::value::Value;

    #[test]
    fn t_table_scalar() {
        let tclass = TClass::new_fieldless("ReadyState", "enum").unwrap();
        assert_eq!(tclass.to_string(), "=#<enum> ReadyState");
        assert_eq!(tclass.comment().to_string(), "enum");
        assert_eq!(tclass.len(), 0);
        let t = Table::new(tclass, "");
        assert_eq!(t.to_string(), "(ReadyState)");
        assert_eq!(t.len(), 0);
        assert_eq!(t.ttype_len(), 0);
        assert!(t.is_empty());
        let v = Value::Table(t);
        assert_eq!(v.to_string(), "(ReadyState)");
        let fields = make_fields(&[("x", "int"), ("y", "int")]).unwrap();
        let tclass = TClass::new("Point", fields, "").unwrap();
        assert_eq!(tclass.to_string(), "=Point x:int y:int");
        let mut t = Table::new(tclass, "");
        assert_eq!(t.to_string(), "(Point)");
        let _ = t.push(vec![0.into(), 0.into()]);
        assert_eq!(t.to_string(), "(Point 0 0)");
        let _ = t.push(t.tclass().record_of_nulls().unwrap());
        assert_eq!(t.to_string(), "(Point 0 0 ? ?)");
        t[1] = vec![(-11).into(), 14.into()];
        assert_eq!(t.to_string(), "(Point 0 0 -11 14)");
        let _ = t.push_empty();
        assert_eq!(t.to_string(), "(Point 0 0 -11 14 ? ?)");
        assert_eq!(t.ttype(), "Point");
        assert!(t.comment().is_empty());
        assert_eq!(t.len(), 3);
        assert_eq!(t.ttype_len(), 2);
        assert!(!t.is_empty());
        // TODO lots more tests with different scalar types
        // TODO lots more tests with get, get_mut, iter, inner, inner_mut
    }

    #[test]
    fn t_table_nested() {
        // TODO nested table of tables of lists of maps of tables etc
    }

    #[test]
    fn t_table_err() {
        // TODO adding to fieldless (should get error)
        // TODO adding row of wrong size (should get error)
    }
}
