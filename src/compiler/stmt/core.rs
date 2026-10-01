use crate::compiler::rules::Precedence::Assignment;

use super::*;

impl Parser {
    pub fn statement(&mut self, scanner: &mut Scanner) -> Stmt {
        let token = self.current.token_type;

        match token {
            TokenType::LeftBrace => {
                self.match_consume(&token, scanner);
                self.begin_scope();
                let stmts = self.block(scanner);
                self.end_scope();

                Stmt::Block(stmts)
            } // done
            TokenType::Let => {
                self.match_consume(&token, scanner);
                self.variable_declaration(scanner)
            } // done
            TokenType::Println => {
                self.match_consume(&token, scanner);
                self.println_statement(scanner)
            } // done
            TokenType::Const => {
                self.match_consume(&token, scanner);
                self.const_declaration(scanner)
            } // done
            TokenType::Fn => {
                self.match_consume(&token, scanner);
                self.fn_declaration(scanner)
            } // done
            TokenType::If => {
                self.match_consume(&token, scanner);
                self.if_stmt(scanner);

                Stmt::NoneStmt
            }
            TokenType::While => {
                self.match_consume(&token, scanner);
                self.while_stmt(scanner);

                Stmt::NoneStmt
            }
            TokenType::Loop => {
                self.match_consume(&token, scanner);
                self.loop_stmt(scanner);

                Stmt::NoneStmt
            }
            TokenType::Stop => {
                self.match_consume(&token, scanner);
                self.stop_stmt(scanner);

                Stmt::NoneStmt
            }
            TokenType::Skip => {
                self.match_consume(&token, scanner);
                self.skip_stmt(scanner);

                Stmt::NoneStmt
            }
            TokenType::Match => {
                self.match_consume(&token, scanner);
                self.match_stmt(scanner);

                Stmt::NoneStmt
            }
            TokenType::For => {
                self.match_consume(&token, scanner);
                self.for_stmt(scanner);

                Stmt::NoneStmt
            }
            TokenType::Return => {
                self.match_consume(&token, scanner);
                self.return_stmt(scanner)
            } // done
            _ => Stmt::Expression(Box::new(self.expression_statement(scanner))), // done
        }
    }

    pub fn expression_statement(&mut self, scanner: &mut Scanner) -> Expr {
        let expr = self.parse_precedence(Assignment, scanner);
        self.consume(TokenType::Semicolon, "Expect ';' after value.", scanner);
        expr
    } // done
}
