use super::*;

pub fn floor(arg_count: usize, args: &Value, generic: Option<&TypeTag>) -> Value {
    if generic.is_some() {
        panic!("floor() {}", UNEXGENARG)
    }

    if arg_count > 1 {
        panic!("floor can not take any arugments!")
    }

    Value::Float(args.as_float().floor())
}
