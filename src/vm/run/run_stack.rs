use crate::vm::InterpretResult::NotHandled;

use super::super::*;

impl Vm {
    pub fn stack_run(&mut self, instructions: u8) -> InterpretResult {
        match instructions {
            x if x == OpCode::Return as u8 => return InterpretResult::Done,
            x if x == OpCode::Constant as u8 => {
                let constant = self.read_constant();
                self.stack.push(constant);
                InterpretResult::Ok
            }
            x if x == OpCode::True as u8 => {
                self.stack.push(Value::Bool(true));
                InterpretResult::Ok
            }
            x if x == OpCode::False as u8 => {
                self.stack.push(Value::Bool(false));
                InterpretResult::Ok
            }
            x if x == OpCode::Void as u8 => {
                self.stack.push(Value::Void);
                InterpretResult::Ok
            }
            x if x == OpCode::Dup as u8 => {
                let value = self.stack.last().cloned().unwrap_or(Void);
                self.stack.push(value);
                InterpretResult::Ok
            }
            x if x == OpCode::GetLocal as u8 => {
                let slot = self.read_byte();
                let base = self.frames.frames[self.frames.frame_count - 1].slots;
                let value = self.stack[base + slot as usize].clone();
                self.stack.push(value);
                InterpretResult::Ok
            }
            x if x == OpCode::SetLocal as u8 => {
                let slot = self.read_byte();
                let base = self.frames.frames[self.frames.frame_count - 1].slots;
                self.stack[base + slot as usize] = self.peek();
                InterpretResult::Ok
            }
            x if x == OpCode::Pop as u8 => {
                self.stack.pop().unwrap_or(Void);
                InterpretResult::Ok
            }
            _ => NotHandled,
        }
    }
}
