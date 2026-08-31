use crate::value::{Value, ValueArray};

#[repr(u8)]
pub enum OpCode {
    Return,
    Constant,

    //Unary operations
    Negate,

    //Binary operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    //Comparison operations
    GreaterThan,
    LessThan,
    GreaterThanEq,
    LessThanEq,
    EqualTo,
    NotEqualTo,

    //Normal Functions
    Print,
    Println,

    //Numbers Functions
    Abs,
    Floor,
    Ceil,
    Round,
    SquareRoot,

    //Strings Functions
    IsEmpty,
    Trim,
    Reverse,

    //Values of the boolean type
    True,
    False,
    Not,

    //Variables
    GetLocal,
    SetLocal,

    //Other
    Void,
    Pop,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    pub line: Vec<u32>,
}

#[allow(warnings)]
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
