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
}
