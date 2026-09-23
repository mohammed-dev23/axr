use crate::vm::InterpretResult::NotHandled;

use super::super::*;

impl Vm {
    pub fn io_run(&mut self, instructions: u8) -> InterpretResult {
        match instructions {
            x if x == OpCode::Println as u8 => {
                let value = self.stack.pop().expect(ERR_POP_MES);
                println!("{}", value);
                InterpretResult::Ok
            }
            _ => NotHandled,
        }
    }
}
