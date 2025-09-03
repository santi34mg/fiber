use crate::parser::{Ast, Statement};

pub struct TypeChecker {
    ast: Ast,
}

impl TypeChecker {
    pub fn new(ast: Ast) -> Self {
        Self { ast }
    }

    pub fn check_ast(self) {
    }

    fn check_statement(&self, statement: Statement) {
    }

    fn get_atomic_type() {
    }
}
