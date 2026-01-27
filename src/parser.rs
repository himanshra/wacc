use crate::lexer::{Lexer, TokenKind};

pub struct Parser {
    lexer: Lexer,
    current: TokenKind,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let first = lexer.next_token().kind;
        Self {
            lexer,
            current: first,
        }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token().kind;
    }

    fn expect(&mut self, expected: TokenKind) {
        if self.current == expected {
            self.advance();
        } else {
            eprintln!(
                "parser error: expected {:?}, found {:?}",
                expected, self.current
            );
            std::process::exit(1);
        }
    }

    pub fn parse_program(&mut self) -> i64 {
        self.expect(TokenKind::Int);
        self.expect(TokenKind::Main);
        self.expect(TokenKind::LParen);
        self.expect(TokenKind::Void);
        self.expect(TokenKind::RParen);
        self.expect(TokenKind::LBrace);
        self.expect(TokenKind::Return);

        let return_value = match &self.current {
            TokenKind::Number(n) => {
                let v = *n;
                self.advance();
                v
            }
            _ => {
                eprintln!("parser error: expected number after return");
                std::process::exit(1);
            }
        };

        self.expect(TokenKind::Semicolon);
        self.expect(TokenKind::RBrace);
        self.expect(TokenKind::Eof);

        return_value
    }
}
