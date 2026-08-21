use std::sync::Arc;

use crate::{
    chunk::{Chunk, OpCode},
    compiler,
    value::Value::{self, Float, Int, Noth},
};

pub struct Vm {
    chunk: Chunk,
    ip: u8,
    stack: Vec<Value>,
}

// ignoried because of that the vm is on an very early stage
#[allow(warnings)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::new(),
        }
    }

    //this fn used for testing the chunk and OpCodes "ByteCode" only
    #[allow(warnings)]
    pub fn test_chunk(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        compiler::compile(source);
        InterpretResult::Ok
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature = "DTE")]
            {
                use crate::debug::disassemble_instruction;
                disassemble_instruction(&self.chunk, self.ip as usize);

                println!();
                for i in &self.stack {
                    println!("[{}]", i)
                }
            }

            let instruction: u8 = self.read_byte();

            match instruction {
                x if x == OpCode::Return as u8 => {
                    return InterpretResult::Ok;
                }
                x if x == OpCode::Constant as u8 => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                }
                x if x == OpCode::Negate as u8 => {
                    let value = self.stack.pop().unwrap_or(Value::Noth);
                    self.stack.push(-value);
                }
                x if x == OpCode::Add as u8 => self.binary_operations('+'),
                x if x == OpCode::Subtract as u8 => self.binary_operations('-'),
                x if x == OpCode::Multiply as u8 => self.binary_operations('*'),
                x if x == OpCode::Divide as u8 => self.binary_operations('/'),
                x if x == OpCode::GreaterThan as u8 => self.comparison_operations(">"),
                x if x == OpCode::LessThan as u8 => self.comparison_operations("<"),
                x if x == OpCode::GreaterThanEq as u8 => self.comparison_operations(">="),
                x if x == OpCode::LessThanEq as u8 => self.comparison_operations("<="),
                x if x == OpCode::EqualTo as u8 => self.comparison_operations("=="),
                x if x == OpCode::NotEqualTo as u8 => self.comparison_operations("!="),
                x if x == OpCode::Modulo as u8 => self.binary_operations('%'),
                x if x == OpCode::Print as u8 => {
                    let value = self.stack.pop().unwrap_or(Noth);
                    print!("{}", value)
                }
                x if x == OpCode::Abs as u8 => {
                    let value = self.stack.pop().unwrap_or(Noth);

                    match value {
                        Float(x) => self.stack.push(Value::Float(x.abs())),
                        Int(x) => self.stack.push(Value::Int(x.abs())),
                        _ => self.stack.push(value),
                    };
                }
                x if x == OpCode::Floor as u8 => {
                    let value = self.stack.pop().unwrap_or(Noth);

                    if value.is_float() {
                        self.stack.push(Value::Float(value.as_float().floor()));
                    } else {
                        self.stack.push(value);
                    }
                }
                x if x == OpCode::Ceil as u8 => {
                    let value = self.stack.pop().unwrap_or(Noth);

                    if value.is_float() {
                        self.stack.push(Value::Float(value.as_float().ceil()));
                    } else {
                        self.stack.push(value);
                    }
                }
                x if x == OpCode::Round as u8 => {
                    let value = self.stack.pop().unwrap_or(Noth);

                    if value.is_float() {
                        self.stack.push(Value::Float(value.as_float().round()));
                    } else {
                        self.stack.push(value);
                    }
                }
                x if x == OpCode::SquareRoot as u8 => {
                    let value = self.stack.pop().unwrap_or(Noth);

                    if value.is_float() || value.is_int() {
                        self.stack.push(Value::Float(value.as_float().sqrt()));
                    } else {
                        self.stack.push(value);
                    }
                }
                x if x == OpCode::IsEmpty as u8 => {
                    let value = self.stack.pop().unwrap_or(Noth);

                    if value.is_str() {
                        self.stack.push(Value::Bool(value.as_str().is_empty()));
                    } else {
                        self.stack.push(value);
                    }
                }
                x if x == OpCode::Trim as u8 => {
                    let value = self.stack.pop().unwrap_or(Noth);

                    if value.is_str() {
                        self.stack
                            .push(Value::Str(Arc::from(value.as_str().trim())));
                    } else {
                        self.stack.push(value);
                    }
                }
                x if x == OpCode::Reverse as u8 => {
                    let value = self.stack.pop().unwrap_or(Noth);

                    if value.is_str() {
                        self.stack.push(Value::Str(Arc::from(
                            value.as_str().chars().rev().collect::<String>(),
                        )));
                    } else {
                        self.stack.push(value);
                    }
                }
                _ => {}
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip as usize];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte() as usize;
        self.chunk.constants.values[index].clone()
    }

    fn binary_operations(&mut self, op: char) {
        let v2 = self.stack.pop().unwrap_or(Noth);
        let v1 = self.stack.pop().unwrap_or(Noth);

        match (v1, v2) {
            (Value::Float(v1), Value::Float(v2)) => self
                .stack
                .push(Value::Float(Self::op(op, v1, v2).unwrap_or(0.0))),
            (Value::Int(v1), Value::Int(v2)) => self
                .stack
                .push(Value::Int(Self::op(op, v1, v2).unwrap_or(0))),
            _ => {}
        }
    }

    fn comparison_operations(&mut self, op: &str) {
        let v2 = self.stack.pop().unwrap_or(Noth);
        let v1 = self.stack.pop().unwrap_or(Noth);

        match (v1, v2) {
            (Value::Float(v1), Value::Float(v2)) => {
                self.stack.push(Value::Bool(Self::cmp_op(op, v1, v2)));
            }
            (Value::Int(v1), Value::Int(v2)) => {
                self.stack.push(Value::Bool(Self::cmp_op(op, v1, v2)));
            }
            _ => {}
        }
    }

    pub fn op<T, R, A>(op: char, v1: T, v2: R) -> Option<A>
    where
        T: std::ops::Add<R, Output = A>
            + std::ops::Sub<R, Output = A>
            + std::ops::Mul<R, Output = A>
            + std::ops::Div<R, Output = A>
            + std::ops::Rem<R, Output = A>,
        R: std::cmp::PartialEq + Default,
    {
        match op {
            '+' => Some(v1 + v2),
            '-' => Some(v1 - v2),
            '*' => Some(v1 * v2),
            '/' => {
                if v2 == R::default() {
                    println!("Can not divide by zero!");
                    return None;
                } else {
                    Some(v1 / v2)
                }
            }
            '%' => Some(v1 % v2),
            _ => None,
        }
    }

    pub fn cmp_op<T, R>(op: &str, v1: T, v2: R) -> bool
    where
        T: std::cmp::PartialEq<R> + std::cmp::PartialOrd<R>,
    {
        match op {
            ">" => v1 > v2,
            "<" => v1 < v2,
            ">=" => v1 >= v2,
            "<=" => v1 <= v2,
            "!=" => v1 != v2,
            "==" => v1 == v2,
            _ => false,
        }
    }
}
