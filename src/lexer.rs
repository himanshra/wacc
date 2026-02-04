#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Int,
    Void,
    Return,
    Main,
    Identifier(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Number(i64),
    Eof,
    Invalid(char),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comments();
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Token {
                kind: TokenKind::Eof,
            };
        }

        let c = self.input[self.pos];

        if c.is_ascii_alphabetic() || c == '_' {
            return self.identifier();
        }

        if c.is_ascii_digit() {
            return self.number();
        }

        self.pos += 1;
        let kind = match c {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            ';' => TokenKind::Semicolon,
            _ => TokenKind::Invalid(c),
        };

        Token { kind }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len()
            && (self.input[self.pos].is_whitespace() || self.input[self.pos] == '\n')
        {
            self.pos += 1;
        }
    }

    fn skip_comments(&mut self) {
        loop {
            if self.pos + 1 < self.input.len()
                && self.input[self.pos] == '/'
                && self.input[self.pos + 1] == '/'
            {
                self.pos += 2;

                while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                    self.pos += 1;
                }

                if self.pos < self.input.len() && self.input[self.pos] == '\n' {
                    self.pos += 1;
                }
            } else {
                break;
            }
        }
    }

    fn identifier(&mut self) -> Token {
        let start = self.pos;

        while self.pos < self.input.len() {
            let ch = self.input[self.pos];
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.pos += 1;
            } else {
                break;
            }
        }

        let word: String = self.input[start..self.pos].iter().collect();

        let kind = match word.as_str() {
            "int" => TokenKind::Int,
            "void" => TokenKind::Void,
            "return" => TokenKind::Return,
            "main" => TokenKind::Main,
            _ => TokenKind::Identifier(word),
        };

        Token { kind }
    }

    fn number(&mut self) -> Token {
        let start = self.pos;

        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        let num: String = self.input[start..self.pos].iter().collect();
        let value = num.parse::<i64>().unwrap();

        Token {
            kind: TokenKind::Number(value),
        }
    }
}
