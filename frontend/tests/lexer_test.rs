use frontend::lex::lexer::Lexer;
use frontend::lex::token::{Token, TokenType};

fn lex(input: &str) -> Vec<TokenType> {
    Lexer::new(input.as_bytes()).map(|t| t.kind).collect()
}

#[test]
fn basic_expression() {
    let tokens = lex("1 + 2 * 3");

    assert_eq!(
        tokens,
        vec![
            TokenType::IntegerLiteral,
            TokenType::Plus,
            TokenType::IntegerLiteral,
            TokenType::Star,
            TokenType::IntegerLiteral,
        ]
    );
}

#[test]
fn parentheses_expression() {
    let tokens = lex("(10 + 20) / 5");

    assert_eq!(
        tokens,
        vec![
            TokenType::OpenParen,
            TokenType::IntegerLiteral,
            TokenType::Plus,
            TokenType::IntegerLiteral,
            TokenType::CloseParen,
            TokenType::Slash,
            TokenType::IntegerLiteral,
        ]
    );
}

#[test]
fn whitespace_and_newlines() {
    let tokens = lex("  12\n+ 34 ");

    assert_eq!(
        tokens,
        vec![
            TokenType::IntegerLiteral,
            TokenType::Plus,
            TokenType::IntegerLiteral,
        ]
    );
}

#[test]
fn keyword_print() {
    let tokens = lex("print");

    assert_eq!(tokens, vec![TokenType::Print]);
}

#[test]
fn unknown_char_becomes_error() {
    let tokens: Vec<Token> = Lexer::new("@".as_bytes()).collect();

    match &tokens[0].kind {
        TokenType::Error(c) => assert_eq!(*c, b'@'),
        _ => panic!("expected error token"),
    }
}
