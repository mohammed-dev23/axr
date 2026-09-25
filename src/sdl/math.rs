use super::*;

pub fn floor(arg_count: usize, args: &[Value], generic: Option<&TypeTag>) -> Value {
    if generic.is_some() {
        panic!("floor() {}", UNEXGENARG)
    }

    if arg_count > 1 {
        panic!("floor() expects 1 arg found {}", arg_count)
    }

    Value::Float(args[0].as_float().floor())
}

pub fn sqrt(arg_count: usize, args: &[Value], generic: Option<&TypeTag>) -> Value {
    if generic.is_some() {
        panic!("floor() {}", UNEXGENARG)
    }

    if arg_count > 1 {
        panic!("sqrt() expects 1 arg found {}", arg_count)
    }

    Value::Float(args[0].as_float().sqrt())
}

pub fn round(arg_count: usize, args: &[Value], generic: Option<&TypeTag>) -> Value {
    if generic.is_some() {
        panic!("round() {}", UNEXGENARG)
    }

    if arg_count > 1 {
        panic!("round() expects 1 arg found {}", arg_count)
    }

    Value::Float(args[0].as_float().round())
}

pub fn ceil(arg_count: usize, args: &[Value], generic: Option<&TypeTag>) -> Value {
    if generic.is_some() {
        panic!("celi() {}", UNEXGENARG)
    }

    if arg_count > 1 {
        panic!("celi() expects 1 arg found {}", arg_count)
    }

    Value::Float(args[0].as_float().ceil())
}

pub fn pow(arg_count: usize, args: &[Value], generic: Option<&TypeTag>) -> Value {
    if generic.is_some() {
        panic!("pow() {}", UNEXGENARG)
    }

    if arg_count > 2 {
        panic!("pow expects 2 arg found {}", arg_count)
    }

    let base = args[0].as_float();
    let exp = args[1].as_float();

    Value::Float(base.powf(exp))
}
