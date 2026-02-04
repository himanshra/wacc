mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = args
        .iter()
        .skip(1)
        .find(|arg| arg.ends_with(".c"))
        .unwrap_or_else(|| {
            eprintln!("no input .c file provided");
            std::process::exit(1);
        });

    let source = fs::read_to_string(input_file).unwrap_or_else(|e| {
        eprintln!("error reading file: {}", e);
        std::process::exit(1);
    });

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);

    match parser.parse_program() {
        Ok(ast) => {
            if std::env::var("DUMP_AST").is_ok() {
                eprintln!("{:#?}", ast);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
