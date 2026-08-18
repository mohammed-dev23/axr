use std::{fmt, ops::Neg};

use crate::value::Value::Bool;

//warnings has been allowed here because we are in a very early stage
// not every value type is being used !
#[allow(warnings)]
#[derive(Debug, Clone, Copy)]
pub enum Value {
    Bool(bool),
    Float(f64),
    Int(i64),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(x) => write!(f, "{}", x),
            Value::Float(x) => write!(f, "{}", x),
            Value::Int(x) => write!(f, "{}", x),
            Value::None => write!(f, "None"),
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Self::Float(x) => Self::Float(-x),
            Self::Int(x) => Self::Int(-x),
            Self::Bool(x) => Bool(x),
            Self::None => Self::None,
        }
    }
}

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn write_valuearray(&mut self, value: Value) {
        self.values.push(value);
    }
}
