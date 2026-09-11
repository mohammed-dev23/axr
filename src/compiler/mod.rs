pub mod builtins;
pub mod core;
pub mod emit;
pub mod expr;
pub mod locals;
pub mod methode;
pub mod rules;
pub mod stmt;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::{locals::Local, rules::Precedence},
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

pub const TYPETAG_ERR: &str = "VM stack underflow — compiler emitted unbalanced bytecode";

use std::{collections::HashMap, fmt, sync::Arc};

pub use {TypeId::Void, TypeTag::Array, TypeTag::Id};

pub struct Parser {
    pub(in crate::compiler) current: Token,
    pub(in crate::compiler) previous: Token,
    pub(in crate::compiler) had_err: bool,
    pub(in crate::compiler) painc_mode: bool,
    pub(in crate::compiler) compiling_chunk: Chunk,
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
