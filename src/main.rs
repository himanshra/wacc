mod lexer;

use lexer::{Lexer, TokenKind};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: {} <file.c>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    println!("Reading: {}", filename);

    let source = fs::read_to_string(filename)
        .expect(format!("failed to read input file: {}", filename).as_str());

    let mut lexer = Lexer::new(&source);

    let mut tok = lexer.next_token();

    while tok.kind != TokenKind::Eof {
        println!("{:?}", tok.kind);
        tok = lexer.next_token();
    }

    println!("{:?}", tok.kind);
}
