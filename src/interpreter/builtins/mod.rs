pub mod print_int;
pub mod println;
pub mod read_int;
pub mod read_char;

use std::collections::HashMap;
use crate::parser::Function;

pub fn load_std_functions() -> HashMap<String, Function> {
    let mut m = HashMap::new();
    m.extend(print_int::register());
    m.extend(println::register());
    m.extend(read_int::register());
    m.extend(read_char::register());
    m
}
