use crate::sdl::io::{input, print};

use super::*;

impl Vm {
    pub fn def_io(&mut self) {
        self.define_native(Arc::from("print"), print);
        self.define_native(Arc::from("input"), input);
    }
}
