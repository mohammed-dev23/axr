use crate::compiler::rules::Precedence::Assignment;

use super::*;

impl Parser {
    pub fn println_statement(&mut self, scanner: &mut Scanner) -> Stmt {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return Stmt::NoneStmt;
        }

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        let value = self.parse_precedence(Assignment, scanner);
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

        Stmt::Println(Box::new(value))
    } // done
}
