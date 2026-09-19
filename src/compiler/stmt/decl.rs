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
            (TypeTag::Array(Arc::new(TypeTag::Void)), false)
        };

        let opt = if annotation_type.is_some_and(|t| t == TokenType::Opt) {
            self.consume(TokenType::LeftBracket, "Exp", scanner);

            self.advance(scanner);

            let opt = match self.previous.token_type {
                TokenType::Int => TypeTag::Opt(Arc::new(TypeTag::Int)),
                TokenType::Unt => TypeTag::Opt(Arc::new(TypeTag::Unt)),
                TokenType::Float => TypeTag::Opt(Arc::new(TypeTag::Float)),
                TokenType::Str => TypeTag::Opt(Arc::new(TypeTag::Str)),
                TokenType::Char => TypeTag::Opt(Arc::new(TypeTag::Char)),
                TokenType::Array => {
                    is_array = true;
                    TypeTag::Opt(Arc::new(self.parse_array_typetag(scanner).0))
                }
                _ => TypeTag::Opt(Arc::new(TypeTag::Void)),
            };

            self.consume(TokenType::RightBracket, "exp", scanner);
            is_opt = true;
            opt
        } else {
            TypeTag::Opt(Arc::new(Void))
        };

        self.expected_type = annotation_type.map(|t| match t {
            TokenType::Int => TypeTag::Int,
            TokenType::Str => TypeTag::Str,
            TokenType::Bool => TypeTag::Bool,
            TokenType::Float => TypeTag::Float,
            TokenType::Char => TypeTag::Char,
            TokenType::Unt => TypeTag::Unt,
            TokenType::Array => array.clone(),
            TokenType::Opt => opt.clone(),
            _ => TypeTag::Void,
        });

        if self.match_consume(&TokenType::Equal, scanner) {
            self.expression(scanner);
        } else {
            self.emit_byte(OpCode::Void as u8);
            self.type_tag.push(TypeTag::Void);
        }

        let type_tag = self.type_tag.pop().expect(TYPETAG_ERR);

        self.compiler.locals[self.compiler.local_count as usize - 1].type_tag = type_tag.clone();

        if let Some(token) = annotation_type {
            match token {
                TokenType::Opt => {
                    if type_tag != TypeTag::Opt(Arc::new(TypeTag::None)) && type_tag != opt {
                        self.error(&format!(
                            "Mismatched types, expected [{}] found [{}]",
                            opt, type_tag
                        ));
                    }
                }
                TokenType::Array => {
                    let array = array;

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
            let array = self.array_type(scanner);
            is_array = true;
            array
        } else {
            TypeTag::Array(Arc::new(TypeTag::Void))
        };

        let opt = if annotation_type == TokenType::Opt {
            self.consume(TokenType::LeftBracket, "Exp", scanner);

            let opt = match self.current.token_type {
                TokenType::Int => TypeTag::Opt(Arc::new(TypeTag::Int)),
                TokenType::Unt => TypeTag::Opt(Arc::new(TypeTag::Unt)),
                TokenType::Float => TypeTag::Opt(Arc::new(TypeTag::Float)),
                TokenType::Str => TypeTag::Opt(Arc::new(TypeTag::Str)),
                TokenType::Bool => TypeTag::Opt(Arc::new(TypeTag::Bool)),
                TokenType::Char => TypeTag::Opt(Arc::new(TypeTag::Char)),
                TokenType::Array => TypeTag::Opt(Arc::new(array.clone())),
                _ => TypeTag::Void,
            };

            self.advance(scanner);
            self.consume(TokenType::RightBracket, "Exp", scanner);
            is_opt = true;
            opt
        } else {
            TypeTag::Opt(Arc::new(Void))
        };

        self.expected_type = match annotation_type {
            TokenType::Int => Some(TypeTag::Int),
            TokenType::Str => Some(TypeTag::Str),
            TokenType::Bool => Some(TypeTag::Bool),
            TokenType::Float => Some(TypeTag::Float),
            TokenType::Char => Some(TypeTag::Char),
            TokenType::Unt => Some(TypeTag::Unt),
            TokenType::Array => Some(array.clone()),
            TokenType::Opt => Some(opt.clone()),
            _ => Some(TypeTag::Void),
        };

        self.consume(TokenType::Equal, "Expect '=' after const name.", scanner);

        let (const_value, type_tag) = self.const_value(scanner);

        match annotation_type {
            TokenType::Opt => {
                if type_tag != TypeTag::Opt(Arc::new(TypeTag::None)) && type_tag != opt {
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
