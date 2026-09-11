use crate::vm::InterpretResult::NotHandled;

use super::super::*;

impl Vm {
    pub fn control_flow_run(&mut self, instructions: u8) -> InterpretResult {
        match instructions {
            x if x == OpCode::JumpIfFalse as u8 => {
                let offset = self.read_short();

                if !self.peek().as_bool() {
                    self.ip += offset as usize
                }

                InterpretResult::Ok
            }
            x if x == OpCode::Jump as u8 => {
                let offset = self.read_short();
                self.ip += offset as usize;
                InterpretResult::Ok
            }
            x if x == OpCode::Loop as u8 => {
                let offset = self.read_short();
                self.ip -= offset as usize;
                InterpretResult::Ok
            }
            _ => NotHandled,
        }
    }
}
