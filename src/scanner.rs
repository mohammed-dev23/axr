// This scanner needs to rebuild entirly
// it's implemnted in C style which is wrong thing to do in rust
// more idomic rust is needed the fastrer the better !

pub struct Scanner<'s> {
    start: &'s str,
    current: &'s str,
    line: usize,
}

#[derive(Default, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: String,
    pub length: usize,
    pub line: usize,
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
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
    Modulo,     // %
    Colon,      // :

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
    Println,
    Abs,
    Floor,
    Ceil,
    Round,
    Let,
    Tilde,
    Const,
    Fn,
    Sqrt,
    IsEmpty,
    Trim,
    Reverse,

    //Values of the boolean type
    True,
    False,

    //Values of types
    Int,
    Str,
    Float,
    Bool,

    //Other
    Error,
    Eof,
    Void,

    #[default]
    Nai,
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
            '~' => self.make_token(TokenType::Tilde),
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
                if self.match_tokens('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_tokens('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '>' => {
                if self.match_tokens('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '<' => {
                if self.match_tokens('=') {
                    self.make_token(TokenType::LesserEqual)
                } else {
                    self.make_token(TokenType::Lesser)
                }
            }
            '%' => self.make_token(TokenType::Modulo),
            '"' => self.strings(),
            ':' => self.make_token(TokenType::Colon),
            x if x.is_numeric() => self.numbers(),
            x if x.is_alphabetic() => self.identifier(),
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

    fn match_tokens(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }

        self.advance();
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

    fn strings(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            self.error_token(&"Unterminated string.");
        }

        self.advance();
        self.make_token(TokenType::String)
    }

    fn numbers(&mut self) -> Token {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphabetic() || self.peek().is_numeric() || self.peek() == '_' {
            self.advance();
        }

        let token_type = self.identifier_type();
        self.make_token(token_type)
    }

    fn identifier_type(&mut self) -> TokenType {
        let text = &self.start[..self.start.len() - self.current.len()];

        match text {
            "print" => TokenType::Print,
            "abs" => TokenType::Abs,
            "floor" => TokenType::Floor,
            "ceil" => TokenType::Ceil,
            "Round" => TokenType::Round,
            "let" => TokenType::Let,
            "sqrt" => TokenType::Sqrt,
            "is_empty" => TokenType::IsEmpty,
            "trim" => TokenType::Trim,
            "rev" => TokenType::Reverse,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "Void" => TokenType::Void,
            "println" => TokenType::Println,
            "const" => TokenType::Const,
            "fn" => TokenType::Fn,
            "int" => TokenType::Int,
            "float" => TokenType::Float,
            "bool" => TokenType::Bool,
            "str" => TokenType::Str,
            _ => TokenType::Identifier,
        }
    }
}
