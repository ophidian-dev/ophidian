use frontend::lexer::Lexer;
use frontend::lexer::token::{Token, TokenKind, TokenStream};
use frontend::span::Span;

// test basic arigthmetic lexing like '+', '-', '(' etc.
#[test]
fn test_lexer_arithmetic() {
    let mut lexer = Lexer::new(b"(1 + 2) * 3");
    let tokens = lexer.collect();
    assert_eq!(
        tokens,
        vec![
            Token::new(TokenKind::OpenParen, Span::new(0, 1), 0, 0), // (
            Token::new(TokenKind::IntegerLiteral, Span::new(1, 1), 0, 1), // 1
            Token::new(TokenKind::Plus, Span::new(3, 1), 0, 3),      // +
            Token::new(TokenKind::IntegerLiteral, Span::new(5, 1), 0, 5), // 2
            Token::new(TokenKind::CloseParen, Span::new(6, 1), 0, 6), // )
            Token::new(TokenKind::Star, Span::new(8, 1), 0, 8),      // *
            Token::new(TokenKind::IntegerLiteral, Span::new(10, 1), 0, 10), // 3
        ]
    );
}
