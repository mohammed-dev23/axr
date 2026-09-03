use super::*;

#[allow(warnings)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * / %
    Unary,      // - !
    Call,       // . ()
    PRIMARY,
}

#[derive(Debug, Clone, Copy)]
pub struct ParseRule {
    pub prefix: Option<fn(&mut Parser, &mut Scanner, bool)>,
    pub infix: Option<fn(&mut Parser, &mut Scanner)>,
    pub precedence: Precedence,
}

const NONE_RULE: ParseRule = ParseRule {
    precedence: Precedence::None,
    prefix: None,
    infix: None,
};

static RULES: [ParseRule; 49] = [
    ParseRule {
        prefix: Some(Parser::grouping),
        infix: None,
        precedence: Precedence::None,
    }, // (
    NONE_RULE, // )
    NONE_RULE, // {
    NONE_RULE, // }
    NONE_RULE, // ,
    NONE_RULE, // .
    ParseRule {
        prefix: Some(Parser::unary),
        infix: Some(Parser::binary),
        precedence: Precedence::Term,
    }, // -
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Term,
    }, // +
    NONE_RULE, // ;
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Factor,
    }, // /,
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Factor,
    }, // *
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Factor,
    }, // %
    NONE_RULE, // :
    ParseRule {
        prefix: Some(Parser::unary),
        infix: None,
        precedence: Precedence::None,
    }, // !
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Equality,
    }, // !=
    NONE_RULE, // =
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Comparison,
    }, // ==
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Equality,
    }, // >
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Comparison,
    }, // >=
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Comparison,
    }, // <
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Comparison,
    }, // <=
    ParseRule {
        prefix: Some(Parser::variable),
        infix: None,
        precedence: Precedence::None,
    }, // Identifier
    ParseRule {
        prefix: Some(Parser::strings),
        infix: None,
        precedence: Precedence::None,
    }, // String
    ParseRule {
        prefix: Some(Parser::number),
        infix: None,
        precedence: Precedence::None,
    }, // Number
    ParseRule {
        prefix: Some(Parser::char),
        infix: None,
        precedence: Precedence::None,
    }, // Char
    NONE_RULE, // Print
    NONE_RULE, // Println
    ParseRule {
        prefix: Some(Parser::abs_expr),
        infix: None,
        precedence: Precedence::None,
    }, // Abs
    ParseRule {
        prefix: Some(Parser::floor_expr),
        infix: None,
        precedence: Precedence::None,
    }, // Floor
    ParseRule {
        prefix: Some(Parser::ceil_expr),
        infix: None,
        precedence: Precedence::None,
    }, // Ceil
    ParseRule {
        prefix: Some(Parser::round_expr),
        infix: None,
        precedence: Precedence::None,
    }, // Round
    NONE_RULE, // Let
    NONE_RULE, // ~
    NONE_RULE, // Const
    NONE_RULE, // Fn
    ParseRule {
        prefix: Some(Parser::squareroot_expr),
        infix: None,
        precedence: Precedence::None,
    }, // Sqrt
    ParseRule {
        prefix: Some(Parser::isempty_expr),
        infix: None,
        precedence: Precedence::None,
    }, // IsEmpty
    ParseRule {
        prefix: Some(Parser::trim_expr),
        infix: None,
        precedence: Precedence::None,
    }, // Trim
    ParseRule {
        prefix: Some(Parser::rev_expr),
        infix: None,
        precedence: Precedence::None,
    }, // Reverse
    ParseRule {
        prefix: Some(Parser::literal),
        infix: None,
        precedence: Precedence::None,
    }, // True
    ParseRule {
        prefix: Some(Parser::literal),
        infix: None,
        precedence: Precedence::None,
    }, // False
    NONE_RULE, // Int
    NONE_RULE, // Str
    NONE_RULE, // Float
    NONE_RULE, // Bool
    NONE_RULE, // Error
    NONE_RULE, // Eof
    ParseRule {
        prefix: Some(Parser::literal),
        infix: None,
        precedence: Precedence::None,
    }, // void
    NONE_RULE, // Nai
];

impl Parser {
    pub fn get_rule(token_type: TokenType) -> ParseRule {
        RULES[token_type as usize]
    }

    pub fn parse_precedence(&mut self, precedence: Precedence, scanner: &mut Scanner) {
        self.advance(scanner);

        let rule = Self::get_rule(self.previous.token_type);
        let can_assign = precedence <= Precedence::Assignment;

        if let Some(prefix) = rule.prefix {
            prefix(self, scanner, can_assign);
        } else {
            self.error("Expect expression.");
            return;
        }

        if can_assign && self.match_consume(&TokenType::Equal, scanner) {
            self.error("Invalid assignment target.");
        }

        while precedence <= Self::get_rule(self.current.token_type).precedence {
            self.advance(scanner);
            let infix_rule = Self::get_rule(self.previous.token_type).infix.unwrap();
            infix_rule(self, scanner);
        }
    }
}
