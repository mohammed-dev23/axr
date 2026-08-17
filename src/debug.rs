use std::u8;

use crate::chunk::{Chunk, OpCode};

// i used it here because debug is a feature enabled by Cfg
#[allow(warnings)]
pub fn disassemble_chunk(chunk: &Chunk) {
    let mut offset: usize = 0;

    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    let instruction = chunk.code[offset as usize];

    if offset > 0 && chunk.line[offset] == chunk.line[offset - 1] {
        println!("    | ")
    } else {
        print!("{} > ", chunk.line[offset])
    }

    match instruction {
        x if x == OpCode::Return as u8 => simple_instruction("Return", offset),
        x if x == OpCode::Constant as u8 => constant_instruction(chunk, "Constant", offset),
        _ => simple_instruction("Unknown opcode", offset),
    }
}

pub fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{:04}<->{}", offset, name);
    offset + 1
}

pub fn constant_instruction(chunk: &Chunk, name: &str, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    println!(
        "{}<->{}<->{}",
        name, constant, chunk.constants.values[constant as usize]
    );
    offset + 2
}
