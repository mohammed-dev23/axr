pub mod builtins;
pub mod core;
pub mod emit;
pub mod expr;
pub mod locals;
pub mod methode;
pub mod prepass;
pub mod rules;
pub mod stmt;
mod type_safety;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::{locals::Local, rules::Precedence},
    scanner::{Scanner, Token, TokenType},
    value::{Function, Value},
};

pub const TYPETAG_ERR: &str = "TypeTag stack underflow — compiler emitted unbalanced typetags";

pub use TypeTag::Void;
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc, sync::Arc};

pub struct Parser {
    pub(in crate::compiler) current: Token,
    pub(in crate::compiler) previous: Token,
    pub(in crate::compiler) prevprev: Token,
    pub(in crate::compiler) had_err: bool,
    pub(in crate::compiler) painc_mode: bool,
    pub(in crate::compiler) compiler: Compiler,
    pub(in crate::compiler) compiler_stack: Vec<Compiler>,
    pub(in crate::compiler) const_table: HashMap<String, (Value, TypeTag)>,
    pub(in crate::compiler) type_tag: Vec<TypeTag>,
    pub(in crate::compiler) expected_type: Option<TypeTag>,
    pub(in crate::compiler) control_flow: ControlFlow,
    pub(in crate::compiler) function_info: FunctionInfo,
    pub(in crate::compiler) info: Info,
}

pub struct Compiler {
    pub(in crate::compiler) locals: Vec<Local>,
    pub(in crate::compiler) local_count: i32,
    pub(in crate::compiler) scope_depth: i32,
    pub(in crate::compiler) function: Functions,
    pub(in crate::compiler) has_returned: bool,
}

pub struct Functions {
    function: Function,
    #[allow(warnings)]
    function_type: FunctionType,
}

pub struct FunctionInfo {
    parameters_type_tag_table: HashMap<String, Rc<RefCell<Vec<TypeTag>>>>,
    return_type_tag_table: HashMap<String, TypeTag>,
}

#[derive(PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum TypeTag {
    Int,
    Float,
    Str,
    Bool,
    Char,
    Unt,
    Void,
    None,
    Array(Arc<TypeTag>),
    Opt(Arc<TypeTag>),
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
            type_tag: TypeTag::Void,
        }];

        Self {
            locals: local,
            local_count: 1,
            scope_depth: 0,
            function: Functions {
                function: Function::new(),
                function_type: function_type,
            },
            has_returned: false,
        }
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current: Token::default(),
            previous: Token::default(),
            prevprev: Token::default(),
            had_err: false,
            painc_mode: false,
            compiler: Compiler::new(FunctionType::Script),
            compiler_stack: Vec::new(),
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
            function_info: FunctionInfo {
                parameters_type_tag_table: HashMap::new(),
                return_type_tag_table: HashMap::new(),
            },
        }
    }
}
