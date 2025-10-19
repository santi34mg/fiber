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
    let print_int_signature = FunctionSignature {
        name: "print_int".to_string(),
        parameters: vec![print_int_param.clone()],
        return_type: None,
    };
    let print_body = FunctionBody::NativeBody(Arc::new(|args: &[Value]| -> Option<Value>{
        if let Some(v) = args.get(0) {
            match v {
                Value::Number(n) => print!("{}", n),
                Value::Boolean(b) => print!("{}", b),
                Value::Char(c) => print!("{}", c),
                Value::None => {}
            }
        }
        None
    }));
    m.insert(
        print_int_signature.name.clone(),
        Function {
            signature: print_int_signature,
            body: print_body,
        },
    );

    m
}
