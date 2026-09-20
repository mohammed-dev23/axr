use super::*;

impl Parser {
    pub fn array(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        let mut array_len = 0;

        if self.check(&TokenType::RightBracket) {
            self.advance(scanner);

            let expected_type = self.expected_type.take().unwrap_or_else(|| {
                self.error("when declaring new array a type annotation is needed.");
                TypeTag::Array(Arc::new(Void))
            });

            self.emit_byte(OpCode::NewArray as u8);
            self.type_tag.push(expected_type);
        } else {
            if self.check(&TokenType::LeftBracket) {
                let (array, mut type_tag) = self.parse_nasted(scanner);
                let checked_type_tag = self.array_type_check(&mut type_tag);
                array_len += array;
                self.type_tag.push(checked_type_tag);
            } else {
                let (array, mut type_tag) = self.parse_array(scanner);
                let checked_type_tag = self.array_type_check(&mut type_tag);
                array_len += array;
                self.type_tag.push(checked_type_tag);
            };

            self.consume(
                TokenType::RightBracket,
                "Expected ']' at the end of array",
                scanner,
            );

            self.emit_byte(OpCode::Array as u8);
            self.emit_byte(array_len as u8);
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

    fn parse_array(&mut self, scanner: &mut Scanner) -> (usize, Vec<TypeTag>) {
        let mut type_tag: Vec<TypeTag> = Vec::new();
        let mut array_len = 0;

        self.expression(scanner);
        array_len += 1;
        type_tag.push(self.type_tag.pop().expect(TYPETAG_ERR));

        while self.current.token_type == TokenType::Comma {
            self.match_consume(&TokenType::Comma, scanner);
            self.expression(scanner);
            array_len += 1;
            type_tag.push(self.type_tag.pop().expect(TYPETAG_ERR));
        }

        (array_len, type_tag)
    }

    fn parse_nasted(&mut self, scanner: &mut Scanner) -> (usize, Vec<TypeTag>) {
        let mut type_tag_stack: Vec<TypeTag> = Vec::new();
        let mut array = 0;

        self.consume(TokenType::LeftBracket, "Expect '[' ", scanner);

        let (array_len, mut type_tag) = self.parse_array(scanner);
        let checked_type_tag = self.array_type_check(&mut type_tag);
        type_tag_stack.push(checked_type_tag);

        array += 1;

        self.consume(TokenType::RightBracket, "Expect ']' ", scanner);

        self.emit_byte(OpCode::Array as u8);
        self.emit_byte(array_len as u8);

        while self.match_consume(&TokenType::Comma, scanner) {
            if !self.check(&TokenType::LeftBracket) {
                self.error(&format!(
                    "Expected a nasted array found [{:?}]",
                    self.current.token_type.clone()
                ));
            }

            let type_tag = self.parse_nasted(scanner).1;

            // the count fo each subling, each time we procces another subling that value incresses.
            array += 1;
            type_tag_stack.extend_from_slice(&type_tag);
        }

        (array, type_tag_stack)
    }

    fn array_type_check(&mut self, type_tag: &mut Vec<TypeTag>) -> TypeTag {
        let mut typetag: TypeTag = TypeTag::Array(Arc::new(TypeTag::Void));

        if let Some(first) = type_tag.first() {
            typetag = first.clone();

            if !type_tag.iter().all(|t| t == first) {
                self.error("Arrays must contain the same type for all of its slots.");
            }
        }

        TypeTag::Array(Arc::new(typetag))
    }
}
