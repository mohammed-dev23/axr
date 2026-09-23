use super::*;

impl Parser {
    pub fn println_statement(&mut self, scanner: &mut Scanner) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }
        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        self.expression(scanner);
        let typetag = self.type_tag.pop().expect(TYPETAG_ERR);

        if typetag.is_opt() {
            self.error("'Opt' is not implemented for 'Println()'");
        }

        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);
        self.consume(
            TokenType::Semicolon,
            "Expect ';' at the end of the statement.",
            scanner,
        );
        self.emit_byte(OpCode::Println as u8);
    }
}
