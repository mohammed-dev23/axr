use super::super::*;

impl Parser {
    pub fn stop_stmt(&mut self, scanner: &mut Scanner) {
        if self.control_flow.stops.is_empty() {
            self.error("'stop' used outside of a loop.");
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            scanner,
        );

        let exit_jump = self.emit_jump(Jump as usize);
        let count = *self.control_flow.locals_in.last().unwrap();

        for _ in count..self.compiler.local_count {
            self.emit_byte(OpCode::Pop as u8);
        }

        if let Some(jumps) = self.control_flow.stops.last_mut() {
            jumps.push(exit_jump as u8);
        }
    }

    pub fn skip_stmt(&mut self, scanner: &mut Scanner) {
        if self.control_flow.loop_starts.is_empty() {
            self.error("'skip' used outside of a loop.");
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            scanner,
        );

        let loop_start = *self.control_flow.loop_starts.last().unwrap();
        let count = *self.control_flow.locals_in.last().unwrap();

        for _ in count..self.compiler.local_count {
            self.emit_byte(OpCode::Pop as u8);
        }

        self.emit_loop(loop_start);
    }

    pub fn return_stmt(&mut self, scanner: &mut Scanner) {
        let function_name = &self.compiler.function.function.name;

        let expected_return_type = match self
            .function_info
            .return_type_tag_table
            .get(function_name)
            .cloned()
        {
            Some(x) => x,
            None => return self.error("Function name was not found in the table!."),
        };

        if self.match_consume(&TokenType::Semicolon, scanner) {
            if expected_return_type != Wrappers::None(TypeTag::Id(Void)) {
                self.error(&format!(
                    "Expected [{}] found [{}]",
                    expected_return_type,
                    Wrappers::None(TypeTag::Id(Void))
                ));
            }

            self.emit_byte(OpCode::Void as u8);
            self.emit_byte(OpCode::Return as u8);
        } else {
            self.expression(scanner);

            let type_tag = self.type_tag.pop().expect(TYPETAG_ERR);

            if expected_return_type != type_tag {
                self.error(&format!(
                    "Expected [{}] found [{}]!. ",
                    expected_return_type, type_tag
                ));
            }

            self.emit_byte(OpCode::Return as u8);

            self.consume(
                TokenType::Semicolon,
                "Expect ';' at the end of the return statement",
                scanner,
            );
        }

        self.compiler.has_returned = true;
    }
}
