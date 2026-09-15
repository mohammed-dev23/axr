use super::*;

impl Vm {
    pub fn call_value(&mut self, value: Value, arg_count: usize) -> InterpretResult {
        match value {
            Value::Function(function) => self.call(function, arg_count),
            _ => {
                self.runtime_err("Uncallable!.");
                InterpretResult::RuntimeError
            }
        }
    }

    pub fn call(&mut self, function: Arc<Function>, arg_count: usize) -> InterpretResult {
        if arg_count != function.arity {
            self.runtime_err(&format!(
                "Expected {} arguments but got {}.",
                function.arity, arg_count
            ));
            return InterpretResult::RuntimeError;
        }

        self.frames.frames.push(CallFrame {
            function: function,
            ip: 0,
            slots: self.stack.len() - arg_count - 1,
        });

        self.frames.frame_count += 1;
        InterpretResult::Ok
    }
}
