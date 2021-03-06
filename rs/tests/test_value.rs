// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#[cfg(test)]
mod tests {
    use uxf::value::Value;

    #[test]
    fn t_single_value() {
        let n = Value::Null;
        assert_eq!(n.to_string(), "?");
        assert_eq!(n.typename(), "null");
        let b = Value::Bool(true);
        assert_eq!(b.to_string(), "yes");
        assert_eq!(b.typename(), "bool");
        let b = Value::Bool(false);
        assert_eq!(b.to_string(), "no");
        assert_eq!(b.typename(), "bool");
        let i = Value::Int(987123);
        assert_eq!(i.to_string(), "987123");
        assert_eq!(i.typename(), "int");
        // TODO lots more tests
    }
}
