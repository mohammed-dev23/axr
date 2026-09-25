use crate::vm::InterpretResult::NotHandled;

use super::super::*;

impl Vm {
    pub fn builtins_run(&mut self, instructions: u8) -> InterpretResult {
        match instructions {
            x if x == OpCode::Abs as u8 => {
                let value = self.stack.pop().expect(ERR_POP_MES);

                match value {
                    Value::Float(x) => self.stack.push(Value::Float(x.abs())),
                    Int(x) => self.stack.push(Value::Int(x.abs())),
                    _ => {}
                };

                InterpretResult::Ok
            }

            x if x == OpCode::Grab as u8 => self.op_grab(),
            _ => NotHandled,
        }
    }
}
