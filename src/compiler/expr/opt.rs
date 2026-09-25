use super::*;

impl Parser {
    pub fn some_expr(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        let outer_expected = self.expected_type.take();
        self.expected_type = outer_expected.clone().map(|w| w.extract().as_ref().clone());

        self.consume(TokenType::LeftParen, "Expect '(' after Some", scanner);
        self.expression(scanner);
        self.type_tag.pop();
        self.consume(TokenType::RigtParen, "Enclosed '(' expect ')'", scanner);

        let type_tag = self.expected_type.take().expect(TYPETAG_ERR);

        let Some(outer) = outer_expected else {
            self.error("Some(...) needs a type context, e.g. `let x : Opt[int] = Some(5);`");
            self.emit_byte(OpCode::Some as u8);

            let idx = self.add_type_tag_to_chunk(type_tag.clone());
            self.emit_byte(idx);
            self.type_tag.push(TypeTag::Opt(type_tag.extract()));

            return;
        };

        let expected_inner = outer.extract().as_ref().clone();

        if type_tag != expected_inner {
            self.error(&format!(
                "Mismatched types, expected [{}] found [{}] inside 'Some(...)'",
                expected_inner, type_tag
            ));
        }

        let res_type = TypeTag::Opt(type_tag.extract());
        self.emit_byte(OpCode::Some as u8);
        let idx = self.add_type_tag_to_chunk(res_type.clone());
        self.emit_byte(idx);
        self.type_tag.push(res_type);
    }
}
