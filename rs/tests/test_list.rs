// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use uxf::list::List;
    use uxf::util::isclose64;
    use uxf::value::{Row, Value};

    #[test]
    fn t_list() {
        let mut l1 = List::default();
        assert!(l1.vtype().is_empty());
        assert!(l1.comment().is_empty());
        assert!(l1.is_empty());
        assert_eq!(l1.len(), 0);
        for value in valid_row() {
            l1.push(value);
        }
        l1.push(Value::Null);
        l1.push(Value::Bool(false));
        l1.push(Value::Int(7831));
        l1.push(Value::Null);
        l1.push(Value::Str("giraffe neck".to_string()));
        l1.push(Value::Real(-2.11e4));
        assert!(!l1.is_empty());
        assert_eq!(l1.len(), 11);
        assert_eq!(l1.get_unchecked(0).as_bool().unwrap(), true);
        assert_eq!(l1.get_unchecked(1).as_int().unwrap(), -919);
        assert!(l1.get_unchecked(2).is_null());
        assert_eq!(l1.get_unchecked(3).as_str().unwrap(), "elephant ears");
        assert!(isclose64(l1.get_unchecked(4).as_real().unwrap(), 1.73e-5));
        assert_eq!(l1[0].as_bool().unwrap(), true);
        assert_eq!(l1[1].as_int().unwrap(), -919);
        assert!(l1[2].is_null());
        assert_eq!(
            l1[3].as_str().unwrap(),
            "elephant ears"
        );
        assert!(isclose64(
            l1[4].as_real().unwrap(),
            1.73e-5
        ));
        *l1.get_unchecked_mut(0) = Value::Int(7070);
        assert_eq!(l1.get_unchecked(0).as_int().unwrap(), 7070);
        *l1.get_unchecked_mut(1) = Value::Bool(false);
        assert_eq!(l1.get_unchecked(1).as_bool().unwrap(), false);
        *l1.get_unchecked_mut(1) = Value::Null;
        assert!(l1.get_unchecked(1).is_null());
        // TODO try setting a string & a real
        // TODO repeat all above for []
        // TODO repeat all above for checked get() & get_mut()
        // TODO nonempty vtype & comment
        // TODO typecheck
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
