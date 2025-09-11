use crate::interpreter::{Interpreter, Value};
use crate::parser::{Ast, Parser};
use crate::type_checker::TypeChecker;
use crate::{lexer::Lexer, token::Token};

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

pub fn run_interpreter(ast: Ast) -> Vec<Value> {
    let interpreter = Interpreter::new();
    interpreter.eval(ast)
}

pub(crate) fn show_tokens(tokens: &Vec<Token>) {
    println!("=====Showing Tokens=====");
    for token in tokens {
        println!("{:?}", token);
    }
}

pub(crate) fn show_ast(ast: &Ast) {
    println!("=====Showing AST=====");
    println!("{:#?}", ast);
}
