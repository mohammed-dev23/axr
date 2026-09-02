pub mod core;
pub mod emit;
pub mod expr;
pub mod locals;
pub mod rules;
pub mod stmt;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::{
        core::{Parser, TypeTag},
        locals::Local,
        rules::Precedence,
    },
    scanner::{Scanner, Token, TokenType},
    value::Value,
};
use std::{collections::HashMap, fmt, sync::Arc};
