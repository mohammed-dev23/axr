use crate::compiler::core::TypeId::Void;

use super::*;

impl Parser {
    pub fn binary(&mut self, scanner: &mut Scanner) {
        let operator_type = self.previous.token_type;
        let rule = Self::get_rule(operator_type);
        self.parse_precedence(rule.precedence, scanner);

        let is_comp = matches!(
            &operator_type,
            TokenType::BangEqual
                | TokenType::EqualEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Lesser
                | TokenType::LesserEqual
        );

        let type_tag2 = self.type_tag.pop().unwrap_or(Id(TypeId::Void));
        let type_tag1 = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match (type_tag1, type_tag2) {
            (Id(TypeId::Int), Id(TypeId::Int))
            | (Id(TypeId::Int), Id(TypeId::Float))
            | (Id(TypeId::Float), Id(TypeId::Int))
            | (Id(TypeId::Str), Id(TypeId::Str))
            | (Id(TypeId::Unt), Id(TypeId::Unt))
            | (Id(TypeId::Unt), Id(TypeId::Float))
            | (Id(TypeId::Float), Id(TypeId::Unt))
            | (Id(TypeId::Float), Id(TypeId::Float))
            | (Id(TypeId::Bool), Id(TypeId::Bool))
            | (Id(TypeId::Char), Id(TypeId::Char))
                if is_comp =>
            {
                self.type_tag.push(Id(TypeId::Bool));
            }
            (Id(TypeId::Int), Id(TypeId::Int)) => {
                self.type_tag.push(Id(TypeId::Int));
            }
            (Id(TypeId::Int), Id(TypeId::Float)) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            (Id(TypeId::Float), Id(TypeId::Int)) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            (Id(TypeId::Str), Id(TypeId::Str)) => {
                self.type_tag.push(Id(TypeId::Str));
            }
            (Id(TypeId::Unt), Id(TypeId::Unt)) => {
                self.type_tag.push(Id(TypeId::Unt));
            }
            (Id(TypeId::Unt), Id(TypeId::Float)) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            (Id(TypeId::Float), Id(TypeId::Unt)) => {
                self.type_tag.push(Id(TypeId::Unt));
            }
            (Id(TypeId::Float), Id(TypeId::Float)) => {
                self.type_tag.push(Id(TypeId::Float));
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

        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Int) => {
                self.type_tag.push(Id(TypeId::Int));
            }
            Id(TypeId::Float) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            Id(TypeId::Bool) => {
                self.type_tag.push(Id(TypeId::Bool));
            }

            _ => self.error(&format!(
                "cannot use [{}] values with [{:?}].",
                type_tag, operator_type
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

        self.info.is_mut.push(is_mut);

        if can_assign && is_mut && self.match_consume(&TokenType::Equal, scanner) {
            self.expression(scanner);

            let rhs_typetag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

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
            self.type_tag.push(Id(TypeId::Float));
        } else if self.expected_type.is_some_and(|t| t == Id(TypeId::Unt)) {
            let unt_value = value.parse::<u64>().unwrap_or_default();
            self.emit_constant(Value::Unt(unt_value));
            self.type_tag.push(Id(TypeId::Unt));
        } else {
            let int_value = value.parse::<i64>();

            if let Ok(int) = int_value {
                self.emit_constant(Value::Int(int));
                self.type_tag.push(Id(TypeId::Int));
            } else {
                let unt_value = value.parse::<u64>().unwrap_or_default();
                self.emit_constant(Value::Unt(unt_value));
                self.type_tag.push(Id(TypeId::Unt));
            }
        }
    }

    pub fn literal(&mut self, _scanner: &mut Scanner, _can_assign: bool) {
        match self.previous.token_type {
            TokenType::True => {
                self.emit_byte(OpCode::True as u8);
                self.type_tag.push(Id(TypeId::Bool));
            }
            TokenType::False => {
                self.emit_byte(OpCode::False as u8);
                self.type_tag.push(Id(TypeId::Bool));
            }
            TokenType::Void => {
                self.emit_byte(OpCode::Void as u8);
                self.type_tag.push(Id(TypeId::Void));
            }
            _ => return,
        }
    }

    pub fn strings(&mut self, _scanner: &mut Scanner, _can_assign: bool) {
        let raw = &self.previous.start;
        let trimmed = &raw[1..raw.len() - 1];
        self.emit_constant(Value::Str(Arc::from(trimmed)));
        self.type_tag.push(Id(TypeId::Str));
    }

    pub fn array(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        let mut array_len = 0;
        let mut type_tag: Vec<TypeTag> = Vec::new();
        let mut typetag: TypeTag = TypeTag::Array(TypeId::Void);

        self.expression(scanner);
        array_len += 1;
        type_tag.push(self.type_tag.pop().unwrap_or(Id(TypeId::Void)));

        while self.current.token_type == TokenType::Comma {
            self.match_consume(&TokenType::Comma, scanner);
            self.expression(scanner);
            array_len += 1;
            type_tag.push(self.type_tag.pop().unwrap_or(Id(TypeId::Void)));
        }

        if let Some(first) = type_tag.first() {
            typetag = *first;

            if !type_tag.iter().all(|t| t == first) {
                self.error("Arrays must contain the same type for all of its slots.");
            }

            if let Some(x) = self.expected_type {
                if x != TypeTag::Array(first.as_typeid()) {
                    self.error(&format!(
                        "Mismatched types, expected {} found Array[{}]",
                        x, first
                    ));
                }
            }
        }

        self.consume(
            TokenType::RightBracket,
            "Expected ']' at the end of array",
            scanner,
        );

        self.emit_byte(OpCode::Array as u8);
        self.emit_byte(array_len as u8);
        self.type_tag.push(TypeTag::Array(typetag.as_typeid()));
    }

    pub fn index_array(&mut self, scanner: &mut Scanner) {
        self.expression(scanner);
        let type_tag = self.type_tag.last().unwrap_or(&Id(TypeId::Void));

        if type_tag != &Id(TypeId::Unt) {
            self.error(&format!(
                "Expected unt type in indexing found [{}]",
                type_tag
            ));
        }

        self.consume(
            TokenType::RightBracket,
            "Expected ']' at the end of array",
            scanner,
        );

        self.emit_byte(OpCode::IndexArray as u8);
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
        self.type_tag.push(Id(TypeId::Char));
    }

    pub fn const_value(&mut self, scanner: &mut Scanner) -> (Value, TypeTag) {
        self.advance(scanner);

        match &self.previous.token_type {
            TokenType::Number => {
                let txt = &self.previous.start;
                if txt.contains('.') {
                    let value = txt.parse::<f64>().unwrap_or(0.0);
                    (Value::Float(value), Id(TypeId::Float))
                } else if self.expected_type.is_some_and(|t| t == Id(TypeId::Unt)) {
                    let value = txt.parse::<u64>().unwrap_or(0);
                    (Value::Unt(value), Id(TypeId::Unt))
                } else {
                    let value = txt.parse::<i64>().unwrap_or(0);
                    (Value::Int(value), Id(TypeId::Int))
                }
            }
            TokenType::String => {
                let raw = &self.previous.start;
                let trimmed = &raw[1..raw.len() - 1];
                (Value::Str(Arc::from(trimmed)), Id(TypeId::Str))
            }
            TokenType::True => (Value::Bool(true), Id(TypeId::Bool)),
            TokenType::False => (Value::Bool(false), Id(TypeId::Bool)),
            TokenType::Char => {
                let raw = &self.previous.start;
                let trimmed = &raw[1..raw.len() - 1];
                let into_chars: Vec<char> = trimmed.chars().collect();

                if into_chars.len() != 1 {
                    self.error("Char type cannot contain more than one char.");
                    return (Value::Void, Id(TypeId::Void));
                }

                (Value::Char(into_chars[0]), Id(TypeId::Char))
            }
            TokenType::LeftBracket => {
                let mut values = Vec::new();

                let (fvalue, ftype_tag) = self.const_value(scanner);
                let ftype_tag = ftype_tag.as_typeid();
                values.push(fvalue);

                while self.current.token_type == TokenType::Comma {
                    self.match_consume(&TokenType::Comma, scanner);
                    let (value, typetag) = self.const_value(scanner);

                    if ftype_tag != typetag.as_typeid() {
                        self.error("Arrays must contain the same type for all of its slots.");
                    }

                    values.push(value);
                }

                let expected_array_type = self.expected_type.take().unwrap_or_else(|| {
                    self.error("Array[Type] annotation needed.");
                    return TypeTag::Array(Void);
                });

                self.consume(
                    TokenType::RightBracket,
                    "Expected ']' at the end of array",
                    scanner,
                );

                (
                    Value::Array(values),
                    TypeTag::Array(expected_array_type.as_typeid()),
                )
            }

            TokenType::Void => (Value::Void, Id(TypeId::Void)),
            _ => {
                self.error("const value must be a literal (number, string, bool, Array,or Void).");
                return (Value::Void, Id(TypeId::Void));
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

    pub fn casting(&mut self, scanner: &mut Scanner) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        self.advance(scanner);
        let token = self.previous.token_type;

        let target = match token {
            TokenType::Int => Id(TypeId::Int),
            TokenType::Unt => Id(TypeId::Unt),
            TokenType::Float => Id(TypeId::Float),
            TokenType::Str => Id(TypeId::Str),
            TokenType::Bool => Id(TypeId::Bool),
            TokenType::Char => Id(TypeId::Char),
            _ => Id(TypeId::Void),
        };

        match (type_tag, target) {
            (Id(TypeId::Str), _) => {
                self.error(&format!("non-primitive cast: `str` to `{}`", target));
            }
            (Id(TypeId::Char), _) => {
                self.error(&format!("non-primitive cast: `char` to `{}`", target));
            }
            (Id(TypeId::Bool), _) => {
                self.error(&format!("non-primitive cast: `bool` to `{}`", target));
            }
            _ => {}
        }

        self.emit_byte(OpCode::Cast as u8);
        self.emit_byte(target.as_bytes());

        self.type_tag.push(target);
    }

    pub fn or_expr(&mut self, scanner: &mut Scanner) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        let else_jump = self.emit_jump(OpCode::JumpIfFalse as usize);
        let end_jump = self.emit_jump(OpCode::Jump as usize);

        self.patch_jump(else_jump as usize);
        self.emit_byte(OpCode::Pop as u8);

        self.parse_precedence(Precedence::Or, scanner);
        let type_tag2 = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match (type_tag, type_tag2) {
            (Id(TypeId::Bool), Id(TypeId::Bool)) => {}
            _ => self.error(&format!(
                "mismatched types cannot use [{}] with [{}]",
                type_tag, type_tag2
            )),
        }

        self.type_tag.push(Id(TypeId::Bool));
        self.patch_jump(end_jump as usize);
    }

    pub fn and_expr(&mut self, scanner: &mut Scanner) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        let end_jump = self.emit_jump(OpCode::JumpIfFalse as usize);
        self.emit_byte(OpCode::Pop as u8);

        self.parse_precedence(Precedence::And, scanner);
        let type_tag2 = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match (type_tag, type_tag2) {
            (Id(TypeId::Bool), Id(TypeId::Bool)) => {}
            _ => self.error(&format!(
                "mismatched types cannot use [{}] with [{}]",
                type_tag, type_tag2
            )),
        }

        self.type_tag.push(Id(TypeId::Bool));
        self.patch_jump(end_jump as usize);
    }
}
