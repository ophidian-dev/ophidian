use crate::lexer::token::{TokenStream, Token};
use crate::diagnostics::Diagnostic;

pub struct Parser<'a, T>
where
    T: TokenStream,
{
    tokenstream: T,
    diagnostics: &'a mut Vec<Diagnostic>,

    // the current token
    current: Token,
    prev: Token,
}

impl<'a, T: TokenStream> Parser<'a, T> {
    pub fn new(mut tokenstream: T, diagnostics: &'a mut Vec<Diagnostic>) -> Self {
        let current = tokenstream.next();
        Self {
            tokenstream,
            diagnostics,
            current,
            prev: Token::dummy(),
        }
    } 

    fn peek(&self) -> Token {
        self.current
    }

    fn advance(&mut self) -> Token {
        Token::dummy()
    }
}