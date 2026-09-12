use crate::chunk::OpCode::JumpIfFalse;

use super::super::*;

impl Parser {
    pub fn while_stmt(&mut self, scanner: &mut Scanner) {
        let loop_start = self.compiling_chunk.code.len();
        self.control_flow.loop_starts.push(loop_start);
        self.control_flow.stops.push(Vec::new());

        self.expression(scanner);

        let exit_jump = self.emit_jump(OpCode::JumpIfFalse as usize);
        self.emit_byte(OpCode::Pop as u8);
        self.statement(scanner);
        self.emit_loop(loop_start);

        self.patch_jump(exit_jump as usize);
        self.emit_byte(OpCode::Pop as u8);

        self.control_flow.loop_starts.pop();
        for i in self.control_flow.stops.pop().unwrap() {
            self.patch_jump(i as usize);
        }
    }

    pub fn loop_stmt(&mut self, scanner: &mut Scanner) {
        let loop_start = self.compiling_chunk.code.len();
        self.control_flow.loop_starts.push(loop_start);
        self.control_flow.stops.push(Vec::new());
        self.control_flow.locals_in.push(self.compiler.local_count);

        self.statement(scanner);
        self.emit_loop(loop_start);

        self.control_flow.loop_starts.pop();
        self.control_flow.locals_in.pop();
        for i in self.control_flow.stops.pop().unwrap() {
            self.patch_jump(i as usize);
        }
    }

    pub fn for_stmt(&mut self, scanner: &mut Scanner) {
        self.consume(
            TokenType::LeftParen,
            "Expect '(' at the start of 'for' clauses body",
            scanner,
        );

        self.consume(
            TokenType::Let,
            "Expect 'let' to initialize a clause",
            scanner,
        );
        self.variable_declaration(scanner);

        // we do not cont the var dec with the code len
        // so we start the mausrement after the dec

        let loop_start = self.compiling_chunk.code.len();

        // the expresion
        self.expression(scanner);
        let jump_if_false = self.emit_jump(JumpIfFalse as usize);
        self.emit_byte(OpCode::Pop as u8);
        //we pop that expresion after emiting it!

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after a condition clause",
            scanner,
        );

        // we skip the fisrt increment over here
        let body_jump = self.emit_jump(OpCode::Jump as usize);
        // it's like uncondtionl jump over the fisrt increment so it works like
        // condtion -> body , instead of condtion -> increment -> body so now 0 for example == 1 instead of 0
        let increment_start = self.compiling_chunk.code.len();
        // we meausere the increment start so we could in the next loop jump to the increment not the body again!
        self.expression(scanner);
        self.emit_byte(OpCode::Pop as u8);
        // we pop the expresion of the stack wich was increment
        self.emit_loop(loop_start);
        // we emit a loop to start over here that loop will go like
        // cond -> body
        self.patch_jump(body_jump);
        // we jump over increment!

        self.consume(
            TokenType::RigtParen,
            "Expect ')' at the end of 'for' clauses body",
            scanner,
        );

        self.control_flow.loop_starts.push(increment_start);
        self.control_flow.stops.push(Vec::new());
        self.control_flow.locals_in.push(self.compiler.local_count);
        //since the increment len is the new one we push it

        self.statement(scanner);
        self.emit_loop(increment_start);
        self.patch_jump(jump_if_false);
        // here the for loop start going throug increment again

        self.control_flow.loop_starts.pop();
        self.control_flow.locals_in.pop();
        for i in self.control_flow.stops.pop().unwrap() {
            self.patch_jump(i as usize);
        }

        // pop the init from the stack !
        self.emit_byte(OpCode::Pop as u8);
    }
}
