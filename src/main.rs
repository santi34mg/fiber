mod driver;
mod token;
mod lexer;
mod parser;

fn main() {
    let file = std::env::args().nth(1).expect("Missing file path");
    let src = std::fs::read_to_string(&file).expect("Failed to read file");
    let tokens = driver::run_lexer(&file, &src);
    let ast = driver::run_parser(tokens);
    println!("{:#?}", ast);
}

