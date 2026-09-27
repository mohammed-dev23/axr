use super::*;
mod io;
mod math;
mod rand;
mod string;

pub static NATIVEMETA: [NativeSig; 10] = [
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
    NativeSig {
        name: "trim",
        parameters: &[TypeTag::Str],
        return_typetag: Some(TypeTag::Str),
    },
    NativeSig {
        name: "is_empty",
        parameters: &[TypeTag::Str],
        return_typetag: Some(TypeTag::Bool),
    },
    NativeSig {
        name: "sqrt",
        parameters: &[TypeTag::Float],
        return_typetag: Some(TypeTag::Float),
    },
    NativeSig {
        name: "round",
        parameters: &[TypeTag::Float],
        return_typetag: Some(TypeTag::Float),
    },
    NativeSig {
        name: "celi",
        parameters: &[TypeTag::Float],
        return_typetag: Some(TypeTag::Float),
    },
    NativeSig {
        name: "pow",
        parameters: &[TypeTag::Float, TypeTag::Float],
        return_typetag: Some(TypeTag::Float),
    },
    NativeSig {
        name: "random_from",
        parameters: &[TypeTag::Generic],
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
        self.def_math();
        self.def_string();
        self.def_rand();
    }
}
