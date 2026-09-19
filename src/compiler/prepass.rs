use super::*;
use crate::scanner::TokenType::Identifier;

impl Parser {
    pub fn pre_pass(&mut self, source: &str) {
        let mut scanner = Scanner::new(source);

        loop {
            self.advance(&mut scanner);

            if self.current.token_type == TokenType::Fn {
                self.fn_prepass(&mut scanner);
            }

            if self.previous.token_type == TokenType::Eof {
                break;
            }
        }
    }

    pub fn fn_prepass(&mut self, scanner: &mut Scanner) {
        self.advance(scanner);

        self.consume(Identifier, "Expect a function's name.", scanner);
        let function_name = self.previous.start.clone();

        self.consume(
            TokenType::LeftParen,
            "Expect '('  after function name.",
            scanner,
        );

        if !self.check(&TokenType::RigtParen) {
            let mut stack: Vec<TypeTag> = Vec::new();

            if function_name == "main" {
                self.error("main() can not have parameters");
            }

            loop {
                self.consume(Identifier, "Expect a parameter's name", scanner);

                self.consume(
                    TokenType::Colon,
                    "Expect ':' after parameter's name for type",
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
                    TokenType::Array => array.clone(),
                    TokenType::Opt => opt,
                    _ => TypeTag::Void,
                };

                stack.push(expected_type);

                if !self.match_consume(&TokenType::Comma, scanner) {
                    break;
                }
            }

            self.function_info.parameters_type_tag_table.insert(
                function_name.to_string(),
                Rc::new(RefCell::new(stack.clone())),
            );
        } else {
            self.function_info
                .parameters_type_tag_table
                .insert(function_name.to_string(), Rc::new(RefCell::new(Vec::new())));
        }

        self.consume(
            TokenType::RigtParen,
            "Expect ')' after parameters.",
            scanner,
        );

        if self.match_consume(&TokenType::Arrow, scanner) {
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

            self.function_info
                .return_type_tag_table
                .insert(function_name.to_string(), expected_type);
        } else {
            self.function_info
                .return_type_tag_table
                .insert(function_name.to_string(), Void);
        }
    }

    pub fn check_main(&mut self) {
        if self
            .function_info
            .parameters_type_tag_table
            .get(&"main".to_string())
            .is_none()
        {
            self.error("Expected main() entry poin");
        }
    }
}
