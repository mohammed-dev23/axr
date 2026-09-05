use super::*;
use crate::compiler::core::TypeTag::Void;

// those are expr that are meant to be fn calls
// must make them as fn calls later when fns are done
impl Parser {
    pub fn abs_expr(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        self.expression(scanner);
        let type_tag = self.type_tag.pop().unwrap_or(Void);
        match type_tag {
            TypeTag::Int => {
                self.type_tag.push(TypeTag::Int);
            }
            TypeTag::Float => {
                self.type_tag.push(TypeTag::Float);
            }
            _ => self.error(&format!(
                "cannot use {} for abs, only float/int values that are allowed",
                type_tag
            )),
        }
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);

        self.emit_byte(OpCode::Abs as u8);
    }

    pub fn floor_expr(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        self.expression(scanner);
        let type_tag = self.type_tag.pop().unwrap_or(Void);
        match type_tag {
            TypeTag::Float => {
                self.type_tag.push(TypeTag::Float);
            }
            _ => self.error(&format!(
                "cannot use {} for floor, only float values that are allowed",
                type_tag
            )),
        }

        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);

        self.emit_byte(OpCode::Floor as u8);
    }

    pub fn ceil_expr(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        self.expression(scanner);
        let type_tag = self.type_tag.pop().unwrap_or(Void);
        match type_tag {
            TypeTag::Float => {
                self.type_tag.push(TypeTag::Float);
            }
            _ => self.error(&format!(
                "cannot use {} for ceil, only float values that are allowed",
                type_tag
            )),
        }
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);

        self.emit_byte(OpCode::Ceil as u8);
    }

    pub fn round_expr(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        self.expression(scanner);
        let type_tag = self.type_tag.pop().unwrap_or(Void);
        match type_tag {
            TypeTag::Float => {
                self.type_tag.push(TypeTag::Float);
            }
            _ => self.error(&format!(
                "cannot use {} for round, only float values that are allowed",
                type_tag
            )),
        }
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);

        self.emit_byte(OpCode::Round as u8);
    }

    pub fn squareroot_expr(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        self.expression(scanner);
        let type_tag = self.type_tag.pop().unwrap_or(Void);
        match type_tag {
            TypeTag::Float => {
                self.type_tag.push(TypeTag::Float);
            }
            TypeTag::Int => {
                self.type_tag.push(TypeTag::Int);
            }
            _ => self.error(&format!(
                "cannot use {} for round, only float/int values that are allowed",
                type_tag
            )),
        }
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);

        self.emit_byte(OpCode::SquareRoot as u8);
    }

    pub fn isempty_expr(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        self.expression(scanner);
        let type_tag = self.type_tag.pop().unwrap_or(Void);
        match type_tag {
            TypeTag::Str => {
                self.type_tag.push(TypeTag::Bool);
            }
            _ => self.error(&format!(
                "cannot use {} for is_empty, only str values that are allowed",
                type_tag
            )),
        }
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);

        self.emit_byte(OpCode::IsEmpty as u8);
    }

    pub fn trim_expr(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        self.expression(scanner);
        let type_tag = self.type_tag.pop().unwrap_or(Void);
        match type_tag {
            TypeTag::Str => {
                self.type_tag.push(TypeTag::Str);
            }
            _ => self.error(&format!(
                "cannot use {} for trim, only str values that are allowed",
                type_tag
            )),
        }
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);

        self.emit_byte(OpCode::Trim as u8);
    }

    pub fn rev_expr(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        self.expression(scanner);
        let type_tag = self.type_tag.pop().unwrap_or(Void);
        match type_tag {
            TypeTag::Str => {
                self.type_tag.push(TypeTag::Str);
            }
            _ => self.error(&format!(
                "cannot use {} for rev, only str values that are allowed",
                type_tag
            )),
        }
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);

        self.emit_byte(OpCode::Reverse as u8);
    }

    pub fn input_expr(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        let expected_type = self.expected_type.take().unwrap_or_else(|| {
            self.error("input() needs a type context, e.g. `let x : str = input();`");
            Void
        });

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        self.expression(scanner);
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);

        self.emit_byte(OpCode::Input as u8);
        self.emit_byte(expected_type as u8);
        self.type_tag.push(expected_type);
    }
}
