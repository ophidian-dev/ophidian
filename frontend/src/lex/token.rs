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

    // a boolean literal representing true
    True,

    // a boolean literal representing false
    False,

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

    // keyword used to declare a binding for a mutable variable
    Let,

    // a colon ':'
    Colon,

    // the type 'int'
    // not to be confused with IntegerLiteral
    Int,

    // the type 'bool'
    // not to be confused with boolean literals i.e. 'true' and 'false'
    Bool,

    // assignment operator
    // i.e. '='
    Equal,

    // equality operator
    // i.e. '=='
    EqualEqual,

    // not equal
    // i.e. '!='
    BangEqual,

    // greatar than '>'
    GreaterThan,

    // less than '<'
    LessThan,

    // greater or equal '>='
    GreaterEq,

    // less or equal '<='
    LessEq,

    // an identifier
    Identifier,

    // '{'
    OpenBrace,

    // '}'
    CloseBrace,

    // logical and '&&'
    And,

    // logical or '||'
    Or,

    // 'if' keyword
    If,

    // 'else' keyword
    Else,

    // 'while' keyword
    While,

    // end of the file
    Eof,
}

impl From<TokenKind> for bool {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::True => true,
            TokenKind::False => false,
            _ => {
                panic!("tried to convert non boolean literal to bool")
            }
        }
    }
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
