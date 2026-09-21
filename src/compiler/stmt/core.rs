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
            TokenType::For => {
                self.match_consume(&token, scanner);
                self.for_stmt(scanner);
            }
            TokenType::Return => {
                self.match_consume(&token, scanner);
                self.return_stmt(scanner);
            }
            _ => self.expression_statement(scanner),
        }
    }

    pub fn expression_statement(&mut self, scanner: &mut Scanner) {
        self.expression(scanner);
        self.consume(TokenType::Semicolon, "Expect ';' after value.", scanner);
        self.emit_byte(OpCode::Pop as u8);
    }
}
