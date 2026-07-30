use crate::span::Span;

pub trait TokenStream {
    fn next(&mut self) -> Token;
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    IntegerLiteral,

    Plus,
    Minus,
    Star,
    Slash,

    OpenParen,
    CloseParen,
}

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub line: usize,
    pub column: usize,
}
