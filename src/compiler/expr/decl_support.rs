use super::*;

impl Parser {
    pub fn parse_variable(&mut self, error_message: &str, scanner: &mut Scanner) -> u8 {
        let is_mut = self.match_consume(&TokenType::Tilde, scanner);
        self.consume(TokenType::Identifier, error_message, scanner);

        self.declare_variable();
        if self.compiler.scope_depth > 0 {
            let idx = self.compiler.local_count as usize - 1;
            self.compiler.locals[idx].is_mut = is_mut;
            return 0;
        }

        let token = self.previous.clone();
        self.identifier_constant(&token)
    }

    pub fn parse_const(&mut self, error_message: &str, scanner: &mut Scanner) -> String {
        self.consume(TokenType::Identifier, error_message, scanner);
        self.previous.start.clone()
    }

    pub fn define_variable(&mut self) {
        self.mark_initialized();
    }

    pub fn define_const(&mut self, name: String, value: Value, type_tag: &TypeTag) {
        self.const_table.insert(name, (value, type_tag.clone()));
    }

    pub fn casting(&mut self, scanner: &mut Scanner) {
        let type_tag = self.type_tag.pop().unwrap_or(TypeTag::Void);

        self.advance(scanner);
        let token = self.previous.token_type;

        let target = match token {
            TokenType::Int => TypeTag::Int,
            TokenType::Unt => TypeTag::Unt,
            TokenType::Float => TypeTag::Float,
            TokenType::Str => TypeTag::Str,
            TokenType::Bool => TypeTag::Bool,
            TokenType::Char => TypeTag::Char,
            _ => TypeTag::Void,
        };

        match (type_tag, target.clone()) {
            (TypeTag::Str, _) => {
                self.error(&format!("non-primitive cast: `str` to `{}`", target));
            }
            (TypeTag::Char, _) => {
                self.error(&format!("non-primitive cast: `char` to `{}`", target));
            }
            (TypeTag::Bool, _) => {
                self.error(&format!("non-primitive cast: `bool` to `{}`", target));
            }
            _ => {}
        }

        self.emit_byte(OpCode::Cast as u8);

        let idx = self.add_type_tag_to_chunk(target.clone());
        self.emit_byte(idx);

        self.type_tag.push(target);
    }
}
