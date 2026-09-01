pub struct Parser {
    current: Token,
    previous: Token,
    had_err: bool,
    painc_mode: bool,
    compiling_chunk: Chunk,
    compiler: Compiler,
    const_table: HashMap<String, Value>,
    type_tag: Vec<TypeTag>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeTag {
    Int,
    Float,
    Str,
    Bool,
    Void,
}

impl fmt::Display for TypeTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeTag::Int => write!(f, "int"),
            TypeTag::Float => write!(f, "float"),
            TypeTag::Bool => write!(f, "bool"),
            TypeTag::Str => write!(f, "str"),
            TypeTag::Void => write!(f, "void"),
        }
    }
}

pub struct Compiler {
    locals: Vec<Local>,
    local_count: i32,
    scope_depth: i32,
}

pub struct Local {
    name: Token,
    depth: i32,
    is_mut: bool,
    type_tag: TypeTag,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            locals: Vec::new(),
            local_count: 0,
            scope_depth: 0,
        }
    }
}

use std::{
    collections::HashMap,
    fmt::{self},
    sync::Arc,
};

use crate::{
    chunk::{
        Chunk,
        OpCode::{self, GetLocal, SetLocal},
    },
    compiler::TypeTag::Void,
    scanner::{
        Scanner, Token,
        TokenType::{self},
    },
    value::Value,
};

#[derive(Debug, Clone, Copy)]
struct ParseRule {
    prefix: Option<fn(&mut Parser, &mut Scanner, bool)>,
    infix: Option<fn(&mut Parser, &mut Scanner)>,
    precedence: Precedence,
}

const NONE_RULE: ParseRule = ParseRule {
    precedence: Precedence::None,
    prefix: None,
    infix: None,
};

static RULES: [ParseRule; 48] = [
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
    }, // Number,
    NONE_RULE, // Print
    NONE_RULE, // Println
    NONE_RULE, // Abs
    NONE_RULE, // Floor
    NONE_RULE, // Ceil
    NONE_RULE, // Round
    NONE_RULE, // Let
    NONE_RULE, // ~
    NONE_RULE, // Const
    NONE_RULE, // Fn
    NONE_RULE, // Sqrt
    NONE_RULE, // IsEmpty
    NONE_RULE, // Trim,
    NONE_RULE, // Reverse
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

// remove later!
#[allow(warnings)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
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

impl Parser {
    pub fn new() -> Self {
        Self {
            current: Token::default(),
            previous: Token::default(),
            had_err: false,
            painc_mode: false,
            compiling_chunk: Chunk::new(),
            compiler: Compiler::new(),
            const_table: HashMap::new(),
            type_tag: Vec::new(),
        }
    }

    pub fn compile(&mut self, source: String, chunk: &mut Chunk) -> bool {
        let mut scanner = Scanner::new(&source);

        self.compiling_chunk = chunk.clone();
        self.had_err = false;
        self.painc_mode = false;

        self.advance(&mut scanner);

        while !self.match_consume(&TokenType::Eof, &mut scanner) {
            self.declaration(&mut scanner);
        }

        self.end_compiler();
        *chunk = self.compiling_chunk.clone();
        !self.had_err
    }

    fn advance(&mut self, scanner: &mut Scanner) {
        self.previous = self.current.clone();

        loop {
            self.current = scanner.scan_tokens();

            if self.current.token_type != TokenType::Error {
                break;
            }
            let token_location = self.current.start.clone();
            self.error_at_current(&token_location);
        }
    }

    fn expression(&mut self, scanner: &mut Scanner) {
        self.parse_precedence(Precedence::Assignment, scanner);
    }

    fn consume(&mut self, token_type: TokenType, message: &str, scanner: &mut Scanner) {
        if self.current.token_type == token_type {
            self.advance(scanner);
            return;
        }

        self.error_at_current(message);
    }

    fn error_at_current(&mut self, message: &str) {
        let current_token = self.current.clone();
        self.error_at(&current_token, message);
    }

    fn error(&mut self, message: &str) {
        let previous_token = self.previous.clone();
        self.error_at(&previous_token, message);
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.painc_mode {
            return;
        }

        self.painc_mode = true;

        eprint!("{} Error", token.line);

        if token.token_type == TokenType::Eof {
            eprint!(" at end")
        } else if token.token_type == TokenType::Error {
        } else {
            eprint!(" at {}+{}", token.length, token.start)
        }

        eprintln!(": {}", message);

        self.had_err = true
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.previous.line as u32;
        self.current_chunk().write_chunk(byte, line);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.compiling_chunk
    }

    fn end_compiler(&mut self) {
        #[cfg(feature = "DPC")]
        {
            use crate::debug::disassemble_chunk;
            if !self.painc_mode {
                disassemble_chunk(self.current_chunk());
            }
        }
        self.emit_byte(OpCode::Return as u8);
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn number(&mut self, _scanner: &mut Scanner, _can_assign: bool) {
        let value = &self.previous.start;

        if value.contains(".") {
            let float_value: f64 = value.parse::<f64>().unwrap_or_default();
            self.emit_constant(Value::Float(float_value));
            self.type_tag.push(TypeTag::Float);
        } else {
            let int_value: i64 = value.parse::<i64>().unwrap_or_default();
            self.emit_constant(Value::Int(int_value));
            self.type_tag.push(TypeTag::Int);
        }
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant as u8, constant);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.current_chunk().add_const(value) as u8;

        if constant > u8::MAX {
            self.error("Too many constants in one chunk.");
            return 0;
        }

        constant
    }

    fn grouping(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        self.expression(scanner);
        self.consume(
            TokenType::RigtParen,
            "Expect ')' after expression.",
            scanner,
        );
    }

    fn unary(&mut self, scanner: &mut Scanner, _can_assign: bool) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(Precedence::Unary, scanner);

        let type_tag = self.type_tag.pop().unwrap_or(Void);

        match type_tag {
            TypeTag::Int | TypeTag::Float => {}

            _ => self.error(&format!(
                "cannot use [{}] values with negate!, only numbers are allowed.",
                type_tag
            )),
        }

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate as u8),
            TokenType::Bang => self.emit_byte(OpCode::Not as u8),
            _ => return,
        }
    }

    fn binary(&mut self, scanner: &mut Scanner) {
        let operator_type = self.previous.token_type;
        let rule = Self::get_rule(operator_type);
        self.parse_precedence(rule.precedence, scanner);

        let type_tag2 = self.type_tag.pop().unwrap_or(Void);
        let type_tag1 = self.type_tag.pop().unwrap_or(Void);

        match (type_tag1, type_tag2) {
            (TypeTag::Int, TypeTag::Int) => {
                self.type_tag.push(TypeTag::Int);
            }
            (TypeTag::Int, TypeTag::Float) => {
                self.type_tag.push(TypeTag::Float);
            }
            (TypeTag::Float, TypeTag::Int) => {
                self.type_tag.push(TypeTag::Float);
            }
            (TypeTag::Str, TypeTag::Str) => {
                self.type_tag.push(TypeTag::Str);
            }
            _ => self.error(&format!(
                "mismatched types cannot use [{}] with [{}]",
                type_tag1, type_tag2
            )),
        }

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            TokenType::Modulo => self.emit_byte(OpCode::Modulo as u8),
            TokenType::BangEqual => self.emit_byte(OpCode::NotEqualTo as u8),
            TokenType::EqualEqual => self.emit_byte(OpCode::EqualTo as u8),
            TokenType::Greater => self.emit_byte(OpCode::GreaterThan as u8),
            TokenType::Lesser => self.emit_byte(OpCode::LessThan as u8),
            TokenType::GreaterEqual => self.emit_byte(OpCode::GreaterThanEq as u8),
            TokenType::LesserEqual => self.emit_byte(OpCode::LessThanEq as u8),
            _ => return,
        }
    }

    fn get_rule(token_type: TokenType) -> ParseRule {
        RULES[token_type as usize]
    }

    fn parse_precedence(&mut self, precedence: Precedence, scanner: &mut Scanner) {
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

    fn literal(&mut self, _scanner: &mut Scanner, _can_assign: bool) {
        match self.previous.token_type {
            TokenType::True => {
                self.emit_byte(OpCode::True as u8);
                self.type_tag.push(TypeTag::Bool);
            }
            TokenType::False => {
                self.emit_byte(OpCode::False as u8);
                self.type_tag.push(TypeTag::Bool);
            }
            TokenType::Void => {
                self.emit_byte(OpCode::Void as u8);
                self.type_tag.push(TypeTag::Void);
            }
            _ => return,
        }
    }

    fn declaration(&mut self, scanner: &mut Scanner) {
        self.statement(scanner);

        if self.painc_mode {
            self.synchronize();
        }
    }

    fn statement(&mut self, scanner: &mut Scanner) {
        let token = self.current.token_type;

        match token {
            TokenType::LeftBrace => {
                self.match_consume(&token, scanner);
                self.begin_scope();
                self.block(scanner);
                self.end_scope();
            }
            TokenType::Let => {
                self.match_consume(&token, scanner);
                self.variable_declaration(scanner);
            }
            TokenType::Println => {
                self.match_consume(&token, scanner);
                self.println_statement(scanner);
            }
            TokenType::Print => {
                self.match_consume(&token, scanner);
                self.print_statement(scanner);
            }
            TokenType::Const => {
                self.match_consume(&token, scanner);
                self.const_declaration(scanner);
            }
            TokenType::Fn => {
                self.match_consume(&token, scanner);
                self.fn_declaration(scanner);
            }
            _ => self.expression_statement(scanner),
        }
    }

    fn print_statement(&mut self, scanner: &mut Scanner) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.expression(scanner);
        self.consume(TokenType::Semicolon, "Expect ';' after value.", scanner);
        self.emit_byte(OpCode::Print as u8);
    }

    fn println_statement(&mut self, scanner: &mut Scanner) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.expression(scanner);
        self.consume(TokenType::Semicolon, "Expect ';' after value.", scanner);
        self.emit_byte(OpCode::Println as u8);
    }

    fn expression_statement(&mut self, scanner: &mut Scanner) {
        self.expression(scanner);
        self.consume(TokenType::Semicolon, "Expect ';' after value.", scanner);
        self.emit_byte(OpCode::Pop as u8);
    }

    fn fn_declaration(&mut self, scanner: &mut Scanner) {
        self.consume(TokenType::Identifier, "Expect name after fn.", scanner);
        self.consume(TokenType::LeftParen, "Expect '(' after fn name.", scanner);
        self.consume(TokenType::RigtParen, "Enclosed ')' expected.", scanner);
        self.consume(TokenType::LeftBrace, "Expect '{' after fn name.", scanner);
        self.begin_scope();
        self.block(scanner);
        self.end_scope();
    }

    fn match_consume(&mut self, token_type: &TokenType, scanner: &mut Scanner) -> bool {
        if !self.check(token_type) {
            return false;
        } else {
            self.advance(scanner);
            return true;
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        &self.current.token_type == token_type
    }

    fn synchronize(&mut self) {
        self.painc_mode = false;

        while self.current.token_type != TokenType::Eof {
            if self.previous.token_type == TokenType::Semicolon {
                return;
            } else {
                match self.current.token_type {
                    TokenType::Print | TokenType::Let => {
                        return;
                    }
                    _ => continue,
                }
            }
        }
    }

    fn variable_declaration(&mut self, scanner: &mut Scanner) {
        if self.compiler.scope_depth == 0 {
            self.error("Statements must be insaid a fn body");
            return;
        }

        self.parse_variable("Expect variable name.", scanner);

        let type_annotation = self.match_consume(&TokenType::Colon, scanner);

        let annotation_type = if type_annotation {
            self.advance(scanner);
            Some(self.previous.token_type)
        } else {
            None
        };

        if self.match_consume(&TokenType::Equal, scanner) {
            self.expression(scanner);
        } else {
            self.emit_byte(OpCode::Void as u8);
            self.type_tag.push(TypeTag::Void);
        }
        let type_tag = self.type_tag.pop().unwrap_or(Void);

        self.compiler.locals[self.compiler.local_count as usize - 1].type_tag = type_tag;

        if let Some(at) = annotation_type {
            match at {
                TokenType::Int => {
                    if type_tag != TypeTag::Int {
                        self.error(&format!(
                            "Mismatched types, expected [int] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Float => {
                    if type_tag != TypeTag::Float {
                        self.error(&format!(
                            "Mismatched types, expected [float] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Str => {
                    if type_tag != TypeTag::Str {
                        self.error(&format!(
                            "Mismatched types, expected [str] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Bool => {
                    if type_tag != TypeTag::Bool {
                        self.error(&format!(
                            "Mismatched types, expected [bool] found [{}]",
                            type_tag
                        ));
                    }
                }
                TokenType::Void => {
                    if type_tag != TypeTag::Void {
                        self.error(&format!(
                            "Mismatched types, expected [void] found [{}]",
                            type_tag
                        ));
                    };
                }
                _ => return,
            }
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            scanner,
        );

        self.define_variable();
    }

    fn const_declaration(&mut self, scanner: &mut Scanner) {
        let const_name = self.parse_const("Expect const name.", scanner);

        if !const_name.chars().all(|c| c.is_uppercase()) {
            self.error("Const name must be all in uppercase");
            return;
        }

        self.consume(TokenType::Equal, "Expect '=' after const name.", scanner);

        let const_value = self.const_value(scanner);

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            scanner,
        );

        self.define_const(const_name, const_value);
    }

    fn const_value(&mut self, scanner: &mut Scanner) -> Value {
        self.advance(scanner);

        match &self.previous.token_type {
            TokenType::Number => {
                let txt = &self.previous.start;
                if txt.contains('.') {
                    let value = txt.parse::<f64>().unwrap_or(0.0);
                    Value::Float(value)
                } else {
                    let value = txt.parse::<i64>().unwrap_or(0);
                    Value::Int(value)
                }
            }
            TokenType::String => {
                let raw = &self.previous.start;
                let trimmed = &raw[1..raw.len() - 1];
                Value::Str(Arc::from(trimmed))
            }
            TokenType::True => Value::Bool(true),
            TokenType::False => Value::Bool(false),
            TokenType::Void => Value::Void,
            _ => {
                self.error("const value must be a literal (number, string, bool, or Void).");
                return Value::Void;
            }
        }
    }

    fn parse_variable(&mut self, error_message: &str, scanner: &mut Scanner) -> u8 {
        let is_mut = self.match_consume(&TokenType::Tilde, scanner);
        self.consume(TokenType::Identifier, error_message, scanner);

        self.declare_variable();
        if self.compiler.scope_depth > 0 {
            let idx = self.compiler.local_count as usize - 1;
            self.compiler.locals[idx].is_mut = is_mut;
            return 0;
        }

        let token = self.previous.clone();
        self.identifier_constant(&token)
    }

    fn parse_const(&mut self, error_message: &str, scanner: &mut Scanner) -> String {
        self.consume(TokenType::Identifier, error_message, scanner);
        self.previous.start.clone()
    }

    fn identifier_constant(&mut self, name: &Token) -> u8 {
        self.make_constant(Value::Str(Arc::from(name.start.to_string())))
    }

    fn define_variable(&mut self) {
        self.mark_initialized();
    }

    fn define_const(&mut self, name: String, value: Value) {
        self.const_table.insert(name, value);
    }

    fn declare_variable(&mut self) {
        if self.compiler.scope_depth == 0 {
            return;
        }

        let name = self.previous.clone();
        self.add_local(name);
    }

    fn variable(&mut self, scanner: &mut Scanner, can_assign: bool) {
        let token = &self.previous.clone();
        self.named_variable(token, scanner, can_assign);
    }

    fn named_variable(&mut self, name: &Token, scanner: &mut Scanner, can_assign: bool) {
        if let Some(value) = self.const_table.get(&name.start).cloned() {
            self.emit_constant(value);
            return;
        }

        let Some((arg, is_mut, type_tag)) = self.resolve_local(name) else {
            self.error("");
            return;
        };

        if can_assign && is_mut && self.match_consume(&TokenType::Equal, scanner) {
            self.expression(scanner);

            let rhs_typetag = self.type_tag.pop().unwrap_or(Void);

            if rhs_typetag != type_tag {
                self.error(&format!(
                    "Mismatched types, expected [{}] found [{}]",
                    type_tag, rhs_typetag
                ));
            }

            self.emit_bytes(SetLocal as u8, arg);
        } else {
            self.type_tag.push(type_tag);
            self.emit_bytes(GetLocal as u8, arg);
        }
    }

    fn strings(&mut self, _scanner: &mut Scanner, _can_assign: bool) {
        let raw = &self.previous.start;
        let trimmed = &raw[1..raw.len() - 1];
        self.emit_constant(Value::Str(Arc::from(trimmed)));
        self.type_tag.push(TypeTag::Str);
    }

    fn block(&mut self, scanner: &mut Scanner) {
        while !self.check(&TokenType::RightBrace) && !self.check(&TokenType::Eof) {
            self.declaration(scanner);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.", scanner);
    }

    fn begin_scope(&mut self) {
        self.compiler.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.compiler.scope_depth -= 1;

        while self.compiler.locals.len() > 0
            && self.compiler.locals[self.compiler.local_count as usize - 1].depth
                > self.compiler.scope_depth
        {
            self.emit_byte(OpCode::Pop as u8);
            self.compiler.locals.pop();
            self.compiler.local_count -= 1;
        }
    }

    fn add_local(&mut self, name: Token) {
        self.compiler.locals.push(Local {
            name: name.clone(),
            depth: -1,
            is_mut: false,
            type_tag: TypeTag::Void,
        });
        self.compiler.local_count += 1;
    }

    fn resolve_local(&mut self, name: &Token) -> Option<(u8, bool, TypeTag)> {
        for i in 0..self.compiler.local_count {
            let local = &self.compiler.locals[i as usize];
            let is_mut = local.is_mut;
            let type_tag = local.type_tag;

            if Self::identifiers_equal(name, &local.name) {
                if local.depth == -1 {
                    self.error("Can't read local variable in its own initializer.");
                }
                return Some((i as u8, is_mut, type_tag));
            }
        }
        None
    }

    fn identifiers_equal(token_a: &Token, token_b: &Token) -> bool {
        if token_a.length != token_b.length {
            return false;
        }

        token_a.start == token_b.start
    }

    fn mark_initialized(&mut self) {
        self.compiler.locals[self.compiler.local_count as usize - 1].depth =
            self.compiler.scope_depth;
    }
}
