use std::collections::HashMap;

use super::value::{Value, StackFrame};
use super::eval::Evaluator;
use crate::parser::{Function, FunctionBody};

pub struct Interpreter {
    pub stack: Vec<StackFrame>,
    pub functions: HashMap<String, Function>,
}

impl Interpreter {
    pub fn register_function(&mut self, function: Function) {
        self.functions.insert(function.signature.name.clone(), function);
    }

    pub fn new() -> Self {
        // load_std_functions is provided by the builtins module at the interpreter root
        let std_functions = super::super::builtins::load_std_functions();
        Self {
            stack: vec![StackFrame { vars: HashMap::new() }],
            functions: std_functions,
        }
    }

    /// Evaluate a whole AST (consumes the interpreter like the previous API did)
    pub fn eval(mut self, ast: crate::parser::Ast) -> Result<(), String> {
        let mut evaluator = Evaluator::new(&mut self);
        evaluator.eval_ast(ast)
    }

    /// Call a registered function by name with arguments and return its Value.
    pub fn call_function(&mut self, name: &str, args: &[Value]) -> Result<Value, String> {
        let function = self
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| format!("call_function: function '{}' not found", name))?;

        if args.len() != function.signature.parameters.len() {
            return Err(format!(
                "call_function: function '{}' expects {} arguments, got {}",
                name,
                function.signature.parameters.len(),
                args.len()
            ));
        }

        // Push new stack frame for function call
        let mut frame = StackFrame { vars: HashMap::new() };
        for (function_parameter, value) in function
            .signature
            .parameters
            .iter()
            .zip(args.iter().cloned())
        {
            frame.vars.insert(function_parameter.parameter_name.clone(), value);
        }
        self.stack.push(frame);

        // Evaluate function body
        let mut return_value: Option<Value> = None;
        match &function.body {
            FunctionBody::UserDefinedBody(statements) => {
                let mut evaluator = Evaluator::new(self);
                for stmt in statements {
                    evaluator.eval_statement(stmt)?;
                }
            }
            // NativeBody expected to return Option<Value>; use it directly
            FunctionBody::NativeBody(f) => return_value = f(args),
        }

        self.stack.pop();

        // If the function returned nothing, represent that as Value::None
        Ok(return_value.unwrap_or(Value::None))
    }

    pub fn current_frame_mut(&mut self) -> Result<&mut StackFrame, String> {
        self.stack
            .last_mut()
            .ok_or_else(|| "No stack frame available".to_string())
    }

    pub fn lookup_var(&self, id: &str) -> Result<Value, String> {
        for frame in self.stack.iter().rev() {
            if let Some(val) = frame.vars.get(id) {
                return Ok(val.clone());
            }
        }
        Err(format!("eval_expr: variable '{}' not found", id))
    }

    pub fn assign_var(&mut self, id: &str, value: Value) {
        for frame in self.stack.iter_mut().rev() {
            if frame.vars.contains_key(id) {
                frame.vars.insert(id.to_string(), value);
                return;
            }
        }
        // If not found, assign in current frame
        if let Ok(frame) = self.current_frame_mut() {
            frame.vars.insert(id.to_string(), value);
        }
    }

    pub fn eval_decl(&mut self, decl: &crate::parser::VarDecl) -> Result<(), String> {
        let decl_val = Evaluator::new(self).eval_expr(&decl.expr)?;
        let frame = self.current_frame_mut()?;
        frame.vars.insert(decl.identifier.clone(), decl_val);
        Ok(())
    }
}
