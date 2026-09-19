use super::*;

impl Parser {
    pub fn array(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        let mut array_len = 0;
        let mut is_opt = false;
        let mut type_tag: Vec<TypeTag> = Vec::new();
        let mut typetag: TypeTag = TypeTag::Array(Arc::new(TypeTag::Void));

        if self.check(&TokenType::RightBracket) {
            self.advance(scanner);

            let expected_type = self.expected_type.take().unwrap_or_else(|| {
                self.error("when declaring new array a type annotation is needed.");
                TypeTag::Array(Arc::new(Void))
            });

            self.emit_byte(OpCode::NewArray as u8);
            self.type_tag.push(expected_type);
        } else {
            self.expression(scanner);
            array_len += 1;

            while self.current.token_type == TokenType::Comma {
                self.match_consume(&TokenType::Comma, scanner);
                self.expression(scanner);
                array_len += 1;
                type_tag.push(self.type_tag.pop().expect(TYPETAG_ERR));
            }

            if let Some(first) = type_tag.first() {
                typetag = first.clone();

                if !type_tag.iter().all(|t| t == first) {
                    self.error("Arrays must contain the same type for all of its slots.");
                }

                if let Some(x) = &self.expected_type {
                    let none_var = TypeTag::Array(Arc::new(typetag.clone()));
                    let opt_var = TypeTag::Opt(Arc::new(TypeTag::Array(Arc::new(typetag.clone()))));

                    if x == &opt_var {
                        is_opt = true;
                    } else if x != &none_var {
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
                self.type_tag.push(TypeTag::Array(Arc::new(typetag)));
            } else {
                self.type_tag
                    .push(TypeTag::Opt(Arc::new(TypeTag::Array(Arc::new(typetag)))))
            }
        }
    }

    pub fn index_array(&mut self, scanner: &mut Scanner) {
        self.expression(scanner);
        let type_tag = self.type_tag.last().expect(TYPETAG_ERR);

        if type_tag != &TypeTag::Unt {
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
