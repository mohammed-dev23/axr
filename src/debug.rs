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
        x if x == OpCode::Negate as u8 => simple_instruction("Negate", offset),
        x if x == OpCode::Add as u8 => simple_instruction("Add", offset),
        x if x == OpCode::Subtract as u8 => simple_instruction("Subtract", offset),
        x if x == OpCode::Multiply as u8 => simple_instruction("Multiply", offset),
        x if x == OpCode::Divide as u8 => simple_instruction("Divide", offset),
        x if x == OpCode::GreaterThan as u8 => simple_instruction("GreaterThan", offset),
        x if x == OpCode::LessThan as u8 => simple_instruction("LessThan", offset),
        x if x == OpCode::GreaterThanEq as u8 => simple_instruction("GreaterThanEq", offset),
        x if x == OpCode::LessThanEq as u8 => simple_instruction("LessThanEq", offset),
        x if x == OpCode::EqualTo as u8 => simple_instruction("EqualTo", offset),
        x if x == OpCode::NotEqualTo as u8 => simple_instruction("NotEqualTo", offset),
        x if x == OpCode::Abs as u8 => simple_instruction("Abs", offset),

        x if x == OpCode::Reverse as u8 => simple_instruction("Reverse", offset),
        x if x == OpCode::True as u8 => simple_instruction("True", offset),
        x if x == OpCode::False as u8 => simple_instruction("False", offset),
        x if x == OpCode::Void as u8 => simple_instruction("Void", offset),
        x if x == OpCode::Not as u8 => simple_instruction("Not", offset),
        x if x == OpCode::Pop as u8 => simple_instruction("Pop", offset),
        x if x == OpCode::GetLocal as u8 => byte_instruction(chunk, "GetLocal", offset),
        x if x == OpCode::SetLocal as u8 => byte_instruction(chunk, "SetLocal", offset),
        x if x == OpCode::JumpIfFalse as u8 => jump_instruction(chunk, "JumpIfFalse", 1, offset),
        x if x == OpCode::Jump as u8 => jump_instruction(chunk, "JumpIfFalse", 1, offset),

        x if x == OpCode::Cast as u8 => simple_instruction("Cast", offset),
        x if x == OpCode::Loop as u8 => jump_instruction(chunk, "Loop", -1, offset),
        x if x == OpCode::Println as u8 => simple_instruction("Println", offset),
        x if x == OpCode::Call as u8 => byte_instruction(chunk, "Call", offset),
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

pub fn byte_instruction(chunk: &Chunk, name: &str, offset: usize) -> usize {
    let slot = chunk.code[offset - 1];
    println!("{}<->{}", name, slot);
    return offset + 2;
}

pub fn jump_instruction(chunk: &Chunk, name: &str, sign: isize, offset: usize) -> usize {
    let mut jump = ((chunk.code[offset + 1] as u16) << 8) as u16;
    let x = chunk.code[offset + 2] as u16;

    jump |= x;
    println!(
        "{:<16} {:4} -> {}",
        name,
        offset,
        offset as isize + 3 + sign * jump as isize
    );
    offset + 3
}
