use super::*;

impl Parser {
    pub fn binary(&mut self, scanner: &mut Scanner) {
        let operator_type = self.previous.token_type;
        let rule = Self::get_rule(operator_type);
        self.parse_precedence(rule.precedence, scanner);

        let type_tag2 = self.type_tag.pop().unwrap_or(TypeTag::Void);
        let type_tag1 = self.type_tag.pop().unwrap_or(TypeTag::Void);

        match (type_tag1, type_tag2) {
            (TypeTag::Int, TypeTag::Int) => {
                self.type_tag.push(TypeTag::Int);
            }
            (TypeTag::Int, TypeTag::Float) => {
                self.type_tag.push(TypeTag::Float);
            }
            (TypeTag::Float, TypeTag::Int) => {
                self.type_tag.push(TypeTag::Float);
            }
            (TypeTag::Str, TypeTag::Str) => {
                self.type_tag.push(TypeTag::Str);
            }
            _ => self.error(&format!(
                "mismatched types cannot use [{}] with [{}]",
                type_tag1, type_tag2
            )),
        }

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            TokenType::Modulo => self.emit_byte(OpCode::Modulo as u8),
            TokenType::BangEqual => self.emit_byte(OpCode::NotEqualTo as u8),
            TokenType::EqualEqual => self.emit_byte(OpCode::EqualTo as u8),
            TokenType::Greater => self.emit_byte(OpCode::GreaterThan as u8),
            TokenType::Lesser => self.emit_byte(OpCode::LessThan as u8),
            TokenType::GreaterEqual => self.emit_byte(OpCode::GreaterThanEq as u8),
            TokenType::LesserEqual => self.emit_byte(OpCode::LessThanEq as u8),
            _ => return,
        }
    }

    pub fn grouping(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        self.expression(scanner);
        self.consume(
            TokenType::RigtParen,
            "Expect ')' after expression.",
            scanner,
        );
    }

    pub fn expression(&mut self, scanner: &mut Scanner) {
        self.parse_precedence(Precedence::Assignment, scanner);
    }

    pub fn unary(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(Precedence::Unary, scanner);

        let type_tag = self.type_tag.pop().unwrap_or(TypeTag::Void);

        match type_tag {
            TypeTag::Int | TypeTag::Float => {}

            _ => self.error(&format!(
                "cannot use [{}] values with negate!, only numbers are allowed.",
                type_tag
            )),
        }

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate as u8),
            TokenType::Bang => self.emit_byte(OpCode::Not as u8),
            _ => return,
        }
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
            self.error("");
            return;
        };

        if can_assign && is_mut && self.match_consume(&TokenType::Equal, scanner) {
            self.expression(scanner);

            let rhs_typetag = self.type_tag.pop().unwrap_or(TypeTag::Void);

            if rhs_typetag != type_tag {
                self.error(&format!(
                    "Mismatched types, expected [{}] found [{}]",
                    type_tag, rhs_typetag
                ));
            }

            self.emit_bytes(OpCode::SetLocal as u8, arg);
        } else {
            self.type_tag.push(type_tag);
            self.emit_bytes(OpCode::GetLocal as u8, arg);
        }
    }

    pub fn number(&mut self, _scanner: &mut Scanner, _can_assign: bool) {
        let value = &self.previous.start;

        if value.contains(".") {
            let float_value: f64 = value.parse::<f64>().unwrap_or_default();
            self.emit_constant(Value::Float(float_value));
            self.type_tag.push(TypeTag::Float);
        } else {
            let int_value: i64 = value.parse::<i64>().unwrap_or_default();
            self.emit_constant(Value::Int(int_value));
            self.type_tag.push(TypeTag::Int);
        }
    }

    pub fn literal(&mut self, _scanner: &mut Scanner, _can_assign: bool) {
        match self.previous.token_type {
            TokenType::True => {
                self.emit_byte(OpCode::True as u8);
                self.type_tag.push(TypeTag::Bool);
            }
            TokenType::False => {
                self.emit_byte(OpCode::False as u8);
                self.type_tag.push(TypeTag::Bool);
            }
            TokenType::Void => {
                self.emit_byte(OpCode::Void as u8);
                self.type_tag.push(TypeTag::Void);
            }
            _ => return,
        }
    }

    pub fn strings(&mut self, _scanner: &mut Scanner, _can_assign: bool) {
        let raw = &self.previous.start;
        let trimmed = &raw[1..raw.len() - 1];
        self.emit_constant(Value::Str(Arc::from(trimmed)));
        self.type_tag.push(TypeTag::Str);
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
        self.type_tag.push(TypeTag::Char);
    }

    pub fn const_value(&mut self, scanner: &mut Scanner) -> (Value, TypeTag) {
        self.advance(scanner);

        match &self.previous.token_type {
            TokenType::Number => {
                let txt = &self.previous.start;
                if txt.contains('.') {
                    let value = txt.parse::<f64>().unwrap_or(0.0);
                    (Value::Float(value), TypeTag::Float)
                } else {
                    let value = txt.parse::<i64>().unwrap_or(0);
                    (Value::Int(value), TypeTag::Int)
                }
            }
            TokenType::String => {
                let raw = &self.previous.start;
                let trimmed = &raw[1..raw.len() - 1];
                (Value::Str(Arc::from(trimmed)), TypeTag::Str)
            }
            TokenType::True => (Value::Bool(true), TypeTag::Bool),
            TokenType::False => (Value::Bool(false), TypeTag::Bool),
            TokenType::Char => {
                let raw = &self.previous.start;
                let trimmed = &raw[1..raw.len() - 1];
                let into_chars: Vec<char> = trimmed.chars().collect();

                if into_chars.len() != 1 {
                    self.error("Char type cannot contain more than one char.");
                    return (Value::Void, TypeTag::Void);
                }

                (Value::Char(into_chars[0]), TypeTag::Char)
            }
            TokenType::Void => (Value::Void, TypeTag::Void),
            _ => {
                self.error("const value must be a literal (number, string, bool, or Void).");
                return (Value::Void, TypeTag::Void);
            }
        }
    }

    pub fn parse_variable(&mut self, error_message: &str, scanner: &mut Scanner) -> u8 {
        let is_mut = self.match_consume(&TokenType::Tilde, scanner);
        self.consume(TokenType::Identifier, error_message, scanner);

        self.declare_variable();
        if self.compiler.scope_depth > 0 {
            let idx = self.compiler.local_count as usize - 1;
            self.compiler.locals[idx].is_mut = is_mut;
            return 0;
        }

        let token = self.previous.clone();
        self.identifier_constant(&token)
    }

    pub fn parse_const(&mut self, error_message: &str, scanner: &mut Scanner) -> String {
        self.consume(TokenType::Identifier, error_message, scanner);
        self.previous.start.clone()
    }

    pub fn define_variable(&mut self) {
        self.mark_initialized();
    }

    pub fn define_const(&mut self, name: String, value: Value, type_tag: &TypeTag) {
        self.const_table.insert(name, (value, *type_tag));
    }
}
