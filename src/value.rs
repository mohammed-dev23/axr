use std::{
    fmt::{self},
    ops::Neg,
    sync::{Arc, Mutex},
};

use crate::{
    chunk::Chunk,
    compiler::TypeTag,
    value::Value::{Array, Bool, Char, Float, Int, Str, Unt},
};

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Float(f64),
    Int(i64),
    Str(Arc<str>),
    Char(char),
    Unt(u64),
    Array(Arc<Mutex<Vec<Value>>>),
    Opt(OptWrapper),
    Function(Arc<Function>),
    NativeFunction(Arc<Native>),
    Void,
}

#[derive(Debug, Clone)]
pub enum OptWrapper {
    Some(Box<Value>),
    None,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub arity: usize,
    pub chunk: Chunk,
    pub name: String,
}

pub type Native = fn(arg_count: usize, args: &[Value], generic: Option<&TypeTag>) -> Value;

impl Function {
    pub fn new() -> Self {
        Self {
            arity: 0,
            chunk: Chunk::new(),
            name: String::new(),
        }
    }
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

    pub fn is_array(&self) -> bool {
        match self {
            Array(_) => true,
            _ => false,
        }
    }

    pub fn is_fn(&self) -> bool {
        match self {
            Self::Function(_) => true,
            _ => false,
        }
    }

    pub fn is_native(&self) -> bool {
        match self {
            Self::NativeFunction(_) => true,
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

    pub fn as_char(&self) -> char {
        match self {
            Char(x) => *x,
            _ => char::default(),
        }
    }

    pub fn as_array(self) -> Option<Arc<Mutex<Vec<Value>>>> {
        match self {
            Array(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_opt(self) -> OptWrapper {
        match self {
            Self::Opt(x) => x,
            _ => OptWrapper::None,
        }
    }

    pub fn as_fn(self) -> Option<Arc<Function>> {
        match self {
            Self::Function(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_native(self) -> Option<Arc<Native>> {
        match self {
            Self::NativeFunction(x) => Some(x),
            _ => None,
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
            Value::Opt(x) => match x {
                OptWrapper::Some(x) => write!(f, "{}", x),
                OptWrapper::None => write!(f, "None"),
            },
            Value::Function(x) => write!(f, "{:?}", x),
            Value::NativeFunction(x) => write!(f, "{:?}", x),
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
            _ => Self::Void,
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
