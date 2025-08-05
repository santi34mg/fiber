mod driver;
mod token;
mod lexer;

fn main() {
    let file = std::env::args().nth(1).expect("Missing file path");
    let src = std::fs::read_to_string(&file).expect("Failed to read file");
    let tokens = driver::run_lexer(&file, &src);
    for token in tokens {
        println!("{:?}", token);
    }
}

