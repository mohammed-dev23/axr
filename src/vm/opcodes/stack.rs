use super::super::*;

impl Vm {
    pub fn op_call(&mut self) -> InterpretResult {
        let arg_count = self.read_byte();
        let function = self.peek_spec(arg_count as usize);
        self.call_value(function, arg_count as usize)
    }

    pub fn op_define_globals(&mut self) -> InterpretResult {
        let name = self.read_constant();
        let value = self.stack.pop().expect(ERR_POP_MES);

        if name.is_str() {
            if self.global_table.contains_key(&name.as_str()) {
                return self.runtime_err("the global varible already exsist!.");
            }

            self.global_table.insert(name.as_str(), value);
        } else {
            return self.runtime_err(&format!(
                "cannot use {} for global varbails declration!",
                value
            ));
        }
        InterpretResult::Ok
    }

    pub fn op_get_global(&mut self) -> InterpretResult {
        let name = self.read_constant();

        if name.is_str() {
            if let Some(value) = self.global_table.get(&name.as_str()) {
                self.stack.push(value.clone());
            } else {
                return self.runtime_err(&format!("Couldn't find {} in the globals table", name));
            }
        } else {
            return self.runtime_err(&format!(
                "cannot use {} there must be no global varible with {}!.",
                name, name
            ));
        }
        InterpretResult::Ok
    }
}
