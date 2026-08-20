// This scanner needs to rebuild entirly
// it's implemnted in C style which is wrong thing to do in rust
// more idomic rust is needed the fastrer the better !

pub struct Scanner<'s> {
    start: &'s str,
    current: &'s str,
    line: usize,
}

pub struct Token {
    pub token_type: TokenType,
    pub start: String,
    pub length: usize,
    pub line: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    //Single-character tokens.
    LeftParen,  // (
    RigtParen,  // )
    LeftBrace,  // {
    RightBrace, // }
    Comma,      // ,
    Dot,        // .
    Minus,      // -
    Plus,       // +
    Semicolon,  // ;
    Slash,      // /
    Star,       // *

    //One or two character tokens.
    Bang,         // !
    BangEqual,    // !=
    Equal,        // =
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >=
    Lesser,       // <
    LesserEqual,  // <=

    //Literals.
    Identifier,
    String,
    Number,

    //Keywords.
    Print,
    Abs,
    Floor,
    Ceil,
    Round,

    //Other
    Error,
    Eof,
}

impl<'s> Scanner<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            start: source,
            current: source,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Token {
        self.inconsumable();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RigtParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.match_tokens("=") {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_tokens("=") {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_tokens("=") {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '>' => {
                if self.match_tokens("=") {
                    self.make_token(TokenType::LesserEqual)
                } else {
                    self.make_token(TokenType::Lesser)
                }
            }
            _ => self.error_token("Unexpected character."),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current.is_empty()
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let length = self.start.len() - self.current.len();

        Token {
            token_type,
            start: self.start[..length].to_string(),
            length,
            line: self.line,
        }
    }

    fn error_token(&mut self, message: &str) -> Token {
        Token {
            token_type: TokenType::Error,
            start: message.to_string(),
            length: message.len(),
            line: self.line,
        }
    }

    fn advance(&mut self) -> char {
        let c = self.current.chars().next().unwrap();
        self.current = &self.current[c.len_utf8()..];
        c
    }

    fn match_tokens(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.current != expected {
            return false;
        }

        self.current.chars().next().unwrap();
        true
    }

    fn inconsumable(&mut self) {
        loop {
            let c = self.peek();

            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    fn peek(&mut self) -> char {
        self.current.chars().next().unwrap_or('\0')
    }

    fn peek_next(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.current[1..].chars().next().unwrap_or('\0')
    }
}
