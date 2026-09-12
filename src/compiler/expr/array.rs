use super::*;

impl Parser {
    pub fn array(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        let mut array_len = 0;
        let mut is_opt = false;
        let mut type_tag: Vec<Wrappers> = Vec::new();
        let mut typetag: Wrappers = Wrappers::None(TypeTag::Array(TypeId::Void));

        if self.check(&TokenType::RightBracket) {
            self.advance(scanner);

            let expected_type = self.expected_type.take().unwrap_or_else(|| {
                self.error("when declaring new array a type annotation is needed.");
                Wrappers::None(TypeTag::Array(Void))
            });

            self.emit_byte(OpCode::NewArray as u8);
            self.type_tag.push(expected_type);
        } else {
            self.expression(scanner);
            array_len += 1;
            type_tag.push(self.type_tag.pop().expect(TYPETAG_ERR));

            while self.current.token_type == TokenType::Comma {
                self.match_consume(&TokenType::Comma, scanner);
                self.expression(scanner);
                array_len += 1;
                type_tag.push(self.type_tag.pop().expect(TYPETAG_ERR));
            }

            if let Some(first) = type_tag.first() {
                typetag = *first;

                if !type_tag.iter().all(|t| t == first) {
                    self.error("Arrays must contain the same type for all of its slots.");
                }

                if let Some(x) = self.expected_type {
                    let none_var = Wrappers::None(TypeTag::Array(first.as_typeid()));
                    let opt_var = Wrappers::Opt(TypeTag::Array(first.as_typeid()));

                    if x == opt_var {
                        is_opt = true;
                    } else if x != none_var {
                        self.error(&format!(
                            "Mismatched types, expected {} found Array[{}]",
                            x, first
                        ));
                    }
                }
            }

            self.consume(
                TokenType::RightBracket,
                "Expected ']' at the end of array",
                scanner,
            );

            self.emit_byte(OpCode::Array as u8);
            self.emit_byte(array_len as u8);

            if !is_opt {
                self.type_tag
                    .push(Wrappers::None(TypeTag::Array(typetag.as_typeid())));
            } else {
                self.type_tag
                    .push(Wrappers::Opt(TypeTag::Array(typetag.as_typeid())))
            }
        }
    }

    pub fn index_array(&mut self, scanner: &mut Scanner) {
        self.expression(scanner);
        let type_tag = self.type_tag.last().expect(TYPETAG_ERR);

        if type_tag != &Wrappers::None(Id(TypeId::Unt)) {
            self.error(&format!(
                "Expected unt type in indexing found [{}]",
                type_tag
            ));
        }

        self.consume(
            TokenType::RightBracket,
            "Expected ']' at the end of array",
            scanner,
        );

        self.emit_byte(OpCode::IndexArray as u8);
    }
}
