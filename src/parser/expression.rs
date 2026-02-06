use crate::token::Operator;
use crate::token::literal::Literal;

#[derive(Debug, Clone)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Unary {
        operator: Operator,
        expression: Box<Expression>,
    },
    Literal(Literal),
    // The identifier expression contains the name of the identifier as a string
    Identifier(String),
    Grouping(Box<Expression>),
    Call {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },
}
