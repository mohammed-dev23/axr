use std::{fmt, ops::Neg, sync::Arc};

use crate::value::Value::{Bool, Float, Int, Noth, Str};

//warnings has been allowed here because we are in a very early stage
// not every value type is being used !
#[allow(warnings)]
#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Float(f64),
    Int(i64),
    Str(Arc<str>),
    Noth,
}

#[allow(warnings)]
impl Value {
    pub fn is_float(&self) -> bool {
        match self {
            Float(_) => true,
            Int(_) => false,
            Bool(_) => false,
            Str(_) => false,
            Noth => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Float(_) => false,
            Int(_) => true,
            Bool(_) => false,
            Str(_) => false,
            Noth => false,
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            Float(_) | Int(_) | Bool(_) | Noth => false,
            Str(_) => true,
        }
    }
}

#[allow(warnings)]
impl Value {
    pub fn as_float(&self) -> f64 {
        match self {
            Float(x) => *x,
            Int(x) => *x as f64,
            _ => f64::default(),
        }
    }

    pub fn as_int(&self) -> i64 {
        match self {
            Float(x) => *x as i64,
            Int(x) => *x,
            _ => i64::default(),
        }
    }

    pub fn as_str(&self) -> Arc<str> {
        match self {
            Str(x) => x.clone(),
            _ => Arc::from(""),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(x) => write!(f, "{}", x),
            Value::Float(x) => write!(f, "{}", x),
            Value::Int(x) => write!(f, "{}", x),
            Value::Str(x) => write!(f, "{}", x),
            Value::Noth => write!(f, "None"),
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
            Self::Str(x) => Str(x),
            Self::Noth => Self::Noth,
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
