use std::{env, fs::read_to_string};

use crate::{
    chunk::{Chunk, OpCode},
    vm::Vm,
};

mod chunk;
mod debug;
mod value;
mod vm;

fn main() -> std::io::Result<()> {
    cli()?;

    let mut chunk = Chunk::new();
    let constant = chunk.add_const(value::Value::Int(1));
    chunk.write_chunk(OpCode::Constant as u8, 123);
    chunk.write_chunk(constant as u8, 123);
    let constant2 = chunk.add_const(value::Value::Int(0));
    chunk.write_chunk(OpCode::Constant as u8, 123);
    chunk.write_chunk(constant2 as u8, 123);
    chunk.write_chunk(OpCode::NotEqualTo as u8, 123);
    chunk.write_chunk(OpCode::Return as u8, 123);
    Vm::new().interpret(chunk);

    Ok(())
}

fn cli() -> std::io::Result<()> {
    let path = env::args().nth(1);

    if let Some(x) = path {
        println!("{}", x);

        let _file = read_to_string(x)?;
    }

    Ok(())
}
