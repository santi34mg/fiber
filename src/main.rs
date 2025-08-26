mod driver;
mod token;
mod lexer;
mod parser;
mod interpreter;

fn main() {
    let file = std::env::args().nth(1).expect("Missing file path");
    let src = std::fs::read_to_string(&file).expect("Failed to read file");
    let tokens = driver::run_lexer(&file, &src);
    driver::show_tokens(&tokens);
    let ast = driver::run_parser(tokens);
    driver::show_ast(&ast);
    let results = driver::run_interpreter(ast);
    println!("{:?}", results);
}

