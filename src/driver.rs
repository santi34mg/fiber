use crate::{lexer::Lexer, token::Token};


pub fn run_lexer(file: &String, src: &String) -> Vec<Token> {
    let lexer = Lexer::new(&src);
    lexer.collect()
}
