// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use uxf::list::List;
    use uxf::util::isclose64;
    use uxf::value::{Row, Value};

    #[test]
    fn t_list1() {
        let mut lst = List::default();
        assert!(lst.vtype().is_empty());
        assert!(lst.comment().is_empty());
        assert!(lst.is_empty());
        assert_eq!(lst.len(), 0);
        for value in valid_row() {
            lst.push(value);
        }
        lst.push(Value::Null);
        lst.push(Value::Bool(false));
        lst.push(Value::Int(7831));
        lst.push(Value::Null);
        lst.push(Value::Str("giraffe neck".to_string()));
        lst.push(Value::Real(-2.11e4));
        assert!(!lst.is_empty());
        assert_eq!(lst.len(), 11);
        assert_eq!(lst.get_unchecked(0).as_bool().unwrap(), true);
        assert_eq!(lst.get_unchecked(1).as_int().unwrap(), -919);
        assert!(lst[2].is_null());
        assert_eq!(lst[3].as_str().unwrap(), "elephant ears");
        assert!(isclose64(lst[4].as_real().unwrap(), 1.73e-5));
        assert_eq!(lst[0].as_bool().unwrap(), true);
        assert_eq!(lst[1].as_int().unwrap(), -919);
        assert!(lst[2].is_null());
        assert_eq!(
            lst[3].as_str().unwrap(),
            "elephant ears"
        );
        assert!(isclose64(
            lst[4].as_real().unwrap(),
            1.73e-5
        ));
        *lst.get_unchecked_mut(0) = Value::Int(7070);
        assert_eq!(lst.get_unchecked(0).as_int().unwrap(), 7070);
        *lst.get_unchecked_mut(1) = Value::Bool(false);
        assert_eq!(lst.get_unchecked(1).as_bool().unwrap(), false);
        lst[1] = Value::Null;
        assert!(lst.get_unchecked(1).is_null());
        assert!(lst[2].is_null());
        lst.push(Value::Null);
        let i = lst.len() - 1;
        lst[i] = Value::Str("dog tail".to_string());
        assert_eq!(lst[i].as_str().unwrap(), "dog tail");
        lst.push(Value::Null);
        let i = lst.len() - 1;
        assert!(lst[i].is_null());
        lst[i] = Value::Real(-9.4);
        assert!(isclose64(lst[i].as_real().unwrap(), -9.4));
        lst[i] = Value::Int(4);
        assert_eq!(lst[i].as_int().unwrap(), 4);
    }

    #[test]
    fn t_list2() {
        let mut lst = List::new("int", "Test of int").unwrap();
        lst.push(Value::Null);
        lst.push(Value::Int(5));
        lst.push(Value::Int(17));
        lst.push(Value::Null);
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
            }
            else if i == 1 {
                assert_eq!(value.as_int().unwrap(), 5);
            }
            else if i == 2 {
                assert_eq!(value.as_int().unwrap(), 17);
            }
        }
        assert!(lst[0].is_null());
        assert!(lst[3].is_null());
        for (i, value) in lst.iter_mut().enumerate() {
            if value.is_null() {
                *value = Value::Int(100 * (i as i64 + 1));
            }
        }
        assert_eq!(lst[0].as_int().unwrap(), 100);
        assert_eq!(lst[3].as_int().unwrap(), 400);
        // TODO .truncate() .clear()
    }

    #[test]
    fn t_list_err() {
        assert!(List::new("$1", "").is_err());
        // TODO a few more err tests, checking specific codes & to show
        // downcast_ref in practice
    }

    fn valid_row() -> Row {
        let mut row = Row::new();
        row.push(Value::Bool(true));
        row.push(Value::Int(-919));
        row.push(Value::Null);
        row.push(Value::Str("elephant ears".to_string()));
        row.push(Value::Real(1.73e-5));
        row
    }
}
