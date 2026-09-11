use super::*;

impl Vm {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        let mut chunk = Chunk::new();
        let mut compiler = compiler::Parser::new();

        if !compiler.compile(source, &mut chunk) {
            return InterpretResult::CompileError;
        }

        self.chunk = chunk;
        self.ip = 0;

        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature = "DTE")]
            {
                use crate::debug::disassemble_instruction;
                disassemble_instruction(&self.chunk, self.ip as usize);

                println!();
                for i in &self.stack {
                    println!("[{}]", i)
                }
            }

            let instruction: u8 = self.read_byte();

            macro_rules! try_handler {
                ($handler:ident) => {
                    match self.$handler(instruction) {
                        InterpretResult::NotHandled => {}
                        InterpretResult::RuntimeError => break RuntimeError,
                        InterpretResult::Done => break InterpretResult::Done,
                        _ => continue,
                    }
                };
            }

            try_handler!(arithmetic_run);
            try_handler!(builtins_run);
            try_handler!(cast_run);
            try_handler!(colloc_run);
            try_handler!(control_flow_run);
            try_handler!(io_run);
            try_handler!(stack_run);
            try_handler!(run_wrappers);

            break self.runtime_err(&format!("unknown OpCode {}", instruction));
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip as usize];
        self.ip += 1;
        byte
    }

    pub fn read_short(&mut self) -> u16 {
        let high = self.read_byte() as u16;
        let low = self.read_byte() as u16;
        (high << 8) | low
    }

    pub fn peek(&mut self) -> Value {
        self.stack.last().unwrap_or(&Void).clone()
    }

    pub fn read_constant(&mut self) -> Value {
        let index = self.read_byte() as usize;
        self.chunk.constants.values[index].clone()
    }

    pub fn op<T, R, A>(op: char, v1: T, v2: R) -> Result<A>
    where
        T: std::ops::Add<R, Output = A>
            + std::ops::Sub<R, Output = A>
            + std::ops::Mul<R, Output = A>
            + std::ops::Div<R, Output = A>
            + std::ops::Rem<R, Output = A>,
        R: std::cmp::PartialEq + Default,
    {
        match op {
            '+' => Ok(v1 + v2),
            '-' => Ok(v1 - v2),
            '*' => Ok(v1 * v2),
            '/' => {
                if v2 == R::default() {
                    eprintln!("Can not divide by zero!");
                    return Err(InterpretResult::RuntimeError);
                } else {
                    Ok(v1 / v2)
                }
            }
            '%' => Ok(v1 % v2),
            _ => {
                eprintln!("undifined op [{}]", op);
                Err(InterpretResult::RuntimeError)
            }
        }
    }

    pub fn cmp_op<T, R>(op: &str, v1: T, v2: R) -> bool
    where
        T: std::cmp::PartialEq<R> + std::cmp::PartialOrd<R>,
    {
        match op {
            ">" => v1 > v2,
            "<" => v1 < v2,
            ">=" => v1 >= v2,
            "<=" => v1 <= v2,
            "!=" => v1 != v2,
            "==" => v1 == v2,
            _ => false,
        }
    }

    pub fn runtime_err(&mut self, message: &str) -> InterpretResult {
        eprintln!("{}", message);

        let instruction = self.ip - 1;
        let line = self.chunk.line[instruction as usize];
        eprintln!("[line {}] in code", line);

        InterpretResult::RuntimeError
    }
}
