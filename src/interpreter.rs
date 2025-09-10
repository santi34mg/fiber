use std::collections::HashMap;

use crate::{
    parser::{Ast, Expr, FuncDecl, Statement, VarDecl},
    token::Operator,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i32),
    Boolean(bool),
}

#[derive(Debug)]
struct StackFrame {
    vars: HashMap<String, Value>,
}

pub struct Interpreter<'a> {
    stack: Vec<StackFrame>,
    functions: HashMap<String, &'a FuncDecl>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            stack: vec![StackFrame {
                vars: HashMap::new(),
            }],
            functions: HashMap::new(),
        }
    }

    fn current_frame_mut(&mut self) -> &mut StackFrame {
        self.stack.last_mut().expect("No stack frame")
    }

    pub fn eval(mut self, ast: &'a Ast) -> Vec<Value> {
        // Collect function declarations first
        let statements = ast.get_stmts();
        for stmt in statements {
            if let Statement::FuncDecl(func) = stmt {
                self.functions.insert(func.name.clone(), func);
            }
        }
        let mut results = Vec::new();
        for stmt in statements {
            match self.eval_statement(stmt) {
                Ok(Some(v)) => results.push(v),
                Ok(None) => {}
                Err(e) => panic!("eval: {}", e),
            }
        }
        results
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Ident(id) => self.lookup_var(id),
            Expr::Binary { left, op, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                match (l, r) {
                    (Value::Number(l), Value::Number(r)) => match op {
                        Operator::Plus => Ok(Value::Number(l + r)),
                        Operator::Minus => Ok(Value::Number(l - r)),
                        Operator::Multply => Ok(Value::Number(l * r)),
                        Operator::Divide => Ok(Value::Number(l / r)),
                        Operator::Equals => Ok(Value::Boolean(l == r)),
                        Operator::Different => Ok(Value::Boolean(l != r)),
                        Operator::GreaterThan => Ok(Value::Boolean(l > r)),
                        Operator::LesserThan => Ok(Value::Boolean(l < r)),
                        Operator::GreaterEqual => Ok(Value::Boolean(l >= r)),
                        Operator::LesserEqual => Ok(Value::Boolean(l <= r)),
                        _ => Err("eval_expr: unsupported operator for numbers".to_string()),
                    },
                    (Value::Boolean(l), Value::Boolean(r)) => match op {
                        Operator::Equals => Ok(Value::Boolean(l == r)),
                        Operator::Different => Ok(Value::Boolean(l != r)),
                        Operator::And => Ok(Value::Boolean(l && r)),
                        Operator::Or => Ok(Value::Boolean(l || r)),
                        _ => Err("eval_expr: unsupported operator for booleans".to_string()),
                    },
                    _ => Err("eval_expr: type mismatch in binary operation".to_string()),
                }
            }
            Expr::Grouping(expr) => self.eval_expr(expr),
            Expr::Call { callee, args } => {
                // Only support identifier calls (e.g., foo(...))
                let func_name = if let Expr::Ident(name) = &**callee {
                    name
                } else {
                    return Err(
                        "eval_expr: only identifier function calls are supported".to_string()
                    );
                };

                let func = self
                    .functions
                    .get(func_name)
                    .cloned()
                    .ok_or_else(|| format!("eval_expr: function '{}' not found", func_name))?;

                if args.len() != func.params.len() {
                    return Err(format!(
                        "eval_expr: function '{}' expects {} arguments, got {}",
                        func_name,
                        func.params.len(),
                        args.len()
                    ));
                }

                // Evaluate arguments
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.eval_expr(arg)?);
                }

                // Push new stack frame for function call
                let mut frame = StackFrame {
                    vars: HashMap::new(),
                };
                for ((param_name, _), value) in func.params.iter().zip(arg_values.into_iter()) {
                    frame.vars.insert(param_name.clone(), value);
                }
                self.stack.push(frame);

                // Evaluate function body
                let mut return_value = None;
                for stmt in &func.body {
                    match self.eval_statement(stmt)? {
                        Some(val) => {
                            return_value = Some(val);
                            break;
                        }
                        None => {}
                    }
                }

                self.stack.pop();

                Ok(return_value.unwrap_or(Value::Number(0)))
            }
        }
    }

    fn eval_statement(&mut self, stmt: &Statement) -> Result<Option<Value>, String> {
        match stmt {
            Statement::VarDecl(decl) => {
                self.eval_decl(decl)?;
                Ok(None)
            }
            Statement::Assignment { identifier, expr } => {
                let value = self.eval_expr(expr)?;
                self.assign_var(identifier, value);
                Ok(None)
            }
            Statement::Expr(expr) => {
                let value = self.eval_expr(expr)?;
                Ok(Some(value))
            }
            Statement::Comment => Ok(None),
            Statement::FuncDecl(_) => Ok(None),
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    let value = self.eval_expr(e)?;
                    return Ok(Some(value));
                } else {
                    return Ok(None);
                }
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_val = self.eval_expr(condition)?;
                match cond_val {
                    Value::Boolean(true) => {
                        for stmt in then_branch {
                            match self.eval_statement(stmt)? {
                                Some(val) => return Ok(Some(val)),
                                None => {}
                            }
                        }
                    }
                    Value::Boolean(false) => {
                        if let Some(else_branch) = else_branch {
                            for stmt in else_branch {
                                match self.eval_statement(stmt)? {
                                    Some(val) => return Ok(Some(val)),
                                    None => {}
                                }
                            }
                        }
                    }
                    _ => return Err("if condition does not evaluate to a boolean".to_string()),
                }
                Ok(None)
            }
        }
    }

    fn eval_decl(&mut self, decl: &VarDecl) -> Result<(), String> {
        let decl_val = self.eval_expr(&decl.expr)?;
        self.current_frame_mut()
            .vars
            .insert(decl.identifier.clone(), decl_val);
        Ok(())
    }

    fn lookup_var(&self, id: &str) -> Result<Value, String> {
        for frame in self.stack.iter().rev() {
            if let Some(val) = frame.vars.get(id) {
                return Ok(val.clone());
            }
        }
        Err(format!("eval_expr: variable '{}' not found", id))
    }

    fn assign_var(&mut self, id: &str, value: Value) {
        for frame in self.stack.iter_mut().rev() {
            if frame.vars.contains_key(id) {
                frame.vars.insert(id.to_string(), value);
                return;
            }
        }
        // If not found, assign in the current frame (like implicit declaration)
        self.current_frame_mut().vars.insert(id.to_string(), value);
    }
}
