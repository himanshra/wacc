mod codegen;
mod lexer;
mod parser;

use crate::lexer::TokenKind;
use codegen::Codegen;
use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (stage, input_file) = if args.len() == 2 {
        ("".to_string(), args[1].to_string())
    } else {
        (args[1].to_string(), args[2].to_string())
    };

    eprintln!("stage: {stage}, input_file: {input_file}");

    let source = fs::read_to_string(input_file).unwrap_or_else(|e| {
        eprintln!("error reading file: {}", e);
        std::process::exit(1);
    });

    let mut lexer = Lexer::new(&source);
    loop {
        let tok = lexer.next_token();
        if let TokenKind::Invalid(c) = tok.kind {
            eprintln!("lexer error: invalid character '{}'", c);
            std::process::exit(1);
        }
        if matches!(tok.kind, TokenKind::Eof) {
            break;
        }
    }

    if stage.eq("--lex") {
        std::process::exit(0);
    }

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program().unwrap();

    if std::env::var("DUMP_AST").is_ok() {
        eprintln!("{:#?}\n", ast);
    }

    if stage.eq("--parse") {
        std::process::exit(0);
    }

    let asm = Codegen::generate(&ast);
    println!("{}", asm);

    /*
    $ gcc out.s -o out
    $ ./out
    $ echo $?
    */
    fs::write("out.s", &asm).expect("failed to write assembly");
}
