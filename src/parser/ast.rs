use crate::parser::statement::Statement;

#[derive(Debug, Clone)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Ast {
    pub fn new() -> Self {
        return Self {
            statements: Vec::new(),
        };
    }
}