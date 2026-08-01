use crate::diagnostics::Diagnostic;
use crate::lexer::token::{Token, TokenKind, TokenStream};
use crate::parser::ast::{BinOpKind, Expr, ExprKind, LitKind, UnaryOpKind};
use crate::parser::node_id::NodeId;
use crate::span::{Span, Spanned};

pub struct Parser<'src, 'diag, T>
where
    T: TokenStream,
{
    // a stream of tokens that we can interate over by calling `.next()` on it
    tokenstream: T,

    // vector of diagnostics
    diagnostics: &'diag mut Vec<Diagnostic>,

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

    pub fn parse(&mut self) -> Expr {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expr {
        self.parse_factor()
    }

    fn parse_factor(&mut self) -> Expr {
        let mut left = self.parse_unary();
        while self.peek().kind == TokenKind::Star || self.peek().kind == TokenKind::Slash {
            let op = self.advance();

            let right = self.parse_unary();

            // we create a copy of the span for right because we cannot access
            // it after we move it into a Box
            let right_span = right.span;

            if op.kind == TokenKind::Star {
                left = Expr::new(
                    self.next_node_id(),
                    ExprKind::BinaryOp(
                        Spanned::new(BinOpKind::Mul, op.span), Box::new(left), Box::new(right)
                    ),
                    op.span.join(right_span)
                )
            } else {
                left = Expr::new(
                    self.next_node_id(),
                    ExprKind::BinaryOp(
                        Spanned::new(BinOpKind::Div, op.span), Box::new(left), Box::new(right) 
                    ),
                    op.span.join(right_span)
                )
            }
        }
        left
    }

    fn parse_unary(&mut self) -> Expr {
        if self.peek().kind == TokenKind::Minus {
            let op_span = self.advance().span;

            let right = self.parse_unary();

            // we save the span of the expr because we move right into a Box
            // before we can pass the span into the function
            let right_span = right.span;

            return Expr::new(
                self.next_node_id(),
                ExprKind::UnaryOp(Spanned::new(UnaryOpKind::Negate, op_span), Box::new(right)),
                right_span,
            );
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expr {
        if self.peek().kind == TokenKind::IntegerLiteral {
            self.advance();

            let lexeme = Span::retrieve_slice(self.source, &self.prev.span);

            let value = std::str::from_utf8(lexeme)
                // we expect the user to uphold the invariant of the source string
                // being valid ascii and therefore valid utf8 so if the user fails
                // to uphold that invariant, the compiler will panic and crash
                .expect("source code is assumed to be valid ascii and therefore valid utf8")
                .parse::<u128>()
                // TODO: handle integer parsing errors more elegantly
                .expect("failed to parse integer");

            return Expr::new(
                self.next_node_id(),
                ExprKind::Literal(LitKind::Int(value)),
                self.prev.span,
            );
        } else if self.peek().kind == TokenKind::OpenParen {
            // advnace past the open parentheses
            self.advance();

            let expr = self.parse_expression();

            if self.peek().kind != TokenKind::CloseParen {
                todo!("handle error");
            }
            self.advance();

            return expr;
        } else {
            todo!("handle unexpected token error");
        }
    }
}
