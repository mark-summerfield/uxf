// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
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
        assert_eq!(m.to_string(), "{(:555846:) 1854 99 ?}");
        assert_eq!(m.len(), 2);
        m.insert(
            NaiveDate::from_ymd(2022, 7, 26).into(),
            "don't <blink> & see!".into(),
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
        m.insert(99.into(), Value::Null);
        m.insert(5.into(), "Five".into());
        m.insert(101.into(), Value::Null);
        m.insert(100.into(), Value::Null);
        m.insert((-17).into(), Value::Null);
        m.insert(152.into(), 18.into());
        assert_eq!(
            m.to_string(),
            "{#<Int keys> int -17 ? 5 <Five> 99 ? 100 ? 101 ? 152 18}"
        );
    }

    #[test]
    fn t_map_strings() {
        let mut m = Map::default();
        assert_eq!(m.to_string(), "{}");
        m.insert(5.into(), Value::Null);
        m.insert(3.into(), (-3).into());
        m.insert(1.into(), (-1).into());
        assert_eq!(m.to_string(), "{1 -1 3 -3 5 ?}");
        let mut m = Map::new("", "", "a comment").unwrap();
        assert_eq!(m.to_string(), "{#<a comment>}");
        m.insert(5.into(), Value::Null);
        m.insert(3.into(), (-3).into());
        m.insert(1.into(), (-1).into());
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
            5.into(),
            NaiveDate::from_ymd(2022, 7, 16).into(),
        );
        m.insert(
            3.into(),
            NaiveDate::from_ymd(2023, 5, 30).into(),
        );
        m.insert(1.into(), NaiveDate::from_ymd(2024, 8, 1).into());
        assert_eq!(m.to_string(),
        "{#<int x date> int date 1 2024-08-01 3 2023-05-30 5 2022-07-16}");
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
            "{int str -4 ? 19 5.0 23 [real 8.0 0.7 ? -3.21 22] \
        97 <&amp;closed&amp;> 917 <&lt;open&gt;>}"
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
            "{4 ? 5 <five> 6 <six> 7 <seven> 8 <eight> 9 ?}"
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
            "{4 ? 5 <five> 6 <six> 7 <seven> 8 <VIII> 9 ?}"
        );
        let v = m.remove(&4.into());
        assert!(v.unwrap().is_null());
        assert_eq!(
            m.to_string(),
            "{5 <five> 6 <six> 7 <seven> 8 <VIII> 9 ?}"
        );
        let v = m.remove(&7.into());
        assert_eq!(v.unwrap().as_str().unwrap(), "seven");
        assert_eq!(m.to_string(), "{5 <five> 6 <six> 8 <VIII> 9 ?}");
        assert_eq!(m.len(), 4);
        assert!(!m.is_empty());
        m.clear();
        assert_eq!(m.len(), 0);
        assert!(m.is_empty());
        assert_eq!(m.to_string(), "{}");
    }

    #[test]
    fn t_map_inner() {
        // TODO inner() inner_mut()
    }

    #[test]
    fn t_map_nested() {
        // TODO nested maps & lists
    }
}
