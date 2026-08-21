use std::sync::Arc;

use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
    vm::Vm,
};

#[test]
pub fn test_rev_opcode() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_const(Value::Str(Arc::from("67")));
    chunk.write_chunk(OpCode::Constant as u8, 123);
    chunk.write_chunk(constant as u8, 123);
    chunk.write_chunk(OpCode::Reverse as u8, 123);
    chunk.write_chunk(OpCode::Print as u8, 123);
    chunk.write_chunk(OpCode::Return as u8, 123);
    Vm::new().test_chunk(chunk);
}
