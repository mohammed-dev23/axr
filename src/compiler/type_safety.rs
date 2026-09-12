use super::*;

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

impl Parser {
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
}
