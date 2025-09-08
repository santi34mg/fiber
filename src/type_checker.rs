use crate::{
    parser::{Ast, Expr, Statement, VarDecl},
    token::TypeIdentifier,
};

pub struct TypeChecker<'a> {
    ast: &'a Ast,
}

pub struct TypedAst {}

#[derive(Debug)]
pub struct TypeCheckerError {
    pub message: String,
}
type TypeCheckerResult<T> = Result<T, TypeCheckerError>;

impl<'a> TypeChecker<'a> {
    pub fn new(ast: &'a Ast) -> Self {
        Self { ast }
    }

    pub fn check_ast(&'a self) {
        for statement in self.ast.get_stmts() {
            if let Some(err) = self.check_statement(statement).err() {
                println!("check_ast: found an error: {:?}", err);
            }
        }
    }

    fn check_statement(&'a self, statement: &Statement) -> TypeCheckerResult<TypeIdentifier>{
        match statement {
            Statement::VarDecl(..) => todo!(),
            Statement::Expr(expr) => {
                self.check_expr(expr)
            }
            Statement::Comment => todo!()
        }
    }

    fn check_expr(&'a self, expr: &Expr) -> TypeCheckerResult<TypeIdentifier> {
        let expr_type = match expr {
            Expr::Binary { left, op, right } => {
                // TODO: check operation
                let left_type = self.check_expr(left)?;
                let right_type = self.check_expr(right)?;
                if left_type != right_type {
                    return Err(TypeCheckerError {
                        message: format!("left and right types are not the same"),
                    });
                }
               left_type 
            }
            Expr::Ident(ident) => todo!(),
            Expr::Number(_) => TypeIdentifier::Number,
            Expr::Boolean(_) => TypeIdentifier::Boolean,
            Expr::Grouping(expr) => self.check_expr(expr)?,
        };
        Ok(expr_type)
    }
}
