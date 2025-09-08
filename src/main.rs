mod driver;
mod token;
mod lexer;
mod parser;
mod interpreter;
mod type_checker;

fn main() {
    let file = std::env::args().nth(1).expect("Missing file path");
    let src = std::fs::read_to_string(&file).expect("Failed to read file");
    let tokens = driver::run_lexer(&file, &src);
    driver::show_tokens(&tokens);
    let ast = driver::run_parser(tokens);
    driver::show_ast(&ast);
    driver::show_ast(&ast);
    driver::run_type_checking(&ast);
    let result = driver::run_interpreter(ast);
    println!("{:?}", result);
}

