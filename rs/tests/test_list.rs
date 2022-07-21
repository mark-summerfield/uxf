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
        l1.push(None); // null value
        l1.push(Some(Value::Bool(false)));
        l1.push(Some(Value::Int(7831)));
        l1.push(None); // null value
        l1.push(Some(Value::Str("giraffe neck".to_string())));
        l1.push(Some(Value::Real(-2.11e4)));
        assert!(!l1.is_empty());
        assert_eq!(l1.len(), 11);
        assert_eq!(l1.get(0).unwrap().as_bool().unwrap(), true);
        assert_eq!(l1.get(1).unwrap().as_int().unwrap(), -919);
        assert!(l1.get(2).is_none());
        assert_eq!(l1.get(3).unwrap().as_str().unwrap(), "elephant ears");
        assert!(isclose64(l1.get(4).unwrap().as_real().unwrap(), 1.73e-5));
        assert_eq!(l1[0].as_ref().unwrap().as_bool().unwrap(), true);
        assert_eq!(l1[1].as_ref().unwrap().as_int().unwrap(), -919);
        assert!(l1[2].as_ref().is_none());
        assert_eq!(
            l1[3].as_ref().unwrap().as_str().unwrap(),
            "elephant ears"
        );
        assert!(isclose64(
            l1[4].as_ref().unwrap().as_real().unwrap(),
            1.73e-5
        ));
        *l1.get_mut(0) = Some(Value::Int(7070));
        assert_eq!(l1.get(0).unwrap().as_int().unwrap(), 7070);
        *l1.get_mut(1) = Some(Value::Bool(false));
        assert_eq!(l1.get(1).unwrap().as_bool().unwrap(), false);
        *l1.get_mut(1) = None;
        assert!(l1.get(1).is_none());
        // TODO try setting a string & a real
        // TODO nonempty vtype & comment
        // TODO [] accessor for set
        // TODO typecheck
    }

    fn valid_row() -> Row {
        let mut row = Row::new();
        row.push(Some(Value::Bool(true)));
        row.push(Some(Value::Int(-919)));
        row.push(None); // null value
        row.push(Some(Value::Str("elephant ears".to_string())));
        row.push(Some(Value::Real(1.73e-5)));
        row
    }
}
