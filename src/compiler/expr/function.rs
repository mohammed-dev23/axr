use super::*;
use crate::scanner::TokenType::RigtParen;

impl Parser {
    pub fn function(
        &mut self,
        function_type: FunctionType,
        scanner: &mut Scanner,
        function_name: &str,
    ) -> Expr {
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

        let stmts = self.block(scanner);

        let arity = self.compiler.function.function.arity;

        // we get the funtion with all it's compieled stuff from end compiler and we save it
        // let function = Arc::new(self.end_compiler());
        // we swap back the last working function
        self.compiler = self.compiler_stack.pop().expect("compiler stack underflow");

        // since functions are fisrt class values we emit them just as if they where normal values
        // like 'Str' or 'Int' etc

        Expr::Function {
            name: function_name.to_string(),
            arity,
            block: Box::new(stmts),
            ftype: function_type,
        }
    }

    pub fn call(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        let (arg_count, args) = self.argument_list(scanner);

        Expr::FunctionCall {
            caller: Box::new(lhs.to_owned()),
            argument_count: arg_count,
            arguments: args,
        }
    }

    pub fn argument_list(&mut self, scanner: &mut Scanner) -> (usize, Vec<Expr>) {
        let mut arg_count = 0;
        let functions_name = &self.info.names.pop().expect("Expected a function's name");
        let table = self.function_info.parameters_type_tag_table.clone();
        let mut generic_param = TypeTag::Void;

        let mut args: Vec<Expr> = Vec::new();

        let stack = table
            .get(functions_name)
            .expect("Expected function found nothing.")
            .as_ref();

        loop {
            if !self.check(&RigtParen) {
                let value = self.parse_precedence(Precedence::Assignment, scanner);

                let type_tag = self.type_tag.pop().expect(TYPETAG_ERR);

                let mut expected_type_tag = {
                    let expected_stack = stack.borrow();
                    expected_stack.get(arg_count).cloned().expect(TYPETAG_ERR)
                };

                if expected_type_tag == TypeTag::Generic {
                    let idx = stack.borrow().iter().position(|t| t == &expected_type_tag);

                    if let Some(idx) = idx {
                        stack.borrow_mut()[idx] = type_tag.clone();
                    }

                    expected_type_tag = type_tag.clone();
                    generic_param = type_tag.clone();
                }

                if type_tag != expected_type_tag {
                    self.error(&format!(
                        "Mismatched types expected [{}] found [{}]",
                        expected_type_tag, type_tag
                    ));
                }

                if arg_count == 255 {
                    self.error("Can't have more than 255 arguments.");
                }

                arg_count += 1;
                args.push(value);

                if !self.match_consume(&TokenType::Comma, scanner) {
                    break;
                }
            } else {
                break;
            }
        }

        if self
            .function_info
            .return_type_tag_table
            .get(functions_name)
            .is_some_and(|t| t == &TypeTag::Generic)
        {
            self.function_info
                .return_type_tag_table
                .insert(functions_name.clone(), generic_param.clone());
        }

        self.consume(RigtParen, "Expect ')' after arguments", scanner);

        let return_type = self
            .function_info
            .return_type_tag_table
            .get(functions_name)
            .expect("a function must return a value!");

        self.type_tag.push(return_type.clone());

        (arg_count, args)
    }

    pub fn turbofish(&mut self, lhs: &Expr, scanner: &mut Scanner) -> Expr {
        let caller = self.info.names.pop().expect("Expected a function's name");

        self.consume(
            TokenType::LeftBracket,
            "Expect '[' at the start of a turbofish body",
            scanner,
        );

        self.advance(scanner);
        let generic = self.previous.token_type.as_typetag().unwrap();

        self.consume(
            TokenType::RightBracket,
            "Expect ']' at the end of a turbofish body",
            scanner,
        );

        if self
            .function_info
            .return_type_tag_table
            .contains_key(&caller)
        {
            self.error(&format!("caller '{}' does not need Turbofish ::[]", caller));
        }

        self.info.names.push(caller.clone());

        let idx = self.add_type_tag_to_chunk(generic.clone());

        self.type_tag.push(generic.clone());

        Expr::Turbofish {
            left: Box::new(lhs.to_owned()),
            caller,
            generic,
            idx,
        }
    }
}
