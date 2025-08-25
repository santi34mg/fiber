use crate::{parser::{Ast, Expr, Statement}, token::Operator};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn eval(&self, ast: Ast) -> Vec<i32> {
        let mut results = Vec::new();
        for stmt in ast.get_stmts() {
            if let Statement::Expr(expr) = stmt {
                results.push(self.eval_expr(expr).expect("could not evaluate expression"));
                
            }
        }
        return results;
    }

    fn eval_expr(&self, expr: Expr) -> Result<i32, String> {
        match expr {
            Expr::Number(n) => Ok(n),
            Expr::Binary { left, op, right } => {
                let l = self.eval_expr(*left).unwrap();
                let r = self.eval_expr(*right).unwrap();
                match op {
                    Operator::Plus => Ok(l + r),
                    Operator::Minus => Ok(l - r),
                    Operator::Multply => Ok(l * r),
                    Operator::Divide => Ok(l / r), // integer division
                    _ => Err("eval_expr: wrong operator".to_string()),
                }
            }
            _ => Err("eval_expr: wrong expression".to_string()),
        }
    }
}
