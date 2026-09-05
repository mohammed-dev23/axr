use super::*;

impl Parser {
    pub fn emit_byte(&mut self, byte: u8) {
        let line = self.previous.line as u32;
        self.current_chunk().write_chunk(byte, line);
    }

    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant as u8, constant);
    }

    pub fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.current_chunk().add_const(value) as u8;

        if constant > u8::MAX {
            self.error("Too many constants in one chunk.");
            return 0;
        }

        constant
    }

    pub fn end_compiler(&mut self) {
        #[cfg(feature = "DPC")]
        {
            use crate::debug::disassemble_chunk;
            if !self.painc_mode {
                disassemble_chunk(self.current_chunk());
            }
        }
        self.emit_byte(OpCode::Return as u8);
        self.emit_return();
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    pub fn identifier_constant(&mut self, name: &Token) -> u8 {
        self.make_constant(Value::Str(Arc::from(name.start.to_string())))
    }

    pub fn emit_jump(&mut self, instruction: u8) -> u8 {
        self.emit_byte(instruction);
        self.emit_byte(0xff);
        self.emit_byte(0xff);
        return (&self.current_chunk().code.len() - 2) as u8;
    }

    pub fn patch_jump(&mut self, offset: u16) {
        let jump = (self.current_chunk().code.len()) as u16 - offset - 2;

        if jump as u16 > u16::MAX {
            self.error("Too much code to jump over.");
        }

        self.compiling_chunk.code[offset as usize] = ((jump >> 8) & 0xff) as u8;
        self.compiling_chunk.code[(offset + 1) as usize] = (jump & 0xff) as u8;
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.compiling_chunk
    }
}
