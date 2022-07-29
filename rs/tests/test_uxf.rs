// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use uxf::event::{Event, EventKind};
    use uxf::field::make_fields;
    use uxf::list::List;
    use uxf::table::Table;
    use uxf::tclass::TClass;
    use uxf::value::Value;
    use uxf::Uxf;

    #[test]
    fn t_uxf_strings() {
        let mut uxo = Uxf::default();
        assert_eq!(uxo.to_string(), "uxf 1.0\n[]\n");
        uxo.set_custom("Geo 1.0.0");
        uxo.set_comment("A Geographical format");
        assert_eq!(
            uxo.to_string(),
            "uxf 1.0 Geo 1.0.0\n#<A Geographical format>\n[]\n"
        );
        let mut uxo = Uxf::new("custom 1", "comment 1", None);
        assert_eq!(
            format!("{:?}", uxo),
            "Uxf { custom: \"custom 1\", comment: \"comment 1\", value: \
        List(List { vtype: \"\", comment: \"\", values: [] }) }"
        );
        assert_eq!(uxo.to_string(), "uxf 1.0 custom 1\n#<comment 1>\n[]\n");
        uxo.set_comment("New text");
        assert_eq!(
            format!("{:?}", uxo),
            "Uxf { custom: \"custom 1\", comment: \"New text\", value: \
        List(List { vtype: \"\", comment: \"\", values: [] }) }"
        );
        assert_eq!(uxo.to_string(), "uxf 1.0 custom 1\n#<New text>\n[]\n");
        uxo.set_custom("Dummy format");
        assert_eq!(format!("{:?}", uxo),
        "Uxf { custom: \"Dummy format\", comment: \"New text\", value: \
        List(List { vtype: \"\", comment: \"\", values: [] }) }");
        assert_eq!(
            uxo.to_string(),
            "uxf 1.0 Dummy format\n#<New text>\n[]\n"
        );
        // TODO
    }

    #[test]
    fn t_uxf_set_value() {
        let tc1 = TClass::new_fieldless("ReadyState", "enum").unwrap();
        let t1 = Table::new(tc1, "");
        let t2 = Table::new_fieldless("WaitState", "enum").unwrap();
        let fields = make_fields(&[("x", "int"), ("y", "int")]).unwrap();
        let tc3 = TClass::new("Point", fields, "").unwrap();
        let mut t3 = Table::new(tc3, "");
        let _ = t3.push(vec![Value::Int(0), 0.into()]);
        let _ = t3.push(vec![Value::Int(-7), 11.into()]);
        let _ = t3.push(t3.tclass().record_of_nulls().unwrap());
        let _ = t3.push(vec![Value::Int(19), (-23).into()]);
        let mut lst = List::default();
        lst.push(t1.into());
        lst.push(t2.into());
        lst.push(t3.into());
        assert_eq!(lst.to_string(), 
        "[(ReadyState) (#<enum> WaitState) (Point 0 0 -7 11 ? ? 19 -23)]");
        let mut uxo = Uxf::default();
        assert_eq!(uxo.to_string(), "uxf 1.0\n[]\n");
        assert!(uxo.set_value(lst.into()).is_ok());
        assert_eq!(
            uxo.to_string(),
            "uxf 1.0\n\
        =Point x:int y:int\n\
        =#<enum> ReadyState\n\
        =WaitState\n\
        [(ReadyState) (#<enum> WaitState) (Point 0 0 -7 11 ? ? 19 -23)]\n"
        );
    }

    #[test]
    fn t_uxf_set_value_invalid() {
        let mut uxo = Uxf::default();
        assert_eq!(uxo.to_string(), "uxf 1.0\n[]\n");
        let event = uxo.set_value(Value::Int(0));
        assert!(event.is_err());
        if let Some(event) = event.unwrap_err().downcast_ref::<Event>() {
            assert_eq!(event.prefix, "uxf");
            assert_eq!(event.kind, EventKind::Fatal);
            assert_eq!(event.code, 100);
            assert_eq!(event.filename, "-");
            assert_eq!(event.lino, 0);
            assert_eq!(
                event.message,
                "Uxf value must be a List, Map, or Table, got int"
            );
        }
        // TODO as above with custom on_event handler
    }
}
