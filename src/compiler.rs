pub struct Parser {
    current: Token,
    previous: Token,
    had_err: bool,
    painc_mode: bool,
    compiling_chunk: Chunk,
}

use crate::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

impl Parser {
    pub fn new() -> Self {
        Self {
            current: Token::default(),
            previous: Token::default(),
            had_err: false,
            painc_mode: false,
            compiling_chunk: Chunk::new(),
        }
    }
    pub fn compile(&mut self, source: String, chunk: &mut Chunk) -> bool {
        let mut scanner = Scanner::new(&source);

        self.compiling_chunk = chunk.clone();
        self.had_err = false;
        self.painc_mode = false;

        self.advance(&mut scanner);
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression.", &mut scanner);

        self.end_compiler();
        !self.had_err
    }

    pub fn advance(&mut self, scanner: &mut Scanner) {
        self.previous = self.current.clone();

        loop {
            self.current = scanner.scan_tokens();

            if self.current.token_type != TokenType::Error {
                break;
            }
        }

        let token_location = self.current.start.clone();
        self.error_at_current(&token_location);
    }

    pub fn expression(&mut self) {}

    pub fn consume(&mut self, token_type: TokenType, message: &str, scanner: &mut Scanner) {
        if self.current.token_type == token_type {
            self.advance(scanner);
            return;
        }

        self.error_at_current(message);
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

    pub fn emit_byte(&mut self, byte: u8) {
        let mut current_chunk = self.current_chunk().clone();
        current_chunk.write_chunk(byte, self.previous.line as u32);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.compiling_chunk
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn number(&mut self) {
        let value: f64 = self.previous.start.parse::<f64>().unwrap_or(0.0);
        self.emit_constant(Value::Float(value));
    }

    pub fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant as u8, constant);
    }

    pub fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.current_chunk().add_const(value) as u8;

        if constant > u8::MAX {
            self.error("Too many constants in one chunk.");
            return 0;
        }

        constant
    }

    pub fn grouping(&mut self, scanner: &mut Scanner) {
        self.expression();
        self.consume(
            TokenType::RigtParen,
            "Expect ')' after expression.",
            scanner,
        );
    }
}
