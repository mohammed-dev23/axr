use super::*;
use crate::compiler::core::{TypeId::Void, TypeTag::Array};

impl TokenType {
    pub fn as_typeid(&self) -> Option<TypeId> {
        match self {
            TokenType::Int => Some(TypeId::Int),
            TokenType::Unt => Some(TypeId::Unt),
            TokenType::Float => Some(TypeId::Float),
            TokenType::Str => Some(TypeId::Str),
            TokenType::Char => Some(TypeId::Char),
            TokenType::Void => Some(TypeId::None),
            _ => None,
        }
    }
}

impl TypeTag {
    pub fn as_bytes(&self) -> u8 {
        match self {
            Id(x) => *x as u8,
            Self::Array(x) => 0x80 | (*x as u8),
        }
    }

    #[allow(warnings)]
    pub fn from_bytes(byte: u8) -> Self {
        let x = TypeId::from_byte(byte & 0x7f);

        if byte & 0x80 != 0 {
            TypeTag::Array(x)
        } else {
            TypeTag::Id(x)
        }
    }

    pub fn as_typeid(&self) -> TypeId {
        match self {
            TypeTag::Array(x) => *x,
            TypeTag::Id(x) => *x,
        }
    }
}

#[allow(warnings)]
impl TypeId {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0 => TypeId::Int,
            1 => TypeId::Float,
            2 => TypeId::Str,
            3 => TypeId::Bool,
            4 => TypeId::Char,
            5 => TypeId::Unt,
            _ => Void,
        }
    }
}

impl fmt::Display for TypeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeTag::Id(x) => write!(f, "{}", x),
            TypeTag::Array(x) => write!(f, "Array[{}]", x),
        }
    }
}

impl Wrappers {
    pub fn extract(self) -> TypeTag {
        match self {
            Self::None(x) => x,
            Self::Opt(x) => x,
        }
    }

    pub fn as_typeid(&self) -> TypeId {
        self.extract().as_typeid()
    }

    pub fn as_bytes(&self) -> u8 {
        match self {
            Wrappers::Opt(x) => 0x40 | x.as_bytes(),
            Wrappers::None(x) => x.as_bytes(),
        }
    }

    #[allow(warnings)]
    pub fn from_bytes(byte: u8) -> Self {
        let inner = TypeTag::from_bytes(byte & !0x40);

        if byte & 0x40 != 0 {
            Wrappers::Opt(inner)
        } else {
            Wrappers::None(inner)
        }
    }

    pub fn is_opt(&self) -> bool {
        match self {
            Self::Opt(_) => true,
            Self::None(_) => false,
        }
    }
}

impl fmt::Display for Wrappers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Wrappers::Opt(x) => write!(f, "Opt[{}]", x),
            Wrappers::None(x) => write!(f, "{}", x),
        }
    }
}

impl fmt::Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeId::Int => write!(f, "int"),
            TypeId::Float => write!(f, "float"),
            TypeId::Bool => write!(f, "bool"),
            TypeId::Str => write!(f, "str"),
            TypeId::Void => write!(f, "void"),
            TypeId::Char => write!(f, "char"),
            TypeId::Unt => write!(f, "unt"),
            TypeId::None => write!(f, "None"),
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
            expected_type: None,
            control_flow: ControlFlow {
                loop_starts: Vec::new(),
                stops: Vec::new(),
                locals_in: Vec::new(),
            },
            info: Info {
                is_mut: Vec::new(),
                last_local_slot: Some(0),
            },
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

    pub fn type_check(
        &mut self,
        type_tag: &Wrappers,
        token: &TokenType,
        is_array: bool,
        is_opt: bool,
    ) {
        if let Some(id) = token.as_typeid() {
            let expected = match (is_array, is_opt) {
                (true, false) => Wrappers::None(Array(id)),
                (true, true) => Wrappers::Opt(Array(id)),
                (false, true) => Wrappers::Opt(Id(id)),
                (false, false) => Wrappers::None(Id(id)),
            };

            if &expected != type_tag {
                let kind = match (is_array, is_opt) {
                    (true, false) => format!("Array[{}]", id),
                    (true, true) => format!("Opt[Array[{}]]", id),
                    (false, true) => format!("Opt[{}]", id),
                    (false, false) => format!("{}", id),
                };

                self.error(&format!(
                    "Mismatched types, expected [{}] found [{}]",
                    kind, type_tag
                ));
            }
        } else {
            return;
        }
    }

    #[allow(warnings)]
    pub fn debug_parser(&self, num: usize) {
        println!("{} >> {}", self.previous.start, num);
        println!("{} >> {}", self.current.start, num)
    }

    pub fn parse_array_typetag(&mut self, scanner: &mut Scanner) -> (TypeTag, bool) {
        self.consume(TokenType::LeftBracket, "Exp", scanner);

        let array = match self.current.token_type {
            TokenType::Int => TypeTag::Array(TypeId::Int),
            TokenType::Unt => TypeTag::Array(TypeId::Unt),
            TokenType::Float => TypeTag::Array(TypeId::Float),
            TokenType::Str => TypeTag::Array(TypeId::Str),
            TokenType::Bool => TypeTag::Array(TypeId::Bool),
            TokenType::Char => TypeTag::Array(TypeId::Char),
            _ => TypeTag::Array(TypeId::Void),
        };

        self.advance(scanner);
        self.consume(TokenType::RightBracket, "exp", scanner);

        (array, true)
    }
}
