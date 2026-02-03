use std::{fs, process};

use crate::interpreter::{Interpreter, Value};
use crate::parser::{Ast, Parser, Statement};
use crate::type_checker::TypeChecker;
use crate::{lexer::Lexer, token::Token};

pub fn run_pipeline(file: String, entry_function: Option<String>) {
    // Run pipeline
    let src = fs::read_to_string(&file).unwrap_or_else(|e| {
        eprintln!("Failed to read '{}': {}", file, e);
        process::exit(1);
    });
    let tokens = run_lexer(&file, &src);
    // Optionally display tokens during development
    // driver::show_tokens(&tokens);
    let ast_opt = run_parser(tokens, file.clone(), src.clone());
    let ast = match ast_opt {
        Some(a) => a,
        None => process::exit(1),
    };
    // driver::show_ast(&ast);
    run_type_checking(&ast);

    // If an entry function is requested, use driver to call it.
    // Otherwise, if there's a function named "main" in the AST, run that automatically.
    // If neither applies, evaluate the program normally.
    if let Some(fn_name) = entry_function {
        match run_entry(ast, &fn_name) {
            Ok(vals) => {
                for v in vals {
                    println!("{:?}", v);
                }
            }
            Err(e) => {
                eprintln!("Error running entry function '{}': {}", fn_name, e);
                process::exit(1);
            }
        }
    } else {
        // Detect a user-defined `main` function and run it automatically if present
        let mut has_main = false;
        for stmt in &ast.statements {
            if let Statement::FunctionDeclaration(function) = stmt {
                if function.signature.name == "main" {
                    has_main = true;
                    break;
                }
            }
        }

        if has_main {
            match run_entry(ast, "main") {
                Ok(vals) => {
                    for v in vals {
                        println!("{:?}", v);
                    }
                }
                Err(e) => {
                    eprintln!("Error running entry function 'main': {}", e);
                    process::exit(1);
                }
            }
        } else {
            match run_interpreter(ast) {
                Ok(()) => {},
                Err(e) => {
                    eprintln!("Interpreter error: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}

pub fn run_lexer(_file: &String, src: &String) -> Vec<Token> {
    let lexer = Lexer::new(&src);
    let tokens = lexer.collect();
    tokens
}

pub fn run_parser(tokens: Vec<Token>, filename: String, source: String) -> Option<Ast> {
    // TODO: improve error handling
    let mut parser = Parser::new(tokens.into_iter(), filename, source);
    match parser.parse_program() {
        Ok(ast) => Some(ast),
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

pub fn run_type_checking(ast: &Ast) {
    let mut type_checker = TypeChecker::new(ast);
    type_checker.check_ast();
}

pub fn run_interpreter(ast: Ast) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    // Register functions from AST
    for stmt in &ast.statements {
        if let Statement::FunctionDeclaration(f) = stmt {
            interpreter.register_function(f.clone());
        }
    }
    interpreter.eval(ast)
}

/// Run a named function from the AST as the program entry point.
/// Returns the function's return values (if any) or an error string.
pub fn run_entry(ast: Ast, function_name: &str) -> Result<Vec<Value>, String> {
    // Find function declaration
    for stmt in &ast.statements {
        if let Statement::FunctionDeclaration(function) = stmt {
            if function.signature.name == function_name {
                // Build an interpreter, register functions, and call the function
                let mut interpreter = Interpreter::new();
                // Register all user-defined functions into interpreter
                for stmt in &ast.statements {
                    if let Statement::FunctionDeclaration(f) = stmt {
                        interpreter.register_function(f.clone());
                    }
                }
                // Call the function with no arguments for now
                let v = interpreter.call_function(function_name, &[])?;
                return Ok(vec![v]);
            }
        }
    }
    Err(format!("entry function '{}' not found", function_name))
}

#[allow(dead_code)]
pub(crate) fn show_tokens(tokens: &Vec<Token>) {
    println!("=====Showing Tokens=====");
    for token in tokens {
        println!("{:?}", token);
    }
}

#[allow(dead_code)]
pub(crate) fn show_ast(ast: &Ast) {
    println!("=====Showing AST=====");
    println!("{:#?}", ast);
}
