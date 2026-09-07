use crate::compiler::core::TypeId::Void;

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
    pub(in crate::compiler) expected_type: Option<TypeTag>,
    pub(in crate::compiler) control_flow: ControlFlow,
    pub(in crate::compiler) info: Info,
}

pub struct Compiler {
    pub(in crate::compiler) locals: Vec<Local>,
    pub(in crate::compiler) local_count: i32,
    pub(in crate::compiler) scope_depth: i32,
}

pub struct ControlFlow {
    pub loop_starts: Vec<usize>,
    pub stops: Vec<Vec<u8>>,
}

pub struct Info {
    pub is_mut: Vec<bool>,
    pub last_local_slot: Option<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeTag {
    Id(TypeId),
    Array(TypeId),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeId {
    Int,
    Float,
    Str,
    Bool,
    Char,
    Unt,
    Void,
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
                    TokenType::Print | TokenType::Let | TokenType::Const | TokenType::Println => {
                        return;
                    }
                    _ => self.advance(scanner),
                }
            }
        }
    }

    pub fn type_check(&mut self, type_tag: TypeTag, token: &TokenType) {
        match token {
            TokenType::Int => {
                if type_tag != Id(TypeId::Int) {
                    self.error(&format!(
                        "Mismatched types, expected [int] found [{}]",
                        type_tag
                    ));
                }
            }
            TokenType::Float => {
                if type_tag != Id(TypeId::Float) {
                    self.error(&format!(
                        "Mismatched types, expected [float] found [{}]",
                        type_tag
                    ));
                }
            }
            TokenType::Str => {
                if type_tag != Id(TypeId::Str) {
                    self.error(&format!(
                        "Mismatched types, expected [str] found [{}]",
                        type_tag
                    ));
                }
            }
            TokenType::Bool => {
                if type_tag != Id(TypeId::Bool) {
                    self.error(&format!(
                        "Mismatched types, expected [bool] found [{}]",
                        type_tag
                    ));
                }
            }
            TokenType::Char => {
                if type_tag != Id(TypeId::Char) {
                    self.error(&format!(
                        "Mismatched types, expected [char] found [{}]",
                        type_tag
                    ))
                }
            }
            TokenType::Unt => {
                if type_tag != Id(TypeId::Unt) {
                    self.error(&format!(
                        "Mismatched types, expected [unt] found [{}]",
                        type_tag
                    ));
                }
            }
            TokenType::Void => {
                if type_tag != Id(TypeId::Void) {
                    self.error(&format!(
                        "Mismatched types, expected [void] found [{}]",
                        type_tag
                    ));
                };
            }
            TokenType::Array => match token {
                TokenType::Int => {
                    if type_tag != TypeTag::Array(TypeId::Int) {
                        self.error(&format!(
                            "Mismatched types, expected [int] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Float => {
                    if type_tag != TypeTag::Array(TypeId::Float) {
                        self.error(&format!(
                            "Mismatched types, expected [float] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Str => {
                    if type_tag != TypeTag::Array(TypeId::Str) {
                        self.error(&format!(
                            "Mismatched types, expected [str] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Bool => {
                    if type_tag != TypeTag::Array(TypeId::Bool) {
                        self.error(&format!(
                            "Mismatched types, expected [bool] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Char => {
                    if type_tag != TypeTag::Array(TypeId::Char) {
                        self.error(&format!(
                            "Mismatched types, expected [char] found [{}]",
                            type_tag
                        ))
                    }
                }
                TokenType::Unt => {
                    if type_tag != TypeTag::Array(TypeId::Unt) {
                        self.error(&format!(
                            "Mismatched types, expected [unt] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Void => {
                    if type_tag != TypeTag::Array(TypeId::Void) {
                        self.error(&format!(
                            "Mismatched types, expected [void] found [{}]",
                            type_tag
                        ));
                    };
                }
                _ => return,
            },
            _ => return,
        }
    }
}
