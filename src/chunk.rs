use crate::value::{Value, ValueArray};

#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    pub line: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: ValueArray::new(),
            line: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.line.push(line);
    }

    pub fn add_const(&mut self, value: Value) -> usize {
        self.constants.write_valuearray(value);
        self.constants.values.len() - 1
    }
}
