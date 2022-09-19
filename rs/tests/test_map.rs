// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use chrono::prelude::*;
use std::collections::HashMap;
use uxf::key::Key;
use uxf::list::List;
use uxf::map::Map;
use uxf::value::Value;

#[test]
fn t_map_empty() {
    let m = Map::default();
    assert!(m.ktype().is_empty());
    assert!(m.vtype().is_empty());
    assert!(m.comment().is_empty());
    assert!(m.is_empty());
    assert_eq!(m.len(), 0);
    assert_eq!(m.to_string(), "{}");
}

#[test]
fn t_map_untyped() {
    let mut m = Map::default();
    assert!(m.ktype().is_empty());
    assert!(m.vtype().is_empty());
    assert!(m.comment().is_empty());
    assert!(m.is_empty());
    assert_eq!(m.len(), 0);
    assert_eq!(m.to_string(), "{}");
    m.insert(99.into(), Value::Null);
    assert_eq!(m.to_string(), "{99 ?}");
    assert!(!m.is_empty());
    assert_eq!(m.len(), 1);
    m.insert(vec![0x55, 0x58, 0x46].into(), 1854.into());
    assert_eq!(m.to_string(), "{(:555846:) 1854\n99 ?}");
    assert_eq!(m.len(), 2);
    m.insert(
        NaiveDate::from_ymd(2022, 7, 26).into(),
        "don't <blink> & see!".into(),
    );
    assert_eq!(
        m.to_string(),
        "{(:555846:) 1854\n2022-07-26 <don't &lt;blink&gt; &amp; \
            see!>\n99 ?}"
    );
    assert_eq!(m.len(), 3);
}

#[test]
fn t_map_ktype() {
    let mut m = Map::new("int", "", "Int keys").unwrap();
    assert_eq!(m.ktype(), "int");
    assert!(m.vtype().is_empty());
    assert_eq!(m.comment(), "Int keys");
    assert!(m.is_empty());
    assert_eq!(m.len(), 0);
    assert_eq!(m.to_string(), "{#<Int keys> int}");
    m.insert(99.into(), Value::Null);
    m.insert(5.into(), "Five".into());
    m.insert(101.into(), Value::Null);
    m.insert(100.into(), Value::Null);
    m.insert((-17).into(), Value::Null);
    m.insert(152.into(), 18.into());
    assert_eq!(
        m.to_string(),
        "{#<Int keys> int -17 ?\n5 <Five>\n99 ?\n100 ?\n101 ?\n152 18}"
    );
}

#[test]
fn t_map_strings() {
    let mut m = Map::default();
    assert_eq!(m.to_string(), "{}");
    m.insert(5.into(), Value::Null);
    m.insert(3.into(), (-3).into());
    m.insert(1.into(), (-1).into());
    assert_eq!(m.to_string(), "{1 -1\n3 -3\n5 ?}");
    let mut m = Map::new("", "", "a comment").unwrap();
    assert_eq!(m.to_string(), "{#<a comment>}");
    m.insert(5.into(), Value::Null);
    m.insert(3.into(), (-3).into());
    m.insert(1.into(), (-1).into());
    assert_eq!(m.to_string(), "{#<a comment> 1 -1\n3 -3\n5 ?}");
    let m = Map::new("", "str", "str values");
    assert!(m.is_err());
    let mut m = Map::new("int", "", "int keys").unwrap();
    assert_eq!(m.to_string(), "{#<int keys> int}");
    m.insert(Key::Int(5), Value::Null);
    m.insert(Key::Int(3), Value::Int(-3));
    m.insert(Key::Int(1), Value::Int(-1));
    assert_eq!(m.to_string(), "{#<int keys> int 1 -1\n3 -3\n5 ?}");
    let mut m = Map::new("int", "date", "int x date").unwrap();
    assert_eq!(m.to_string(), "{#<int x date> int date}");
    m.insert(5.into(), NaiveDate::from_ymd(2022, 7, 16).into());
    m.insert(3.into(), NaiveDate::from_ymd(2023, 5, 30).into());
    m.insert(1.into(), NaiveDate::from_ymd(2024, 8, 1).into());
    assert_eq!(
        m.to_string(),
        "{#<int x date> int date 1 2024-08-01\n3 2023-05-30\n\
        5 2022-07-16}"
    );
}

#[test]
fn t_map_typed() {
    let mut m = Map::new("int", "str", "").unwrap();
    assert_eq!(m.to_string(), "{int str}");
    m.insert(917.into(), "<open>".into());
    m.insert(97.into(), "&closed&".into());
    m.insert((-4).into(), Value::Null);
    m.insert(19.into(), 5e0.into());
    let mut lst = List::new("real", "").unwrap();
    lst.push(8e0.into());
    lst.push((0.7).into());
    lst.push(Value::Null);
    lst.push((-3.21).into());
    lst.push(22.into());
    m.insert(23.into(), lst.into());
    assert_eq!(
        m.to_string(),
        "{int str -4 ?\n19 5.0\n23 [real 8.0\n0.7\n?\n-3.21\n22]\n\
        97 <&amp;closed&amp;>\n917 <&lt;open&gt;>}"
    );
}

#[test]
fn t_map_get_etc() {
    let mut m = Map::default();
    assert_eq!(m.to_string(), "{}");
    m.insert(4.into(), Value::Null);
    assert!(m.get(&Key::Int(8)).is_none());
    for (key, value) in [(8, "eight"), (7, "seven"), (6, "six")] {
        m.insert(key.into(), value.into());
    }
    assert!(m.get(&5.into()).is_none());
    m.insert(5.into(), "five".into());
    assert!(m.get(&9.into()).is_none());
    m.insert(9.into(), Value::Null);
    assert_eq!(
        m.to_string(),
        "{4 ?\n5 <five>\n6 <six>\n7 <seven>\n8 <eight>\n9 ?}"
    );
    let k8 = Key::Int(8);
    assert_eq!(m.get(&k8).unwrap().as_str().unwrap(), "eight");
    if let Some(v) = m.get_mut(&k8) {
        *v = "VIII".into();
    }
    assert_eq!(m.get(&k8).unwrap().as_str().unwrap(), "VIII");
    assert!(m.get(&(-9).into()).is_none());
    assert_eq!(
        m.to_string(),
        "{4 ?\n5 <five>\n6 <six>\n7 <seven>\n8 <VIII>\n9 ?}"
    );
    let v = m.remove(&4.into());
    assert!(v.unwrap().is_null());
    assert_eq!(
        m.to_string(),
        "{5 <five>\n6 <six>\n7 <seven>\n8 <VIII>\n9 ?}"
    );
    let v = m.remove(&7.into());
    assert_eq!(v.unwrap().as_str().unwrap(), "seven");
    assert_eq!(m.to_string(), "{5 <five>\n6 <six>\n8 <VIII>\n9 ?}");
    assert_eq!(m.len(), 4);
    assert!(!m.is_empty());
    m.clear();
    assert_eq!(m.len(), 0);
    assert!(m.is_empty());
    assert_eq!(m.to_string(), "{}");
}

#[test]
fn t_map_inner() {
    let mut m = Map::default();
    assert_eq!(m.to_string(), "{}");
    {
        let items = m.inner_mut();
        for (n, s) in [(1, "I"), (5, "V"), (10, "X"), (50, "L")] {
            items.insert(n.into(), s.into());
        }
    }
    assert_eq!(m.to_string(), "{1 <I>\n5 <V>\n10 <X>\n50 <L>}");
    {
        let mut counts = HashMap::new();
        let items = m.inner();
        for key in items.keys() {
            let counter = counts.entry(key).or_insert(0);
            *counter += 1;
        }
        assert_eq!(counts.len(), items.len());
        for value in counts.values() {
            assert_eq!(*value, 1);
        }
    }
}

#[test]
fn t_map_nested() {
    let mut m = Map::new("str", "", "").unwrap();
    m.insert("alpha".into(), List::new("int", "").unwrap().into());
    assert_eq!(m.to_string(), "{str <alpha> [int]}");
    if let Some(value) = m.get_mut(&"alpha".into()) {
        if let Some(lst) = value.as_list_mut() {
            lst.push(391.into());
            lst.push(9870.into());
            lst.push((-16).into());
        }
    }
    assert_eq!(m.to_string(), "{str <alpha> [int 391\n9870\n-16]}");
    m.insert("bravo".into(), Map::default().into());
    assert_eq!(
        m.to_string(),
        "{str <alpha> [int 391\n9870\n-16]\n<bravo> {}}"
    );
    if let Some(value) = m.get_mut(&"bravo".into()) {
        if let Some(bm) = value.as_map_mut() {
            bm.insert(1.into(), "one".into());
            bm.insert(10.into(), "ten".into());
            bm.insert("charlie".into(), List::default().into());
            bm.insert("delta".into(), Map::default().into());
        }
    }
    assert_eq!(
        m.to_string(),
        "{str <alpha> [int 391\n9870\n-16]\n<bravo> {1 <one>\n\
            10 <ten>\n<charlie> []\n<delta> {}}}"
    );
    if let Some(value) = m.get_mut(&"bravo".into()) {
        if let Some(bm) = value.as_map_mut() {
            if let Some(charlie) = bm.get_mut(&"charlie".into()) {
                if let Some(lst) = charlie.as_list_mut() {
                    lst.push("I".into());
                    lst.push("V".into());
                    lst.push("X".into());
                }
            }
        }
    }
    assert_eq!(
        m.to_string(),
        "{str <alpha> [int 391\n9870\n-16]\n<bravo> {1 <one>\n\
            10 <ten>\n<charlie> [<I>\n<V>\n<X>]\n<delta> {}}}"
    );
    if let Some(value) = m.get_mut(&"bravo".into()) {
        if let Some(bm) = value.as_map_mut() {
            if let Some(delta) = bm.get_mut(&"delta".into()) {
                if let Some(dm) = delta.as_map_mut() {
                    dm.insert("L".into(), 50.into());
                    dm.insert("C".into(), 100.into());
                    dm.insert("D".into(), 500.into());
                    dm.insert("M".into(), 1000.into());
                }
            }
        }
    }
    assert_eq!(
        m.to_string(),
        "{str <alpha> [int 391\n9870\n-16]\n<bravo> {1 <one>\n\
            10 <ten>\n<charlie> [<I>\n<V>\n<X>]\n<delta> {<C> 100\n\
            <D> 500\n<L> 50\n<M> 1000}}}"
    );
}

#[test]
fn t_map_ordering() {
    let mut m = Map::default();
    assert_eq!(m.to_string(), "{}");
    {
        let items = m.inner_mut();
        for (n, s) in [
            (5, "V"),
            (100, "C"),
            (500, "D"),
            (1000, "M"),
            (10, "X"),
            (50, "L"),
            (1, "I"),
        ] {
            items.insert(n.into(), s.clone().into());
            items.insert(s.into(), n.into());
        }
    }
    assert_eq!(
        m.to_string(),
        "{1 <I>\n5 <V>\n10 <X>\n50 <L>\n100 <C>\n500 <D>\n1000 <M>\n\
          <C> 100\n<D> 500\n<I> 1\n<L> 50\n<M> 1000\n<V> 5\n<X> 10}"
    );
    m.clear();
    {
        let items = m.inner_mut();
        for (a, b) in [("Zone", 3), ("zed", 2), ("art", 0), ("Cane", 1)] {
            items.insert(a.into(), b.into());
        }
        for (a, b) in [
            (NaiveDate::from_ymd(2022, 10, 22).and_hms(1, 0, 0), 2),
            (NaiveDate::from_ymd(2022, 10, 22).and_hms(0, 2, 2), 1),
            (NaiveDate::from_ymd(2022, 10, 22).and_hms(0, 1, 2), 0),
        ] {
            items.insert(a.into(), b.into());
        }
        for (a, b) in [(20, 3), (15, 2), (-7, 0), (0, 1), (381, 4)] {
            items.insert(a.into(), b.into());
        }
        for (a, b) in [
            (NaiveDate::from_ymd(2024, 1, 17), 2),
            (NaiveDate::from_ymd(1999, 11, 8), 1),
            (NaiveDate::from_ymd(1901, 12, 24), 0),
        ] {
            items.insert(a.into(), b.into());
        }
        for (a, b) in [("mn", 3), ("DE", 1), ("ab", 2), ("Cd", 0)] {
            items.insert(a.as_bytes().into(), b.into());
        }
    }
    assert_eq!(
        m.to_string(),
        "{(:4364:) 0\n(:4445:) 1\n(:6162:) 2\n(:6D6E:) 3\n\
        1901-12-24 0\n1999-11-08 1\n2024-01-17 2\n\
        2022-10-22T00:01:02 0\n2022-10-22T00:02:02 1\n\
        2022-10-22T01:00:00 2\n-7 0\n0 1\n15 2\n20 3\n381 4\n\
        <art> 0\n<Cane> 1\n<zed> 2\n<Zone> 3}"
    );
}

/*
#[test]
fn t_map_err() {
    // TODO
}
*/
