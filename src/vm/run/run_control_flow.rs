use crate::vm::InterpretResult::NotHandled;

use super::super::*;

impl Vm {
    pub fn control_flow_run(&mut self, instructions: u8) -> InterpretResult {
        match instructions {
            x if x == OpCode::JumpIfFalse as u8 => {
                let offset = self.read_short();

                if !self.peek().as_bool() {
                    self.frames.frames[self.frames.frame_count - 1].ip += offset as usize
                }

                InterpretResult::Ok
            }
            x if x == OpCode::Jump as u8 => {
                let offset = self.read_short();
                self.frames.frames[self.frames.frame_count - 1].ip += offset as usize;
                InterpretResult::Ok
            }
            x if x == OpCode::Loop as u8 => {
                let offset = self.read_short();
                self.frames.frames[self.frames.frame_count - 1].ip -= offset as usize;
                InterpretResult::Ok
            }
            _ => NotHandled,
        }
    }
}
