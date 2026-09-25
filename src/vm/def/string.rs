use crate::sdl::string::{is_empty, trim};

use super::*;

impl Vm {
    pub fn def_string(&mut self) {
        self.define_native(Arc::from("trim"), trim);
        self.define_native(Arc::from("is_empty"), is_empty);
    }
}
