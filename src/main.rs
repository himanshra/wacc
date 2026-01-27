mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: {} <file.c>", args[0]);
        std::process::exit(1);
    }

    let source = fs::read_to_string(&args[1]).expect("failed to read input file");

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);

    let return_value = parser.parse_program();

    println!("parsed successfully, return value = {}", return_value);
}
