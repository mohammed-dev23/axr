use crate::sdl::math::{ceil, floor, pow, round, sqrt};

use super::*;

impl Vm {
    pub fn def_math(&mut self) {
        self.define_native(Arc::from("floor"), floor);
        self.define_native(Arc::from("sqrt"), sqrt);
        self.define_native(Arc::from("round"), round);
        self.define_native(Arc::from("ceil"), ceil);
        self.define_native(Arc::from("pow"), pow);
    }
}
