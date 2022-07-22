// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use uxf::table::Table;
    use uxf::tclass::TClass;
    use uxf::value::Value;

    #[test]
    fn t_table() {
        let tclass = TClass::new_fieldless("Point", "").unwrap();
        let t = Table::new(tclass, "");
        let v = Value::Table(t);
        assert_eq!(v.to_string(), "Table ttype:Point comment: records:[]");
        // TODO lots more tests
    }
}
