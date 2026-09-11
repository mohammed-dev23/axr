use super::super::*;

impl Vm {
    pub fn run_wrappers(&mut self, instructions: u8) -> InterpretResult {
        match instructions {
            x if x == OpCode::Some as u8 => self.op_opt_some(),
            x if x == OpCode::None as u8 => {
                self.stack.push(Value::Opt(OptWrapper::None));
                InterpretResult::Ok
            }
            _ => InterpretResult::NotHandled,
        }
    }
}
