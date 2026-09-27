use ::rand::random_range;

use super::*;

pub fn random_from(arg_count: usize, args: &[Value], generic: Option<&TypeTag>) -> Value {
    if arg_count > 1 {
        panic!("Expected 1 arg found {}", &arg_count)
    }

    if !args[0].is_range() {
        panic!("Expected Range[T] found {}", args[0])
    }

    if args[0].as_typetag().unwrap().extract().as_ref() != generic.unwrap() {
        panic!(
            "Expected '{}' due to 'Range[{}]'",
            generic.unwrap(),
            generic.unwrap()
        )
    }

    let range = args[0]
        .clone()
        .as_range()
        .expect("Expected range found none!");

    if let Some(t) = generic {
        match t {
            t if t == &TypeTag::Int => {
                let irange = range.as_int_range().unwrap();
                Value::Int(random_range(irange))
            }
            t if t == &TypeTag::Unt => {
                let urange = range.as_unt_range().unwrap();
                Value::Unt(random_range(urange))
            }
            t if t == &TypeTag::Float => {
                let frange = range.as_float_range().unwrap();
                Value::Float(random_range(frange))
            }
            _ => panic!("Unexpected return type {}", t),
        }
    } else {
        panic!("Expected ::[T]");
    }
}
