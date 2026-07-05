use crate::{
    lex::token::{Token, TokenType},
    parse::span::Span,
};

pub struct Lexer<'a> {
    current: usize,
    start: usize,
    line: usize,
    column: usize,
    start_column: usize,

    pub source: &'a [u8],
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            current: 0,
            start: 0,
            line: 0,
            column: 0,
            start_column: 0,
            source,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.current).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let c: Option<u8> = self.peek();
        if let Some(d) = c {
            if d == b'\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
        self.current += 1;
        c
    }

    fn create_token(&self, kind: TokenType) -> Token {
        Token {
            kind: kind,
            span: Span::new(self.start, self.current - self.start),
            line: self.line,
            column: self.start_column,
        }
    }

    fn create_error(&self, unexpected: u8) -> Token {
        Token {
            kind: TokenType::Error(unexpected),
            span: Span::new(self.start, self.current - self.start),
            line: self.line,
            column: self.start_column,
        }
    }

    fn get_keyword(&self, s: &[u8]) -> TokenType {
        match s {
            b"print" => TokenType::Print,
            _ => {
                todo!("implement identifiers");
            }
        }
    }
}

const WHITESPACE_LOOPUP: [bool; 256] = {
    let mut arr = [false; 256];
    let mut i: usize = 0;
    while i < 256 {
        if i >= 9 && i <= 13 || i == 32 {
            arr[i] = true;
        }
        i += 1;
    }
    arr
};

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(c) = self.peek() {
                let b = c as usize;
                if WHITESPACE_LOOPUP[b] {
                    self.advance();
                    continue;
                }

                self.start = self.current;
                self.start_column = self.column;

                match c {
                    b'+' => {
                        self.advance();
                        return Some(self.create_token(TokenType::Plus));
                    }
                    b'-' => {
                        self.advance();
                        return Some(self.create_token(TokenType::Minus));
                    }
                    b'*' => {
                        self.advance();
                        return Some(self.create_token(TokenType::Star));
                    }
                    b'/' => {
                        self.advance();
                        return Some(self.create_token(TokenType::Slash));
                    }
                    b'(' => {
                        self.advance();
                        return Some(self.create_token(TokenType::OpenParen));
                    }
                    b')' => {
                        self.advance();
                        return Some(self.create_token(TokenType::CloseParen));
                    }
                    b';' => {
                        self.advance();
                        return Some(self.create_token(TokenType::Semicolon));
                    }
                    _ => {
                        if c.is_ascii_digit() {
                            self.advance();
                            while let Some(a) = self.peek() {
                                if !a.is_ascii_digit() {
                                    break;
                                }
                                self.advance();
                            }
                            return Some(self.create_token(TokenType::IntegerLiteral));
                        } else if c.is_ascii_alphabetic() {
                            let mut s: Vec<u8> = Vec::new();
                            // we unwrap because c is already guarenteed to be Some
                            s.push(self.advance().unwrap());
                            while let Some(a) = self.peek() {
                                if !a.is_ascii_alphabetic() {
                                    break;
                                }
                                // we unwrap here because a is guarenteed to be Some
                                s.push(self.advance().unwrap());
                            }
                            let token = self.get_keyword(&s);
                            return Some(self.create_token(token));
                        } else {
                            self.advance();
                            return Some(self.create_error(c));
                        }
                    }
                }
            }
            return None;
        }
    }
}
