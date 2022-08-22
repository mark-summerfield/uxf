// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};
    use uxf::event::Event;
    use uxf::field::make_fields;
    use uxf::list::List;
    use uxf::map::Map;
    use uxf::table::Table;
    use uxf::tclass::TClass;
    use uxf::test_utils::assert_fatal;
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
        assert_eq!(
            lst.to_string(),
            "[(ReadyState)\n(#<enum> WaitState)\n(Point 0 0\n-7 11\n? ?\n\
        19 -23)]"
        );
        let mut uxo = Uxf::default();
        assert_eq!(uxo.to_string(), "uxf 1.0\n[]\n");
        assert!(uxo.set_value(lst.into()).is_ok());
        assert_eq!(
            uxo.to_string(),
            "uxf 1.0\n\
        =Point x:int y:int\n\
        =#<enum> ReadyState\n\
        =WaitState\n\
        [(ReadyState)\n(#<enum> WaitState)\n(Point 0 0\n-7 11\n? ?\n\
        19 -23)]\n"
        );
    }

    #[test]
    fn t_uxf_set_value_invalid() {
        // using default on_event() handler
        let mut uxo = Uxf::default();
        assert_eq!(uxo.to_string(), "uxf 1.0\n[]\n");
        let event = uxo.set_value(0.into());
        assert!(event.is_err());
        if let Some(event) = event.unwrap_err().downcast_ref::<Event>() {
            assert_fatal(
                &event,
                100,
                "Uxf value must be a List, Map, or Table, got int",
            );
        }
    }

    #[test]
    fn t_uxf_on_event1() {
        // using custom on_event() handler that accumulates events
        let events = Rc::new(RefCell::new(Vec::<Event>::new()));
        assert!(&events.borrow().is_empty());
        let mut uxo = Uxf::new(
            "MyUXF",
            "No comment",
            Some(Rc::new({
                let events = Rc::clone(&events);
                move |event| {
                    let mut events = events.borrow_mut();
                    events.push(event.clone());
                    Ok(())
                }
            })),
        );
        assert_eq!(uxo.to_string(), "uxf 1.0 MyUXF\n#<No comment>\n[]\n");
        let mut m = Map::default();
        m.insert(1.into(), "one".into());
        m.insert(2.into(), "two".into());
        assert_eq!(m.to_string(), "{1 <one>\n2 <two>}");
        assert!(uxo.set_value(m.into()).is_ok());
        assert_eq!(
            uxo.to_string(),
            "uxf 1.0 MyUXF\n#<No comment>\n{1 <one>\n2 <two>}\n"
        );
        assert!(&events.borrow().is_empty());
        assert_eq!(*&events.borrow().len(), 0);
        assert!(uxo.set_value(1.into()).is_ok());
        assert!(!&events.borrow().is_empty());
        assert_eq!(*&events.borrow().len(), 1);
        let event = &events.borrow()[0].clone();
        assert_fatal(
            &event,
            100,
            "Uxf value must be a List, Map, or Table, got int",
        );
        assert!(uxo.set_value("x".into()).is_ok());
        assert_eq!(*&events.borrow().len(), 2);
        let event = &events.borrow()[1].clone();
        assert_fatal(
            &event,
            100,
            "Uxf value must be a List, Map, or Table, got str",
        );
    }

    #[test]
    fn t_uxf_on_event2() {
        // using custom on_event() handler that accumulates events
        let events = Rc::new(RefCell::new(Vec::<Event>::new()));
        assert!(&events.borrow().is_empty());
        let mut uxo = Uxf::new_on_event(Rc::new({
            let events = Rc::clone(&events);
            move |event| {
                let mut events = events.borrow_mut();
                events.push(event.clone());
                Ok(())
            }
        }));
        assert_eq!(uxo.to_string(), "uxf 1.0\n[]\n");
        uxo.set_custom("MyUXF");
        uxo.set_comment("No comment");
        assert_eq!(uxo.to_string(), "uxf 1.0 MyUXF\n#<No comment>\n[]\n");
        let mut m = Map::default();
        m.insert(1.into(), "one".into());
        m.insert(2.into(), "two".into());
        assert_eq!(m.to_string(), "{1 <one>\n2 <two>}");
        assert!(uxo.set_value(m.into()).is_ok());
        assert_eq!(
            uxo.to_string(),
            "uxf 1.0 MyUXF\n#<No comment>\n{1 <one>\n2 <two>}\n"
        );
        assert!(&events.borrow().is_empty());
        assert_eq!(*&events.borrow().len(), 0);
        assert!(uxo.set_value(1.into()).is_ok());
        assert!(!&events.borrow().is_empty());
        assert_eq!(*&events.borrow().len(), 1);
        let event = &events.borrow()[0].clone();
        assert_fatal(
            &event,
            100,
            "Uxf value must be a List, Map, or Table, got int",
        );
        assert!(uxo.set_value("x".into()).is_ok());
        assert_eq!(*&events.borrow().len(), 2);
        let event = &events.borrow()[1].clone();
        assert_fatal(
            &event,
            100,
            "Uxf value must be a List, Map, or Table, got str",
        );
    }

    #[test]
    fn t_uxf_parse() {
        if let Ok(uxo) = uxf::parse("uxf 1.0\n[]") {
            assert_eq!(uxo.to_string(), "uxf 1.0\n[]\n");
        }
    }

    #[test]
    fn t_uxf_parse_options() {
        // TODO
    }
}
