use super::*;

pub fn print(_arg_count: usize, args: &Value) -> Value {
    print!("{}", args);
    Value::Void
}
