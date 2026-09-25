use super::*;

impl Vm {
    pub fn new() -> Self {
        let mut new = Self {
            stack: Vec::new(),
            frames: Frame {
                frames: Vec::new(),
                frame_count: 0,
            },
            global_table: HashMap::new(),
        };

        new.def();

        new
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        let mut chunk = Chunk::new();
        let mut compiler = compiler::Parser::new();

        let function = match compiler.compile(source, &mut chunk) {
            Some(x) => Arc::new(x),
            None => return InterpretResult::CompileError,
        };

        self.stack.push(Value::Function(function.clone()));

        self.frames.frames.push(CallFrame {
            function: function,
            ip: 0,
            slots: 0,
        });
        self.frames.frame_count += 1;

        let res = self.run();

        if res != InterpretResult::Ok {
            return res;
        }

        match self.global_table.get(&Arc::from("main")) {
            Some(x) => {
                self.stack.push(x.clone());
                self.call_value(x.clone(), 0);

                let res = self.run();

                if res != InterpretResult::Ok {
                    return res;
                }
            }
            None => return self.runtime_err("Expected main entry point"),
        };

        InterpretResult::Ok
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature = "DTE")]
            {
                use crate::debug::disassemble_instruction;
                disassemble_instruction(
                    &self.frames.frames[self.frames.frame_count - 1]
                        .function
                        .chunk,
                    self.frames.frames[self.frames.frame_count - 1].ip,
                );

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
                        InterpretResult::Done => break InterpretResult::Ok,
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
        let frame = &mut self.frames.frames[self.frames.frame_count - 1];
        let byte = frame.function.chunk.code[frame.ip];
        frame.ip += 1;
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

    pub fn peek_spec(&mut self, des: usize) -> Value {
        self.stack[self.stack.len() - des - 1].clone()
    }

    pub fn read_constant(&mut self) -> Value {
        let index = self.read_byte() as usize;
        let frame = &mut self.frames.frames[self.frames.frame_count - 1];
        frame.function.chunk.constants.values[index].clone()
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
        for i in (0..self.frames.frame_count).rev() {
            let frame = &mut self.frames.frames[i];
            let function = &frame.function;
            let instruction = frame.ip;

            if function.name == "" {
                eprintln!(
                    "[script] > [line {}] > [{}]",
                    function.chunk.line[instruction], message
                );
            } else {
                eprintln!(
                    "[{}] > [line {}] > [{}]",
                    function.name, function.chunk.line[instruction], message
                );
            }
        }

        InterpretResult::RuntimeError
    }

    pub fn get_type(&mut self) -> &TypeTag {
        // we read the index that has been emited by the compiler
        let idx = self.read_byte();

        // we index in the type tag stack and return the value
        &self.frames.frames[self.frames.frame_count - 1]
            .function
            .chunk
            .type_tag[idx as usize]
    }
}
