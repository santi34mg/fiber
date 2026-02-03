use crate::interpreter::Value;
use crate::token::{TypeIdentifier};
use crate::parser::statement::Statement;
use std::sync::Arc;

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

#[derive(Clone)]
pub enum FunctionBody {
    UserDefinedBody(Vec<Statement>),
    NativeBody(Arc<dyn Fn(&[crate::interpreter::Value]) -> Option<Value> + Send + Sync>),
}
impl std::fmt::Debug for FunctionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionBody::UserDefinedBody(stmts) => {
                f.debug_tuple("UserDefinedBody").field(stmts).finish()
            }
            FunctionBody::NativeBody(_) => {
                f.debug_tuple("NativeBody").field(&"<native fn>").finish()
            }
        }
    }
}