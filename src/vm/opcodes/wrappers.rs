use crate::{
    compiler::TypeTag::{Char, Float, Int, Str, Unt, Void},
    value::Value::Opt,
};

use super::super::*;

impl Vm {
    pub fn op_opt_some(&mut self) -> InterpretResult {
        let type_tag = self.read_byte();
        let value = self.stack.pop().expect(ERR_POP_MES);

        if value.is_array() {
            let array_value = value.clone();

            let array = match array_value.as_array() {
                Some(v) => v,
                None => return self.runtime_err("Paniced on None value!."),
            };

            match type_tag {
                t if t
                    == TypeTag::Opt(Arc::new(TypeTag::Array(Arc::new(TypeTag::Int))))
                        .as_bytes() =>
                {
                    self.stack
                        .push(Value::Opt(OptWrapper::Some(Box::new(Value::Array(array)))));
                }
                t if t
                    == TypeTag::Opt(Arc::new(TypeTag::Array(Arc::new(TypeTag::Unt))))
                        .as_bytes() =>
                {
                    self.stack
                        .push(Value::Opt(OptWrapper::Some(Box::new(Value::Array(array)))));
                }
                t if t
                    == TypeTag::Opt(Arc::new(TypeTag::Array(Arc::new(TypeTag::Float))))
                        .as_bytes() =>
                {
                    self.stack
                        .push(Value::Opt(OptWrapper::Some(Box::new(Value::Array(array)))));
                }
                t if t
                    == TypeTag::Opt(Arc::new(TypeTag::Array(Arc::new(TypeTag::Str))))
                        .as_bytes() =>
                {
                    self.stack
                        .push(Value::Opt(OptWrapper::Some(Box::new(Value::Array(array)))));
                }
                t if t
                    == TypeTag::Opt(Arc::new(TypeTag::Array(Arc::new(TypeTag::Char))))
                        .as_bytes() =>
                {
                    self.stack
                        .push(Value::Opt(OptWrapper::Some(Box::new(Value::Array(array)))));
                }
                t if t
                    == TypeTag::Opt(Arc::new(TypeTag::Array(Arc::new(TypeTag::Void))))
                        .as_bytes() =>
                {
                    self.stack
                        .push(Value::Opt(OptWrapper::Some(Box::new(Value::Array(array)))));
                }
                _ => return self.runtime_err(&format!("invaild wrapping type [{}]", value)),
            }
        } else {
            match type_tag {
                t if t == TypeTag::Opt(Arc::new(Int)).as_bytes() => {
                    self.stack
                        .push(Value::Opt(OptWrapper::Some(Box::new(Value::Int(
                            value.as_int(),
                        )))))
                }
                t if t == TypeTag::Opt(Arc::new(Float)).as_bytes() => {
                    self.stack.push(Opt(OptWrapper::Some(Box::new(Value::Float(
                        value.as_float(),
                    )))));
                }
                t if t == TypeTag::Opt(Arc::new(Unt)).as_bytes() => {
                    self.stack
                        .push(Opt(OptWrapper::Some(Box::new(Value::Unt(value.as_unt())))));
                }
                t if t == TypeTag::Opt(Arc::new(Str)).as_bytes() => {
                    self.stack
                        .push(Opt(OptWrapper::Some(Box::new(Value::Str(value.as_str())))));
                }
                t if t == TypeTag::Opt(Arc::new(Char)).as_bytes() => {
                    self.stack.push(Opt(OptWrapper::Some(Box::new(Value::Char(
                        value.as_char(),
                    )))));
                }
                t if t == TypeTag::Opt(Arc::new(Void)).as_bytes() => {
                    self.stack
                        .push(Opt(OptWrapper::Some(Box::new(Value::Void))));
                }
                _ => return self.runtime_err(&format!("invaild wrapping type [{}]", value)),
            }
        }

        InterpretResult::Ok
    }
}
