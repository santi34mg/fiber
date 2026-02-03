use std::{collections::HashMap, sync::Arc};
use std::io;
use crate::parser::{Function, FunctionBody, FunctionSignature};
use crate::token::TypeIdentifier;
use crate::interpreter::core::Value;

pub fn register() -> HashMap<String, Function> {
    let mut m = HashMap::new();

    let read_char_signature = FunctionSignature {
        name: "read_char".to_string(),
        parameters: vec![],
        return_type: Some(TypeIdentifier::Char),
    };
    let read_char_body = FunctionBody::NativeBody(Arc::new(|_args: &[Value]| -> Option<Value> {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        if let Some(c) = input.chars().next() {
            Some(Value::Char(c))
        } else {
            None
        }
    }));
    m.insert(
        read_char_signature.name.clone(),
        Function {
            signature: read_char_signature,
            body: read_char_body,
        },
    );

    m
}
