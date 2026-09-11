use crate::vm::InterpretResult::NotHandled;

use super::super::*;

impl Vm {
    pub fn cast_run(&mut self, instructions: u8) -> InterpretResult {
        match instructions {
            x if x == OpCode::Cast as u8 => self.op_cast(),
            x if x == OpCode::AddAdd as u8 => self.op_add_add(),
            x if x == OpCode::MinusMinus as u8 => self.op_minus_minus(),
            _ => NotHandled,
        }
    }
}
