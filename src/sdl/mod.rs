pub use crate::value::Value;
pub mod io;
pub mod math;

use crate::compiler::TypeTag;
use crate::value::Value::{Char, Int, Str, Unt, Void};
use std::io::{Write, stdin, stdout};
use std::sync::Arc;
