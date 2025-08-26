use std::collections::HashMap;

use crate::{
    parser::{Ast, Expr, LetDecl, Statement},
    token::Operator,
};

pub struct Interpreter {
    env: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub fn eval(mut self, ast: Ast) -> Vec<i32> {
        let mut results = Vec::new();
        for stmt in ast.get_stmts() {
            match stmt {
                Statement::Expr(expr) => {
                    let evaluated_expr = self.eval_expr(expr).expect("eval: could not evaluate expression");
                    results.push(evaluated_expr)
                },
                Statement::LetDecl(decl) => {
                    self.eval_decl(decl).expect("eval: could not evaluate declaration")
                },
                Statement::Comment => {},
            }
        }
        return results;
    }

    fn eval_expr(&self, expr: Expr) -> Result<i32, String> {
        match expr {
            Expr::Number(n) => Ok(n),
            Expr::Boolean(b) => {
                if b {
                    Ok(0)
                } else {
                    Ok(1)
                }
            }
            Expr::Ident(id) => self
                .env
                .get(&id)
                .copied()
                .ok_or("eval_expr: no value".to_string()),
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

    fn eval_decl(&mut self, decl: LetDecl) -> Result<(), String> {
        let decl_val = self.eval_expr(decl.expr)?;
        self.env.insert(decl.identifier, decl_val);
        Ok(())
    }
}
