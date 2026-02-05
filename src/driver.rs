use std::path::Path;
use std::{fs, process};

use crate::parser::{Ast, Parser};
use crate::type_checker::TypeChecker;
use crate::{lexer::Lexer, token::Token};

pub fn run_pipeline(file: &Path, is_debug_mode: bool) {
    // Run pipeline
    let src = fs::read_to_string(&file).unwrap_or_else(|e| {
        eprintln!("Failed to read '{:?}': {}", file, e);
        process::exit(1);
    });
    let tokens = run_lexer(&src);
    // Optionally display tokens during development
    if is_debug_mode {
        show_tokens(&tokens);
    }
    let ast_opt = run_parser(tokens, file.to_string_lossy().to_string(), src.clone());
    let ast = match ast_opt {
        Some(a) => a,
        None => process::exit(1),
    };
    if is_debug_mode {
        show_ast(&ast);
    }
    run_type_checking(&ast);
}

pub fn run_lexer(src: &String) -> Vec<Token> {
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

#[allow(dead_code)]
pub(crate) fn show_tokens(tokens: &Vec<Token>) {
    println!("====START TOKENS=======");
    for token in tokens {
        println!("{:?}", token);
    }
    println!("====END TOKENS=========");
}

#[allow(dead_code)]
pub(crate) fn show_ast(ast: &Ast) {
    println!("====START AST==========");
    println!("{:#?}", ast);
    println!("====END AST============");
}
