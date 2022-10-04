// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

mod utils;

use std::{cell::RefCell, rc::Rc};
use utils::{assert_event, check_error};
use uxf::event::{Event, EventKind};
use uxf::field::make_fields;
use uxf::list::List;
use uxf::map::Map;
use uxf::table::Table;
use uxf::tclass::TClass;
use uxf::value::Value;
use uxf::Uxf;

#[test]
fn t_uxf_strings() {
    let mut uxo = Uxf::default();
    assert_eq!(uxo.to_string(), "uxf 1\n[]\n");
    uxo.set_custom("Geo 1.0.0");
    uxo.set_comment("A Geographical format");
    assert_eq!(
        uxo.to_string(),
        "uxf 1 Geo 1.0.0\n#<A Geographical format>\n[]\n"
    );
    let mut uxo = Uxf::new("custom 1", "comment 1");
    assert_eq!(
        format!("{uxo:?}"),
        "Uxf { custom: \"custom 1\", comment: \"comment 1\", value: \
        List(List { vtype: \"\", comment: \"\", values: [] }) }"
    );
    assert_eq!(uxo.to_string(), "uxf 1 custom 1\n#<comment 1>\n[]\n");
    uxo.set_comment("New text");
    assert_eq!(
        format!("{uxo:?}"),
        "Uxf { custom: \"custom 1\", comment: \"New text\", value: \
        List(List { vtype: \"\", comment: \"\", values: [] }) }"
    );
    assert_eq!(uxo.to_string(), "uxf 1 custom 1\n#<New text>\n[]\n");
    uxo.set_custom("Dummy format");
    assert_eq!(
        format!("{uxo:?}"),
        "Uxf { custom: \"Dummy format\", comment: \"New text\", value: \
        List(List { vtype: \"\", comment: \"\", values: [] }) }"
    );
    assert_eq!(uxo.to_string(), "uxf 1 Dummy format\n#<New text>\n[]\n");
}

#[test]
fn t_uxf_set_value() {
    let tc1 = TClass::new_fieldless("ReadyState", "enum").unwrap();
    let t1 = Table::new(tc1, "");
    let t2 = Table::new_fieldless("WaitState", "enum").unwrap();
    let fields = make_fields(&[("x", "int"), ("y", "int")]).unwrap();
    let tc3 = TClass::new("Point", fields, "").unwrap();
    let mut t3 = Table::new(tc3, "");
    let _ = t3.append(vec![Value::Int(0), 0.into()]);
    let _ = t3.append(vec![Value::Int(-7), 11.into()]);
    let _ = t3.append(t3.tclass().record_of_nulls().unwrap());
    let _ = t3.append(vec![Value::Int(19), (-23).into()]);
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
    assert_eq!(uxo.to_string(), "uxf 1\n[]\n");
    assert!(uxo.set_value(lst.into()).is_ok());
    assert_eq!(
        uxo.to_string(),
        "uxf 1\n\
        =Point x:int y:int\n\
        =#<enum> ReadyState\n\
        =WaitState\n\
        [(ReadyState)\n(#<enum> WaitState)\n(Point 0 0\n-7 11\n? ?\n\
        19 -23)]\n"
    );
}

#[test]
fn t_uxf_value_mut() {
    let mut uxo = Uxf::default();
    assert_eq!(uxo.to_string(), "uxf 1\n[]\n");
    let tc1 = TClass::new_fieldless("ReadyState", "enum").unwrap();
    uxo.add_tclass(tc1.clone());
    let t1 = Table::new(tc1, "");
    uxo.add_tclass(t1.tclass().clone());
    let t2 = Table::new_fieldless("WaitState", "enum").unwrap();
    uxo.add_tclass(t2.tclass().clone());
    let fields = make_fields(&[("x", "int"), ("y", "int")]).unwrap();
    let tc3 = TClass::new("Point", fields, "").unwrap();
    uxo.add_tclass(tc3.clone());
    let mut t3 = Table::new(tc3, "");
    let _ = t3.append(vec![Value::Int(0), 0.into()]);
    let _ = t3.append(vec![Value::Int(-7), 11.into()]);
    let _ = t3.append(t3.tclass().record_of_nulls().unwrap());
    let _ = t3.append(vec![Value::Int(19), (-23).into()]);
    let value = uxo.value_mut();
    if let Some(lst) = value.as_list_mut() {
        lst.push(t1.into());
        lst.push(t2.into());
        lst.push(t3.into());
    }
    assert_eq!(
        value.to_string(),
        "[(ReadyState)\n(#<enum> WaitState)\n(Point 0 0\n-7 11\n? ?\n\
        19 -23)]"
    );
    assert_eq!(
        uxo.to_string(),
        "uxf 1\n\
        =Point x:int y:int\n\
        =#<enum> ReadyState\n\
        =WaitState\n\
        [(ReadyState)\n(#<enum> WaitState)\n(Point 0 0\n-7 11\n? ?\n\
        19 -23)]\n"
    );
}

#[test]
fn t_uxf_set_value_invalid() {
    let mut uxo = Uxf::default();
    assert_eq!(uxo.to_string(), "uxf 1\n[]\n");
    let err = uxo.set_value(0.into()).unwrap_err();
    assert_eq!(
        err.to_string(),
        "E100:-:0:Uxf value must be a List, Map, or Table, got int",
    );
}

#[test]
fn t_uxf_1() {
    let mut uxo = Uxf::new("MyUXF", "A comment");
    assert_eq!(uxo.to_string(), "uxf 1 MyUXF\n#<A comment>\n[]\n");
    let mut m = Map::default();
    m.insert(1.into(), "one".into());
    m.insert(2.into(), "two".into());
    assert_eq!(m.to_string(), "{1 <one>\n2 <two>}");
    assert!(uxo.set_value(m.into()).is_ok());
    assert_eq!(
        uxo.to_string(),
        "uxf 1 MyUXF\n#<A comment>\n{1 <one>\n2 <two>}\n"
    );
}

#[test]
fn t_uxf_2() {
    let mut uxo = Uxf::default();
    assert_eq!(uxo.to_string(), "uxf 1\n[]\n");
    uxo.set_custom("MyUXF");
    uxo.set_comment("A comment");
    assert_eq!(uxo.to_string(), "uxf 1 MyUXF\n#<A comment>\n[]\n");
    let mut m = Map::default();
    m.insert(1.into(), "one".into());
    m.insert(2.into(), "two".into());
    assert_eq!(m.to_string(), "{1 <one>\n2 <two>}");
    assert!(uxo.set_value(m.into()).is_ok());
    assert_eq!(
        uxo.to_string(),
        "uxf 1 MyUXF\n#<A comment>\n{1 <one>\n2 <two>}\n"
    );
}

#[test]
fn t_uxf_parse_os2() {
    let events = Rc::new(RefCell::new(Vec::<Event>::new()));
    assert!(&events.borrow().is_empty());
    let err = uxf::parse_options(
        "uxf 1", // invalid since no data: interpreted as filename!
        uxf::ParserOptions::default(),
        Some(Rc::new({
            let events = Rc::clone(&events);
            move |event| {
                let mut events = events.borrow_mut();
                events.push(event.clone());
            }
        })),
    )
    .unwrap_err();
    assert!(err.to_string().contains("(os error 2)"));
    assert!(&events.borrow().is_empty());
}

#[test]
fn t_uxf_parse120() {
    let err = uxf::parse("uxf1.0\n").unwrap_err(); // invalid
    check_error(&err.to_string(), 120, "");
}

#[test]
fn t_uxf_parse130() {
    let err = uxf::parse("Uxf 1\n").unwrap_err(); // invalid
    check_error(&err.to_string(), 130, "");
}

#[test]
fn t_uxf_parse141() {
    let events = Rc::new(RefCell::new(Vec::<Event>::new()));
    assert!(&events.borrow().is_empty());
    let _uxo = uxf::parse_options(
        "uxf 99\n[]",
        uxf::ParserOptions::default(),
        Some(Rc::new({
            let events = Rc::clone(&events);
            move |event| {
                let mut events = events.borrow_mut();
                events.push(event.clone());
            }
        })),
    )
    .unwrap();
    assert!(!&events.borrow().is_empty());
    assert_eq!(*&events.borrow().len(), 1);
    let event = &events.borrow()[0].clone();
    assert_event(
        &event,
        EventKind::Warning,
        141,
        "-",
        1,
        "version 99 > current 1",
    );
}

#[test]
fn t_uxf_parse151a() {
    let err = uxf::parse("uxf 1x\n").unwrap_err(); // invalid version
    check_error(&err.to_string(), 151, "");
}

#[test]
fn t_uxf_parse151b() {
    let err = uxf::parse("uxf 1.0\n").unwrap_err(); // invalid version
    check_error(&err.to_string(), 151, "");
}

#[test]
fn t_uxf_parse160() {
    let err = uxf::parse("uxf 1\n#comment\n").unwrap_err();
    check_error(&err.to_string(), 160, "c");
}

#[test]
fn t_uxf_parse_ok() {
    let uxo = uxf::parse("uxf 1\n[]").unwrap();
    assert_eq!(uxo.to_string(), "uxf 1\n[]\n");
    let uxo = uxf::parse("uxf 1 My <Custom> Format 5.8\n[]").unwrap();
    assert_eq!(uxo.to_string(), "uxf 1 My <Custom> Format 5.8\n[]\n");
    assert_eq!(uxo.custom(), "My <Custom> Format 5.8");
    let uxo = uxf::parse("uxf 1\n#<A &lt;Big&gt; comment!>\n[]").unwrap();
    assert_eq!(uxo.to_string(), "uxf 1\n#<A &lt;Big&gt; comment!>\n[]\n");
    assert_eq!(uxo.comment(), "A <Big> comment!");
    let uxo = uxf::parse("uxf 1\n{}").unwrap();
    assert_eq!(uxo.to_string(), "uxf 1\n{}\n");
    let uxo = uxf::parse("uxf 1\n[{} {} ? []]").unwrap();
    assert_eq!(uxo.to_string(), "uxf 1\n[{}\n{}\n?\n[]]\n");
    let uxo = uxf::parse("uxf 1\n[[? ?] {} ? [[] [] ?]]").unwrap();
    assert_eq!(uxo.to_string(), "uxf 1\n[[?\n?]\n{}\n?\n[[]\n[]\n?]]\n");
    let uxo = uxf::parse("uxf 1\n=Point x y\n[]").unwrap();
    assert_eq!(uxo.to_string(), "uxf 1\n=Point x y\n[]\n");
    let uxo = uxf::parse("uxf 1\n=Point x:real y:real\n[]").unwrap();
    assert_eq!(uxo.to_string(), "uxf 1\n=Point x:real y:real\n[]\n");
    let uxo = uxf::parse("uxf 1\n=Enum\n=Point x:real y:real\n[]").unwrap();
    assert_eq!(uxo.to_string(), "uxf 1\n=Enum\n=Point x:real y:real\n[]\n");
    let uxo = uxf::parse("uxf 1\n=Point x y\n(Point)").unwrap();
    assert_eq!(uxo.to_string(), "uxf 1\n=Point x y\n(Point)\n");
    let uxo = uxf::parse(
        "uxf 1\n=Pair first second\n[(Pair <A &amp; B> <C &lt; D>)]",
    )
    .unwrap();
    assert_eq!(
        uxo.to_string(),
        "uxf 1\n=Pair first second\n[(Pair <A &amp; B> <C &lt; D>)]\n"
    );
    let uxo = uxf::parse("uxf 1\n=Point x y\n[(Point 1 2)]").unwrap();
    assert_eq!(uxo.to_string(), "uxf 1\n=Point x y\n[(Point 1 2)]\n");
    // TODO
}

#[test]
fn t_no_such_file() {
    let err = uxf::parse("no-such-file.uxf").unwrap_err();
    assert!(err.to_string().contains("(os error 2)"));
}

#[test]
fn t_not_a_uxf_file() {
    let err = uxf::parse("UXF 1\n[]").unwrap_err();
    assert_eq!(err.to_string(), "E130:-:1:not a UXF file");
}

/*
#[test]
fn t_uxf_parse_options_ok() {
    // TODO
}
*/
