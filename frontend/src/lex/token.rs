use crate::span::Span;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TokenType {
    IntegerLiteral,

    BooleanLiteral(bool),

    Plus,
    Minus,
    Star,
    Slash,

    OpenParen,
    CloseParen,

    OpenBrace,
    CloseBrace,

    Error(u8),

    Print,

    Semicolon,

    Let,

    Int,
    Bool,

    Equal,

    EqualEqual,
    BangEqual,
    GreaterEqual,
    LesserEqual,
    Greater,
    Lesser,

    Bang,
    Or,
    And,

    Identifier,

    Colon,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenType,
    pub span: Span,
    pub line: usize,
    pub column: usize,
}
