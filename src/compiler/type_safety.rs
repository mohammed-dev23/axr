use super::*;

impl TokenType {
    pub fn as_typetag(&self) -> Option<TypeTag> {
        match self {
            TokenType::Int => Some(TypeTag::Int),
            TokenType::Unt => Some(TypeTag::Unt),
            TokenType::Float => Some(TypeTag::Float),
            TokenType::Str => Some(TypeTag::Str),
            TokenType::Char => Some(TypeTag::Char),
            TokenType::Void => Some(TypeTag::None),
            _ => None,
        }
    }
}

impl TypeTag {
    pub fn as_bytes(&self) -> u8 {
        match self {
            Self::Int => 0,
            Self::Unt => 1,
            Self::Float => 2,
            Self::Str => 3,
            Self::Char => 4,
            Self::Bool => 5,
            Self::Void => 6,
            Self::None => 7,
            Self::Array(x) => x.as_bytes(),
            Self::Opt(x) => x.as_bytes(),
        }
    }
}

impl TypeTag {
    pub fn extract(self) -> Arc<TypeTag> {
        match self {
            Self::Array(x) => x,
            Self::Opt(x) => x,
            _ => Arc::new(Void),
        }
    }

    pub fn is_opt(&self) -> bool {
        match self {
            Self::Opt(_) => true,
            _ => false,
        }
    }
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
            TypeTag::Unt => write!(f, "unt"),
            TypeTag::None => write!(f, "None"),
            TypeTag::Opt(x) => write!(f, "{}", x),
            TypeTag::Array(x) => write!(f, "{}", x),
        }
    }
}

impl Parser {
    pub fn parse_array_typetag(&mut self, scanner: &mut Scanner) -> (TypeTag, bool) {
        let array = self.array_type(scanner);
        (array, true)
    }

    pub fn type_check(
        &mut self,
        type_tag: &TypeTag,
        token: &TokenType,
        is_array: bool,
        is_opt: bool,
    ) {
        if let Some(id) = token.as_typetag() {
            let expected = match (is_array, is_opt) {
                (true, false) => TypeTag::Array(Arc::new(id.clone())),
                (true, true) => TypeTag::Opt(Arc::new(TypeTag::Array(Arc::new(id.clone())))),
                (false, true) => TypeTag::Opt(Arc::new(id.clone())),
                (false, false) => id.clone(),
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

    pub fn array_type(&mut self, scanner: &mut Scanner) -> TypeTag {
        self.consume(TokenType::LeftBracket, "Exp", scanner);
        self.advance(scanner);

        let array = match self.previous.token_type {
            TokenType::Int => TypeTag::Array(Arc::new(TypeTag::Int)),
            TokenType::Unt => TypeTag::Array(Arc::new(TypeTag::Unt)),
            TokenType::Float => TypeTag::Array(Arc::new(TypeTag::Float)),
            TokenType::Str => TypeTag::Array(Arc::new(TypeTag::Str)),
            TokenType::Bool => TypeTag::Array(Arc::new(TypeTag::Bool)),
            TokenType::Char => TypeTag::Array(Arc::new(TypeTag::Char)),
            TokenType::Array => self.array_type(scanner),
            _ => TypeTag::Array(Arc::new(TypeTag::Void)),
        };

        self.consume(TokenType::RightBracket, "Exp", scanner);
        array
    }
}
