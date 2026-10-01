use crate::compiler::Expr::{Grouping, NoneExpr};

use super::*;

impl Parser {
    pub fn grouping(&mut self, scanner: &mut Scanner, _can_assign: bool) -> Expr {
        let inner = self.parse_precedence(Precedence::Assignment, scanner);

        self.consume(
            TokenType::RigtParen,
            "Expect ')' after expression.",
            scanner,
        );

        Grouping(Box::new(inner))
    }

    pub fn variable(&mut self, scanner: &mut Scanner, can_assign: bool) -> Expr {
        let token = &self.previous.clone();
        self.named_variable(token, scanner, can_assign)
    }

    pub fn named_variable(
        &mut self,
        name: &Token,
        scanner: &mut Scanner,
        can_assign: bool,
    ) -> Expr {
        if let Some((value, type_tag)) = self.const_table.get(&name.start).cloned() {
            self.type_tag.push(type_tag);
            return Expr::Const(value);
        }

        let Some((arg, is_mut, type_tag)) = self.resolve_local(name) else {
            self.info.names.push(name.start.clone());

            return Expr::Global {
                name: name.to_owned(),
            };
        };

        self.info.is_mut.push(is_mut);

        if can_assign && is_mut && self.match_consume(&TokenType::Equal, scanner) {
            let rhs = self.parse_precedence(Precedence::Assignment, scanner);

            let rhs_typetag = self.type_tag.pop().expect(TYPETAG_ERR);

            if rhs_typetag != type_tag {
                self.error(&format!(
                    "Mismatched types, expected [{}] found [{}]",
                    type_tag, rhs_typetag
                ));
            }

            Expr::Assign {
                slot: arg,
                right: Box::new(rhs),
            }
        } else {
            self.type_tag.push(type_tag);
            self.info.last_local_slot = Some(arg);
            Expr::Variable { slot: arg }
        }
    }

    pub fn number(&mut self, _scanner: &mut Scanner, _can_assign: bool) -> Expr {
        let value = &self.previous.start;

        if value.contains(".") {
            let float_value: f64 = value.parse::<f64>().unwrap_or_default();
            self.type_tag.push(TypeTag::Float);
            Expr::Literal(Value::Float(float_value))
        } else if self
            .expected_type
            .clone()
            .is_some_and(|t| t == TypeTag::Unt)
        {
            let unt_value = value.parse::<u64>().unwrap_or_default();
            self.type_tag.push(TypeTag::Unt);
            Expr::Literal(Value::Unt(unt_value))
        } else {
            let int_value = value.parse::<i64>();

            if let Ok(int) = int_value {
                self.type_tag.push(TypeTag::Int);
                Expr::Literal(Value::Int(int))
            } else {
                let unt_value = value.parse::<u64>().unwrap_or_default();
                self.type_tag.push(TypeTag::Unt);
                Expr::Literal(Value::Unt(unt_value))
            }
        }
    }

    pub fn literal(&mut self, _scanner: &mut Scanner, _can_assign: bool) -> Expr {
        match self.previous.token_type {
            TokenType::True => {
                self.type_tag.push(TypeTag::Bool);
                Expr::Literal(Value::Bool(true))
            }
            TokenType::False => {
                self.type_tag.push(TypeTag::Bool);
                Expr::Literal(Value::Bool(false))
            }
            TokenType::Void => {
                self.type_tag.push(TypeTag::Void);
                Expr::Literal(Value::Void)
            }
            TokenType::None => {
                self.type_tag.push(TypeTag::Opt(Arc::new(TypeTag::None)));
                Expr::Literal(Value::Void)
            }
            _ => {
                return {
                    self.error(&format!(
                        "Unexpected token '{:?}' ",
                        &self.previous.token_type
                    ));
                    NoneExpr
                };
            }
        }
    }

    pub fn strings(&mut self, _scanner: &mut Scanner, _can_assign: bool) -> Expr {
        let raw = &self.previous.start;
        let trimmed = &raw[1..raw.len() - 1];
        self.type_tag.push(TypeTag::Str);
        Expr::Literal(Value::Str(Arc::from(trimmed)))
    }

    pub fn char(&mut self, _scanner: &mut Scanner, _can_assign: bool) -> Expr {
        let raw = &self.previous.start;
        let trimmed = &raw[1..raw.len() - 1];
        let into_chars: Vec<char> = trimmed.chars().collect();

        if into_chars.len() != 1 {
            self.error("Char type cannot contain more than one char.");
            return NoneExpr;
        }

        self.type_tag.push(TypeTag::Char);
        Expr::Literal(Value::Char(into_chars[0]))
    }

    pub fn range(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        let lhs_typetag = self.type_tag.pop().expect(TYPETAG_ERR);

        let rhs = self.parse_precedence(Precedence::Term, scanner);

        let rhs_typetag = self.type_tag.pop().expect(TYPETAG_ERR);

        if lhs_typetag != rhs_typetag {
            self.error(&format!(
                "Expected '{}' due to '{}'",
                lhs_typetag, lhs_typetag
            ));
        }

        let range_type = match (lhs_typetag, rhs_typetag) {
            (TypeTag::Int, TypeTag::Int) => TypeTag::Range(Arc::new(TypeTag::Int)),
            (TypeTag::Unt, TypeTag::Unt) => TypeTag::Range(Arc::new(TypeTag::Unt)),
            (TypeTag::Float, TypeTag::Float) => TypeTag::Range(Arc::new(TypeTag::Float)),
            _ => {
                return {
                    self.error("Unexpected range type!");
                    NoneExpr
                };
            }
        };

        self.type_tag.push(range_type.clone());
        let idx = self.add_type_tag_to_chunk(range_type);

        Expr::Range {
            left: Box::new(lhs.to_owned()),
            right: Box::new(rhs),
            type_idx: idx,
        }
    }
}
