use super::*;

pub fn print(_arg_count: usize, args: &[Value], generic: Option<&TypeTag>) -> Value {
    if generic.is_some_and(|t| t != &TypeTag::Void) {
        panic!("print does not expect a turbofish or a generic specifier!")
    }

    print!("{}", args[0]);
    Value::Void
}

pub fn input(arg_count: usize, args: &[Value], generic: Option<&TypeTag>) -> Value {
    if arg_count > 1 {
        panic!("input() expect one argument!.");
    }

    let txt = args[0].as_str();

    print!("{}", txt);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if let Some(x) = generic {
        let value = match x {
            TypeTag::Int => Int(input.parse::<i64>().unwrap()),
            TypeTag::Str => Str(Arc::from(input)),
            TypeTag::Float => Value::Float(input.parse::<f64>().unwrap()),
            TypeTag::Char => {
                let into_char: Vec<char> = input.chars().collect();
                let c: Value;

                if into_char.len() != 1 {
                    panic!(
                        "Char type cannot contain more than one char as an input. Expected char found {}",
                        input
                    );
                } else {
                    c = Char(into_char[0]);
                };

                c
            }
            TypeTag::Unt => Unt(input.parse::<u64>().unwrap()),
            _ => Void,
        };

        return value;
    } else {
        panic!("input expected a ::[T]");
    }
}
