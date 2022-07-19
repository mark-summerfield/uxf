// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use uxf::table::Table;
    use uxf::tclass::TClass;
    use uxf::test_utils::value_to_str;
    use uxf::value::Value;

    #[test]
    fn t_table() {
        let tclass = TClass::new_fieldless("Point", "").unwrap();
        let t = Table::new(tclass, "");
        let v = Value::Table(t);
        assert_eq!(
            value_to_str(v),
            "Table { tclass: TClass { ttype: \"Point\", fields: [], \
            comment: \"\" }, comment: \"\", records: [] }"
        );
        // TODO lots more tests
    }
}
