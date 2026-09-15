use super::*;

impl Parser {
    pub fn declaration(&mut self, scanner: &mut Scanner) {
        self.statement(scanner);

        if self.painc_mode {
            self.synchronize(scanner);
        }
    }

    pub fn fn_declaration(&mut self, scanner: &mut Scanner) {
        self.consume(TokenType::Identifier, "Expect a functions name", scanner);
        let function_name = self.previous.start.clone();

        let global_slot = if self.compiler.scope_depth == 0 {
            self.declare_variable();
            Some(self.identifier_constant(&self.previous.clone()))
        } else {
            self.declare_variable();
            self.mark_initialized();
            None
        };

        self.function(FunctionType::Function, scanner, &function_name);

        if let Some(x) = global_slot {
            self.emit_bytes(OpCode::DefineGlobal as u8, x);
        }
    }

    pub fn variable_declaration(&mut self, scanner: &mut Scanner) {
        if self.compiler.scope_depth == 0 {
            self.error("Statements must be insaid a fn body");
            return;
        }

        self.parse_variable("Expect variable name.", scanner);

        let type_annotation = self.match_consume(&TokenType::Colon, scanner);

        let annotation_type = if type_annotation {
            self.advance(scanner);
            Some(self.previous.token_type)
        } else {
            None
        };

        let mut is_opt = false;

        let (array, mut is_array) = if annotation_type.is_some_and(|t| t == TokenType::Array) {
            self.parse_array_typetag(scanner)
        } else {
            (TypeTag::Array(TypeId::Void), false)
        };

        let opt = if annotation_type.is_some_and(|t| t == TokenType::Opt) {
            self.consume(TokenType::LeftBracket, "Exp", scanner);

            self.advance(scanner);

            let opt = match self.previous.token_type {
                TokenType::Int => Wrappers::Opt(Id(TypeId::Int)),
                TokenType::Unt => Wrappers::Opt(Id(TypeId::Unt)),
                TokenType::Float => Wrappers::Opt(Id(TypeId::Float)),
                TokenType::Str => Wrappers::Opt(Id(TypeId::Str)),
                TokenType::Char => Wrappers::Opt(Id(TypeId::Char)),
                TokenType::Array => {
                    is_array = true;
                    Wrappers::Opt(self.parse_array_typetag(scanner).0)
                }
                _ => Wrappers::Opt(Id(TypeId::Void)),
            };

            self.consume(TokenType::RightBracket, "exp", scanner);
            is_opt = true;
            opt
        } else {
            Wrappers::Opt(TypeTag::Id(Void))
        };

        self.expected_type = annotation_type.map(|t| match t {
            TokenType::Int => Wrappers::None(Id(TypeId::Int)),
            TokenType::Str => Wrappers::None(Id(TypeId::Str)),
            TokenType::Bool => Wrappers::None(Id(TypeId::Bool)),
            TokenType::Float => Wrappers::None(Id(TypeId::Float)),
            TokenType::Char => Wrappers::None(Id(TypeId::Char)),
            TokenType::Unt => Wrappers::None(Id(TypeId::Unt)),
            TokenType::Array => Wrappers::None(array),
            TokenType::Opt => opt,
            _ => Wrappers::None(Id(TypeId::Void)),
        });

        if self.match_consume(&TokenType::Equal, scanner) {
            self.expression(scanner);
        } else {
            self.emit_byte(OpCode::Void as u8);
            self.type_tag.push(Wrappers::None(Id(TypeId::Void)));
        }

        let type_tag = self.type_tag.pop().expect(TYPETAG_ERR);

        self.compiler.locals[self.compiler.local_count as usize - 1].type_tag = type_tag;

        if let Some(token) = annotation_type {
            match token {
                TokenType::Opt => {
                    if type_tag != Wrappers::Opt(TypeTag::Id(TypeId::None)) && type_tag != opt {
                        self.error(&format!(
                            "Mismatched types, expected [{}] found [{}]",
                            opt, type_tag
                        ));
                    }
                }
                TokenType::Array => {
                    let array = Wrappers::None(array);

                    if type_tag != array {
                        self.error(&format!(
                            "Mismatched types, expected [{}] found [{}]",
                            opt, type_tag
                        ));
                    }
                }
                _ => self.type_check(&type_tag, &token, is_array, is_opt),
            }
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            scanner,
        );

        self.define_variable();
    }

    pub fn const_declaration(&mut self, scanner: &mut Scanner) {
        let const_name = self.parse_const("Expect const name.", scanner);

        if !const_name.chars().all(|c| c.is_uppercase()) {
            self.error("Const name must be all in uppercase");
            return;
        }

        self.consume(
            TokenType::Colon,
            "expected a type annotation for const values",
            scanner,
        );

        self.advance(scanner);
        let annotation_type = self.previous.token_type;

        let mut is_array = false;
        let mut is_opt = false;

        let array = if annotation_type == TokenType::Array {
            self.consume(TokenType::LeftBracket, "Exp", scanner);

            let array = match self.current.token_type {
                TokenType::Int => Wrappers::None(TypeTag::Array(TypeId::Int)),
                TokenType::Unt => Wrappers::None(TypeTag::Array(TypeId::Unt)),
                TokenType::Float => Wrappers::None(TypeTag::Array(TypeId::Float)),
                TokenType::Str => Wrappers::None(TypeTag::Array(TypeId::Str)),
                TokenType::Bool => Wrappers::None(TypeTag::Array(TypeId::Bool)),
                TokenType::Char => Wrappers::None(TypeTag::Array(TypeId::Char)),
                _ => Wrappers::None(TypeTag::Array(TypeId::Void)),
            };

            self.advance(scanner);
            self.consume(TokenType::RightBracket, "Exp", scanner);
            is_array = true;
            array
        } else {
            Wrappers::None(Array(Void))
        };

        let opt = if annotation_type == TokenType::Opt {
            self.consume(TokenType::LeftBracket, "Exp", scanner);

            let opt = match self.current.token_type {
                TokenType::Int => Wrappers::Opt(TypeTag::Id(TypeId::Int)),
                TokenType::Unt => Wrappers::Opt(TypeTag::Id(TypeId::Unt)),
                TokenType::Float => Wrappers::Opt(TypeTag::Id(TypeId::Float)),
                TokenType::Str => Wrappers::Opt(TypeTag::Id(TypeId::Str)),
                TokenType::Bool => Wrappers::Opt(TypeTag::Id(TypeId::Bool)),
                TokenType::Char => Wrappers::Opt(TypeTag::Id(TypeId::Char)),
                TokenType::Array => Wrappers::Opt(array.extract()),
                _ => Wrappers::None(TypeTag::Id(TypeId::Void)),
            };

            self.advance(scanner);
            self.consume(TokenType::RightBracket, "Exp", scanner);
            is_opt = true;
            opt
        } else {
            Wrappers::Opt(Id(Void))
        };

        self.expected_type = match annotation_type {
            TokenType::Int => Some(Wrappers::None(Id(TypeId::Int))),
            TokenType::Str => Some(Wrappers::None(Id(TypeId::Str))),
            TokenType::Bool => Some(Wrappers::None(Id(TypeId::Bool))),
            TokenType::Float => Some(Wrappers::None(Id(TypeId::Float))),
            TokenType::Char => Some(Wrappers::None(Id(TypeId::Char))),
            TokenType::Unt => Some(Wrappers::None(Id(TypeId::Unt))),
            TokenType::Array => Some(array),
            TokenType::Opt => Some(opt),
            _ => Some(Wrappers::None(Id(TypeId::Void))),
        };

        self.consume(TokenType::Equal, "Expect '=' after const name.", scanner);

        let (const_value, type_tag) = self.const_value(scanner);

        match annotation_type {
            TokenType::Opt => {
                if type_tag != Wrappers::Opt(TypeTag::Id(TypeId::None)) && type_tag != opt {
                    self.error(&format!(
                        "Mismatched types, expected [{}] found [{}]",
                        opt, type_tag
                    ));
                }
            }
            TokenType::Array => {
                if type_tag != array {
                    self.error(&format!(
                        "Mismatched types, expected [{}] found [{}]",
                        opt, type_tag
                    ));
                }
            }
            _ => self.type_check(&type_tag, &annotation_type, is_array, is_opt),
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            scanner,
        );

        self.define_const(const_name, const_value, &type_tag);
    }

    pub fn declare_variable(&mut self) {
        if self.compiler.scope_depth == 0 {
            return;
        }

        let name = self.previous.clone();
        self.add_local(name);
    }
}
