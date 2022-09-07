use anyhow::Result;
use std::{cell::RefCell, fmt, rc::Rc};

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
    let mut root: Value = 0.into(); // fake root to be overwritten
    let mut stack = vec![];
    for token in tokens {
        if token == B {
            let lst = Value::from(List::new()?);
            if root.is_int() {
                root = lst.clone();
                stack.push(Rc::new(RefCell::new(lst)));
            } else {
                let top = stack.last().unwrap().borrow_mut();
                if let Some(sublst) = top.as_list() {
                    let mut sublst = sublst.borrow_mut();
                    // add new list to current list
                    sublst.push(lst.clone());
                }
                drop(top);
                // make new list the current list
                stack.push(Rc::new(RefCell::new(lst)));
            }
        } else if token == E {
            stack.pop();
        } else {
            let top = stack.last().unwrap().borrow_mut();
            if let Some(sublst) = top.as_list() {
                let mut sublst = sublst.borrow_mut();
                sublst.push(token.into());
            }
        }
    }
    Ok(Value::from(root))
}

fn get_expected() -> Result<Value> {
    let mut lst = List::new()?;
    lst.push(1.into());
    lst.push(2.into());
    lst.push(Value::from(List::new()?));
    if let Some(sublst) = lst.inner().last().unwrap().as_list() {
        let mut sublst = sublst.borrow_mut();
        sublst.push(30.into());
        sublst.push(40.into());
        sublst.push(Value::from(List::new()?));
        if let Some(subsublst) = sublst.inner().last().unwrap().as_list() {
            let mut subsublst = subsublst.borrow_mut();
            subsublst.push(500.into());
            subsublst.push(Value::from(List::new()?));
        }
        sublst.push(70.into());
        sublst.push(Value::from(List::new()?));
        if let Some(subsublst) = sublst.inner().last().unwrap().as_list() {
            let mut subsublst = subsublst.borrow_mut();
            subsublst.push(800.into());
            subsublst.push(900.into());
        }
        sublst.push(1000.into());
    }
    lst.push(1100.into());
    Ok(Value::from(lst))
}

pub type Values = Vec<Value>; // For Lists

#[derive(Clone, Debug)]
pub enum Value {
    // The full Value has other scalar & collection types
    Int(i64),
    List(Rc<RefCell<List>>),
}

impl Value {
    pub fn as_list(&self) -> Option<Rc<RefCell<List>>> {
        if let Value::List(value) = self {
            Some(Rc::clone(value))
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
                Value::List(lst) => lst.borrow().to_string(),
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
        Value::List(Rc::new(RefCell::new(lst)))
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
