use crate::value::Native;

use super::*;

impl Vm {
    pub fn call_value(&mut self, value: Value, arg_count: usize) -> InterpretResult {
        match value {
            Value::Function(function) => self.call(function, arg_count),
            Value::NativeFunction(native) => {
                let native = native.as_ref();

                let value = native(
                    arg_count,
                    &self.stack.clone()[self.stack.len() - arg_count],
                    Some(&TypeTag::from_bytes(self.read_byte())),
                );

                self.stack.truncate(self.stack.len() - (arg_count + 1));

                self.stack.push(value);
                InterpretResult::Ok
            }
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

    pub fn define_native(&mut self, name: Arc<str>, native: Native) {
        self.stack.push(Value::NativeFunction(Arc::new(native)));
        self.stack.push(Str(name));

        self.global_table.insert(
            self.stack.pop().expect(ERR_POP_MES).as_str(),
            self.stack.pop().expect(ERR_POP_MES).clone(),
        );
    }
}
