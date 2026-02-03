use std::{collections::HashMap, sync::Arc};

use crate::{
    parser::{
        Ast, Expr, Function, FunctionBody, FunctionParameter, FunctionSignature, Statement, VarDecl,
    },
    token::{Operator, TypeIdentifier},
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

pub struct Interpreter {
    stack: Vec<StackFrame>,
    functions: HashMap<String, Function>,
}

pub struct FunctionBuilder {
    name: String,
    parameters: Vec<FunctionParameter>,
    return_type: Option<TypeIdentifier>,
    body: Option<FunctionBody>,
}

impl FunctionBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            parameters: Vec::new(),
            return_type: None,
            body: None,
        }
    }

    pub fn param(mut self, name: &str, ty: TypeIdentifier) -> Self {
        self.parameters.push(FunctionParameter {
            parameter_name: name.to_string(),
            parameter_type: ty,
        });
        self
    }

    pub fn returns(mut self, ty: TypeIdentifier) -> Self {
        self.return_type = Some(ty);
        self
    }

    pub fn returns_none(mut self) -> Self {
        self.return_type = None;
        self
    }

    pub fn native<F>(mut self, func: F) -> Self
    where
        F: Fn(&[Value]) -> Value + Send + Sync + 'static,
    {
        self.body = Some(FunctionBody::NativeBody(Arc::new(func)));
        self
    }

    pub fn user_defined(mut self, stmts: Vec<Statement>) -> Self {
        self.body = Some(FunctionBody::UserDefinedBody(stmts));
        self
    }

    pub fn build(self) -> Function {
        let signature = FunctionSignature {
            name: self.name,
            parameters: self.parameters,
            return_type: self.return_type,
        };
        let body = self
            .body
            .expect("Function must have a body before calling build()");
        Function { signature, body }
    }
}

fn load_std_functions() -> HashMap<String, Function> {
    let mut std_functions = HashMap::new();
    let alloc = FunctionBuilder::new("alloc")
        .param("size", TypeIdentifier::Number)
        .returns(TypeIdentifier::Number)
        .native(|args| {
            if let Some(Value::Number(size)) = args.get(0) {
                println!("Allocating {} bytes", size);
                Value::Number(*size)
            } else {
                panic!("alloc: expected a number argument");
            }
        })
        .build();
    std_functions.insert("alloc".to_string(), alloc);

    std_functions
}

impl Interpreter {
    pub fn new() -> Self {
        let std_functions = load_std_functions();
        Self {
            stack: vec![StackFrame {
                vars: HashMap::new(),
            }],
            functions: std_functions,
        }
    }

    fn current_frame_mut(&mut self) -> &mut StackFrame {
        self.stack.last_mut().expect("No stack frame")
    }

    pub fn eval_ast(&mut self, ast: Ast) -> Result<(), String> {
        // Collect function declarations first
        let statements = ast.statements;
        for stmt in &statements {
            if let Statement::FunctionDeclaration(function) = stmt {
                self.interp
                    .functions
                    .insert(function.signature.name.clone(), function.clone());
            }
        }
        for stmt in statements {
            self.eval_statement(&stmt)?;
        }
        Ok(())
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Char(c) => Ok(Value::Char(*c)),
            Expr::Ident(id) => self.interp.lookup_var(id),
            Expr::Binary { left, op, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                match (l, r) {
                    (Value::Number(l), Value::Number(r)) => match op {
                        Operator::Plus => Ok(Value::Number(l + r)),
                        Operator::Minus => Ok(Value::Number(l - r)),
                        Operator::Multply => Ok(Value::Number(l * r)),
                        Operator::Divide => {
                            if r == 0 {
                                return Err("eval_expr: division by zero".to_string());
                            }
                            Ok(Value::Number(l / r))
                        }
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

                let function = self
                    .interp
                    .functions
                    .get(func_name)
                    .cloned()
                    .ok_or_else(|| format!("eval_expr: function '{}' not found", func_name))?;
                if args.len() != function.signature.parameters.len() {
                    return Err(format!(
                        "eval_expr: function '{}' expects {} arguments, got {}",
                        func_name,
                        function.signature.parameters.len(),
                        args.len()
                    ));
                }

                // Evaluate arguments
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.eval_expr(arg)?);
                }

                // Push new stack frame for function call
                let mut frame = super::value::StackFrame {
                    vars: HashMap::new(),
                };
                for (function_parameter, value) in function
                    .signature
                    .parameters
                    .iter()
                    .zip(arg_values.clone().into_iter())
                {
                    frame
                        .vars
                        .insert(function_parameter.parameter_name.clone(), value);
                }
                self.interp.stack.push(frame);

                // Evaluate function body
                let mut return_value: Option<Value> = None;
                match &function.body {
                    FunctionBody::UserDefinedBody(statements) => {
                        for stmt in statements {
                            self.eval_statement(stmt)?;
                        }
                    }
                    // NativeBody expected to return Option<Value>; use it directly
                    FunctionBody::NativeBody(f) => return_value = f(arg_values.as_slice()),
                }

                self.interp.stack.pop();

                Ok(return_value.unwrap_or(Value::None))
            }
            Expr::Unary { op, expr } => {
                if let Operator::Not = op {
                    if let Value::Boolean(b) = self.eval_expr(expr)? {
                        return Ok(Value::Boolean(!b));
                    } else {
                        return Err("eval_expr: non boolean type for ! operator".to_string());
                    }
                } else {
                    return Err("eval_expr: unsupported unary operator".to_string());
                }
            }
        }
    }

    pub fn eval_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::VarDecl(decl) => {
                self.interp.eval_decl(decl)?;
            }
            Statement::Assignment { identifier, expr } => {
                let value = self.eval_expr(expr)?;
                self.interp.assign_var(identifier, value);
            }
            Statement::Expr(expr) => {
                let _ = self.eval_expr(expr)?;
            }
            Statement::FunctionDeclaration(_) => {}
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    let _ = self.eval_expr(e)?;
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
                            self.eval_statement(stmt)?;
                        }
                    }
                    Value::Boolean(false) => {
                        if let Some(else_branch) = else_branch {
                            for stmt in else_branch {
                                self.eval_statement(stmt)?;
                            }
                        }
                    }
                    _ => return Ok(()),
                }
            }
        }
        Ok(())
    }
}
