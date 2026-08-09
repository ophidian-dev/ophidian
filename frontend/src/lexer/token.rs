use crate::span::Span;

// abstraction for the tokenstream
pub trait TokenStream {
    fn next(&mut self) -> Token;

    fn collect(&mut self) -> Vec<Token> {
        let mut v = Vec::new();

        loop {
            match self.next() {
                t if t.kind == TokenKind::Eof => {
                    break;
                }
                t => {
                    v.push(t);
                }
            }
        }

        v
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // base 10 integer literals
    // e.g. '10'
    IntegerLiteral,

    // plus character
    // i.e. '+'
    Plus,
    // minus character
    // i.e. '-'
    Minus,
    // star character
    // i.e. '*'
    Star,
    // slash character
    // i.e. '/'
    Slash,

    // open parentheses
    // i.e. '('
    OpenParen,
    // close parentheses
    // i.e. ')'
    CloseParen,

    // error token containing the offending character
    Error(u8),

    // print keyword
    // this is a builtin statement which acts like a function
    // until functions are implemented
    Print,

    // a semicolon 
    // i.e. ';'
    Semicolon,

    // end of the file
    Eof,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token {
    // type of token
    pub kind: TokenKind,
    // offset and length in source string
    pub span: Span,
    // line number
    // 0 based
    pub line: usize,
    // column number
    // 0 based
    pub column: usize,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span, line: usize, column: usize) -> Self {
        Self {
            kind,
            span,
            line,
            column,
        }
    }

    // a temporary token that is used as a placehodler and will be thrown away
    pub const fn dummy() -> Self {
        Self::new(TokenKind::Error(u8::MAX), Span::dummy(), 0, 0)
    }
}
