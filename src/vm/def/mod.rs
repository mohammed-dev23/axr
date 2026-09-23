use super::*;
mod io;

pub static NATIVEMETA: [NativeSig; 2] = [
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
];

pub struct NativeSig {
    pub name: &'static str,
    pub parameters: &'static [TypeTag],
    pub return_typetag: Option<TypeTag>,
}

impl Vm {
    pub fn def(&mut self) {
        self.def_io();
    }
}
