use std::fmt;

//warnings has been allowed here because we are in a very early stage
// not every value type is being used !
#[allow(warnings)]
#[derive(Debug, Clone, Copy)]
pub enum Value {
    Bool(bool),
    Float(f64),
    Int(i64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(x) => write!(f, "{}", x),
            Value::Float(x) => write!(f, "{}", x),
            Value::Int(x) => write!(f, "{}", x),
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
