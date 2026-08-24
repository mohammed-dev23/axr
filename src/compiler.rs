pub struct Parser {
    current: Token,
    previous: Token,
    had_err: bool,
    painc_mode: bool,
    compiling_chunk: Chunk,
}

use crate::{
    chunk::{Chunk, OpCode},
    scanner::{
        Scanner, Token,
        TokenType::{self},
    },
    value::Value,
};

#[derive(Debug, Clone, Copy)]
pub struct ParseRule {
    pub prefix: Option<fn(&mut Parser, &mut Scanner)>,
    pub infix: Option<fn(&mut Parser, &mut Scanner)>,
    pub precedence: Precedence,
}

const NONE_RULE: ParseRule = ParseRule {
    precedence: Precedence::None,
    prefix: None,
    infix: None,
};

static RULES: [ParseRule; 37] = [
    ParseRule {
        prefix: Some(Parser::grouping),
        infix: None,
        precedence: Precedence::None,
    }, // (
    NONE_RULE, // )
    NONE_RULE, // {
    NONE_RULE, // }
    NONE_RULE, // ,
    NONE_RULE, // .
    ParseRule {
        prefix: Some(Parser::unary),
        infix: Some(Parser::binary),
        precedence: Precedence::Term,
    }, // -
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Term,
    }, // +
    NONE_RULE, // ;
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Factor,
    }, // /,
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Factor,
    }, // *
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Factor,
    }, // %
    NONE_RULE, // !
    NONE_RULE, // !=
    NONE_RULE, // =
    NONE_RULE, // ==
    NONE_RULE, // >
    NONE_RULE, // >=
    NONE_RULE, // <
    NONE_RULE, // <=
    NONE_RULE, // Identifier
    NONE_RULE, // String
    ParseRule {
        prefix: Some(Parser::number),
        infix: None,
        precedence: Precedence::None,
    }, // Number,
    NONE_RULE, // Print
    NONE_RULE, // Abs
    NONE_RULE, // Floor
    NONE_RULE, // Ceil
    NONE_RULE, // Round
    NONE_RULE, // Set
    NONE_RULE, // Fix
    NONE_RULE, // Sqrt
    NONE_RULE, // IsEmpty
    NONE_RULE, // Trim,
    NONE_RULE, // Reverse
    NONE_RULE, // Error
    NONE_RULE, // Eof
    NONE_RULE, // Nai
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * / %
    Unary,      // - !
    Call,       // . ()
    PRIMARY,
}

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
        self.expression(&mut scanner);
        self.consume(TokenType::Eof, "Expect end of expression.", &mut scanner);

        self.end_compiler();

        *chunk = self.compiling_chunk.clone();

        !self.had_err
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

    pub fn expression(&mut self, scanner: &mut Scanner) {
        self.parse_precedence(Precedence::Assignment, scanner);
    }

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
        let line = self.previous.line as u32;
        self.current_chunk().write_chunk(byte, line);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.compiling_chunk
    }

    fn end_compiler(&mut self) {
        #[cfg(feature = "DPC")]
        {
            use crate::debug::disassemble_chunk;
            if !self.painc_mode {
                disassemble_chunk(self.current_chunk());
            }
        }
        // place houlder until we get print working!
        self.emit_byte(OpCode::Print as u8);
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn number(&mut self, _scanner: &mut Scanner) {
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
        self.expression(scanner);
        self.consume(
            TokenType::RigtParen,
            "Expect ')' after expression.",
            scanner,
        );
    }

    pub fn unary(&mut self, scanner: &mut Scanner) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(Precedence::Unary, scanner);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate as u8),
            _ => return,
        }
    }

    pub fn binary(&mut self, scanner: &mut Scanner) {
        let operator_type = self.previous.token_type;
        let rule = Self::get_rule(operator_type);
        self.parse_precedence(rule.precedence, scanner);

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            TokenType::Modulo => self.emit_byte(OpCode::Modulo as u8),
            _ => return,
        }
    }

    pub fn get_rule(token_type: TokenType) -> ParseRule {
        RULES[token_type as usize]
    }

    pub fn parse_precedence(&mut self, precedence: Precedence, scanner: &mut Scanner) {
        self.advance(scanner);

        let rule = Self::get_rule(self.previous.token_type);

        if let Some(prefix) = rule.prefix {
            prefix(self, scanner);
        } else {
            self.error("Expect expression.");
            return;
        }

        while precedence <= Self::get_rule(self.current.token_type).precedence {
            self.advance(scanner);
            let infix_rule = Self::get_rule(self.previous.token_type).infix.unwrap();
            infix_rule(self, scanner);
        }
    }
}
