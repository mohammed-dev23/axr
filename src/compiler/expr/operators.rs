use crate::compiler::Expr::NoneExpr;

use super::*;

impl Parser {
    pub fn binary(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        let op = self.previous.clone();

        let operator_type = self.previous.token_type;
        let rule = Self::get_rule(operator_type);

        let rhs = self.parse_precedence(rule.precedence, scanner);

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

        Expr::Binary {
            left: Box::new(lhs.clone()),
            operator: op,
            right: Box::new(rhs),
        }
    }

    pub fn unary(&mut self, scanner: &mut Scanner, _can_assign: bool) -> Expr {
        let operator = self.previous.to_owned();

        let rhs = self.parse_precedence(Precedence::Unary, scanner);

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
                type_tag, &operator.token_type
            )),
        }

        Expr::Unary {
            operator,
            right: Box::new(rhs),
        }
    }

    pub fn or_expr(&mut self, _lhs: &Expr, scanner: &mut Scanner) -> Expr {
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

        NoneExpr
    }

    pub fn and_expr(&mut self, _lhs: &Expr, scanner: &mut Scanner) -> Expr {
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

        NoneExpr
    }

    pub fn add_add_expr(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        if !self.info.is_mut.pop().unwrap_or(false) {
            self.error("Value must be mutated in order to use += on it!");
        }

        let type_tag_lhs = self.type_tag.pop().expect(TYPETAG_ERR);
        let rhs = self.parse_precedence(Precedence::Assignment, scanner);
        let type_tag_rhs = self.type_tag.pop().expect(TYPETAG_ERR);

        let idx = match (&type_tag_rhs, &type_tag_lhs) {
            (TypeTag::Int, TypeTag::Int) => {
                self.type_tag.push(TypeTag::Int);
                self.add_type_tag_to_chunk(TypeTag::Int)
            }
            (TypeTag::Unt, TypeTag::Unt) => {
                self.type_tag.push(TypeTag::Unt);
                self.add_type_tag_to_chunk(TypeTag::Unt)
            }
            (TypeTag::Float, &TypeTag::Float) => {
                self.type_tag.push(TypeTag::Float);
                self.add_type_tag_to_chunk(TypeTag::Float)
            }
            (TypeTag::Int | TypeTag::Unt | TypeTag::Float, _) => {
                self.error(&format!(
                    "Missmatched types expected [{}] found [{}]",
                    type_tag_rhs, type_tag_lhs
                ));
                0
            }
            (_, TypeTag::Int | TypeTag::Unt | TypeTag::Float) => {
                self.error(&format!(
                    "Missmatched types expected [{}] found [{}]",
                    type_tag_rhs, type_tag_lhs
                ));
                0
            }
            _ => {
                self.error("modifers like += and -= can be used only on numbers");
                0
            }
        };

        let Expr::Variable { slot } = lhs else {
            self.error("invalid target!");
            return NoneExpr;
        };

        Expr::CompoundAssign {
            slot: *slot,
            operator: TokenType::AddAdd,
            right: Box::new(rhs),
            type_idx: idx,
        }
    }

    pub fn minus_minus_expr(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        if !self.info.is_mut.pop().unwrap_or(false) {
            self.error("Value must be mutated in order to use -= on it!");
        }

        let type_tag_lhs = self.type_tag.pop().expect(TYPETAG_ERR);
        let rhs = self.parse_precedence(Precedence::None, scanner);
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

        let idx = self.add_type_tag_to_chunk(type_tag_lhs);

        let Expr::Variable { slot } = lhs else {
            self.error("invalid target!");
            return NoneExpr;
        };

        Expr::CompoundAssign {
            slot: *slot,
            operator: TokenType::MinusMinus,
            right: Box::new(rhs),
            type_idx: idx,
        }
    }
}
