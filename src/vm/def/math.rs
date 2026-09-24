use crate::sdl::math::floor;

use super::*;

impl Vm {
    pub fn def_math(&mut self) {
        self.define_native(Arc::from("floor"), floor);
    }
}
