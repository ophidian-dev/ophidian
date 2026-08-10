use crate::lexer::token::{Token, TokenKind, TokenStream};
use crate::span::Span;

#[derive(Default)]
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
    pub const fn new(source: &'src [u8]) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            line: 0,
            column: 0,
            start_column: 0,
        }
    }

    // returns an optional byte
    // gets the character self.current is currently pointing at
    // if self.current is pointing past the end of the source buffer
    // then peek returns None
    // otherwise it clones the character and wraps it in Some(T)
    fn peek(&self) -> Option<u8> {
        return self.source.get(self.current).cloned();
    }

    // advances the lexer cursor
    // return None at EOF
    fn advance(&mut self) -> Option<u8> {
        let c = self.peek();
        match c {
            Some(c) => {
                if c == b'\n' {
                    self.column = 0;
                    self.line += 1
                } else {
                    self.column += 1;
                    self.current += 1;
                }
            }
            None => {
                return c;
            }
        }
        c
    }

    fn create_token(&self, kind: TokenKind) -> Token {
        let span = Span::new(self.start, self.current - self.start);
        Token::new(kind, span, self.line, self.start_column)
    }

    fn get_identifier_type(&self, ident: &[u8]) -> TokenKind {
        match ident {
            b"print" => TokenKind::Print,
            _ => {
                unreachable!("unknown: {:?}", ident);
            }
        }
    }
}

impl<'src> TokenStream for Lexer<'src> {
    fn next(&mut self) -> Token {
        loop {
            if let Some(c) = self.peek() {
                if c.is_ascii_whitespace() {
                    self.advance();
                    continue;
                }
                break;
            } else {
                return self.create_token(TokenKind::Eof);
            }
        }

        self.start = self.current;
        self.start_column = self.column;

        let c = match self.peek() {
            Some(x) => x,
            None => {
                return self.create_token(TokenKind::Eof);
            }
        };

        match c {
            b'+' => {
                self.advance();
                return self.create_token(TokenKind::Plus);
            }
            b'-' => {
                self.advance();
                return self.create_token(TokenKind::Minus);
            }
            b'*' => {
                self.advance();
                return self.create_token(TokenKind::Star);
            }
            b'/' => {
                self.advance();
                return self.create_token(TokenKind::Slash);
            }
            b'(' => {
                self.advance();
                return self.create_token(TokenKind::OpenParen);
            }
            b')' => {
                self.advance();
                return self.create_token(TokenKind::CloseParen);
            }
            b';' => {
                self.advance();
                return self.create_token(TokenKind::Semicolon);
            }
            _ => {
                if c.is_ascii_digit() {
                    while let Some(d) = self.peek() {
                        if d.is_ascii_digit() {
                            self.advance();
                            continue;
                        }
                        break;
                    }
                    return self.create_token(TokenKind::IntegerLiteral);
                } else if c.is_ascii_alphabetic() {
                    let mut ident = Vec::<u8>::new();
                    while let Some(d) = self.peek() {
                        if d.is_ascii_alphabetic() {
                            ident.push(d);
                            self.advance();
                            continue;
                        }
                        break;
                    }
                    let ident_kind = self.get_identifier_type(&ident);
                    return self.create_token(ident_kind);
                } else {
                    return self.create_token(TokenKind::Error(c));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // creates a lexer with a source string and returns it
    fn new_lexer() -> Lexer<'static> {
        Lexer::new(b"(1 + 2) * 3")
    }

    #[test]
    // tests to see if peek at eof will return none
    fn test_peek_return_none_on_eof() {
        let lexer = Lexer::new(b"");
        assert_eq!(None, lexer.peek());
    }

    #[test]
    // test to see if when peek is not called at eof
    // it returns the orrect byte wrapped in Some
    fn test_peek_return_some_with_correct_value() {
        let lexer = new_lexer();
        assert_eq!(Some(b'('), lexer.peek());
    }

    #[test]
    // tests to see if the token construction function
    // computes offsets and lengths correctly
    fn test_token_creation() {
        let mut lexer = new_lexer();
        lexer.advance();
        let tok = lexer.create_token(TokenKind::OpenParen);
        assert_eq!(
            tok,
            Token::new(
                TokenKind::OpenParen,
                Span::new(lexer.start, lexer.current - lexer.start),
                lexer.line,
                lexer.start_column
            )
        );
    }

    #[test]
    // see if line count in lexer increments correctly when it encounters a newline
    fn test_newline_handling() {
        let mut lexer = Lexer::new(b"(\n)");
        lexer.advance();
        lexer.advance();
        assert_eq!(1, lexer.line);
    }

    #[test]
    fn test_column_tracking() {
        let mut lexer = new_lexer();
        lexer.advance();
        assert_eq!(1, lexer.column);
    }

    #[test]
    fn test_column_tracking_with_newline() {
        let mut lexer = Lexer::new(b"(\n");
        lexer.advance();
        lexer.advance();
        assert_eq!(0, lexer.column);
    }

    #[test]
    fn test_whitespace_skipping() {
        let mut lexer = Lexer::new(b"1  +2");
        lexer.next();
        let tok = lexer.next();
        assert_eq!(tok.span.start(), 3);
    }

    #[test]
    fn test_print_kw_detection() {
        let lexer = Lexer::default();
        assert_eq!(lexer.get_identifier_type(b"print"), TokenKind::Print);
    }
}
