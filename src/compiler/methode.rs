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
            "push" => self.push_methode(scanner),
            "pop" => self.pop_methode(),
            "len" => self.len_methode(),
            "grab" => self.grab_methode(),
            _ => {
                self.error(&format!("The methode [{}] doesn't exsist.", &methode_name));
            }
        }
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);
    }

    pub fn trim_methode(&mut self) {
        let type_tag = self
            .type_tag
            .pop()
            .unwrap_or(Wrappers::None(Id(TypeId::Void)));

        match type_tag {
            Wrappers::None(Id(TypeId::Str)) => {
                self.type_tag.push(Wrappers::None(Id(TypeId::Str)));
            }
            _ => self.error(&format!(
                "cannot use {} for trim, only str values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Trim as u8);
    }

    pub fn isempty_methode(&mut self) {
        let type_tag = self
            .type_tag
            .pop()
            .unwrap_or(Wrappers::None(Id(TypeId::Void)));

        match type_tag {
            Wrappers::None(Id(TypeId::Str)) => {
                self.type_tag.push(Wrappers::None(Id(TypeId::Bool)));
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

        let type_tag = self
            .type_tag
            .pop()
            .unwrap_or(Wrappers::None(Id(TypeId::Void)));

        match type_tag {
            Wrappers::None(Id(TypeId::Str)) => {
                self.type_tag.push(Wrappers::None(Id(TypeId::Str)));
            }
            Wrappers::None(TypeTag::Array(x)) => {
                self.type_tag.push(Wrappers::None(TypeTag::Array(x)));
            }
            _ => self.error(&format!(
                "cannot use {} for rev, only str/arrays values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Reverse as u8);
    }

    pub fn sqrt_methode(&mut self) {
        let type_tag = self
            .type_tag
            .pop()
            .unwrap_or(Wrappers::None(Id(TypeId::Void)));

        match type_tag {
            Wrappers::None(Id(TypeId::Float)) => {
                self.type_tag.push(Wrappers::None(Id(TypeId::Float)));
            }
            _ => self.error(&format!(
                "cannot use {} for round, only float values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::SquareRoot as u8);
    }

    pub fn round_methode(&mut self) {
        let type_tag = self
            .type_tag
            .pop()
            .unwrap_or(Wrappers::None(Id(TypeId::Void)));

        match type_tag {
            Wrappers::None(Id(TypeId::Float)) => {
                self.type_tag.push(Wrappers::None(Id(TypeId::Float)));
            }
            _ => self.error(&format!(
                "cannot use {} for round, only float values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Round as u8);
    }

    pub fn celi_methode(&mut self) {
        let type_tag = self
            .type_tag
            .pop()
            .unwrap_or(Wrappers::None(Id(TypeId::Void)));

        match type_tag {
            Wrappers::None(Id(TypeId::Float)) => {
                self.type_tag.push(Wrappers::None(Id(TypeId::Float)));
            }
            _ => self.error(&format!(
                "cannot use {} for ceil, only float values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Ceil as u8);
    }

    pub fn floor_methode(&mut self) {
        let type_tag = self
            .type_tag
            .pop()
            .unwrap_or(Wrappers::None(Id(TypeId::Void)));

        match type_tag {
            Wrappers::None(Id(TypeId::Float)) => {
                self.type_tag.push(Wrappers::None(Id(TypeId::Float)));
            }
            _ => self.error(&format!(
                "cannot use {} for floor, only float values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Floor as u8);
    }

    pub fn abs_methode(&mut self) {
        let type_tag = self
            .type_tag
            .pop()
            .unwrap_or(Wrappers::None(Id(TypeId::Void)));

        match type_tag {
            Wrappers::None(Id(TypeId::Int)) => {
                self.type_tag.push(Wrappers::None(Id(TypeId::Int)));
            }
            Wrappers::None(Id(TypeId::Float)) => {
                self.type_tag.push(Wrappers::None(Id(TypeId::Float)));
            }
            _ => self.error(&format!(
                "cannot use {} for abs, only float/int values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Abs as u8);
    }

    pub fn push_methode(&mut self, scanner: &mut Scanner) {
        if !self.info.is_mut.pop().unwrap_or(false) {
            self.error("Value must be mutated in order to use push() on it!");
        }

        let array_type = self
            .type_tag
            .pop()
            .unwrap_or(Wrappers::None(TypeTag::Array(Void)));
        self.expression(scanner);
        let values_typetag = self.type_tag.pop().expect(TYPETAG_ERR);

        match (array_type, values_typetag) {
            (Wrappers::None(TypeTag::Array(TypeId::Int)), Wrappers::None(Id(TypeId::Int)))
            | (Wrappers::None(TypeTag::Array(TypeId::Unt)), Wrappers::None(Id(TypeId::Unt)))
            | (Wrappers::None(TypeTag::Array(TypeId::Float)), Wrappers::None(Id(TypeId::Float)))
            | (Wrappers::None(TypeTag::Array(TypeId::Str)), Wrappers::None(Id(TypeId::Str)))
            | (Wrappers::None(TypeTag::Array(TypeId::Char)), Wrappers::None(Id(TypeId::Char)))
            | (Wrappers::None(TypeTag::Array(TypeId::Bool)), Wrappers::None(Id(TypeId::Bool)))
            | (Wrappers::None(TypeTag::Array(TypeId::Void)), Wrappers::None(Id(TypeId::Void))) => {}
            _ => {
                self.error(&format!(
                    "Mismatched types array for type [{}] found [{}]",
                    array_type, values_typetag
                ));
            }
        }

        self.emit_byte(OpCode::Push as u8);
    }

    pub fn pop_methode(&mut self) {
        if !self.info.is_mut.pop().unwrap_or(false) {
            self.error("Value must be mutated in order to use push() on it!");
        }

        let type_tag = self
            .type_tag
            .pop()
            .unwrap_or(Wrappers::None(Array(Void)))
            .as_typeid();

        self.type_tag.push(Wrappers::Opt(Id(type_tag)));
        self.emit_byte(OpCode::PopArray as u8);
    }

    pub fn len_methode(&mut self) {
        let type_tag = self.type_tag.pop().expect(TYPETAG_ERR);

        match type_tag {
            Wrappers::None(Array(_)) => {}
            Wrappers::None(Id(TypeId::Str)) => {}
            _ => {
                self.error(&format!("Can not use len() on [{}]", type_tag));
            }
        }

        self.emit_byte(OpCode::Len as u8);
        self.type_tag.push(Wrappers::None(Id(TypeId::Unt)));
    }

    pub fn grab_methode(&mut self) {
        let type_tag = self.type_tag.pop().expect(TYPETAG_ERR);

        match type_tag {
            Wrappers::Opt(x) => match x {
                TypeTag::Id(x) => {
                    self.type_tag.push(Wrappers::None(Id(x)));
                }
                TypeTag::Array(x) => {
                    self.type_tag.push(Wrappers::None(Array(x)));
                }
            },
            _ => {
                self.error("Expect 'Opt' type wrapper to use 'grab()' one");
            }
        }

        self.emit_byte(OpCode::Grab as u8);
    }
}
