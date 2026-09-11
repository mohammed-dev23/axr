use super::super::*;

impl Vm {
    pub fn binary_operations(&mut self, op: char) -> InterpretResult {
        let v2 = self.stack.pop().expect(ERR_POP_MES);
        let v1 = self.stack.pop().expect(ERR_POP_MES);

        match (&v1, &v2) {
            (Value::Float(v1), Value::Float(v2)) => {
                let res = Self::op(op, v1, *v2);

                if let Ok(res) = res {
                    self.stack.push(Value::Float(res));

                    InterpretResult::Ok
                } else {
                    return InterpretResult::RuntimeError;
                }
            }
            (Value::Int(v1), Value::Int(v2)) => {
                let res = Self::op(op, v1, *v2);

                if let Ok(res) = res {
                    self.stack.push(Value::Int(res));

                    return InterpretResult::Ok;
                } else {
                    return RuntimeError;
                }
            }
            (Value::Float(v1), Value::Int(v2)) => {
                let res = Self::op(op, v1, *v2 as f64);

                if let Ok(res) = res {
                    self.stack.push(Value::Float(res));

                    return InterpretResult::Ok;
                } else {
                    return RuntimeError;
                }
            }
            (Value::Int(v1), Value::Float(v2)) => {
                let res = Self::op(op, *v1 as f64, *v2);

                if let Ok(res) = res {
                    self.stack.push(Value::Float(res));

                    return InterpretResult::Ok;
                } else {
                    RuntimeError
                }
            }
            (Value::Str(v1), Value::Str(v2)) => {
                self.stack.push(Value::Str(Arc::from(v1.to_string() + v2)));
                InterpretResult::Ok
            }
            (Value::Unt(v1), Value::Unt(v2)) => {
                let res = Self::op(op, v1, *v2);

                if let Ok(res) = res {
                    self.stack.push(Value::Unt(res));
                    return InterpretResult::Ok;
                } else {
                    return RuntimeError;
                }
            }
            (Value::Unt(v1), Value::Float(v2)) => {
                let res = Self::op(op, *v1 as f64, *v2);

                if let Ok(res) = res {
                    self.stack.push(Value::Float(res));

                    return InterpretResult::Ok;
                } else {
                    RuntimeError
                }
            }
            (Value::Float(v1), Value::Unt(v2)) => {
                let res = Self::op(op, *v1, *v2 as f64);

                if let Ok(res) = res {
                    self.stack.push(Value::Float(res));
                    return InterpretResult::Ok;
                } else {
                    InterpretResult::RuntimeError
                }
            }
            _ => {
                return RuntimeError;
            }
        }
    }

    pub fn comparison_operations(&mut self, op: &str) {
        let v2 = self.stack.pop().expect(ERR_POP_MES);
        let v1 = self.stack.pop().expect(ERR_POP_MES);

        match (v1, v2) {
            (Value::Float(v1), Value::Float(v2)) => {
                self.stack.push(Value::Bool(Self::cmp_op(op, v1, v2)));
            }
            (Value::Int(v1), Value::Int(v2)) => {
                self.stack.push(Value::Bool(Self::cmp_op(op, v1, v2)));
            }
            (Value::Str(v1), Value::Str(v2)) => {
                self.stack.push(Value::Bool(Self::cmp_op(op, v1, v2)));
            }
            (Value::Unt(v1), Value::Unt(v2)) => {
                self.stack.push(Value::Bool(Self::cmp_op(op, v1, v2)));
            }
            (Value::Char(v1), Value::Char(v2)) => {
                self.stack.push(Value::Bool(Self::cmp_op(op, v1, v2)));
            }
            _ => {}
        }
    }

    pub fn op_not(&mut self) -> InterpretResult {
        let value = self.stack.pop().expect(ERR_POP_MES);

        if value.is_bool() {
            self.stack.push(Value::Bool(!value.as_bool()));
        } else {
            self.runtime_err(&format!("cannot use {} with Not/! opratoier.", value));
            self.stack.push(value);
        }

        InterpretResult::Ok
    }
}
