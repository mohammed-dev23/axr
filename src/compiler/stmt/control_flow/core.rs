use super::super::*;

impl Parser {
    pub fn block(&mut self, scanner: &mut Scanner) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.check(&TokenType::Eof) {
            let stmt = self.declaration(scanner);
            stmts.push(stmt);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.", scanner);
        stmts
    } // done
}
