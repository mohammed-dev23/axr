use std::ops::DerefMut;

use super::super::*;

impl Vm {
    pub fn op_array(&mut self) -> InterpretResult {
        let mut array: Vec<Value> = Vec::new();
        let array_len = self.read_byte();

        for _ in 0..array_len {
            let value = self.stack.pop().expect(ERR_POP_MES);
            array.push(value);
        }

        array.reverse();
        self.stack.push(Array(Arc::new(Mutex::new(array))));
        InterpretResult::Ok
    }

    pub fn op_array_index(&mut self) -> InterpretResult {
        let index = self.stack.pop().unwrap_or(Void);
        let array = self.stack.pop().unwrap_or(Void);
        let value = array.clone();

        if array.is_array() {
            let array = array.as_array().unwrap_or_else(|| {
                self.runtime_err(&format!("Expected [Array] found [{}]", value));
                Arc::new(Mutex::new(Vec::new()))
            });

            if array.lock().unwrap().len() < index.as_unt() as usize {
                return self.runtime_err("index out of bond");
            }

            self.stack
                .push(array.lock().unwrap()[index.as_unt() as usize].clone());
        }

        InterpretResult::Ok
    }

    pub fn op_push(&mut self) -> InterpretResult {
        let value = self.stack.pop().expect(ERR_POP_MES);
        let array = self.stack.pop().expect(ERR_POP_MES);
        let none = array.clone();

        let array = array.as_array().unwrap_or_else(|| {
            self.runtime_err(&format!("Expected [Array] found [{}]", none));
            Arc::new(Mutex::new(Vec::new()))
        });

        array.lock().unwrap().deref_mut().push(value);
        self.stack.push(Value::Void);
        InterpretResult::Ok
    }

    pub fn op_pop_array(&mut self) -> InterpretResult {
        let array = self.stack.pop().expect(ERR_POP_MES);
        let none = array.clone();

        let array = array.as_array().unwrap_or_else(|| {
            self.runtime_err(&format!("Expected [Array] found [{}]", none));
            Arc::new(Mutex::new(Vec::new()))
        });

        let value = match array.lock().unwrap().deref_mut().pop() {
            Some(v) => OptWrapper::Some(Box::new(v)),
            None => OptWrapper::None,
        };

        self.stack.push(Value::Opt(value));
        InterpretResult::Ok
    }

    pub fn op_len(&mut self) -> InterpretResult {
        let value = self.stack.pop().expect(ERR_POP_MES);

        if value.is_str() {
            let value = value.as_str();
            let len_value = value.len() as u64;

            self.stack.push(Value::Unt(len_value));
        }

        if value.is_array() {
            let none = value.clone();

            let value = value.as_array().unwrap_or_else(|| {
                self.runtime_err(&format!("Expected [Array] found [{}]", none));
                Arc::new(Mutex::new(Vec::new()))
            });

            let len_value = value.lock().unwrap().len() as u64;
            self.stack.push(Value::Unt(len_value));
        }

        InterpretResult::Ok
    }

    pub fn op_reverse(&mut self) -> InterpretResult {
        let value = self.stack.pop().expect(ERR_POP_MES);

        if value.is_str() {
            self.stack.push(Value::Str(Arc::from(
                value.as_str().chars().rev().collect::<String>(),
            )));
        }

        if value.is_array() {
            let array = value.as_array().unwrap();
            array.lock().unwrap().reverse();
        }
        InterpretResult::Ok
    }

    pub fn op_range(&mut self) -> InterpretResult {
        let rhs_value = self.stack.pop().expect(ERR_POP_MES);
        let lhs_value = self.stack.pop().expect(ERR_POP_MES);
        let type_tag = self.get_type();

        match type_tag {
            t if t == &TypeTag::Range(Arc::new(TypeTag::Int)) => {
                self.stack.push(Value::Range(RangeType::RangeInt(
                    lhs_value.as_int()..rhs_value.as_int(),
                )));

                InterpretResult::Ok
            }
            t if t == &TypeTag::Range(Arc::new(TypeTag::Unt)) => {
                self.stack.push(Value::Range(RangeType::RangeUnt(
                    lhs_value.as_unt()..rhs_value.as_unt(),
                )));

                InterpretResult::Ok
            }
            t if t == &TypeTag::Range(Arc::new(TypeTag::Float)) => {
                self.stack.push(Value::Range(RangeType::RangeFloat(
                    lhs_value.as_float()..rhs_value.as_float(),
                )));

                InterpretResult::Ok
            }
            _ => RuntimeError,
        }
    }
}
