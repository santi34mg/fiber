use std::{collections::HashMap, sync::Arc};
use crate::parser::{Function, FunctionBody, FunctionParameter, FunctionSignature};
use crate::token::TypeIdentifier;
use crate::interpreter::core::Value;

pub fn register() -> HashMap<String, Function> {
    let mut m = HashMap::new();

    let print_int_param = FunctionParameter {
        parameter_name: "n".to_string(),
        parameter_type: TypeIdentifier::Number,
    };
    let println_signature = FunctionSignature {
        name: "println".to_string(),
        parameters: vec![print_int_param],
        return_type: None,
    };
    let println_body = FunctionBody::NativeBody(Arc::new(|args: &[Value]| -> Option<Value>{
        if let Some(v) = args.get(0) {
            match v {
                Value::Number(n) => println!("{}", n),
                Value::Boolean(b) => println!("{}", b),
                Value::Char(c) => println!("{}", c),
                Value::None => {}
            }
        } else {
            println!();
        }
        None
    }));
    m.insert(
        println_signature.name.clone(),
        Function {
            signature: println_signature,
            body: println_body,
        },
    );

    m
}
