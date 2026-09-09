use crate::{
    compiler::core::{TypeId::Void, TypeTag::Array},
    value::Value::Int,
};

use super::*;

impl Parser {
    pub fn methode(&mut self, scanner: &mut Scanner) {
        if self.compiler.scope_depth == 0 {
            self.error("Statement must be insaid a fn body");
            return;
        }

        self.consume(TokenType::Identifier, "expected methode name", scanner);
        let methode_name = self.previous.start.clone();

        self.consume(TokenType::LeftParen, "Expect '(' before value.", scanner);
        match methode_name.trim() {
            "trim" => self.trim_methode(),
            "is_empty" => self.isempty_methode(),
            "rev" => self.rev_methode(),
            "sqrt" => self.sqrt_methode(),
            "round" => self.round_methode(),
            "celi" => self.celi_methode(),
            "floor" => self.floor_methode(),
            "abs" => self.abs_methode(),
            "push" => self.push_methode(scanner),
            "pop" => self.pop_methode(),
            "len" => self.len_methode(),
            _ => {
                self.error(&format!("The methode [{}] doesn't exsist.", &methode_name));
            }
        }
        self.consume(TokenType::RigtParen, "Expect ')' after value.", scanner);
    }

    pub fn trim_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Str) => {
                self.type_tag.push(Id(TypeId::Str));
            }
            _ => self.error(&format!(
                "cannot use {} for trim, only str values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Trim as u8);
    }

    pub fn isempty_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Str) => {
                self.type_tag.push(Id(TypeId::Bool));
            }
            _ => self.error(&format!(
                "cannot use {} for is_empty, only str values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::IsEmpty as u8);
    }

    pub fn rev_methode(&mut self) {
        if !self.info.is_mut.pop().unwrap_or(false) {
            self.error("Value must be mutated in order to use rev on it!");
        }

        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Str) => {
                self.type_tag.push(Id(TypeId::Str));
            }
            TypeTag::Array(x) => {
                self.type_tag.push(TypeTag::Array(x));
            }
            _ => self.error(&format!(
                "cannot use {} for rev, only str/arrays values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Reverse as u8);

        if let Some(x) = self.info.last_local_slot {
            self.emit_bytes(OpCode::SetLocal as u8, x);
        } else {
            self.error("rev() can only be used directly on a local variable.");
        }
    }

    pub fn sqrt_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Float) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            _ => self.error(&format!(
                "cannot use {} for round, only float values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::SquareRoot as u8);
    }

    pub fn round_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Float) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            _ => self.error(&format!(
                "cannot use {} for round, only float values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Round as u8);
    }

    pub fn celi_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Float) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            _ => self.error(&format!(
                "cannot use {} for ceil, only float values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Ceil as u8);
    }

    pub fn floor_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Float) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            _ => self.error(&format!(
                "cannot use {} for floor, only float values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Floor as u8);
    }

    pub fn abs_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap_or(Id(TypeId::Void));

        match type_tag {
            Id(TypeId::Int) => {
                self.type_tag.push(Id(TypeId::Int));
            }
            Id(TypeId::Float) => {
                self.type_tag.push(Id(TypeId::Float));
            }
            _ => self.error(&format!(
                "cannot use {} for abs, only float/int values that are allowed",
                type_tag
            )),
        }

        self.emit_byte(OpCode::Abs as u8);
    }

    pub fn push_methode(&mut self, scanner: &mut Scanner) {
        if !self.info.is_mut.pop().unwrap_or(false) {
            self.error("Value must be mutated in order to use push() on it!");
        }

        let array_type = self.type_tag.pop().unwrap_or(TypeTag::Array(Void));
        self.expression(scanner);
        let values_typetag = self.type_tag.pop().unwrap_or(TypeTag::Id(Void));

        match (array_type, values_typetag) {
            (TypeTag::Array(TypeId::Int), TypeTag::Id(TypeId::Int))
            | (TypeTag::Array(TypeId::Unt), TypeTag::Id(TypeId::Unt))
            | (TypeTag::Array(TypeId::Float), TypeTag::Id(TypeId::Float))
            | (TypeTag::Array(TypeId::Str), TypeTag::Id(TypeId::Str))
            | (TypeTag::Array(TypeId::Char), TypeTag::Id(TypeId::Char))
            | (TypeTag::Array(TypeId::Bool), TypeTag::Id(TypeId::Bool))
            | (TypeTag::Array(TypeId::Void), TypeTag::Id(TypeId::Void)) => {}
            _ => {
                self.error(&format!(
                    "Mismatched types array for type [{}] found [{}]",
                    array_type, values_typetag
                ));
            }
        }

        self.emit_byte(OpCode::Push as u8);

        if let Some(x) = self.info.last_local_slot {
            self.emit_bytes(OpCode::SetLocal as u8, x);
        } else {
            self.error("push() can only be used directly on an array.");
        }
    }

    pub fn pop_methode(&mut self) {
        if !self.info.is_mut.pop().unwrap_or(false) {
            self.error("Value must be mutated in order to use push() on it!");
        }

        let type_tag = self.type_tag.pop().unwrap_or(Array(Void)).as_typeid();
        self.type_tag.push(Id(type_tag));

        self.emit_byte(OpCode::PopArray as u8);

        if let Some(x) = self.info.last_local_slot {
            self.emit_bytes(OpCode::SetLocal as u8, x);
        } else {
            self.error("pop() can only be used directly on an array.");
        }
    }

    pub fn len_methode(&mut self) {
        let type_tag = self.type_tag.pop().unwrap();

        match type_tag {
            Array(_) => {}
            Id(TypeId::Str) => {}
            _ => {
                self.error(&format!("Can not use len() on [{}]", type_tag));
            }
        }

        self.emit_byte(OpCode::Len as u8);
        self.type_tag.push(Id(TypeId::Unt));
    }
}
