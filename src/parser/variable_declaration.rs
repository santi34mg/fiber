use crate::parser::expression::Expression;
use crate::token::TypeIdentifier;

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub identifier: String,
    pub variable_type: TypeIdentifier,
    pub expression: Expression,
}

impl VariableDeclaration {
    pub fn new(identifier: String, variable_type: TypeIdentifier, expression: Expression) -> Self {
        Self {
            identifier,
            variable_type: variable_type,
            expression,
        }
    }
}
