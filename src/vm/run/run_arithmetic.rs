use crate::vm::InterpretResult::NotHandled;

use super::super::*;

impl Vm {
    pub fn arithmetic_run(&mut self, instruction: u8) -> InterpretResult {
        match instruction {
            x if x == OpCode::Negate as u8 => {
                let value = self.stack.pop().expect(ERR_POP_MES);
                self.stack.push(-value);
                InterpretResult::Ok
            }
            x if x == OpCode::Add as u8 => self.binary_operations('+'),
            x if x == OpCode::Subtract as u8 => self.binary_operations('-'),
            x if x == OpCode::Multiply as u8 => self.binary_operations('*'),
            x if x == OpCode::Divide as u8 => self.binary_operations('/'),
            x if x == OpCode::GreaterThan as u8 => {
                self.comparison_operations(">");
                InterpretResult::Ok
            }
            x if x == OpCode::LessThan as u8 => {
                self.comparison_operations("<");
                InterpretResult::Ok
            }
            x if x == OpCode::GreaterThanEq as u8 => {
                self.comparison_operations(">=");
                InterpretResult::Ok
            }
            x if x == OpCode::LessThanEq as u8 => {
                self.comparison_operations("<=");
                InterpretResult::Ok
            }
            x if x == OpCode::EqualTo as u8 => {
                self.comparison_operations("==");
                InterpretResult::Ok
            }
            x if x == OpCode::NotEqualTo as u8 => {
                self.comparison_operations("!=");
                InterpretResult::Ok
            }
            x if x == OpCode::Not as u8 => self.op_not(),
            x if x == OpCode::Modulo as u8 => self.binary_operations('%'),
            _ => NotHandled,
        }
    }
}
