use super::*;
mod io;
mod math;

pub static NATIVEMETA: [NativeSig; 3] = [
    NativeSig {
        name: "print",
        parameters: &[TypeTag::Str],
        return_typetag: Some(TypeTag::Void),
    },
    NativeSig {
        name: "input",
        parameters: &[TypeTag::Str],
        return_typetag: None,
    },
    NativeSig {
        name: "floor",
        parameters: &[TypeTag::Float],
        return_typetag: Some(TypeTag::Float),
    },
];

pub struct NativeSig {
    pub name: &'static str,
    pub parameters: &'static [TypeTag],
    pub return_typetag: Option<TypeTag>,
}

impl Vm {
    pub fn def(&mut self) {
        self.def_io();
        self.def_math();
    }
}
