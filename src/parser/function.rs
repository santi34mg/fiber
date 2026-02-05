use crate::parser::statement::Statement;
use crate::token::TypeIdentifier;

#[derive(Debug, Clone)]
pub struct Function {
    pub signature: FunctionSignature,
    pub body: FunctionBody,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<TypeIdentifier>,
}

#[derive(Debug, Clone)]
pub struct FunctionParameter {
    pub parameter_name: String,
    pub parameter_type: TypeIdentifier,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FunctionBody {
    Statements(Vec<Statement>),
    Empty,
}
