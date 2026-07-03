use crate::lex::lexer::Lexer;
use crate::lex::token::Token;
use crate::parse::ast::{BinopType, Expr};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>
}

pub enum ParseError {

}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current: Option<Token> = lexer.next();
        Self {
            lexer,
            current
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.current.as_ref()
    }

    fn advance(&mut self) {
        self.current = self.lexer.next();
    }

    pub fn generate_ast(&mut self) -> Expr {
        Expr::IntegerLiteral(2)
    }
}