use std::{
    ffi::OsStr,
    fs::read_to_string,
    io::{Error, ErrorKind},
    path::Path,
};

use crate::vm::Vm;

mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

#[cfg(test)]
mod tests;

fn main() -> std::io::Result<()> {
    Vm::new().interpret(cli()?);
    Ok(())
}

fn cli() -> std::io::Result<String> {
    let path = std::env::args().nth(1);
    let mut code = String::new();

    if let Some(x) = path {
        let x = Path::new(&x);

        if x.extension().unwrap_or(OsStr::new(&"Unknown extension")) != "ax" {
            return Err(Error::new(
                ErrorKind::InvalidFilename,
                "Unknown extension; make sure it's ax before trying again",
            ));
        }

        code = read_to_string(x)?;
    }

    Ok(code)
}
