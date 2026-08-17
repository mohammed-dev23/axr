use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

pub struct Vm {
    chunk: Chunk,
    ip: u8,
}

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
            }

            let instruction: u8 = self.read_byte();

            match instruction {
                x if x == OpCode::Return as u8 => return InterpretResult::Ok,
                x if x == OpCode::Constant as u8 => {
                    let constant = self.read_constant();
                    println!("{}", constant);
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
        self.chunk.constants.values[index]
    }
}
