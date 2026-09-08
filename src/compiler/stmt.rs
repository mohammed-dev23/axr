use super::*;
use crate::chunk::OpCode::Jump;
use crate::compiler::TypeTag::Id;
use crate::compiler::core::TypeId::Void;

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
            TokenType::If => {
                self.match_consume(&token, scanner);
                self.if_stmt(scanner);
            }
            TokenType::While => {
                self.match_consume(&token, scanner);
                self.while_stmt(scanner);
            }
            TokenType::Loop => {
                self.match_consume(&token, scanner);
                self.loop_stmt(scanner);
            }
            TokenType::Stop => {
                self.match_consume(&token, scanner);
                self.stop_stmt(scanner);
            }
            TokenType::Skip => {
                self.match_consume(&token, scanner);
                self.skip_stmt(scanner);
            }
            TokenType::Match => {
                self.match_consume(&token, scanner);
                self.match_stmt(scanner);
            }
            _ => self.expression_statement(scanner),
        }
    }

    pub fn print_statement(&mut self, scanner: &mut Scanner) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }
        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        self.expression(scanner);
        self.type_tag.pop();
        self.consume(TokenType::RigtParen, "Expect ')' aftre value.", scanner);
        self.consume(
            TokenType::Semicolon,
            "Expect ';' at the end of the statement.",
            scanner,
        );
        self.emit_byte(OpCode::Print as u8);
    }

    pub fn println_statement(&mut self, scanner: &mut Scanner) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }
        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        self.expression(scanner);
        self.type_tag.pop();
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);
        self.consume(
            TokenType::Semicolon,
            "Expect ';' at the end of the statement.",
            scanner,
        );
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
            self.synchronize(scanner);
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

        let array = if annotation_type.is_some_and(|t| t == TokenType::Array) {
            self.consume(TokenType::LeftBracket, "Exp", scanner);

            let array = match self.current.token_type {
                TokenType::Int => TypeTag::Array(TypeId::Int),
                TokenType::Unt => TypeTag::Array(TypeId::Unt),
                TokenType::Float => TypeTag::Array(TypeId::Float),
                TokenType::Str => TypeTag::Array(TypeId::Str),
                TokenType::Bool => TypeTag::Array(TypeId::Bool),
                TokenType::Char => TypeTag::Array(TypeId::Char),
                _ => TypeTag::Array(TypeId::Void),
            };

            self.advance(scanner);
            self.consume(TokenType::RightBracket, "exp", scanner);
            array
        } else {
            TypeTag::Array(TypeId::Void)
        };

        self.expected_type = annotation_type.map(|t| match t {
            TokenType::Int => Id(TypeId::Int),
            TokenType::Str => Id(TypeId::Str),
            TokenType::Bool => Id(TypeId::Bool),
            TokenType::Float => Id(TypeId::Float),
            TokenType::Char => Id(TypeId::Char),
            TokenType::Unt => Id(TypeId::Unt),
            TokenType::Array => array,
            _ => Id(TypeId::Void),
        });

        if self.match_consume(&TokenType::Equal, scanner) {
            self.expression(scanner);
        } else {
            self.emit_byte(OpCode::Void as u8);
            self.type_tag.push(Id(TypeId::Void));
        }
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        self.compiler.locals[self.compiler.local_count as usize - 1].type_tag = type_tag;

        if let Some(token) = annotation_type {
            self.type_check(type_tag, &token);
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

        let array = {
            self.consume(TokenType::LeftBracket, "Exp", scanner);

            let array = match self.current.token_type {
                TokenType::Int => TypeTag::Array(TypeId::Int),
                TokenType::Unt => TypeTag::Array(TypeId::Unt),
                TokenType::Float => TypeTag::Array(TypeId::Float),
                TokenType::Str => TypeTag::Array(TypeId::Str),
                TokenType::Bool => TypeTag::Array(TypeId::Bool),
                TokenType::Char => TypeTag::Array(TypeId::Char),
                _ => TypeTag::Array(TypeId::Void),
            };

            self.advance(scanner);
            self.consume(TokenType::RightBracket, "Exp", scanner);
            array
        };

        self.expected_type = match annotation_type {
            TokenType::Int => Some(Id(TypeId::Int)),
            TokenType::Str => Some(Id(TypeId::Str)),
            TokenType::Bool => Some(Id(TypeId::Bool)),
            TokenType::Float => Some(Id(TypeId::Float)),
            TokenType::Char => Some(Id(TypeId::Char)),
            TokenType::Unt => Some(Id(TypeId::Unt)),
            TokenType::Array => Some(array),
            _ => Some(Id(TypeId::Void)),
        };

        self.consume(TokenType::Equal, "Expect '=' after const name.", scanner);

        let (const_value, type_tag) = self.const_value(scanner);

        self.type_check(type_tag, &annotation_type);

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

    pub fn if_stmt(&mut self, scanner: &mut Scanner) {
        self.expression(scanner);

        let then_jump = self.emit_jump(OpCode::JumpIfFalse as usize);
        self.emit_byte(OpCode::Pop as u8);
        self.statement(scanner);

        let else_jump = self.emit_jump(OpCode::Jump as usize);

        self.patch_jump(then_jump as usize);

        self.emit_byte(OpCode::Pop as u8);
        if self.match_consume(&TokenType::Else, scanner) {
            if self.match_consume(&TokenType::If, scanner) {
                self.if_stmt(scanner);
            } else {
                self.statement(scanner);
            }
        }
        self.patch_jump(else_jump as usize);
    }

    pub fn while_stmt(&mut self, scanner: &mut Scanner) {
        let loop_start = self.compiling_chunk.code.len();
        self.control_flow.loop_starts.push(loop_start);
        self.control_flow.stops.push(Vec::new());

        self.expression(scanner);

        let exit_jump = self.emit_jump(OpCode::JumpIfFalse as usize);
        self.emit_byte(OpCode::Pop as u8);
        self.statement(scanner);
        self.emit_loop(loop_start);

        self.patch_jump(exit_jump as usize);
        self.emit_byte(OpCode::Pop as u8);

        self.control_flow.loop_starts.pop();
        for i in self.control_flow.stops.pop().unwrap() {
            self.patch_jump(i as usize);
        }
    }

    pub fn loop_stmt(&mut self, scanner: &mut Scanner) {
        let loop_start = self.compiling_chunk.code.len();
        self.control_flow.loop_starts.push(loop_start);
        self.control_flow.stops.push(Vec::new());
        self.control_flow.locals_in.push(self.compiler.local_count);

        self.statement(scanner);
        self.emit_loop(loop_start);

        self.control_flow.loop_starts.pop();
        self.control_flow.locals_in.pop();
        for i in self.control_flow.stops.pop().unwrap() {
            self.patch_jump(i as usize);
        }
    }

    pub fn stop_stmt(&mut self, scanner: &mut Scanner) {
        if self.control_flow.stops.is_empty() {
            self.error("'stop' used outside of a loop.");
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            scanner,
        );

        let exit_jump = self.emit_jump(Jump as usize);
        let count = *self.control_flow.locals_in.last().unwrap();

        for _ in count..self.compiler.local_count {
            self.emit_byte(OpCode::Pop as u8);
        }

        if let Some(jumps) = self.control_flow.stops.last_mut() {
            jumps.push(exit_jump as u8);
        }
    }

    pub fn skip_stmt(&mut self, scanner: &mut Scanner) {
        if self.control_flow.loop_starts.is_empty() {
            self.error("'skip' used outside of a loop.");
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            scanner,
        );

        let loop_start = *self.control_flow.loop_starts.last().unwrap();
        let count = *self.control_flow.locals_in.last().unwrap();

        for _ in count..self.compiler.local_count {
            self.emit_byte(OpCode::Pop as u8);
        }

        self.emit_loop(loop_start);
    }

    pub fn match_stmt(&mut self, scanner: &mut Scanner) {
        let mut jumps: Vec<usize> = Vec::new();
        let mut type_tags: Vec<TypeTag> = Vec::new();

        self.expression(scanner);
        let ftype_tag = self.type_tag.pop().unwrap_or(Id(Void));

        self.begin_scope();

        self.consume(
            TokenType::LeftBrace,
            "Expect '{' after match body.",
            scanner,
        );

        self.emit_byte(OpCode::Dup as u8);
        self.expression(scanner);
        type_tags.push(self.type_tag.pop().unwrap_or(Id(Void)));
        self.emit_byte(OpCode::EqualTo as u8);

        let next_jump = self.emit_jump(OpCode::JumpIfFalse as usize);

        self.consume(TokenType::FatArrowLeft, "Expect '=>' after case.", scanner);

        self.begin_scope();
        self.consume(
            TokenType::LeftBrace,
            "Expect '{' at the start of the case",
            scanner,
        );

        self.emit_byte(OpCode::Pop as u8);
        self.emit_byte(OpCode::Pop as u8);
        self.block(scanner);
        jumps.push(self.emit_jump(OpCode::Jump as usize));

        self.patch_jump(next_jump as usize);
        self.emit_byte(OpCode::Pop as u8);

        self.end_scope();
        self.consume(
            TokenType::Comma,
            "Expect ',' at the end of the case's block.",
            scanner,
        );

        while self.current.token_type != TokenType::WildCard {
            self.emit_byte(OpCode::Dup as u8);
            self.expression(scanner);
            type_tags.push(self.type_tag.pop().unwrap_or(Id(Void)));
            self.emit_byte(OpCode::EqualTo as u8);

            let next_jump = self.emit_jump(OpCode::JumpIfFalse as usize);

            self.consume(TokenType::FatArrowLeft, "Expect '=>' after case.", scanner);

            self.begin_scope();
            self.consume(
                TokenType::LeftBrace,
                "Expect '{' at the start of the case",
                scanner,
            );

            self.emit_byte(OpCode::Pop as u8);
            self.emit_byte(OpCode::Pop as u8);
            self.block(scanner);
            jumps.push(self.emit_jump(OpCode::Jump as usize));
            self.patch_jump(next_jump as usize);
            self.emit_byte(OpCode::Pop as u8);

            self.end_scope();
            self.consume(
                TokenType::Comma,
                "Expect ',' at the end of the case's block.",
                scanner,
            );
        }

        self.consume(
            TokenType::WildCard,
            "Expect '_' at the end of the of the match cases.",
            scanner,
        );

        self.consume(TokenType::FatArrowLeft, "Expect '=>' after a case", scanner);
        self.begin_scope();
        self.consume(
            TokenType::LeftBrace,
            "Expect '{' after a fat arrow",
            scanner,
        );

        self.emit_byte(OpCode::Pop as u8);
        self.block(scanner);
        jumps.push(self.emit_jump(OpCode::Jump as usize));

        self.end_scope();

        self.consume(
            TokenType::RightBrace,
            "Excpet '}' at the end of the match body.",
            scanner,
        );
        self.end_scope();

        for i in type_tags {
            if ftype_tag != i {
                self.error(&format!(
                    "Missmatched type expected [{}] due the value matched on was [{}] found [{}]",
                    ftype_tag, ftype_tag, i
                ));
            }
        }

        for i in jumps {
            self.patch_jump(i);
        }
    }
}
