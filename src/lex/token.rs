

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub offset: usize,
    pub len: usize,
    pub line: usize,
    pub column: usize,
}