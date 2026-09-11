use super::super::*;

impl Vm {
    pub fn op_grab(&mut self) -> InterpretResult {
        let value = self.stack.pop().expect(ERR_POP_MES);

        let opt_value = value.as_opt();

        let value = match opt_value {
            OptWrapper::Some(x) => x,
            OptWrapper::None => return self.runtime_err("Paniced on None value!."),
        };

        self.stack.push(*value);
        InterpretResult::Ok
    }
}
