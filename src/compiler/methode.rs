use super::*;

impl Parser {
    pub fn methode(&mut self, scanner: &mut Scanner) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.consume(TokenType::Identifier, "expected methode name", scanner);
        let methode_name = self.previous.start.clone();

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        match methode_name.trim() {
            "trim" => self.trim_methode(),
            "is_empty" => self.isempty_methode(),
            "rev" => self.rev_methode(),
            "sqrt" => self.sqrt_methode(),
            "round" => self.round_methode(),
            "celi" => self.celi_methode(),
            "floor" => self.floor_methode(),
            "abs" => self.abs_methode(),
            _ => {
                self.error(&format!("The methode [{}] doesn't exsist.", &methode_name));
            }
        }
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);
    }

    pub fn trim_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Str) => {
                self.type_tag.push(Id(TypeId::Str));
            }
            _ => self.error(&format!(
                "cannot use {} for trim, only str values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Trim as u8);
    }

    pub fn isempty_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Str) => {
                self.type_tag.push(Id(TypeId::Bool));
            }
            _ => self.error(&format!(
                "cannot use {} for is_empty, only str values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::IsEmpty as u8);
    }

    pub fn rev_methode(&mut self) {
        if !self.info.is_mut.pop().unwrap_or(false) {
            self.error("Value must be mutated in order to use rev on it!");
        }

        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Str) => {
                self.type_tag.push(Id(TypeId::Str));
            }
            TypeTag::Array(x) => {
                self.type_tag.push(TypeTag::Array(x));
            }
            _ => self.error(&format!(
                "cannot use {} for rev, only str/arrays values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Reverse as u8);

        if let Some(x) = self.info.last_local_slot {
            self.emit_bytes(OpCode::SetLocal as u8, x);
        } else {
            self.error("rev() can only be used directly on a local variable.");
        }
    }

    pub fn sqrt_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Float) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            _ => self.error(&format!(
                "cannot use {} for round, only float values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::SquareRoot as u8);
    }

    pub fn round_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Float) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            _ => self.error(&format!(
                "cannot use {} for round, only float values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Round as u8);
    }

    pub fn celi_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Float) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            _ => self.error(&format!(
                "cannot use {} for ceil, only float values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Ceil as u8);
    }

    pub fn floor_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Float) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            _ => self.error(&format!(
                "cannot use {} for floor, only float values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Floor as u8);
    }

    pub fn abs_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Int) => {
                self.type_tag.push(Id(TypeId::Int));
            }
            Id(TypeId::Float) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            _ => self.error(&format!(
                "cannot use {} for abs, only float/int values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Abs as u8);
    }
}
