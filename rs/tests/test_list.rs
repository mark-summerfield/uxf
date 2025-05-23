// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

mod utils;

use utils::check_error;
use uxf::list::List;
use uxf::util::isclose64;
use uxf::value::{Value, Values};

#[test]
fn t_list1() {
    let mut lst = List::default();
    assert!(lst.vtype().is_empty());
    assert!(lst.comment().is_empty());
    assert!(lst.is_empty());
    assert_eq!(lst.len(), 0);
    assert_eq!(lst.to_string(), "[]");
    for value in valid_row() {
        lst.push(value);
    }
    assert_eq!(
        lst.to_string(),
        "[yes\n-919\n?\n<elephant &lt;ears&gt;>\n0.0000173]"
    );
    lst.push(Value::Null);
    lst.push(false.into());
    lst.push(7831.into());
    lst.push(Value::Null);
    lst.push("giraffe neck".into());
    lst.push((-2.11e4).into());
    assert_eq!(
        lst.to_string(),
        "[yes\n-919\n?\n<elephant &lt;ears&gt;>\n0.0000173\n?\nno\n\
            7831\n?\n<giraffe neck>\n-21100.0]"
    );
    assert!(!lst.is_empty());
    assert_eq!(lst.len(), 11);
    assert_eq!(lst[0].as_bool().unwrap(), true);
    assert_eq!(lst[1].as_int().unwrap(), -919);
    assert!(lst[2].is_null());
    assert_eq!(lst[3].as_str().unwrap(), "elephant <ears>");
    assert!(isclose64(lst[4].as_real().unwrap(), 1.73e-5));
    assert_eq!(lst[0].as_bool().unwrap(), true);
    assert_eq!(lst[1].as_int().unwrap(), -919);
    assert!(lst[2].is_null());
    assert_eq!(lst[3].as_str().unwrap(), "elephant <ears>");
    assert!(isclose64(lst[4].as_real().unwrap(), 1.73e-5));
    lst[0] = 7070.into();
    assert_eq!(lst[0].as_int().unwrap(), 7070);
    lst[1] = false.into();
    assert_eq!(lst[1].as_bool().unwrap(), false);
    lst[1] = Value::Null;
    assert!(lst[1].is_null());
    assert!(lst[2].is_null());
    lst.push(Value::Null);
    let i = lst.len() - 1;
    lst[i] = "dog & tail".into();
    assert_eq!(lst[i].as_str().unwrap(), "dog & tail");
    lst.push(Value::Null);
    let i = lst.len() - 1;
    assert!(lst[i].is_null());
    lst[i] = (-9.4).into();
    assert!(isclose64(lst[i].as_real().unwrap(), -9.4));
    lst[i] = 4.into();
    assert_eq!(lst[i].as_int().unwrap(), 4);
    assert_eq!(
        lst.to_string(),
        "[7070\n?\n?\n<elephant &lt;ears&gt;>\n0.0000173\n?\nno\n\
            7831\n?\n<giraffe neck>\n-21100.0\n<dog &amp; tail>\n4]"
    );
}

#[test]
fn t_list2() {
    let mut lst = List::new("int", "Test of int").unwrap();
    lst.push_many(&[Value::Null, 5.into(), 17.into(), Value::Null]);
    assert_eq!(lst.to_string(), "[#<Test of int> int ?\n5\n17\n?]");
    assert_eq!(lst.len(), 4);
    assert!(!lst.is_empty());
    assert!(lst[0].is_null());
    assert!(lst.get(0).unwrap().is_null());
    assert_eq!(lst[2].as_int().unwrap(), 17);
    assert_eq!(lst.get(2).unwrap().as_int().unwrap(), 17);
    assert_eq!(lst.vtype(), "int");
    assert_eq!(lst.comment(), "Test of int");
    for (i, value) in lst.iter().enumerate() {
        if i == 0 || i == 3 {
            assert!(value.is_null())
        } else if i == 1 {
            assert_eq!(value.as_int().unwrap(), 5);
        } else if i == 2 {
            assert_eq!(value.as_int().unwrap(), 17);
        }
    }
    assert!(lst[0].is_null());
    assert!(lst[3].is_null());
    for (i, value) in lst.iter_mut().enumerate() {
        if value.is_null() {
            *value = (100 * (i as i64 + 1)).into();
        }
    }
    assert_eq!(lst[0].as_int().unwrap(), 100);
    assert_eq!(lst[3].as_int().unwrap(), 400);
    lst[1] = Value::Int(-11 * lst[1].as_int().unwrap());
    lst[2] = 917.into();
    lst.push(8888.into());
    assert_eq!(lst.len(), 5);
    assert_eq!(
        lst.to_string(),
        "[#<Test of int> int 100\n-55\n917\n400\n8888]"
    );
    lst.truncate(3);
    assert_eq!(lst.to_string(), "[#<Test of int> int 100\n-55\n917]");
    assert_eq!(lst.len(), 3);
    lst.clear();
    assert_eq!(lst.len(), 0);
    assert!(lst.is_empty());
    assert_eq!(lst.to_string(), "[#<Test of int> int]");
}

#[test]
fn t_list3() {
    let mut lst = List::new("int", "").unwrap();
    assert_eq!(lst.to_string(), "[int]");
    {
        let values = lst.inner_mut();
        values.push(Value::Null);
        values.push(5.into());
        values.push(17.into());
        values.push(Value::Null);
        assert_eq!(values.len(), 4);
        assert!(!values.is_empty());
        assert!(values[0].is_null());
        assert!(values.get(0).unwrap().is_null());
        assert_eq!(values[2].as_int().unwrap(), 17);
        assert_eq!(values.get(2).unwrap().as_int().unwrap(), 17);
        for (i, value) in values.iter().enumerate() {
            if i == 0 || i == 3 {
                assert!(value.is_null())
            } else if i == 1 {
                assert_eq!(value.as_int().unwrap(), 5);
            } else if i == 2 {
                assert_eq!(value.as_int().unwrap(), 17);
            }
        }
        assert!(values[0].is_null());
        assert!(values[3].is_null());
        for (i, value) in values.iter_mut().enumerate() {
            if value.is_null() {
                *value = (100 * (i as i64 + 1)).into();
            }
        }
        assert_eq!(values[0].as_int().unwrap(), 100);
        assert_eq!(values[3].as_int().unwrap(), 400);
        values[1] = (-11 * values[1].as_int().unwrap()).into();
        values[2] = 917.into();
        values.push(8888.into());
        assert_eq!(values.len(), 5);
    }
    assert_eq!(lst.to_string(), "[int 100\n-55\n917\n400\n8888]");
    assert_eq!(lst.len(), 5);
    for (index, value) in
        [(0, 100), (1, -55), (2, 917), (3, 400), (4, 8888)]
    {
        assert_eq!(lst[index].as_int().unwrap(), value);
    }
    {
        let values = lst.inner_mut();
        values.truncate(3);
        assert_eq!(values.len(), 3);
        values.clear();
        assert_eq!(values.len(), 0);
        assert!(values.is_empty());
    }
    assert_eq!(lst.len(), 0);
    assert!(lst.is_empty());
    assert_eq!(lst.to_string(), "[int]");
}

#[test]
fn t_list_comment() {
    let mut lst = List::new("", "A <comment> &tc.").unwrap();
    assert_eq!(lst.to_string(), "[#<A &lt;comment&gt; &amp;tc.>]");
    lst.push(Value::Null);
    assert_eq!(lst.to_string(), "[#<A &lt;comment&gt; &amp;tc.> ?]");
    lst.push(Value::Null);
    assert_eq!(lst.to_string(), "[#<A &lt;comment&gt; &amp;tc.> ?\n?]");
}

#[test]
fn t_list_nested() {
    let mut lst = List::default(); // always succeeds
    assert_eq!(lst.to_string(), "[]");
    assert_eq!(lst.len(), 0);
    assert!(lst.is_empty());
    assert!(lst.vtype().is_empty());
    assert!(lst.comment().is_empty());
    lst.push_many(&[Value::Null, Value::Null, Value::Null]); // 0 1 2
    assert_eq!(lst.to_string(), "[?\n?\n?]");
    lst.push(List::default().into()); // 3
    assert_eq!(lst.to_string(), "[?\n?\n?\n[]]");
    if let Some(sublist) = lst[0].as_list() {
        assert_eq!(sublist.len(), 0);
        assert!(sublist.is_empty());
    }
    assert_eq!(lst.len(), 4);
    assert!(!lst.is_empty());
    lst.push(998877.into()); // 4
    assert_eq!(lst.to_string(), "[?\n?\n?\n[]\n998877]");
    if let Some(sublist) = lst[3].as_list_mut() {
        sublist.push("this & that".into());
        sublist.push("is <bold> &tc.!".into());
    }
    if let Some(sublist) = lst[3].as_list() {
        assert_eq!(sublist.len(), 2);
        assert!(!sublist.is_empty());
    }
    assert_eq!(
        lst.to_string(),
        "[?\n?\n?\n[<this &amp; that>\n<is &lt;bold&gt; \
        &amp;tc.!>]\n998877]"
    );
    if let Some(sublist) = lst[3].as_list_mut() {
        sublist.push(List::new("real", "<Totals>").unwrap().into());
        if let Some(subsublist) = sublist[2].as_list_mut() {
            subsublist.push_many(&[
                (7.9).into(),
                (1e2).into(),
                (-19.357).into(),
            ]);
        }
    }
    assert_eq!(
        lst.to_string(),
        "[?\n?\n?\n[<this &amp; that>\n<is &lt;bold&gt; &amp;tc.!>\n\
        [#<&lt;Totals&gt;> real 7.9\n100.0\n-19.357]]\n998877]"
    );
}

#[test]
fn t_list_err() {
    assert!(List::new("$1", "").is_err());
    let err = List::new("-x", "").unwrap_err();
    check_error(&err.to_string(), 300, "-x");
    let err = List::new(&"y".repeat(33), "").unwrap_err();
    check_error(&err.to_string(), 306, "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy");
    let err = List::new("alpha_b=", "").unwrap_err();
    check_error(&err.to_string(), 310, "alpha_b=");
}

#[test]
fn t_list_str() {
    let mut lst = List::new("str", "").unwrap();
    for s in ["one", "two", "<three>", "four & five", "six", "seven"] {
        lst.push(s.into());
    }
    assert_eq!(
        lst.to_string(),
        "[str <one>\n<two>\n<&lt;three&gt;>\n\
            <four &amp; five>\n<six>\n<seven>]"
    );
}

fn valid_row() -> Values {
    let mut values = Values::new();
    // Normally we'd use .into() for all except Null, but just to show
    values.push(Value::Bool(true));
    values.push(Value::Int(-919));
    values.push(Value::Null);
    values.push(Value::from("elephant <ears>"));
    values.push(Value::Real(1.73e-5));
    values
}
