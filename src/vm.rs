use std::{
    io::{Write, stdin, stdout},
    sync::Arc,
};

use crate::{
    chunk::{
        Chunk,
        OpCode::{self},
    },
    compiler::{self, core::TypeTag::Id},
    value::{
        self,
        Value::{self, Array, Char, Int, Str, Unt, Void},
    },
    vm::InterpretResult::CompileError,
};

pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

// ignoried because of that the vm is on an very early stage
#[allow(warnings)]
#[derive(Debug, PartialEq, Eq)]
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
        let mut chunk = Chunk::new();
        let mut compiler = compiler::core::Parser::new();

        if !compiler.compile(source, &mut chunk) {
            return InterpretResult::CompileError;
        }

        self.chunk = chunk;
        self.ip = 0;

        self.run()
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
                    let value = self.stack.pop().unwrap_or(Value::Void);
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
                    let value = self.stack.pop().unwrap_or(Void);
                    print!("{}", value)
                }
                x if x == OpCode::Println as u8 => {
                    let value = self.stack.pop().unwrap_or(Void);
                    println!("{}", value);
                }
                x if x == OpCode::Abs as u8 => {
                    let value = self.stack.pop().unwrap_or(Void);

                    match value {
                        Value::Float(x) => self.stack.push(Value::Float(x.abs())),
                        Int(x) => self.stack.push(Value::Int(x.abs())),
                        _ => {}
                    };
                }
                x if x == OpCode::Floor as u8 => {
                    let value = self.stack.pop().unwrap_or(Void);
                    self.stack.push(Value::Float(value.as_float().floor()));
                }
                x if x == OpCode::Ceil as u8 => {
                    let value = self.stack.pop().unwrap_or(Void);
                    self.stack.push(Value::Float(value.as_float().ceil()));
                }
                x if x == OpCode::Round as u8 => {
                    let value = self.stack.pop().unwrap_or(Void);
                    self.stack.push(Value::Float(value.as_float().round()));
                }
                x if x == OpCode::SquareRoot as u8 => {
                    let value = self.stack.pop().unwrap_or(Void);
                    self.stack.push(Value::Float(value.as_float().sqrt()));
                }
                x if x == OpCode::IsEmpty as u8 => {
                    let value = self.stack.pop().unwrap_or(Void);
                    self.stack.push(Value::Bool(value.as_str().is_empty()));
                }
                x if x == OpCode::Trim as u8 => {
                    let value = self.stack.pop().unwrap_or(Void);
                    self.stack
                        .push(Value::Str(Arc::from(value.as_str().trim())));
                }
                x if x == OpCode::Reverse as u8 => {
                    let value = self.stack.pop().unwrap_or(Void);

                    if value.is_str() {
                        self.stack.push(Value::Str(Arc::from(
                            value.as_str().chars().rev().collect::<String>(),
                        )));
                    }

                    if value.is_array() {
                        let mut array = value.as_array().unwrap();
                        array.reverse();
                        self.stack.push(Value::Array(array));
                    }
                }
                x if x == OpCode::True as u8 => {
                    self.stack.push(Value::Bool(true));
                }
                x if x == OpCode::False as u8 => {
                    self.stack.push(Value::Bool(false));
                }
                x if x == OpCode::Void as u8 => {
                    self.stack.push(Value::Void);
                }
                x if x == OpCode::Not as u8 => {
                    let value = self.stack.pop().unwrap_or(Void);

                    if value.is_bool() {
                        self.stack.push(Value::Bool(!value.as_bool()));
                    } else {
                        self.runtime_err(&format!("cannot use {} with Not/! opratoier.", value));
                        self.stack.push(value);
                    }
                }
                x if x == OpCode::Pop as u8 => {
                    self.stack.pop().unwrap_or(Void);
                    continue;
                }

                x if x == OpCode::GetLocal as u8 => {
                    let slot = self.read_byte();
                    let value = self.stack[slot as usize].clone();
                    self.stack.push(value);
                }
                x if x == OpCode::SetLocal as u8 => {
                    let slot = self.read_byte();
                    self.stack[slot as usize] = self.peek();
                }
                x if x == OpCode::Input as u8 => {
                    let expected_type = self.read_byte();
                    let txt = self.stack.pop().unwrap_or(Void);

                    print!("{}", txt);
                    stdout().flush().unwrap();

                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    let input = input.trim();

                    let value = match expected_type {
                        t if t == Id(compiler::core::TypeId::Int).as_bytes() => {
                            Int(input.parse::<i64>().unwrap_or_else(|_| {
                                self.runtime_err(&format!("Expected int found {}", input));
                                return 0;
                            }))
                        }
                        t if t == Id(compiler::core::TypeId::Str).as_bytes() => {
                            Str(Arc::from(input))
                        }
                        t if t == Id(compiler::core::TypeId::Float).as_bytes() => {
                            Value::Float(input.parse::<f64>().unwrap_or_else(|_| {
                                self.runtime_err(&format!("Expected float found {}", input));
                                return 0.0;
                            }))
                        }
                        t if t == Id(compiler::core::TypeId::Char).as_bytes() => {
                            let into_char: Vec<char> = input.chars().collect();
                            let c: Value;

                            if into_char.len() != 1 {
                                self.runtime_err(
                                    &format!("Char type cannot contain more than one char as an input. Expected char found {}" , input),
                                );
                                return InterpretResult::RuntimeError;
                            } else {
                                c = Char(into_char[0]);
                            };

                            c
                        }
                        t if t == Id(compiler::core::TypeId::Unt).as_bytes() => {
                            Unt(input.parse::<u64>().unwrap_or_else(|_| {
                                self.runtime_err(&format!("Expected unt found {}", input));
                                return 0;
                            }))
                        }
                        _ => Void,
                    };

                    self.stack.push(value);
                }
                x if x == OpCode::Cast as u8 => {
                    let target = self.read_byte();
                    let value = self.stack.pop().unwrap_or(Void);

                    match target {
                        t if t == Id(compiler::core::TypeId::Int).as_bytes() => {
                            self.stack.push(Value::Int(value.cast_int().unwrap()))
                        }
                        t if t == Id(compiler::core::TypeId::Float).as_bytes() => {
                            self.stack.push(Value::Float(value.cast_float().unwrap()))
                        }
                        t if t == Id(compiler::core::TypeId::Unt).as_bytes() => {
                            self.stack.push(Value::Unt(value.cast_unt().unwrap()));
                        }
                        _ => return CompileError,
                    }
                }
                x if x == OpCode::JumpIfFalse as u8 => {
                    let offset = self.read_short();

                    if !self.peek().as_bool() {
                        self.ip += offset as usize
                    }
                }
                x if x == OpCode::Jump as u8 => {
                    let offset = self.read_short();
                    self.ip += offset as usize;
                }
                x if x == OpCode::Loop as u8 => {
                    let offset = self.read_short();
                    self.ip -= offset as usize
                }
                x if x == OpCode::Array as u8 => {
                    let mut array: Vec<Value> = Vec::new();
                    let array_len = self.read_byte();

                    for _ in 0..array_len {
                        let value = self.stack.pop().unwrap_or(Void);
                        array.push(value);
                    }

                    array.reverse();
                    self.stack.push(Array(array));
                }
                x if x == OpCode::IndexArray as u8 => {
                    let index = self.stack.pop().unwrap_or(Void);
                    let array = self.stack.pop().unwrap_or(Void);
                    let value = array.clone();

                    if array.is_array() {
                        let array = array.as_array().unwrap_or_else(|| {
                            self.runtime_err(&format!("Expected [Array] found [{}]", value));
                            Vec::new()
                        });
                        self.stack.push(array[index.as_unt() as usize].clone());
                    }
                }
                x if x == OpCode::Dup as u8 => {
                    let value = self.stack.last().cloned().unwrap_or(Void);
                    self.stack.push(value);
                }
                x if x == OpCode::NewArray as u8 => {
                    self.stack.push(Array(Vec::new()));
                }
                x if x == OpCode::Push as u8 => {
                    let value = self.stack.pop().unwrap_or(Void);
                    let array = self.stack.pop().unwrap_or(Void);
                    let none = array.clone();

                    let mut array = array.as_array().unwrap_or_else(|| {
                        self.runtime_err(&format!("Expected [Array] found [{}]", none));
                        Vec::new()
                    });

                    array.push(value);

                    self.stack.push(Array(array));
                }
                x if x == OpCode::PopArray as u8 => {
                    let array = self.stack.pop().unwrap_or(Void);
                    let none = array.clone();

                    let mut array = array.as_array().unwrap_or_else(|| {
                        self.runtime_err(&format!("Expected [Array] found [{}]", none));
                        Vec::new()
                    });

                    let value = array.pop().unwrap_or(Void);

                    self.stack.push(value);
                    self.stack.push(Array(array));
                }
                x if x == OpCode::Len as u8 => {
                    let value = self.stack.pop().unwrap_or(Void);

                    if value.is_str() {
                        let value = value.as_str();
                        let len_value = value.len() as u64;

                        self.stack.push(Value::Unt(len_value));
                    }

                    if value.is_array() {
                        let none = value.clone();

                        let value = value.as_array().unwrap_or_else(|| {
                            self.runtime_err(&format!("Expected [Array] found [{}]", none));
                            Vec::new()
                        });

                        let len_value = value.len() as u64;
                        self.stack.push(Value::Unt(len_value));
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

    fn read_short(&mut self) -> u16 {
        let high = self.read_byte() as u16;
        let low = self.read_byte() as u16;
        (high << 8) | low
    }

    fn peek(&mut self) -> Value {
        self.stack.last().unwrap_or(&Void).clone()
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte() as usize;
        self.chunk.constants.values[index].clone()
    }

    fn binary_operations(&mut self, op: char) {
        let v2 = self.stack.pop().unwrap_or(Void);
        let v1 = self.stack.pop().unwrap_or(Void);

        match (&v1, &v2) {
            (Value::Float(v1), Value::Float(v2)) => {
                self.stack
                    .push(Value::Float(Self::op(op, v1, *v2).unwrap_or(0.0)));
            }
            (Value::Int(v1), Value::Int(v2)) => {
                self.stack
                    .push(Value::Int(Self::op(op, v1, *v2).unwrap_or(0)));
            }
            (Value::Float(v1), Value::Int(v2)) => {
                self.stack
                    .push(Value::Float(Self::op(op, v1, *v2 as f64).unwrap_or(0.0)));
            }
            (Value::Int(v1), Value::Float(v2)) => {
                self.stack
                    .push(Value::Float(Self::op(op, *v1 as f64, *v2).unwrap_or(0.0)));
            }
            (Value::Str(v1), Value::Str(v2)) => {
                self.stack.push(Value::Str(Arc::from(v1.to_string() + v2)));
            }
            (Value::Unt(v1), Value::Unt(v2)) => {
                self.stack
                    .push(Value::Unt(Self::op(op, v1, *v2).unwrap_or(0)));
            }
            (Value::Unt(v1), Value::Float(v2)) => {
                self.stack
                    .push(Value::Float(Self::op(op, *v1 as f64, *v2).unwrap_or(0.0)));
            }
            (Value::Float(v1), Value::Unt(v2)) => {
                self.stack
                    .push(Value::Float(Self::op(op, *v1, *v2 as f64).unwrap_or(0.0)));
            }
            _ => {
                return;
            }
        }
    }

    fn comparison_operations(&mut self, op: &str) {
        let v2 = self.stack.pop().unwrap_or(Void);
        let v1 = self.stack.pop().unwrap_or(Void);

        match (v1, v2) {
            (Value::Float(v1), Value::Float(v2)) => {
                self.stack.push(Value::Bool(Self::cmp_op(op, v1, v2)));
            }
            (Value::Int(v1), Value::Int(v2)) => {
                self.stack.push(Value::Bool(Self::cmp_op(op, v1, v2)));
            }
            (Value::Str(v1), Value::Str(v2)) => {
                self.stack.push(Value::Bool(Self::cmp_op(op, v1, v2)));
            }
            (Value::Unt(v1), Value::Unt(v2)) => {
                self.stack.push(Value::Bool(Self::cmp_op(op, v1, v2)));
            }
            (Value::Char(v1), Value::Char(v2)) => {
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
                    eprintln!("Can not divide by zero!");
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

    fn runtime_err(&mut self, message: &str) -> InterpretResult {
        eprintln!("{}", message);

        let instruction = self.ip - 1;
        let line = self.chunk.line[instruction as usize];
        eprintln!("[line {}] in code", line);

        InterpretResult::RuntimeError
    }
}
