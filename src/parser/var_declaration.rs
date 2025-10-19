use crate::parser::expression::Expr;
use crate::token::TypeIdentifier;

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub identifier: String,
    pub var_type: TypeIdentifier,
    pub expr: Expr,
}

impl VarDecl {
    pub fn new(identifier: String, var_type: TypeIdentifier, expr: Expr) -> Self {
        Self {
            identifier,
            var_type,
            expr,
        }
    }
}