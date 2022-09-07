use anyhow::{bail, Result};
use std::fmt;

const B: i64 = -1; // list begin
const E: i64 = -2; // list end

fn main() -> Result<()> {
    let tokens = vec![
        B, 1, 2, B, 30, 40, B, 500, B, E, E, 70, B, 800, 900, E, 1000, E,
        1100, E,
    ];
    assert_eq!(
        tokens.iter().filter(|&x| *x == B).count(),
        tokens.iter().filter(|&x| *x == E).count()
    );
    let expected = get_expected()?;
    let actual = parse(tokens)?;
    println!("Actual   {}", actual);
    println!("Expected {}", expected);
    assert_eq!(actual.to_string(), expected.to_string());
    Ok(())
}

fn parse(tokens: Vec<i64>) -> Result<Value> {
    let mut value: Option<Value> = None;
    let mut stack: Vec<Value> = vec![];
    for token in tokens {
        if let Some(element) = value.take() {
            if let Some(lst) = stack.last_mut() {
                if let Some(lst) = lst.as_list_mut() {
                    lst.push(element);
                }
            }
        }
        value = if token == B {
            stack.push(Value::from(List::new()?));
            None
        } else if token == E {
            Some(stack.pop().unwrap())
        } else {
            Some(token.into())
        };
    }
    match (stack.len(), value) {
        (0, Some(value)) => Ok(value),
        (0, None) => bail!("E993:-:0:missing collection"),
        _ => bail!("E994:-:0:unclosed collection"),
    }
}

fn get_expected() -> Result<Value> {
    let mut lst = List::new()?;
    lst.push(1.into());
    lst.push(2.into());
    lst.push(Value::from(List::new()?));
    if let Some(sublst) = lst.inner_mut().last_mut() {
        if let Some(sublst) = sublst.as_list_mut() {
            sublst.push(30.into());
            sublst.push(40.into());
            sublst.push(Value::from(List::new()?));
            if let Some(subsublst) = sublst.inner_mut().last_mut() {
                if let Some(subsublst) = subsublst.as_list_mut() {
                    subsublst.push(500.into());
                    subsublst.push(Value::from(List::new()?));
                }
            }
            sublst.push(70.into());
            sublst.push(Value::from(List::new()?));
            if let Some(subsublst) = sublst.inner_mut().last_mut() {
                if let Some(subsublst) = subsublst.as_list_mut() {
                    subsublst.push(800.into());
                    subsublst.push(900.into());
                }
            }
            sublst.push(1000.into());
        }
    }
    lst.push(1100.into());
    Ok(Value::from(lst))
}

pub type Values = Vec<Value>; // For Lists

#[derive(Clone, Debug)]
pub enum Value {
    // The full Value has other scalar & collection types
    Int(i64),
    List(List),
}

impl Value {
    pub fn as_list(&self) -> Option<&List> {
        if let Value::List(value) = self {
            Some(value)
        } else {
            None
        }
    }
    pub fn as_list_mut(&mut self) -> Option<&mut List> {
        if let Value::List(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Int(i) => i.to_string(),
                Value::List(lst) => lst.to_string(),
            }
        )
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<List> for Value {
    fn from(lst: List) -> Self {
        Value::List(lst)
    }
}

#[derive(Clone, Debug)]
pub struct List {
    values: Values,
}

impl List {
    pub fn new() -> Result<Self> {
        Ok(List { values: Values::new() })
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn iter(&self) -> std::slice::Iter<Value> {
        self.values.iter()
    }

    pub fn inner(&self) -> &Values {
        &self.values
    }

    pub fn inner_mut(&mut self) -> &mut Values {
        &mut self.values
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts = vec!["[".to_string()];
        let mut sep = "";
        for value in self.iter() {
            parts.push(sep.to_string());
            parts.push(value.to_string());
            sep = ", ";
        }
        parts.push("]".to_string());
        write!(f, "{}", parts.join(""))
    }
}
