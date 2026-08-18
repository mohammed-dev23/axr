use crate::{
    chunk::{Chunk, OpCode},
    value::Value::{self, None as nai},
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

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
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
                    let value = self.stack.pop();
                    println!("{}", value.unwrap_or(nai));
                    return InterpretResult::Ok;
                }
                x if x == OpCode::Constant as u8 => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                }
                x if x == OpCode::Negate as u8 => {
                    let value = self.stack.pop().unwrap_or(Value::None);
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
        self.chunk.constants.values[index]
    }

    fn binary_operations(&mut self, op: char) {
        let v2 = self.stack.pop().unwrap_or(nai);
        let v1 = self.stack.pop().unwrap_or(nai);

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
        let v2 = self.stack.pop().unwrap_or(nai);
        let v1 = self.stack.pop().unwrap_or(nai);

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
            + std::ops::Div<R, Output = A>,
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
