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
            let mut stack: Vec<Wrappers> = Vec::new();

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
                        TokenType::Int => Wrappers::None(TypeTag::Array(TypeId::Int)),
                        TokenType::Unt => Wrappers::None(TypeTag::Array(TypeId::Unt)),
                        TokenType::Float => Wrappers::None(TypeTag::Array(TypeId::Float)),
                        TokenType::Str => Wrappers::None(TypeTag::Array(TypeId::Str)),
                        TokenType::Bool => Wrappers::None(TypeTag::Array(TypeId::Bool)),
                        TokenType::Char => Wrappers::None(TypeTag::Array(TypeId::Char)),
                        _ => Wrappers::None(TypeTag::Array(TypeId::Void)),
                    };

                    self.advance(scanner);
                    self.consume(TokenType::RightBracket, "Exp", scanner);
                    array
                } else {
                    Wrappers::None(Array(Void))
                };

                let opt = if annotation_type == TokenType::Opt {
                    self.consume(TokenType::LeftBracket, "Exp", scanner);

                    let opt = match self.current.token_type {
                        TokenType::Int => Wrappers::Opt(TypeTag::Id(TypeId::Int)),
                        TokenType::Unt => Wrappers::Opt(TypeTag::Id(TypeId::Unt)),
                        TokenType::Float => Wrappers::Opt(TypeTag::Id(TypeId::Float)),
                        TokenType::Str => Wrappers::Opt(TypeTag::Id(TypeId::Str)),
                        TokenType::Bool => Wrappers::Opt(TypeTag::Id(TypeId::Bool)),
                        TokenType::Char => Wrappers::Opt(TypeTag::Id(TypeId::Char)),
                        TokenType::Array => Wrappers::Opt(array.extract()),
                        _ => Wrappers::None(TypeTag::Id(TypeId::Void)),
                    };

                    self.advance(scanner);
                    self.consume(TokenType::RightBracket, "Exp", scanner);
                    opt
                } else {
                    Wrappers::Opt(Id(Void))
                };

                let expected_type = match annotation_type {
                    TokenType::Int => Wrappers::None(Id(TypeId::Int)),
                    TokenType::Str => Wrappers::None(Id(TypeId::Str)),
                    TokenType::Bool => Wrappers::None(Id(TypeId::Bool)),
                    TokenType::Float => Wrappers::None(Id(TypeId::Float)),
                    TokenType::Char => Wrappers::None(Id(TypeId::Char)),
                    TokenType::Unt => Wrappers::None(Id(TypeId::Unt)),
                    TokenType::Array => array,
                    TokenType::Opt => opt,
                    _ => Wrappers::None(Id(TypeId::Void)),
                };

                stack.push(expected_type);

                if !self.match_consume(&TokenType::Comma, scanner) {
                    break;
                }
            }

            self.parameters.type_tag_table.insert(
                function_name.to_string(),
                Rc::new(RefCell::new(stack.clone())),
            );
        } else {
            self.parameters
                .type_tag_table
                .insert(function_name.to_string(), Rc::new(RefCell::new(Vec::new())));
        }

        self.consume(
            TokenType::RigtParen,
            "Expect ')' after parameters.",
            scanner,
        );
    }
}
