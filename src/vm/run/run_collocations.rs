use crate::vm::InterpretResult::NotHandled;

use super::super::*;

impl Vm {
    pub fn colloc_run(&mut self, instructions: u8) -> InterpretResult {
        match instructions {
            x if x == OpCode::Array as u8 => self.op_array(),
            x if x == OpCode::IndexArray as u8 => self.op_array_index(),
            x if x == OpCode::NewArray as u8 => {
                self.stack.push(Array(Arc::new(Mutex::new(Vec::new()))));
                InterpretResult::Ok
            }
            x if x == OpCode::Push as u8 => self.op_push(),
            x if x == OpCode::PopArray as u8 => self.op_pop_array(),
            x if x == OpCode::Len as u8 => self.op_len(),
            x if x == OpCode::Reverse as u8 => self.op_reverse(),
            _ => NotHandled,
        }
    }
}
