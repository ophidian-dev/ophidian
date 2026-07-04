use crate::parse::span::Span;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TokenType {
    IntegerLiteral,
    Plus,
    Minus,
    Star,
    Slash,
    OpenParen,
    CloseParen,
    Error(u8),
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenType,
    pub span: Span,
    pub line: usize,
    pub column: usize,
}
