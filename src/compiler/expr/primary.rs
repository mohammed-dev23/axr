use super::*;

impl Parser {
    pub fn grouping(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        self.expression(scanner);
        self.consume(
            TokenType::RigtParen,
            "Expect ')' after expression.",
            scanner,
        );
    }

    pub fn variable(&mut self, scanner: &mut Scanner, can_assign: bool) {
        let token = &self.previous.clone();
        self.named_variable(token, scanner, can_assign);
    }

    pub fn named_variable(&mut self, name: &Token, scanner: &mut Scanner, can_assign: bool) {
        if let Some((value, type_tag)) = self.const_table.get(&name.start).cloned() {
            self.emit_constant(value);
            self.type_tag.push(type_tag);
            return;
        }

        let Some((arg, is_mut, type_tag)) = self.resolve_local(name) else {
            let slot = self.identifier_constant(name);
            self.emit_bytes(OpCode::GetGlobal as u8, slot);
            return;
        };

        self.info.is_mut.push(is_mut);

        if can_assign && is_mut && self.match_consume(&TokenType::Equal, scanner) {
            self.expression(scanner);

            let rhs_typetag = self.type_tag.pop().expect(TYPETAG_ERR);

            if rhs_typetag != type_tag {
                self.error(&format!(
                    "Mismatched types, expected [{}] found [{}]",
                    type_tag, rhs_typetag
                ));
            }

            self.emit_bytes(OpCode::SetLocal as u8, arg);
        } else {
            self.type_tag.push(type_tag);
            self.info.last_local_slot = Some(arg);
            self.emit_bytes(OpCode::GetLocal as u8, arg);
        }
    }

    pub fn number(&mut self, _scanner: &mut Scanner, _can_assign: bool) {
        let value = &self.previous.start;

        if value.contains(".") {
            let float_value: f64 = value.parse::<f64>().unwrap_or_default();
            self.emit_constant(Value::Float(float_value));
            self.type_tag.push(Wrappers::None(Id(TypeId::Float)));
        } else if self
            .expected_type
            .is_some_and(|t| t == Wrappers::None(Id(TypeId::Unt)))
        {
            let unt_value = value.parse::<u64>().unwrap_or_default();
            self.emit_constant(Value::Unt(unt_value));
            self.type_tag.push(Wrappers::None(Id(TypeId::Unt)));
        } else {
            let int_value = value.parse::<i64>();

            if let Ok(int) = int_value {
                self.emit_constant(Value::Int(int));
                self.type_tag.push(Wrappers::None(Id(TypeId::Int)));
            } else {
                let unt_value = value.parse::<u64>().unwrap_or_default();
                self.emit_constant(Value::Unt(unt_value));
                self.type_tag.push(Wrappers::None(Id(TypeId::Unt)));
            }
        }
    }

    pub fn literal(&mut self, _scanner: &mut Scanner, _can_assign: bool) {
        match self.previous.token_type {
            TokenType::True => {
                self.emit_byte(OpCode::True as u8);
                self.type_tag.push(Wrappers::None(Id(TypeId::Bool)));
            }
            TokenType::False => {
                self.emit_byte(OpCode::False as u8);
                self.type_tag.push(Wrappers::None(Id(TypeId::Bool)));
            }
            TokenType::Void => {
                self.emit_byte(OpCode::Void as u8);
                self.type_tag.push(Wrappers::None(Id(TypeId::Void)));
            }
            TokenType::None => {
                self.emit_byte(OpCode::None as u8);
                self.type_tag.push(Wrappers::Opt(TypeTag::Id(TypeId::None)));
            }
            _ => return,
        }
    }

    pub fn strings(&mut self, _scanner: &mut Scanner, _can_assign: bool) {
        let raw = &self.previous.start;
        let trimmed = &raw[1..raw.len() - 1];
        self.emit_constant(Value::Str(Arc::from(trimmed)));
        self.type_tag.push(Wrappers::None(Id(TypeId::Str)));
    }

    pub fn char(&mut self, _scanner: &mut Scanner, _can_assign: bool) {
        let raw = &self.previous.start;
        let trimmed = &raw[1..raw.len() - 1];
        let into_chars: Vec<char> = trimmed.chars().collect();

        if into_chars.len() != 1 {
            self.error("Char type cannot contain more than one char.");
            return;
        }

        self.emit_constant(Value::Char(into_chars[0]));
        self.type_tag.push(Wrappers::None(Id(TypeId::Char)));
    }
}
