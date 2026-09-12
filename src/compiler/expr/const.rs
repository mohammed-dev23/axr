use super::*;

impl Parser {
    pub fn const_value(&mut self, scanner: &mut Scanner) -> (Value, Wrappers) {
        self.advance(scanner);

        match &self.previous.token_type {
            TokenType::Number => {
                let txt = &self.previous.start;
                if txt.contains('.') {
                    let value = txt.parse::<f64>().unwrap_or(0.0);
                    (Value::Float(value), Wrappers::None(Id(TypeId::Float)))
                } else if self
                    .expected_type
                    .is_some_and(|t| t == Wrappers::None(Id(TypeId::Unt)))
                {
                    let value = txt.parse::<u64>().unwrap_or(0);
                    (Value::Unt(value), Wrappers::None(Id(TypeId::Unt)))
                } else {
                    let value = txt.parse::<i64>().unwrap_or(0);
                    (Value::Int(value), Wrappers::None(Id(TypeId::Int)))
                }
            }
            TokenType::String => {
                let raw = &self.previous.start;
                let trimmed = &raw[1..raw.len() - 1];
                (
                    Value::Str(Arc::from(trimmed)),
                    Wrappers::None(Id(TypeId::Str)),
                )
            }
            TokenType::True => (Value::Bool(true), Wrappers::None(Id(TypeId::Bool))),
            TokenType::False => (Value::Bool(false), Wrappers::None(Id(TypeId::Bool))),
            TokenType::Char => {
                let raw = &self.previous.start;
                let trimmed = &raw[1..raw.len() - 1];
                let into_chars: Vec<char> = trimmed.chars().collect();

                if into_chars.len() != 1 {
                    self.error("Char type cannot contain more than one char.");
                    return (Value::Void, Wrappers::None(Id(TypeId::Void)));
                }

                (Value::Char(into_chars[0]), Wrappers::None(Id(TypeId::Char)))
            }
            TokenType::LeftBracket => {
                let mut values = Vec::new();

                let (fvalue, ftype_tag) = self.const_value(scanner);
                let ftype_tag = ftype_tag.as_typeid();
                values.push(fvalue);

                while self.current.token_type == TokenType::Comma {
                    self.match_consume(&TokenType::Comma, scanner);
                    let (value, typetag) = self.const_value(scanner);

                    if ftype_tag != typetag.as_typeid() {
                        self.error("Arrays must contain the same type for all of its slots.");
                    }

                    values.push(value);
                }

                let expected_array_type = self.expected_type.take().unwrap_or_else(|| {
                    self.error("Array[Type] annotation needed.");
                    Wrappers::None(TypeTag::Array(Void))
                });

                self.consume(
                    TokenType::RightBracket,
                    "Expected ']' at the end of array",
                    scanner,
                );

                (
                    Value::Array(Arc::new(Mutex::new(values))),
                    Wrappers::None(TypeTag::Array(expected_array_type.as_typeid())),
                )
            }

            TokenType::Void => (Value::Void, Wrappers::None(Id(TypeId::Void))),
            TokenType::Some => {
                let outer_expected = self.expected_type.take();
                self.expected_type = outer_expected.map(|w| Wrappers::None(w.extract()));

                self.consume(TokenType::LeftParen, "Expect '(' after Some", scanner);
                let (value, inner_type) = self.const_value(scanner);
                self.consume(TokenType::RigtParen, "Enclosed '(' expect ')'", scanner);

                if let Some(outer) = outer_expected {
                    let expected_inner = Wrappers::None(outer.extract());
                    if inner_type != expected_inner {
                        self.error(&format!(
                            "Mismatched types, expected [{}] found [{}] inside 'Some(...)'",
                            expected_inner, inner_type
                        ));
                    }
                } else {
                    self.error(
                        "Some(...) needs a type context, e.g. `const X : Opt[int] = Some(5);`",
                    );
                }

                (
                    Value::Opt(crate::value::OptWrapper::Some(Box::new(value))),
                    Wrappers::Opt(inner_type.extract()),
                )
            }
            TokenType::None => (
                Value::Opt(crate::value::OptWrapper::None),
                Wrappers::Opt(TypeTag::Id(TypeId::None)),
            ),
            _ => {
                self.error("const value must be a literal (number, string, bool, Array,or Void).");
                (Value::Void, Wrappers::None(Id(TypeId::Void)))
            }
        }
    }
}
