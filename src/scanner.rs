pub struct Scanner {
    start: String,
    current: String,
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
    Comma,      // :
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

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            start: source.clone(),
            current: source,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Token {
        self.start = self.current.clone();

        if self.is_at_end() {
            self.make_token(TokenType::Eof);
        }

        self.error_token("Unexpected character.".to_string())
    }

    fn is_at_end(&self) -> bool {
        self.current == "\0"
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start.clone(),
            length: (self.current.len() - self.start.len()),
            line: self.line,
        }
    }

    fn error_token(&mut self, message: String) -> Token {
        Token {
            token_type: TokenType::Error,
            start: self.start.clone(),
            length: message.len(),
            line: self.line,
        }
    }
}
