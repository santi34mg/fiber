use std::{collections::HashMap, fmt};

use crate::{
    parser::{
        Ast, Expression, Function, FunctionBody, FunctionParameter, FunctionSignature, Statement,
        VariableDeclaration,
    },
    token::{Literal, Operator, TypeIdentifier},
};

pub struct TypeChecker<'a> {
    ast: &'a Ast,
    variables: HashMap<String, TypeIdentifier>,
    functions: HashMap<String, FunctionSignature>,
}

#[derive(Debug)]
pub struct TypeCheckerError {
    pub message: String,
}
impl fmt::Display for TypeCheckerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Standard format: file:line:column
        writeln!(f, "{}", self.message)
    }
}

type TypeCheckerResult<T> = Result<T, TypeCheckerError>;

fn load_std_functions() -> HashMap<String, FunctionSignature> {
    let mut function_signatures = HashMap::new();

    // print_int(n int) -> None (accept any type)
    let print_param = FunctionParameter {
        parameter_name: "n".to_string(),
        parameter_type: TypeIdentifier::Number,
    };
    let print_signature = FunctionSignature {
        name: "print_int".to_string(),
        parameters: vec![print_param.clone()],
        return_type: None,
    };
    function_signatures.insert("print_int".to_string(), print_signature);

    // println(v) -> None (accept any type)
    let println_signature = FunctionSignature {
        name: "println".to_string(),
        parameters: vec![print_param],
        return_type: None,
    };
    function_signatures.insert("println".to_string(), println_signature);

    // read_int() -> Number
    let read_int_signature = FunctionSignature {
        name: "read_int".to_string(),
        parameters: vec![],
        return_type: Some(TypeIdentifier::Number),
    };
    function_signatures.insert("read_int".to_string(), read_int_signature);

    // read_char() -> Char
    let read_char_signature = FunctionSignature {
        name: "read_char".to_string(),
        parameters: vec![],
        return_type: Some(TypeIdentifier::Char),
    };
    function_signatures.insert("read_char".to_string(), read_char_signature);

    function_signatures
}

impl<'a> TypeChecker<'a> {
    pub fn new(ast: &'a Ast) -> Self {
        let std_functions = load_std_functions();
        Self {
            ast,
            variables: HashMap::new(),
            functions: std_functions,
        }
    }

    pub fn check_ast(&mut self) {
        for statement in &self.ast.statements {
            if let Some(err) = self.check_statement(statement).err() {
                println!("{}", err);
            }
        }
    }

    fn check_statement(&mut self, statement: &Statement) -> TypeCheckerResult<TypeIdentifier> {
        match statement {
            Statement::VariableDeclaration(var_decl) => self.check_var_decl(var_decl),
            Statement::Expression(expr) => self.check_expr(expr),
            Statement::Assignment { identifier, expr } => self.check_assignment(identifier, expr),
            Statement::FunctionDeclaration(func_decl) => self.check_func_decl(func_decl),
            Statement::Return(expr) => self.check_return(expr),
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => self.check_if(condition, then_branch, else_branch),
        }
    }

    fn check_if(
        &mut self,
        condition: &Expression,
        then_branch: &Vec<Statement>,
        else_branch: &Option<Vec<Statement>>,
    ) -> TypeCheckerResult<TypeIdentifier> {
        let condition_type = self.check_expr(condition)?;
        if condition_type != TypeIdentifier::Boolean {
            return Err(TypeCheckerError {
                message: format!("If condition has to be boolean"),
            });
        };
        for statement in then_branch {
            self.check_statement(statement)?;
        }
        if let Some(else_statements) = else_branch {
            for statement in else_statements {
                self.check_statement(statement)?;
            }
        }
        Ok(TypeIdentifier::Boolean)
    }

    fn check_var_decl(
        &mut self,
        var_decl: &VariableDeclaration,
    ) -> TypeCheckerResult<TypeIdentifier> {
        let ident = &var_decl.identifier;
        let var_type = var_decl.variable_type.clone();
        let expr_type = self.check_expr(&var_decl.expression)?;
        if var_type != expr_type {
            return Err(TypeCheckerError {
                message: format!(
                    "Type mismatch in variable declaration '{}': declared as {:?}, but got {:?}",
                    ident, var_type, expr_type
                ),
            });
        }
        self.variables.insert(ident.clone(), var_type.clone());
        Ok(var_type)
    }

    fn check_assignment(
        &mut self,
        identifier: &String,
        expr: &Expression,
    ) -> TypeCheckerResult<TypeIdentifier> {
        let expr_type = self.check_expr(expr)?;
        match self.variables.get(identifier) {
            Some(var_type) => {
                if *var_type != expr_type {
                    return Err(TypeCheckerError {
                        message: format!(
                            "Type mismatch in assignment to '{}': variable is {:?}, but got {:?}",
                            identifier, var_type, expr_type
                        ),
                    });
                }
                Ok(var_type.clone())
            }
            None => Err(TypeCheckerError {
                message: format!("Assignment to undeclared variable '{}'", identifier),
            }),
        }
    }

    fn check_func_decl(&mut self, function: &Function) -> TypeCheckerResult<TypeIdentifier> {
        let function_name = function.signature.name.clone();
        let return_type = function.signature.return_type.clone();
        self.functions
            .insert(function_name.clone(), function.signature.clone());

        // Create and swap in a new local scope for function body, then restore outer scope
        let outer_scope = std::mem::take(&mut self.variables);
        // populate parameters into the current (now empty clone) scope using parameter names
        for parameter in &function.signature.parameters {
            self.variables.insert(
                parameter.parameter_name.clone(),
                parameter.parameter_type.clone(),
            );
        }

        let mut found_return = false;
        if let FunctionBody::Statements(statements) = &function.body {
            for stmt in statements {
                if let Statement::Return(expr) = stmt {
                    found_return = true;
                    let ret_type = self.check_return(&expr)?;
                    if let Some(expected) = return_type.clone() {
                        if expected != ret_type {
                            // restore outer scope before returning
                            self.variables = outer_scope;
                            return Err(TypeCheckerError {
                                message: format!(
                                    "Function '{}' returns {:?}, but declared as {:?}",
                                    function_name.clone(),
                                    ret_type,
                                    expected
                                ),
                            });
                        }
                    }
                } else {
                    self.check_statement(&stmt)?;
                }
            }
        }

        // restore outer scope after checking the function body
        self.variables = outer_scope;
        // Optionally: check for missing return in non-void functions
        if function.signature.return_type.is_some() && !found_return {
            return Err(TypeCheckerError {
                message: format!("Function '{}' is missing a return statement", function_name),
            });
        }
        Ok(TypeIdentifier::UserDefinedType)
    }

    fn check_return(&mut self, expr: &Option<Expression>) -> TypeCheckerResult<TypeIdentifier> {
        match expr {
            Some(e) => self.check_expr(e),
            None => Ok(TypeIdentifier::UserDefinedType), // or a special Void type if you have one
        }
    }

    fn check_expr(&self, expr: &Expression) -> TypeCheckerResult<TypeIdentifier> {
        let expr_type = match expr {
            Expression::Binary {
                left,
                operator: op,
                right,
            } => {
                let left_type = self.check_expr(left)?;
                let right_type = self.check_expr(right)?;
                if left_type != right_type {
                    return Err(TypeCheckerError {
                        message: format!("left and right types are not the same"),
                    });
                }
                match op {
                    Operator::Plus | Operator::Minus | Operator::Multiply | Operator::Divide => {
                        if left_type != TypeIdentifier::Number {
                            return Err(TypeCheckerError {
                                message: "Arithmetic operators require number types".to_string(),
                            });
                        }
                        TypeIdentifier::Number
                    }
                    Operator::Equals
                    | Operator::Different
                    | Operator::GreaterThan
                    | Operator::LesserThan
                    | Operator::GreaterEqual
                    | Operator::LesserEqual => {
                        // require both sides to be numbers
                        if left_type != TypeIdentifier::Number
                            || right_type != TypeIdentifier::Number
                        {
                            return Err(TypeCheckerError {
                                message: "Comparison operators require number types".to_string(),
                            });
                        }
                        TypeIdentifier::Boolean
                    }
                    Operator::And | Operator::Or => {
                        // require both sides to be booleans
                        if left_type != TypeIdentifier::Boolean
                            || right_type != TypeIdentifier::Boolean
                        {
                            return Err(TypeCheckerError {
                                message: "Logical operators require boolean types".to_string(),
                            });
                        }
                        TypeIdentifier::Boolean
                    }
                    _ => {
                        return Err(TypeCheckerError {
                            message: "Unsupported operator in binary expression".to_string(),
                        });
                    }
                }
            }
            Expression::Ident(ident) => {
                self.variables.get(ident).cloned().ok_or(TypeCheckerError {
                    message: format!("Use of undeclared variable '{}'", ident),
                })?
            }
            Expression::Literal(lit) => match &lit {
                Literal::Integer(_) => TypeIdentifier::Number,
                Literal::Boolean(_) => TypeIdentifier::Boolean,
                Literal::Character(_) => TypeIdentifier::Char,
                _ => {
                    todo!()
                }
            },
            Expression::Grouping(expr) => self.check_expr(expr)?,
            Expression::Call { callee, args } => self.check_call(callee, args)?,
            Expression::Unary {
                operator: op,
                expression: expr,
            } => {
                let expr_type = self.check_expr(expr)?;
                match op {
                    Operator::Not => {
                        if expr_type != TypeIdentifier::Boolean {
                            return Err(TypeCheckerError {
                                message: "Logical operators require boolean types".to_string(),
                            });
                        }
                        TypeIdentifier::Boolean
                    }
                    _ => {
                        return Err(TypeCheckerError {
                            message: "Unsupported operator in unary expression".to_string(),
                        });
                    }
                }
            }
        };
        Ok(expr_type)
    }

    fn check_call(
        &self,
        callee: &Box<Expression>,
        args: &Vec<Expression>,
    ) -> TypeCheckerResult<TypeIdentifier> {
        // Only support identifier calls (e.g., foo(...))
        if let Expression::Ident(func_name) = &**callee {
            // Lookup function signature
            let function_signature = self.functions.get(func_name).ok_or(TypeCheckerError {
                message: format!("check_call: Call to undefined function '{}'", func_name),
            })?;

            // Check argument count
            if args.len() != function_signature.parameters.len() {
                return Err(TypeCheckerError {
                    message: format!(
                        "Function '{}' expects {} arguments, got {}",
                        func_name,
                        function_signature.parameters.len(),
                        args.len()
                    ),
                });
            }

            // Check argument types
            for (i, (arg, expected_ty)) in args
                .iter()
                .zip(function_signature.parameters.iter())
                .enumerate()
            {
                let arg_ty = self.check_expr(arg)?;
                if arg_ty != expected_ty.parameter_type {
                    return Err(TypeCheckerError {
                        message: format!(
                            "Type mismatch in argument {} of '{}': expected {:?}, got {:?}",
                            i + 1,
                            func_name,
                            expected_ty,
                            arg_ty
                        ),
                    });
                }
            }

            // Return the function's return type (or UserDefinedType if None)
            return Ok(function_signature
                .return_type
                .clone()
                .unwrap_or(TypeIdentifier::UserDefinedType));
        } else {
            return Err(TypeCheckerError {
                message: "Only identifier function calls are supported".to_string(),
            });
        }
    }
}
