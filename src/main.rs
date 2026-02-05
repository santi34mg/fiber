mod cli;
mod driver;
mod lexer;
mod parser;
mod token;
mod type_checker;

fn main() {
    let args = cli::parse_args();

    cli::exec_command(args);
}
