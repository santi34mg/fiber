mod driver;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod type_checker;

fn main() {
    let file = std::env::args().nth(1).expect("Missing file path");
    let src = std::fs::read_to_string(&file).expect("Failed to read file");
    let tokens = driver::run_lexer(&file, &src);
    driver::show_tokens(&tokens);
    let ast = driver::run_parser(tokens, file, src);
    if ast.is_none() {
        return;
    }
    let ast = ast.unwrap();
    driver::show_ast(&ast);
    driver::run_type_checking(&ast);
    let result = driver::run_interpreter(ast);
    println!("{:?}", result);
}
