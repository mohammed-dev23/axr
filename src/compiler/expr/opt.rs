use super::*;

impl Parser {
    pub fn some_expr(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        let outer_expected = self.expected_type.take();
        self.expected_type = outer_expected.map(|w| Wrappers::None(w.extract()));

        self.consume(TokenType::LeftParen, "Expect '(' after Some", scanner);
        self.expression(scanner);
        self.type_tag.pop();
        self.consume(TokenType::RigtParen, "Enclosed '(' expect ')'", scanner);

        let type_tag = self.expected_type.take().expect(TYPETAG_ERR);

        let Some(outer) = outer_expected else {
            self.error("Some(...) needs a type context, e.g. `let x : Opt[int] = Some(5);`");
            self.emit_byte(OpCode::Some as u8);
            self.emit_byte(type_tag.as_bytes());
            self.type_tag.push(Wrappers::Opt(type_tag.extract()));
            return;
        };

        let expected_inner = Wrappers::None(outer.extract());

        if type_tag != expected_inner {
            self.error(&format!(
                "Mismatched types, expected [{}] found [{}] inside 'Some(...)'",
                expected_inner, type_tag
            ));
        }

        let res_type = Wrappers::Opt(type_tag.extract());
        self.emit_byte(OpCode::Some as u8);
        self.emit_byte(res_type.as_bytes());
        self.type_tag.push(res_type);
    }
}
