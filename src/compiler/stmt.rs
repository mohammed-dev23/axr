use super::*;

impl Parser {
    pub fn statement(&mut self, scanner: &mut Scanner) {
        let token = self.current.token_type;

        match token {
            TokenType::LeftBrace => {
                self.match_consume(&token, scanner);
                self.begin_scope();
                self.block(scanner);
                self.end_scope();
            }
            TokenType::Let => {
                self.match_consume(&token, scanner);
                self.variable_declaration(scanner);
            }
            TokenType::Println => {
                self.match_consume(&token, scanner);
                self.println_statement(scanner);
            }
            TokenType::Print => {
                self.match_consume(&token, scanner);
                self.print_statement(scanner);
            }
            TokenType::Const => {
                self.match_consume(&token, scanner);
                self.const_declaration(scanner);
            }
            TokenType::Fn => {
                self.match_consume(&token, scanner);
                self.fn_declaration(scanner);
            }
            _ => self.expression_statement(scanner),
        }
    }

    pub fn print_statement(&mut self, scanner: &mut Scanner) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.expression(scanner);
        self.consume(TokenType::Semicolon, "Expect ';' after value.", scanner);
        self.emit_byte(OpCode::Print as u8);
    }

    pub fn println_statement(&mut self, scanner: &mut Scanner) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.expression(scanner);
        self.consume(TokenType::Semicolon, "Expect ';' after value.", scanner);
        self.emit_byte(OpCode::Println as u8);
    }

    pub fn expression_statement(&mut self, scanner: &mut Scanner) {
        self.expression(scanner);
        self.consume(TokenType::Semicolon, "Expect ';' after value.", scanner);
        self.emit_byte(OpCode::Pop as u8);
    }

    pub fn declaration(&mut self, scanner: &mut Scanner) {
        self.statement(scanner);

        if self.painc_mode {
            self.synchronize();
        }
    }

    pub fn fn_declaration(&mut self, scanner: &mut Scanner) {
        self.consume(TokenType::Identifier, "Expect name after fn.", scanner);
        self.consume(TokenType::LeftParen, "Expect '(' after fn name.", scanner);
        self.consume(TokenType::RigtParen, "Enclosed ')' expected.", scanner);
        self.consume(TokenType::LeftBrace, "Expect '{' after fn name.", scanner);
        self.begin_scope();
        self.block(scanner);
        self.end_scope();
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

        if self.match_consume(&TokenType::Equal, scanner) {
            self.expression(scanner);
        } else {
            self.emit_byte(OpCode::Void as u8);
            self.type_tag.push(TypeTag::Void);
        }
        let type_tag = self.type_tag.pop().unwrap_or(TypeTag::Void);

        self.compiler.locals[self.compiler.local_count as usize - 1].type_tag = type_tag;

        if let Some(at) = annotation_type {
            match at {
                TokenType::Int => {
                    if type_tag != TypeTag::Int {
                        self.error(&format!(
                            "Mismatched types, expected [int] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Float => {
                    if type_tag != TypeTag::Float {
                        self.error(&format!(
                            "Mismatched types, expected [float] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Str => {
                    if type_tag != TypeTag::Str {
                        self.error(&format!(
                            "Mismatched types, expected [str] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Bool => {
                    if type_tag != TypeTag::Bool {
                        self.error(&format!(
                            "Mismatched types, expected [bool] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Char => {
                    if type_tag != TypeTag::Char {
                        self.error(&format!(
                            "Mismatched types, expected [char] found [{}]",
                            type_tag
                        ))
                    }
                }
                TokenType::Void => {
                    if type_tag != TypeTag::Void {
                        self.error(&format!(
                            "Mismatched types, expected [void] found [{}]",
                            type_tag
                        ));
                    };
                }
                _ => return,
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

        self.consume(TokenType::Equal, "Expect '=' after const name.", scanner);

        let (const_value, type_tag) = self.const_value(scanner);

        match &annotation_type {
            TokenType::Int => {
                if type_tag != TypeTag::Int {
                    self.error(&format!(
                        "Mismatched types, expected [int] found [{}]",
                        type_tag
                    ));
                }
            }
            TokenType::Float => {
                if type_tag != TypeTag::Float {
                    self.error(&format!(
                        "Mismatched types, expected [float] found [{}]",
                        type_tag
                    ));
                }
            }
            TokenType::Str => {
                if type_tag != TypeTag::Str {
                    self.error(&format!(
                        "Mismatched types, expected [str] found [{}]",
                        type_tag
                    ));
                }
            }
            TokenType::Bool => {
                if type_tag != TypeTag::Bool {
                    self.error(&format!(
                        "Mismatched types, expected [bool] found [{}]",
                        type_tag
                    ));
                }
            }
            TokenType::Char => {
                if type_tag != TypeTag::Char {
                    if type_tag != TypeTag::Char {
                        self.error(&format!(
                            "Mismatched types, expected [char] found [{}]",
                            type_tag
                        ))
                    }
                }
            }
            TokenType::Void => {
                if type_tag != TypeTag::Void {
                    self.error(&format!(
                        "Mismatched types, expected [void] found [{}]",
                        type_tag
                    ));
                };
            }
            _ => return,
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

    pub fn block(&mut self, scanner: &mut Scanner) {
        while !self.check(&TokenType::RightBrace) && !self.check(&TokenType::Eof) {
            self.declaration(scanner);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.", scanner);
    }
}
