use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i32),
    Boolean(bool),
    Char(char),
    None,
}

#[derive(Debug)]
pub struct StackFrame {
    pub vars: HashMap<String, Value>,
}
