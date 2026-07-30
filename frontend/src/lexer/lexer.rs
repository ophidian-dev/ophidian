use crate::lexer::token::{Token, TokenKind, TokenStream};
use crate::span::Span;

pub struct Lexer<'src> {
    // source string to be lexed
    source: &'src [u8],

    // index of which the lexer is currently pointing to
    current: usize,
    // start index of the current token
    start: usize,
    // line number of the current token
    line: usize,
    // current column the lexer is pointing at
    column: usize,
    // index of where the column started for the current token
    start_column: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src [u8]) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            line: 0,
            column: 0,
            start_column: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        return self.source.get(self.current).cloned()
    }

    fn advance(&mut self) -> Option<u8> {
        let c = self.peek();
        self.current += 1;
        c
    }

    fn create_token(&self, kind: TokenKind) -> Token {
        let span = Span::new(self.start, self.current - self.start);
        Token::new(kind, span, self.line, self.start_column)
    }
}

impl<'src> TokenStream for Lexer<'src> {
    fn next(&mut self) -> Option<Token> {
        loop {
            if let Some(c) = self.peek() {
                if c.is_ascii_whitespace() {
                    self.advance();
                    continue;
                } 
                break;
            } else {
                return None;
            }
        }

        self.start = self.current;
        self.start_column = self.column;

        let c = match self.peek() {
            Some(x) => {
                x
            }
            None => {
                return None;
            }
        };

        match c {
            b'+' => {
                self.advance();
                return Some(self.create_token(TokenKind::Plus));
            }
            b'-' => {
                self.advance();
                return Some(self.create_token(TokenKind::Minus));
            }
            b'*' => {
                self.advance();
                return Some(self.create_token(TokenKind::Star));
            }
            b'/' => {
                self.advance();
                return Some(self.create_token(TokenKind::Slash));
            }
            b'(' => {
                self.advance();
                return Some(self.create_token(TokenKind::OpenParen));
            }
            b')' => {
                self.advance();
                return Some(self.create_token(TokenKind::CloseParen));
            }
            _ => {
                if c.is_ascii_digit() {
                    while let Some(d) = self.peek() {
                        if d.is_ascii_digit() {
                            self.advance();
                        }
                        break;
                    }
                    return Some(self.create_token(TokenKind::IntegerLiteral));
                } else {
                    return Some(self.create_token(TokenKind::Error(c)))
                }
            }
        }
    }
}
