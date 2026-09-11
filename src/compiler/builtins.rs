use super::*;

impl Parser {
    pub fn input_expr(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        let expected_type = self.expected_type.take().unwrap_or_else(|| {
            self.error("input() needs a type context, e.g. `let x : str = input();`");
            Wrappers::None(Id(TypeId::Void))
        });

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);

        self.consume(
            TokenType::String,
            "input() requires a string literal prompt",
            scanner,
        );
        let raw = &self.previous.start;
        let trimmed = &raw[1..raw.len() - 1];
        self.emit_constant(Value::Str(Arc::from(trimmed)));

        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);

        self.emit_byte(OpCode::Input as u8);
        self.emit_byte(expected_type.as_bytes());
        self.type_tag.push(expected_type);
    }
}
