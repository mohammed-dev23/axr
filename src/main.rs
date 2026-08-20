use std::fs::read_to_string;

use crate::{
    chunk::{Chunk, OpCode},
    vm::Vm,
};

mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

fn main() -> std::io::Result<()> {
    cli()?;

    let mut chunk = Chunk::new();
    let constant = chunk.add_const(value::Value::Float(1.23));
    chunk.write_chunk(OpCode::Constant as u8, 123);
    chunk.write_chunk(constant as u8, 123);
    let constant2 = chunk.add_const(value::Value::Float(-2.9));
    chunk.write_chunk(OpCode::Constant as u8, 123);
    chunk.write_chunk(constant2 as u8, 123);
    chunk.write_chunk(OpCode::Add as u8, 123);
    chunk.write_chunk(OpCode::Abs as u8, 123);
    chunk.write_chunk(OpCode::Round as u8, 123);
    chunk.write_chunk(OpCode::Print as u8, 123);
    chunk.write_chunk(OpCode::Return as u8, 123);
    Vm::new().interpret(cli()?);

    Ok(())
}

fn cli() -> std::io::Result<String> {
    let path = std::env::args().nth(1);
    let mut code = String::new();

    if let Some(x) = path {
        println!("{}", x);

        code = read_to_string(x)?;
    }

    Ok(code)
}
