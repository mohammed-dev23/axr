use super::*;

pub struct Parser {
    pub(in crate::compiler) current: Token,
    pub(in crate::compiler) previous: Token,
    pub(in crate::compiler) had_err: bool,
    pub(in crate::compiler) painc_mode: bool,
    pub(in crate::compiler) compiling_chunk: Chunk,
    pub(in crate::compiler) compiler: Compiler,
    pub(in crate::compiler) const_table: HashMap<String, (Value, TypeTag)>,
    pub(in crate::compiler) type_tag: Vec<TypeTag>,
}

pub struct Compiler {
    pub(in crate::compiler) locals: Vec<Local>,
    pub(in crate::compiler) local_count: i32,
    pub(in crate::compiler) scope_depth: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeTag {
    Int,
    Float,
    Str,
    Bool,
    Char,
    Void,
}

impl fmt::Display for TypeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeTag::Int => write!(f, "int"),
            TypeTag::Float => write!(f, "float"),
            TypeTag::Bool => write!(f, "bool"),
            TypeTag::Str => write!(f, "str"),
            TypeTag::Void => write!(f, "void"),
            TypeTag::Char => write!(f, "char"),
        }
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            locals: Vec::new(),
            local_count: 0,
            scope_depth: 0,
        }
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current: Token::default(),
            previous: Token::default(),
            had_err: false,
            painc_mode: false,
            compiling_chunk: Chunk::new(),
            compiler: Compiler::new(),
            const_table: HashMap::new(),
            type_tag: Vec::new(),
        }
    }

    pub fn compile(&mut self, source: String, chunk: &mut Chunk) -> bool {
        let mut scanner = Scanner::new(&source);

        self.compiling_chunk = chunk.clone();
        self.had_err = false;
        self.painc_mode = false;

        self.advance(&mut scanner);

        while !self.match_consume(&TokenType::Eof, &mut scanner) {
            self.declaration(&mut scanner);
        }

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

    pub fn synchronize(&mut self) {
        self.painc_mode = false;

        while self.current.token_type != TokenType::Eof {
            if self.previous.token_type == TokenType::Semicolon {
                return;
            } else {
                match self.current.token_type {
                    TokenType::Print | TokenType::Let | TokenType::Const | TokenType::Println => {
                        return;
                    }
                    _ => continue,
                }
            }
        }
    }
}
