mod driver;
mod cli;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod type_checker;

fn main() {
    let (file_path, entry_function) = cli::parse_args();

    let file = file_path.to_string_lossy().to_string();

    driver::run_pipeline(file, entry_function);
}