use crate::sdl::Value::Bool;

use super::*;

pub fn trim(arg_count: usize, args: &[Value], generic: Option<&TypeTag>) -> Value {
    if arg_count > 1 {
        panic!("trim() expects one arg found {}", arg_count);
    }

    if generic.is_some() {
        panic!("trim() {}", UNEXGENARG)
    }

    Str(args[0].as_str())
}

pub fn is_empty(arg_count: usize, args: &[Value], generic: Option<&TypeTag>) -> Value {
    if arg_count > 1 {
        panic!("is_empty() expects one arg found {}", arg_count);
    }

    if generic.is_some() {
        panic!("is_empty() {}", UNEXGENARG)
    }

    Bool(args[0].as_str().is_empty())
}
