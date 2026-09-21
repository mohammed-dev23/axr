use crate::sdl::io::print;

use super::*;

impl Vm {
    pub fn def_io(&mut self) {
        self.define_native(Arc::from("print"), print);
    }
}
