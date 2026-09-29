use super::*;
use std::sync::Mutex;

mod array;
mod r#const;
mod decl_support;
mod function;
mod operators;
mod opt;
mod primary;

impl Parser {
    pub fn expression(&mut self, scanner: &mut Scanner) {
        let tree = self.parse_precedence(Precedence::Assignment, scanner);
        self.codegen(&tree);
    }
}
