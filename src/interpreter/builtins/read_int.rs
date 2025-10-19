use std::{collections::HashMap, sync::Arc};
use std::io;
use crate::parser::{Function, FunctionBody, FunctionSignature};
use crate::token::TypeIdentifier;
use crate::interpreter::core::Value;

pub fn register() -> HashMap<String, Function> {
    let mut m = HashMap::new();

    let read_int_signature = FunctionSignature {
        name: "read_int".to_string(),
        parameters: vec![],
        return_type: Some(TypeIdentifier::Number),
    };
    let read_int_body = FunctionBody::NativeBody(Arc::new(|_args: &[Value]| -> Option<Value> {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let trimmed = input.trim();
        match trimmed.parse::<i32>() {
            Ok(n) => Some(Value::Number(n)),
            Err(_) => None,
        }
    }));
    m.insert(
        read_int_signature.name.clone(),
        Function {
            signature: read_int_signature,
            body: read_int_body,
        },
    );

    m
}
