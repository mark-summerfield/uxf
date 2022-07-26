// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use uxf::list::List;
    use uxf::map::Map;
    use uxf::value::{Key, Value};

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
        m.insert(Key::Int(99), Value::Null);
        assert_eq!(m.to_string(), "{99 ?}");
        assert!(!m.is_empty());
        assert_eq!(m.len(), 1);
        m.insert(Key::Bytes(vec![0x55, 0x58, 0x46]), Value::Int(1854));
        assert_eq!(m.to_string(), "{(:555846:) 1854 99 ?}");
        assert_eq!(m.len(), 2);
        m.insert(
            Key::Date(NaiveDate::from_ymd(2022, 7, 26)),
            Value::Str("don't <blink> & see!".to_string()),
        );
        assert_eq!(
            m.to_string(),
            "{(:555846:) 1854 2022-07-26 <don't &lt;blink&gt; &amp; see!> \
        99 ?}"
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
        m.insert(Key::Int(99), Value::Null);
        m.insert(Key::Int(5), Value::Str("Five".to_string()));
        m.insert(Key::Int(101), Value::Null);
        m.insert(Key::Int(100), Value::Null);
        m.insert(Key::Int(-17), Value::Null);
        m.insert(Key::Int(152), Value::Int(18));
        assert_eq!(
            m.to_string(),
            "{#<Int keys> int -17 ? 5 <Five> 99 ? 100 ? 101 ? 152 18}"
        );
    }

    #[test]
    fn t_map_strings() {
        let mut m = Map::default();
        assert_eq!(m.to_string(), "{}");
        m.insert(Key::Int(5), Value::Null);
        m.insert(Key::Int(3), Value::Int(-3));
        m.insert(Key::Int(1), Value::Int(-1));
        assert_eq!(m.to_string(), "{1 -1 3 -3 5 ?}");
        let mut m = Map::new("", "", "a comment").unwrap();
        assert_eq!(m.to_string(), "{#<a comment>}");
        m.insert(Key::Int(5), Value::Null);
        m.insert(Key::Int(3), Value::Int(-3));
        m.insert(Key::Int(1), Value::Int(-1));
        assert_eq!(m.to_string(), "{#<a comment> 1 -1 3 -3 5 ?}");
        let m = Map::new("", "str", "str values");
        assert!(m.is_err());
        let mut m = Map::new("int", "", "int keys").unwrap();
        assert_eq!(m.to_string(), "{#<int keys> int}");
        m.insert(Key::Int(5), Value::Null);
        m.insert(Key::Int(3), Value::Int(-3));
        m.insert(Key::Int(1), Value::Int(-1));
        assert_eq!(m.to_string(), "{#<int keys> int 1 -1 3 -3 5 ?}");
        let mut m = Map::new("int", "date", "int x date").unwrap();
        assert_eq!(m.to_string(), "{#<int x date> int date}");
        m.insert(
            Key::Int(5),
            Value::Date(NaiveDate::from_ymd(2022, 7, 16)),
        );
        m.insert(
            Key::Int(3),
            Value::Date(NaiveDate::from_ymd(2023, 5, 30)),
        );
        m.insert(Key::Int(1), Value::Date(NaiveDate::from_ymd(2024, 8, 1)));
        assert_eq!(m.to_string(),
        "{#<int x date> int date 1 2024-08-01 3 2023-05-30 5 2022-07-16}");
    }

    #[test]
    fn t_map_typed() {
        let mut m = Map::new("int", "str", "").unwrap();
        assert_eq!(m.to_string(), "{int str}");
        m.insert(Key::Int(917), Value::Str("<open>".to_string()));
        m.insert(Key::Int(97), Value::Str("&closed&".to_string()));
        m.insert(Key::Int(-4), Value::Null);
        m.insert(Key::Int(19), Value::Real(5e0));
        let mut lst = List::new("real", "").unwrap();
        lst.push(Value::Real(8e0));
        lst.push(Value::Real(0.7));
        lst.push(Value::Null);
        lst.push(Value::Real(-3.21));
        lst.push(Value::Int(22));
        m.insert(Key::Int(23), Value::List(lst));
        assert_eq!(m.to_string(),
        "{int str -4 ? 19 5.0 23 [real 8.0 0.7 ? -3.21 22] \
        97 <&amp;closed&amp;> 917 <&lt;open&gt;>}");
    }

    // TODO inner() inner_mut() get() get_mut() remove() clear()
    // nested maps & lists
}
