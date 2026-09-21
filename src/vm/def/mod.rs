use super::*;
mod io;

pub static NATIVEMETA: [NativeSig; 1] = [NativeSig {
    name: "print",
    parameters: &[TypeTag::Str],
    return_typetag: TypeTag::Void,
}];

pub struct NativeSig {
    pub name: &'static str,
    pub parameters: &'static [TypeTag],
    pub return_typetag: TypeTag,
}

impl Vm {
    pub fn def(&mut self) {
        self.def_io();
    }
}
