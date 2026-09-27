use super::*;
use crate::sdl::rand::random_from;

impl Vm {
    pub fn def_rand(&mut self) {
        self.define_native(Arc::from("random_from"), random_from);
    }
}
