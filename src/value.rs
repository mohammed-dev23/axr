use std::{fmt, ops::Neg, sync::Arc};

use crate::value::Value::{Array, Bool, Char, Float, Int, Str, Unt};

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Float(f64),
    Int(i64),
    Str(Arc<str>),
    Char(char),
    Unt(u64),
    Array(Vec<Value>),
    Void,
}

#[allow(warnings)]
impl Value {
    pub fn is_float(&self) -> bool {
        match self {
            Float(_) => true,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Int(_) => true,
            _ => false,
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            Str(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Bool(_) => true,
            _ => false,
        }
    }
}

#[allow(warnings)]
impl Value {
    pub fn as_float(&self) -> f64 {
        match self {
            Float(x) => *x,
            _ => f64::default(),
        }
    }

    pub fn as_int(&self) -> i64 {
        match self {
            Int(x) => *x as i64,
            _ => i64::default(),
        }
    }

    pub fn as_str(&self) -> Arc<str> {
        match self {
            Str(x) => x.clone(),
            _ => Arc::from(""),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Bool(x) => *x,
            _ => bool::default(),
        }
    }

    pub fn as_unt(&self) -> u64 {
        match self {
            Unt(x) => *x,
            _ => u64::default(),
        }
    }
}

impl Value {
    pub fn cast_int(&self) -> Option<i64> {
        match self {
            Int(x) => Some(*x),
            Float(x) => Some(*x as i64),
            Unt(x) => Some(*x as i64),
            Bool(x) => Some(*x as i64),
            Char(x) => Some(*x as i64),
            _ => None,
        }
    }

    pub fn cast_float(&self) -> Option<f64> {
        match self {
            Int(x) => Some(*x as f64),
            Float(x) => Some(*x),
            Unt(x) => Some(*x as f64),
            Bool(x) => Some((*x as i64) as f64),
            Char(x) => Some((*x as i64) as f64),
            _ => None,
        }
    }

    pub fn cast_unt(&self) -> Option<u64> {
        match self {
            Int(x) => Some(*x as u64),
            Float(x) => Some(*x as u64),
            Unt(x) => Some(*x),
            Bool(x) => Some(*x as u64),
            Char(x) => Some(*x as u64),
            _ => None,
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
            Value::Char(x) => write!(f, "{}", x),
            Value::Unt(x) => write!(f, "{}", x),
            Value::Array(x) => write!(f, "{:?}", x),
            Value::Void => write!(f, "Void"),
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
            Self::Char(x) => Char(x),
            Self::Unt(x) => Unt(x),
            Self::Array(x) => Array(x),
            Self::Void => Self::Void,
        }
    }
}

#[derive(Debug, Clone)]
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
