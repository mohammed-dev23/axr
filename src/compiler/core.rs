use super::*;

impl Parser {
    pub fn compile(&mut self, source: String, chunk: &mut Chunk) -> Option<Function> {
        let mut scanner = Scanner::new(&source);

        self.pre_pass(&source);
        self.check_main();

        self.compiler.function.function.chunk = chunk.clone();
        self.had_err = false;
        self.painc_mode = false;

        self.advance(&mut scanner);

        while !self.match_consume(&TokenType::Eof, &mut scanner) {
            self.declaration(&mut scanner);
        }

        let function = self.end_compiler();
        *chunk = self.current_chunk().clone();

        if self.had_err { None } else { Some(function) }
    }

    pub fn advance(&mut self, scanner: &mut Scanner) {
        self.previous = self.current.clone();

        loop {
            self.current = scanner.scan_tokens();

            if self.current.token_type != TokenType::Error {
                break;
            }
            let token_location = self.current.start.clone();
            self.error_at_current(&token_location);
        }
    }

    pub fn error_at_current(&mut self, message: &str) {
        let current_token = self.current.clone();
        self.error_at(&current_token, message);
    }

    pub fn error(&mut self, message: &str) {
        let previous_token = self.previous.clone();
        self.error_at(&previous_token, message);
    }

    pub fn error_at(&mut self, token: &Token, message: &str) {
        if self.painc_mode {
            return;
        }

        self.painc_mode = true;

        eprint!("{} Error", token.line);

        if token.token_type == TokenType::Eof {
            eprint!(" at end")
        } else if token.token_type == TokenType::Error {
        } else {
            eprint!(" at {}+{}", token.length, token.start)
        }

        eprintln!(": {}", message);

        self.had_err = true
    }

    pub fn match_consume(&mut self, token_type: &TokenType, scanner: &mut Scanner) -> bool {
        if !self.check(token_type) {
            return false;
        } else {
            self.advance(scanner);
            return true;
        }
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        &self.current.token_type == token_type
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str, scanner: &mut Scanner) {
        if self.current.token_type == token_type {
            self.advance(scanner);
            return;
        }

        self.error_at_current(message);
    }

    pub fn synchronize(&mut self, scanner: &mut Scanner) {
        self.painc_mode = false;

        while self.current.token_type != TokenType::Eof {
            if self.previous.token_type == TokenType::Semicolon {
                return;
            } else {
                match self.current.token_type {
                    TokenType::Print
                    | TokenType::Println
                    | TokenType::Let
                    | TokenType::Const
                    | TokenType::Fn
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Loop
                    | TokenType::Match
                    | TokenType::LeftBrace
                    | TokenType::RightBrace => {
                        return;
                    }
                    _ => self.advance(scanner),
                }
            }
        }
    }
}
