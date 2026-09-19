use super::super::*;

impl Vm {
    pub fn op_cast(&mut self) -> InterpretResult {
        let target = self.read_byte();
        let value = self.stack.pop().expect(ERR_POP_MES);

        match target {
            t if t == TypeTag::Int.as_bytes() => {
                self.stack.push(Value::Int(value.cast_int().unwrap()))
            }
            t if t == TypeTag::Float.as_bytes() => {
                self.stack.push(Value::Float(value.cast_float().unwrap()))
            }
            t if t == TypeTag::Unt.as_bytes() => {
                self.stack.push(Value::Unt(value.cast_unt().unwrap()));
            }
            _ => return InterpretResult::RuntimeError,
        }
        InterpretResult::Ok
    }
    pub fn op_add_add(&mut self) -> InterpretResult {
        let lhs_value = self.stack.pop().expect(ERR_POP_MES);
        let rhs_value = self.stack.pop().expect(ERR_POP_MES);

        let lhs_type_tag = self.read_byte();

        match lhs_type_tag {
            t if t == TypeTag::Int.as_bytes() => {
                let lhs = lhs_value.as_int();
                let mut rhs = rhs_value.as_int();
                rhs += lhs;

                self.stack.push(Value::Int(rhs));
            }
            t if t == TypeTag::Unt.as_bytes() => {
                let lhs = lhs_value.as_unt();
                let mut rhs = rhs_value.as_unt();
                rhs += lhs;

                self.stack.push(Value::Unt(rhs));
            }
            t if t == TypeTag::Float.as_bytes() => {
                let lhs = lhs_value.as_float();
                let mut rhs = rhs_value.as_float();
                rhs += lhs;

                self.stack.push(Value::Float(rhs));
            }
            _ => return RuntimeError,
        }
        InterpretResult::Ok
    }

    pub fn op_minus_minus(&mut self) -> InterpretResult {
        let lhs_value = self.stack.pop().expect(ERR_POP_MES);
        let rhs_value = self.stack.pop().expect(ERR_POP_MES);

        let lhs_type_tag = self.read_byte();

        match lhs_type_tag {
            t if t == TypeTag::Int.as_bytes() => {
                let lhs = lhs_value.as_int();
                let mut rhs = rhs_value.as_int();
                rhs -= lhs;

                self.stack.push(Value::Int(rhs));
            }
            t if t == TypeTag::Unt.as_bytes() => {
                let lhs = lhs_value.as_unt();
                let mut rhs = rhs_value.as_unt();
                rhs -= lhs;

                self.stack.push(Value::Unt(rhs));
            }
            t if t == TypeTag::Float.as_bytes() => {
                let lhs = lhs_value.as_float();
                let mut rhs = rhs_value.as_float();
                rhs -= lhs;

                self.stack.push(Value::Float(rhs));
            }
            _ => return RuntimeError,
        }
        InterpretResult::Ok
    }
}
