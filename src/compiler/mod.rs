pub mod builtins;
pub mod core;
pub mod emit;
pub mod expr;
pub mod locals;
pub mod methode;
pub mod rules;
pub mod stmt;
mod type_safety;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::{locals::Local, rules::Precedence},
    scanner::{Scanner, Token, TokenType},
    value::{Function, Value},
};

pub const TYPETAG_ERR: &str = "VM stack underflow — compiler emitted unbalanced bytecode";

use std::{collections::HashMap, fmt, sync::Arc};

pub use {TypeId::Void, TypeTag::Array, TypeTag::Id};

pub struct Parser {
    pub(in crate::compiler) current: Token,
    pub(in crate::compiler) previous: Token,
    pub(in crate::compiler) had_err: bool,
    pub(in crate::compiler) painc_mode: bool,
    pub(in crate::compiler) compiler: Compiler,
    pub(in crate::compiler) const_table: HashMap<String, (Value, Wrappers)>,
    pub(in crate::compiler) type_tag: Vec<Wrappers>,
    pub(in crate::compiler) expected_type: Option<Wrappers>,
    pub(in crate::compiler) control_flow: ControlFlow,
    pub(in crate::compiler) info: Info,
}

pub struct Compiler {
    pub(in crate::compiler) locals: Vec<Local>,
    pub(in crate::compiler) local_count: i32,
    pub(in crate::compiler) scope_depth: i32,
    pub(in crate::compiler) function: Functions,
}

#[allow(warnings)]
pub struct Functions {
    function: Function,
    function_type: FunctionType,
}

#[allow(warnings)]
pub enum FunctionType {
    Function,
    Script,
}

pub struct ControlFlow {
    pub loop_starts: Vec<usize>,
    pub stops: Vec<Vec<u8>>,
    pub locals_in: Vec<i32>,
}

pub struct Info {
    pub is_mut: Vec<bool>,
    pub last_local_slot: Option<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeTag {
    Id(TypeId),
    Array(TypeId),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Wrappers {
    Opt(TypeTag),
    None(TypeTag),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeId {
    Int,
    Float,
    Str,
    Bool,
    Char,
    Unt,
    #[allow(warnings)]
    None,
    Void,
}

impl Compiler {
    pub fn new(function_type: FunctionType) -> Self {
        let local = vec![Local {
            name: Token {
                start: "".to_string(),
                ..Default::default()
            },
            depth: 0,
            is_mut: false,
            type_tag: Wrappers::None(Id(Void)),
        }];

        Self {
            locals: local,
            local_count: 1,
            scope_depth: 0,
            function: Functions {
                function: Function::new(),
                function_type: function_type,
            },
        }
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current: Token::default(),
            previous: Token::default(),
            had_err: false,
            painc_mode: false,
            compiler: Compiler::new(FunctionType::Script),
            const_table: HashMap::new(),
            type_tag: Vec::new(),
            expected_type: None,
            control_flow: ControlFlow {
                loop_starts: Vec::new(),
                stops: Vec::new(),
                locals_in: Vec::new(),
            },
            info: Info {
                is_mut: Vec::new(),
                last_local_slot: Some(0),
            },
        }
    }
}
