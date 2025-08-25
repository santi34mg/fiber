use crate::parser::{Ast, Parser};
use crate::interpreter::Interpreter;
use crate::{lexer::Lexer, token::Token};

pub fn run_lexer(_file: &String, src: &String) -> Vec<Token> {
    let lexer = Lexer::new(&src);
    let tokens = lexer.collect();
    println!("{:?}", tokens);
    tokens
}

pub fn run_parser(tokens: Vec<Token>) -> Ast {
    let mut parser = Parser::new(tokens.into_iter());
    let ast = parser.parse_program().expect("Could not parse the program");
    ast
}

pub fn run_interpreter(ast: Ast) -> Vec<i32> {
    let interpreter = Interpreter::new();
    interpreter.eval(ast)
}
