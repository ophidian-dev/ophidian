pub mod ast;
mod parser;

use crate::diagnostics::{Diagnostic, Severity};
use crate::lex::token::{Token, TokenKind, TokenStream};
use crate::parse::ast::NodeId;
use crate::span::Span;

pub struct Parser<'src, 'diag, T>
where
    T: TokenStream,
{
    // a stream of tokens that we can iterate over by calling `.next()` on it
    tokenstream: T,

    // vector of diagnostics
    pub diagnostics: &'diag mut Vec<Diagnostic>,

    // a reference to the source string
    source: &'src [u8],

    // the current token
    current: Token,
    // previous token
    prev: Token,

    // current nodeid
    curr_nodeid: NodeId,
}

impl<'src, 'diag, T: TokenStream> Parser<'src, 'diag, T> {
    pub fn new(
        mut tokenstream: T,
        diagnostics: &'diag mut Vec<Diagnostic>,
        source: &'src [u8],
    ) -> Self {
        let current = tokenstream.next();
        Self {
            tokenstream,
            diagnostics,
            current,
            prev: Token::dummy(),
            curr_nodeid: NodeId(0),
            source,
        }
    }

    fn peek(&self) -> Token {
        self.current
    }

    fn advance(&mut self) -> Token {
        self.prev = self.current;
        self.current = self.tokenstream.next();
        self.prev
    }

    fn next_node_id(&mut self) -> NodeId {
        let tmp = self.curr_nodeid;
        self.curr_nodeid.increment();
        tmp
    }

    fn error<M: Into<String>>(&mut self, message: M, span: Span) {
        self.diagnostics
            .push(Diagnostic::new(message.into(), span, Severity::Error));
    }

    fn sync(&mut self) {
        while self.peek().kind != TokenKind::Eof {
            let kind = self.peek().kind;
            match kind {
                TokenKind::Semicolon => {
                    self.advance();
                    return;
                }
                TokenKind::Print => {
                    return;
                }
                TokenKind::Let => {
                    return;
                }
                TokenKind::If => {
                    return;
                }
                TokenKind::While => {
                    return;
                }
                TokenKind::For => {
                    return;
                }
                TokenKind::Break => {
                    return;
                }
                TokenKind::Continue => {
                    return;
                }
                TokenKind::OpenBrace => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }
}
