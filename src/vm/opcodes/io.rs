use super::super::*;

impl Vm {
    pub fn op_input(&mut self) -> InterpretResult {
        let expected_type = self.read_byte();
        let txt = self.stack.pop().expect(ERR_POP_MES);

        print!("{}", txt);
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let value = match expected_type {
            t if t == Id(TypeId::Int).as_bytes() => Int(input
                .parse::<i64>()
                .ok()
                .ok_or_else(|| self.runtime_err(&format!("Expected int found {}", input)))
                .unwrap()),
            t if t == Id(TypeId::Str).as_bytes() => Str(Arc::from(input)),
            t if t == Id(TypeId::Float).as_bytes() => Value::Float(
                input
                    .parse::<f64>()
                    .ok()
                    .ok_or_else(|| self.runtime_err(&format!("Expected float found {}", input)))
                    .unwrap(),
            ),
            t if t == Id(TypeId::Char).as_bytes() => {
                let into_char: Vec<char> = input.chars().collect();
                let c: Value;

                if into_char.len() != 1 {
                    self.runtime_err(
                        &format!("Char type cannot contain more than one char as an input. Expected char found {}" , input),
                    );
                    return InterpretResult::RuntimeError;
                } else {
                    c = Char(into_char[0]);
                };

                c
            }
            t if t == Id(TypeId::Unt).as_bytes() => Unt(input
                .parse::<u64>()
                .ok()
                .ok_or_else(|| self.runtime_err(&format!("Expected unt found {}", input)))
                .unwrap()),
            _ => Void,
        };

        self.stack.push(value);
        InterpretResult::Ok
    }
}
