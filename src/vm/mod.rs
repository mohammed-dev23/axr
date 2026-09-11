use std::{
    io::{Write, stdin, stdout},
    sync::Arc,
    sync::Mutex,
};

use crate::{
    chunk::{
        Chunk,
        OpCode::{self},
    },
    compiler::{
        self, TypeId,
        TypeTag::{self, Id},
        Wrappers,
    },
    value::{
        OptWrapper,
        Value::{self, Array, Char, Int, Str, Unt, Void},
    },
    vm::InterpretResult::RuntimeError,
};

mod core;
mod opcodes {
    mod arithmetic;
    mod builtins;
    mod cast;
    mod collocations;
    mod control_flow;
    mod io;
    mod stack;
    mod wrappers;
}

mod run {
    mod run_arithmetic;
    mod run_builtins;
    mod run_cast;
    mod run_collocations;
    mod run_control_flow;
    mod run_io;
    mod run_stack;
    mod run_wrappers;
}

pub const ERR_POP_MES: &str = "VM stack underflow — compiler emitted unbalanced bytecode";

pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

#[allow(warnings)]
#[derive(Debug, PartialEq, Eq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
    NotHandled,
    Done,
}

pub type Result<T> = std::result::Result<T, InterpretResult>;
