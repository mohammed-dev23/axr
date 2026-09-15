use crate::scanner::TokenType::RigtParen;

use super::*;

impl Parser {
    pub fn function(
        &mut self,
        function_type: FunctionType,
        scanner: &mut Scanner,
        function_name: &str,
    ) {
        // here we swap the main stream compiler with the compiler of this function so
        // it could compiler the fn as normal.
        let enclosing = std::mem::replace(&mut self.compiler, Compiler::new(function_type));
        // the old compiler the swaped one we push to the stack of compilers so we dont lose
        // track for it
        self.compiler_stack.push(enclosing);

        //we hand the function's name to it as a value, or like metadata so another
        // things could inspict that name and make changes to the function
        self.compiler.function.function.name = function_name.to_string();

        self.begin_scope();

        self.consume(
            TokenType::LeftParen,
            "Expect '('  after function name.",
            scanner,
        );

        if !self.check(&TokenType::RigtParen) {
            loop {
                self.compiler.function.function.arity += 1;

                if self.compiler.function.function.arity > 255 {
                    self.error_at_current("Can't have more than 255 parameters");
                }

                self.consume(TokenType::Identifier, "Expect parameters name.", scanner);
                self.variable_declaration(scanner);
                self.mark_initialized();

                if !self.match_consume(&TokenType::Comma, scanner) {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RigtParen,
            "Expect ')' after parameters.",
            scanner,
        );

        self.consume(
            TokenType::LeftBrace,
            "Expect '{' before function body.",
            scanner,
        );
        self.block(scanner);

        // we get the funtion with all it's compieled stuff from end compiler and we save it
        let function = Arc::new(self.end_compiler());
        // we swap back the last working function
        self.compiler = self.compiler_stack.pop().expect("compiler stack underflow");

        // since functions are fisrt class values we emit them just as if they where normal values
        // like 'Str' or 'Int' etc
        let function_value = self.make_constant(Value::Function(function));
        self.emit_bytes(OpCode::Constant as u8, function_value);
    }

    pub fn call(&mut self, scanner: &mut Scanner) {
        let arg_count = self.argument_list(scanner);
        self.emit_bytes(OpCode::Call as u8, arg_count as u8);
    }

    pub fn argument_list(&mut self, scanner: &mut Scanner) -> usize {
        let mut arg_count = 0;

        loop {
            if !self.check(&RigtParen) {
                self.expression(scanner);

                if arg_count == 255 {
                    self.error("Can't have more than 255 arguments.");
                }

                arg_count += 1;
                if !self.match_consume(&TokenType::Comma, scanner) {
                    break;
                }
            } else {
                break;
            }
        }

        self.consume(RigtParen, "Expect ')' after arguments", scanner);
        arg_count
    }
}
