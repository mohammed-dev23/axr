use super::super::*;

impl Vm {
    pub fn op_cast(&mut self) -> InterpretResult {
        let value = self.stack.pop().expect(ERR_POP_MES);
        let target = self.get_type();

        match target {
            TypeTag::Int => self.stack.push(Value::Int(value.cast_int().unwrap())),
            TypeTag::Float => self.stack.push(Value::Float(value.cast_float().unwrap())),
            TypeTag::Unt => {
                self.stack.push(Value::Unt(value.cast_unt().unwrap()));
            }
            _ => return InterpretResult::RuntimeError,
        }
        InterpretResult::Ok
    }
    pub fn op_add_add(&mut self) -> InterpretResult {
        let lhs_value = self.stack.pop().expect(ERR_POP_MES);
        let rhs_value = self.stack.pop().expect(ERR_POP_MES);

        let lhs_type_tag = self.get_type();

        match lhs_type_tag {
            TypeTag::Int => {
                let lhs = lhs_value.as_int();
                let mut rhs = rhs_value.as_int();
                rhs += lhs;

                self.stack.push(Value::Int(rhs));
            }
            TypeTag::Unt => {
                let lhs = lhs_value.as_unt();
                let mut rhs = rhs_value.as_unt();
                rhs += lhs;

                self.stack.push(Value::Unt(rhs));
            }
            TypeTag::Float => {
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

        let lhs_type_tag = self.get_type();

        match lhs_type_tag {
            TypeTag::Int => {
                let lhs = lhs_value.as_int();
                let mut rhs = rhs_value.as_int();
                rhs -= lhs;

                self.stack.push(Value::Int(rhs));
            }
            TypeTag::Unt => {
                let lhs = lhs_value.as_unt();
                let mut rhs = rhs_value.as_unt();
                rhs -= lhs;

                self.stack.push(Value::Unt(rhs));
            }
            TypeTag::Float => {
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
