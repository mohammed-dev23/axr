use super::*;

pub struct Local {
    pub(in crate::compiler) name: Token,
    pub(in crate::compiler) depth: i32,
    pub(in crate::compiler) is_mut: bool,
    pub(in crate::compiler) type_tag: TypeTag,
}

impl Parser {
    pub fn begin_scope(&mut self) {
        self.compiler.scope_depth += 1;
    }

    pub fn end_scope(&mut self) {
        self.compiler.scope_depth -= 1;

        while self.compiler.locals.len() > 0
            && self.compiler.locals[self.compiler.local_count as usize - 1].depth
                > self.compiler.scope_depth
        {
            self.emit_byte(OpCode::Pop as u8);
            self.compiler.locals.pop();
            self.compiler.local_count -= 1;
        }
    }

    pub fn add_local(&mut self, name: Token) {
        self.compiler.locals.push(Local {
            name: name.clone(),
            depth: -1,
            is_mut: false,
            type_tag: TypeTag::Void,
        });
        self.compiler.local_count += 1;
    }

    pub fn resolve_local(&mut self, name: &Token) -> Option<(u8, bool, TypeTag)> {
        for i in 0..self.compiler.local_count {
            let local = &self.compiler.locals[i as usize];
            let is_mut = local.is_mut;
            let type_tag = local.type_tag;

            if Self::identifiers_equal(name, &local.name) {
                if local.depth == -1 {
                    self.error("Can't read local variable in its own initializer.");
                }
                return Some((i as u8, is_mut, type_tag));
            }
        }
        None
    }

    pub fn identifiers_equal(token_a: &Token, token_b: &Token) -> bool {
        if token_a.length != token_b.length {
            return false;
        }

        token_a.start == token_b.start
    }

    pub fn mark_initialized(&mut self) {
        self.compiler.locals[self.compiler.local_count as usize - 1].depth =
            self.compiler.scope_depth;
    }
}
