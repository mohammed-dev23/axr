use crate::{chunk::OpCode::GetLocal, compiler::ast::Stmt::NoneStmt};

use super::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal(Value),
    Const(Value),
    Grouping(Box<Expr>),
    Variable {
        slot: u8,
    },
    Assign {
        slot: u8,
        right: Box<Expr>,
    },
    Global {
        name: Token,
    },
    Range {
        left: Box<Expr>,
        right: Box<Expr>,
        type_idx: u8,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    CompoundAssign {
        slot: u8,
        operator: TokenType,
        right: Box<Expr>,
        type_idx: u8,
    },
    Array {
        len: u8,
    },
    FunctionCall {
        caller: Box<Expr>,
        argument_count: usize,
        arguments: Vec<Expr>,
    },
    Turbofish {
        left: Box<Expr>,
        caller: String,
        generic: TypeTag,
        idx: u8,
    },
    Cast {
        left: Box<Expr>,
        right: u8,
    },
    Function {
        name: String,
        arity: usize,
        block: Box<Vec<Stmt>>,
        ftype: FunctionType,
    },
    NoneExpr,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        value: Box<Expr>,
    },
    Fn {
        value: Box<Expr>,
        name: Token,
    },
    Const {
        name: String,
        value: Value,
        type_tag: TypeTag,
    },
    Block(Vec<Stmt>),
    Expression(Box<Expr>),
    Println(Box<Expr>),
    Return(Option<Box<Expr>>),
    NoneStmt,
}

impl Parser {
    pub fn codegen(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                self.codegen(left);
                self.codegen(right);

                match operator.token_type {
                    TokenType::Plus => self.emit_byte(OpCode::Add as u8),
                    TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
                    TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
                    TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
                    TokenType::Modulo => self.emit_byte(OpCode::Modulo as u8),
                    TokenType::BangEqual => self.emit_byte(OpCode::NotEqualTo as u8),
                    TokenType::EqualEqual => self.emit_byte(OpCode::EqualTo as u8),
                    TokenType::Greater => self.emit_byte(OpCode::GreaterThan as u8),
                    TokenType::Lesser => self.emit_byte(OpCode::LessThan as u8),
                    TokenType::GreaterEqual => self.emit_byte(OpCode::GreaterThanEq as u8),
                    TokenType::LesserEqual => self.emit_byte(OpCode::LessThanEq as u8),
                    _ => {}
                }
            }
            Expr::Literal(x) => self.emit_constant(x.to_owned()),
            Expr::Const(x) => self.emit_constant(x.to_owned()),
            Expr::Grouping(x) => self.codegen(x),
            Expr::Assign { slot, right } => {
                self.codegen(right);

                self.emit_bytes(OpCode::SetLocal as u8, *slot)
            }
            Expr::Variable { slot } => self.emit_bytes(OpCode::GetLocal as u8, *slot),
            Expr::Global { name } => {
                let slot = self.identifier_constant(name);
                self.emit_bytes(OpCode::GetGlobal as u8, slot);
            }
            Expr::Range {
                left,
                right,
                type_idx,
            } => {
                self.codegen(left);
                self.codegen(right);

                self.emit_byte(OpCode::Range as u8);
                self.emit_byte(*type_idx);
            }
            Expr::Unary { operator, right } => {
                self.codegen(right);

                match operator.token_type {
                    TokenType::Minus => self.emit_byte(OpCode::Negate as u8),
                    TokenType::Bang => self.emit_byte(OpCode::Not as u8),
                    _ => return,
                }
            }
            Expr::CompoundAssign {
                slot,
                operator,
                right,
                type_idx,
            } => {
                self.emit_bytes(GetLocal as u8, *slot);

                self.codegen(right);

                let mut set = |v: &str| {
                    if v == "+=" {
                        self.emit_byte(OpCode::AddAdd as u8);
                    } else {
                        self.emit_byte(OpCode::MinusMinus as u8);
                    }

                    self.emit_byte(*type_idx);

                    self.emit_bytes(OpCode::SetLocal as u8, *slot);
                };

                match operator {
                    TokenType::AddAdd => set("+="),
                    TokenType::MinusMinus => set("-="),
                    _ => return,
                }
            }
            Expr::Array { len } => {
                if *len == 0 {
                    self.emit_byte(OpCode::NewArray as u8);
                } else {
                    self.emit_byte(OpCode::Array as u8);
                    self.emit_byte(*len);
                }
            }
            Expr::FunctionCall {
                caller,
                argument_count,
                arguments,
            } => {
                let (caller, idx) = match &**caller {
                    Expr::Turbofish {
                        left,
                        caller: _,
                        generic: _,
                        idx,
                    } => (left, idx),
                    other => (
                        &Box::from(other.to_owned()),
                        &self.add_type_tag_to_chunk(TypeTag::Nai),
                    ),
                };

                self.codegen(caller);

                for i in arguments {
                    self.codegen(i);
                }

                self.emit_bytes(OpCode::Call as u8, *argument_count as u8);

                self.emit_byte(*idx);
            }
            Expr::Turbofish {
                left: _,
                caller,
                generic,
                idx: _,
            } => {
                self.function_info
                    .return_type_tag_table
                    .insert(caller.to_owned(), generic.to_owned());
            }
            Expr::Cast { left, right } => {
                self.codegen(left);

                self.emit_byte(OpCode::Cast as u8);

                self.emit_byte(*right);
            }
            Expr::Function {
                name,
                arity,
                block,
                ftype,
            } => {
                let enclosing = std::mem::replace(&mut self.compiler, Compiler::new(*ftype));

                self.compiler_stack.push(enclosing);

                for i in block.as_ref() {
                    self.stmt_gen(i);
                }

                let function = Function {
                    name: name.to_string(),
                    arity: *arity,
                    chunk: self.compiler.function.function.chunk.clone(),
                };

                self.compiler = self.compiler_stack.pop().expect("compiler stack underflow");

                let function = self.make_constant(Value::Function(Arc::new(function)));
                self.emit_bytes(OpCode::Constant as u8, function);
            }
            Expr::NoneExpr => {}
        }
    }

    pub fn stmt_gen(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { value } => {
                self.codegen(&value);
            }
            Stmt::Fn { value, name } => {
                self.codegen(value);

                let global_slot = if self.compiler.scope_depth == 0 {
                    self.declare_variable();
                    Some(self.identifier_constant(&name))
                } else {
                    self.declare_variable();
                    self.mark_initialized();
                    None
                };

                if let Some(x) = global_slot {
                    self.emit_bytes(OpCode::DefineGlobal as u8, x);
                }
            }
            Stmt::Block(x) => {
                for i in x {
                    self.stmt_gen(i);
                }
            }
            Stmt::Expression(x) => {
                self.codegen(x);

                self.emit_byte(OpCode::Pop as u8);
            }
            Stmt::Const {
                name,
                value,
                type_tag,
            } => {
                self.const_table
                    .insert(name.to_owned(), (value.to_owned(), type_tag.to_owned()));
            }
            Stmt::Println(x) => {
                self.codegen(x);

                self.emit_byte(OpCode::Println as u8);
            }
            Stmt::Return(x) => {
                if let Some(y) = x {
                    self.codegen(y);
                } else {
                    self.emit_byte(OpCode::Void as u8);
                }

                self.emit_byte(OpCode::Return as u8);
            }
            NoneStmt => {}
        }
    }
}
