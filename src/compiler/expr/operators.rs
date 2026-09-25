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

        let type_tag2 = self.type_tag.pop().expect(TYPETAG_ERR);
        let type_tag1 = self.type_tag.pop().expect(TYPETAG_ERR);

        match (type_tag1.clone(), type_tag2.clone()) {
            (TypeTag::Int, TypeTag::Int)
            | (TypeTag::Int, TypeTag::Float)
            | (TypeTag::Float, TypeTag::Int)
            | (TypeTag::Str, TypeTag::Str)
            | (TypeTag::Unt, TypeTag::Unt)
            | (TypeTag::Unt, TypeTag::Float)
            | (TypeTag::Float, TypeTag::Unt)
            | (TypeTag::Float, TypeTag::Float)
            | (TypeTag::Bool, TypeTag::Bool)
            | (TypeTag::Char, TypeTag::Char)
                if is_comp =>
            {
                self.type_tag.push(TypeTag::Bool);
            }
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
            (TypeTag::Unt, TypeTag::Unt) => {
                self.type_tag.push(TypeTag::Unt);
            }
            (TypeTag::Unt, TypeTag::Float) => {
                self.type_tag.push(TypeTag::Float);
            }
            (TypeTag::Float, TypeTag::Unt) => {
                self.type_tag.push(TypeTag::Unt);
            }
            (TypeTag::Float, TypeTag::Float) => {
                self.type_tag.push(TypeTag::Float);
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

    pub fn unary(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(Precedence::Unary, scanner);

        let type_tag = self.type_tag.pop().expect(TYPETAG_ERR);

        match type_tag {
            TypeTag::Int => {
                self.type_tag.push(TypeTag::Int);
            }
            TypeTag::Float => {
                self.type_tag.push(TypeTag::Float);
            }
            TypeTag::Bool => {
                self.type_tag.push(TypeTag::Bool);
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

    pub fn or_expr(&mut self, scanner: &mut Scanner) {
        let type_tag = self.type_tag.pop().unwrap_or(TypeTag::Void);

        let else_jump = self.emit_jump(OpCode::JumpIfFalse as usize);
        let end_jump = self.emit_jump(OpCode::Jump as usize);

        self.patch_jump(else_jump as usize);
        self.emit_byte(OpCode::Pop as u8);

        self.parse_precedence(Precedence::Or, scanner);
        let type_tag2 = self.type_tag.pop().unwrap_or(TypeTag::Void);

        match (type_tag.clone(), type_tag2.clone()) {
            (TypeTag::Bool, TypeTag::Bool) => {}
            _ => self.error(&format!(
                "mismatched types cannot use [{}] with [{}]",
                type_tag, type_tag2
            )),
        }

        self.type_tag.push(TypeTag::Bool);
        self.patch_jump(end_jump as usize);
    }

    pub fn and_expr(&mut self, scanner: &mut Scanner) {
        let type_tag = self.type_tag.pop().unwrap_or(TypeTag::Void);

        let end_jump = self.emit_jump(OpCode::JumpIfFalse as usize);
        self.emit_byte(OpCode::Pop as u8);

        self.parse_precedence(Precedence::And, scanner);
        let type_tag2 = self.type_tag.pop().unwrap_or(TypeTag::Void);

        match (&type_tag, &type_tag2) {
            (TypeTag::Bool, TypeTag::Bool) => {}
            _ => self.error(&format!(
                "mismatched types cannot use [{}] with [{}]",
                type_tag, type_tag2
            )),
        }

        self.type_tag.push(TypeTag::Bool);
        self.patch_jump(end_jump as usize);
    }

    pub fn add_add_expr(&mut self, scanner: &mut Scanner) {
        if !self.info.is_mut.pop().unwrap_or(false) {
            self.error("Value must be mutated in order to use += on it!");
        }

        let type_tag_lhs = self.type_tag.pop().expect(TYPETAG_ERR);
        self.expression(scanner);
        let type_tag_rhs = self.type_tag.pop().expect(TYPETAG_ERR);

        match (&type_tag_rhs, &type_tag_lhs) {
            (TypeTag::Int, TypeTag::Int) => {
                self.type_tag.push(TypeTag::Int);
            }
            (TypeTag::Unt, TypeTag::Unt) => {
                self.type_tag.push(TypeTag::Unt);
            }
            (TypeTag::Float, &TypeTag::Float) => {
                self.type_tag.push(TypeTag::Float);
            }
            (TypeTag::Int | TypeTag::Unt | TypeTag::Float, _) => {
                self.error(&format!(
                    "Missmatched types expected [{}] found [{}]",
                    type_tag_rhs, type_tag_lhs
                ));
            }
            (_, TypeTag::Int | TypeTag::Unt | TypeTag::Float) => {
                self.error(&format!(
                    "Missmatched types expected [{}] found [{}]",
                    type_tag_rhs, type_tag_lhs
                ));
            }
            _ => {
                self.error("modifers like += and -= can be used only on numbers");
            }
        }

        self.emit_byte(OpCode::AddAdd as u8);
        let idx = self.add_type_tag_to_chunk(type_tag_lhs);
        self.emit_byte(idx);

        if let Some(x) = self.info.last_local_slot {
            self.emit_bytes(OpCode::SetLocal as u8, x);
        } else {
            self.error("'+=' can only be used directly on an var.");
        }
    }

    pub fn minus_minus_expr(&mut self, scanner: &mut Scanner) {
        if !self.info.is_mut.pop().unwrap_or(false) {
            self.error("Value must be mutated in order to use -= on it!");
        }

        let type_tag_lhs = self.type_tag.pop().expect(TYPETAG_ERR);
        self.expression(scanner);
        let type_tag_rhs = self.type_tag.pop().expect(TYPETAG_ERR);

        match (&type_tag_rhs, &type_tag_lhs) {
            (TypeTag::Int, TypeTag::Int) => {
                self.type_tag.push(TypeTag::Int);
            }
            (TypeTag::Unt, TypeTag::Unt) => {
                self.type_tag.push(TypeTag::Unt);
            }
            (TypeTag::Float, TypeTag::Float) => {
                self.type_tag.push(TypeTag::Float);
            }
            (TypeTag::Int | TypeTag::Unt | TypeTag::Float, _) => {
                self.error(&format!(
                    "Missmatched types expected [{}] found [{}]",
                    type_tag_rhs, type_tag_lhs
                ));
            }
            (_, TypeTag::Int | TypeTag::Unt | TypeTag::Float) => {
                self.error(&format!(
                    "Missmatched types expected [{}] found [{}]",
                    type_tag_rhs, type_tag_lhs
                ));
            }
            _ => {
                self.error("modifers like += and -= can be used only on numbers");
            }
        }

        self.emit_byte(OpCode::MinusMinus as u8);

        let idx = self.add_type_tag_to_chunk(type_tag_lhs);
        self.emit_byte(idx);

        if let Some(x) = self.info.last_local_slot {
            self.emit_bytes(OpCode::SetLocal as u8, x);
        } else {
            self.error("'-=' can only be used directly on an var.");
        }
    }
}
