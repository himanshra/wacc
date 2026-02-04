use crate::lexer::{Lexer, TokenKind};

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub body: Stmt,
}

#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Constant(i64),
}

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

    fn advance(&mut self) -> Result<(), String> {
        let next = self.lexer.next_token().kind;

        if let TokenKind::Invalid(c) = next {
            return Err(format!("lexer error: invalid character '{}'", c));
        }

        self.current = next;
        Ok(())
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), String> {
        if self.current == expected {
            self.advance()?;
            Ok(())
        } else {
            Err(format!(
                "parser error: expected {:?}, found {:?}",
                expected, self.current
            ))
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match &self.current {
            TokenKind::Number(n) => {
                let value = *n;
                self.advance()?;
                Ok(Expr::Constant(value))
            }
            _ => Err("expected number".to_string()),
        }
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::Return)?;
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Return(expr))
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        self.expect(TokenKind::Int)?;

        let name = match &self.current {
            TokenKind::Main => {
                self.advance()?;
                "main".to_string()
            }
            TokenKind::Identifier(_) => {
                return Err("only 'main' function is allowed".to_string());
            }
            _ => {
                return Err("expected function name".to_string());
            }
        };

        self.expect(TokenKind::LParen)?;
        self.expect(TokenKind::Void)?;
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;

        let body = self.parse_return()?;

        self.expect(TokenKind::RBrace)?;
        self.expect(TokenKind::Eof)?;

        Ok(Program {
            function: Function { name, body },
        })
    }
}
