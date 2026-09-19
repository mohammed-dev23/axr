use super::super::*;

impl Parser {
    pub fn match_stmt(&mut self, scanner: &mut Scanner) {
        let mut jumps: Vec<usize> = Vec::new();
        let mut type_tags: Vec<TypeTag> = Vec::new();

        self.expression(scanner);
        let ftype_tag = self.type_tag.pop().expect(TYPETAG_ERR);

        self.begin_scope();

        self.consume(
            TokenType::LeftBrace,
            "Expect '{' after match body.",
            scanner,
        );

        self.emit_byte(OpCode::Dup as u8);
        self.expression(scanner);
        type_tags.push(self.type_tag.pop().expect(TYPETAG_ERR));
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
            type_tags.push(self.type_tag.pop().expect(TYPETAG_ERR));
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
}
