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

                self.parse_variable("Expect a parameter's name", scanner);
                self.define_variable();

                self.consume(
                    TokenType::Colon,
                    "Expect ':' after paramters, and a type",
                    scanner,
                );

                self.advance(scanner);

                let annotation_type = self.previous.token_type;

                let array = if annotation_type == TokenType::Array {
                    self.consume(TokenType::LeftBracket, "Exp", scanner);

                    let array = match self.current.token_type {
                        TokenType::Int => TypeTag::Array(Arc::new(TypeTag::Int)),
                        TokenType::Unt => TypeTag::Array(Arc::new(TypeTag::Unt)),
                        TokenType::Float => TypeTag::Array(Arc::new(TypeTag::Float)),
                        TokenType::Str => TypeTag::Array(Arc::new(TypeTag::Str)),
                        TokenType::Bool => TypeTag::Array(Arc::new(TypeTag::Bool)),
                        TokenType::Char => TypeTag::Array(Arc::new(TypeTag::Char)),
                        _ => TypeTag::Array(Arc::new(TypeTag::Void)),
                    };

                    self.advance(scanner);
                    self.consume(TokenType::RightBracket, "Exp", scanner);
                    array
                } else {
                    TypeTag::Array(Arc::new(TypeTag::Void))
                };

                let opt = if annotation_type == TokenType::Opt {
                    self.consume(TokenType::LeftBracket, "Exp", scanner);

                    let opt = match self.current.token_type {
                        TokenType::Int => TypeTag::Opt(Arc::new(TypeTag::Int)),
                        TokenType::Unt => TypeTag::Opt(Arc::new(TypeTag::Unt)),
                        TokenType::Float => TypeTag::Opt(Arc::new(TypeTag::Float)),
                        TokenType::Str => TypeTag::Opt(Arc::new(TypeTag::Str)),
                        TokenType::Bool => TypeTag::Opt(Arc::new(TypeTag::Bool)),
                        TokenType::Char => TypeTag::Opt(Arc::new(TypeTag::Char)),
                        TokenType::Array => TypeTag::Opt(Arc::new(array.clone())),
                        _ => TypeTag::Void,
                    };

                    self.advance(scanner);
                    self.consume(TokenType::RightBracket, "Exp", scanner);
                    opt
                } else {
                    TypeTag::Opt(Arc::new(Void))
                };

                let expected_type = match annotation_type {
                    TokenType::Int => TypeTag::Int,
                    TokenType::Str => TypeTag::Str,
                    TokenType::Bool => TypeTag::Bool,
                    TokenType::Float => TypeTag::Float,
                    TokenType::Char => TypeTag::Char,
                    TokenType::Unt => TypeTag::Unt,
                    TokenType::Array => array,
                    TokenType::Opt => opt,
                    _ => TypeTag::Void,
                };

                self.compiler.locals[(self.compiler.local_count - 1) as usize].type_tag =
                    expected_type;

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

        if self.match_consume(&TokenType::Arrow, scanner) {
            self.advance(scanner);
        }

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
        let (arg_count, turbofish, generic) = self.argument_list(scanner);
        self.emit_bytes(OpCode::Call as u8, arg_count as u8);

        if turbofish {
            self.emit_byte(generic.as_bytes());
        } else {
            self.emit_byte(false as u8);
        }
    }

    pub fn argument_list(&mut self, scanner: &mut Scanner) -> (usize, bool, TypeTag) {
        let mut arg_count = 0;
        let mut turbofish = false;
        let mut type_tag = TypeTag::Void;
        let functions_name = &self.prevprev.start.clone();
        let table = self.function_info.parameters_type_tag_table.clone();

        let stack = table
            .get(functions_name)
            .expect("Expected function found nothing.")
            .borrow_mut();

        loop {
            if !self.check(&RigtParen) {
                self.expression(scanner);

                let type_tag = self.type_tag.pop().expect(TYPETAG_ERR);

                let expected_type_tag = stack.get(arg_count).expect(TYPETAG_ERR);

                if &type_tag != expected_type_tag {
                    self.error(&format!(
                        "Mismatched types expected [{}] found [{}]",
                        expected_type_tag, type_tag
                    ));
                }

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

        if self.match_consume(&TokenType::DoubleColon, scanner) {
            self.consume(
                TokenType::LeftBracket,
                "Expect '[' after :: for turbofish",
                scanner,
            );

            self.advance(scanner);
            let generic = self.previous.token_type.as_typetag().unwrap();

            self.consume(TokenType::RightBracket, "Expect ']' after generic", scanner);

            self.function_info
                .return_type_tag_table
                .insert(functions_name.clone(), generic.clone());

            type_tag = generic;
            turbofish = true;
        }

        let expected_return_type = match self
            .function_info
            .return_type_tag_table
            .get(&self.prevprev.start)
            .cloned()
        {
            Some(x) => x,
            None => {
                self.error("Function must return value! , if it was a call than it need a ::[T].");
                return (0, false, TypeTag::Void);
            }
        };

        self.type_tag.push(expected_return_type);

        if turbofish {
            (arg_count, true, type_tag)
        } else {
            (arg_count, false, TypeTag::Void)
        }
    }
}
