use crate::scanner::{Scanner, TokenType};

pub fn compile(source: String) {
    let mut scanner = Scanner::new(source);
    let mut line: isize = -1;

    loop {
        let token = scanner.scan_tokens();

        if token.line as isize != line {
            print!("{}", token.line);
            line = token.line as isize;
        } else {
            print!("   | ");
        }

        println!("{:?} {} {}", token.token_type, token.length, token.start);

        if token.token_type == TokenType::Eof {
            break;
        }
    }
}
