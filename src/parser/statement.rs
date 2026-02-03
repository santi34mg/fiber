use crate::parser::VarDecl;
use crate::parser::expression::Expr;
use crate::parser::function::Function;

#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl(VarDecl),
    Assignment {
        identifier: String,
        expr: Expr,
    },
    Expr(Expr),
    FunctionDeclaration(Function),
    Return(Option<Expr>),
    If {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
}