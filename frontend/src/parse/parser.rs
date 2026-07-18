use crate::diagnostics::{Diagnostic, Severity};
use crate::lex::Lexer;
use crate::lex::{Token, TokenType};
use crate::parse::ast::{BinaryOp, BinopType, Expr, Program, Stmt, UnaryOp, UnaryopType, NodeId};
use crate::semantic::typed::Type;
use crate::span::Span;

pub struct Parser<'src, 'diag> {
    lexer: Lexer<'src>,
    current: Option<Token>,
    previous: Option<Token>,
    diagnostics: &'diag mut Vec<Diagnostic>,
    nodeid: usize,
}

impl<'src, 'diag> Parser<'src, 'diag> {
    pub fn new(mut lexer: Lexer<'src>, diagnostics: &'diag mut Vec<Diagnostic>) -> Self {
        let current: Option<Token> = lexer.next();
        Self {
            lexer,
            current,
            previous: None,
            diagnostics,
            nodeid: 0,
        }
    }

    pub(crate) fn next_id(&mut self) -> NodeId {
        let id = NodeId(self.nodeid);
        self.nodeid += 1;
        id
    }

    fn peek(&self) -> Option<&Token> {
        self.current.as_ref()
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();
        self.current = self.lexer.next();
    }

    fn get_slice(&self, span: Span) -> &[u8] {
        &self.lexer.source[span.offset()..span.end()]
    }

    /// Centralized lookahead checker that handles error reporting and recovery
    /// without manual boilerplate blocks.
    fn consume(&mut self, expected: TokenType, error_msg: &str) -> Result<Token, ()> {
        match self.peek() {
            Some(tok) if tok.kind == expected => {
                let tok = *tok;
                self.advance();
                Ok(tok)
            }
            Some(tok) => {
                let tok = *tok;

                // FIX: If a token is missing, anchor the error to the END of the previous token
                if let Some(prev) = &self.previous {
                    self.error(error_msg, prev.span);
                } else {
                    self.error(error_msg, tok.span);
                }

                self.sync();
                Err(())
            }
            None => {
                // FIX: Handle EOF gracefully by pointing right after the final token in the file
                if let Some(prev) = &self.previous {
                    self.error(error_msg, prev.span);
                } else {
                    // Extreme fallback if the file is completely empty
                    let fallback = Token {
                        kind: TokenType::Semicolon,
                        span: Span::new(0, 0),
                        line: 0,
                        column: 0,
                    };
                    self.error(error_msg, fallback.span);
                }

                self.sync();
                Err(())
            }
        }
    }

    fn sync(&mut self) {
        while let Some(tok) = self.peek() {
            match tok.kind {
                TokenType::Semicolon | TokenType::CloseBrace => {
                    self.advance();
                    return;
                }
                TokenType::Print => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn is_var_type(&self, tok_type: TokenType) -> bool {
        match tok_type {
            TokenType::Int => true,
            _ => false,
        }
    }

    fn get_fallback_span(&self) -> Span {
        let fallback_span = self
            .previous
            .as_ref()
            .map(|t| t.span)
            .unwrap_or_else(|| Span::new(0, 0));
        fallback_span
    }

    fn error<T: Into<String>>(&mut self, message: T, span: Span) {
        self.diagnostics
            .push(Diagnostic::new(message.into(), span, Severity::Error));
    }

    fn parse_primary(&mut self) -> Expr {
        if let Some(t) = self.peek() {
            match t.kind {
                TokenType::IntegerLiteral => {
                    let tok: Token = self.peek().unwrap().clone();
                    self.advance();
                    let value: &[u8] = self.get_slice(tok.span);
                    let s: &str = std::str::from_utf8(value).unwrap();
                    let n: i32 = s.parse().unwrap();
                    self.create_integer_literal(n, tok.span)
                }
                TokenType::BooleanLiteral(b) => {
                    let tok = self.peek().unwrap().clone();
                    self.advance();
                    self.create_boolean_literal(b, tok.span)
                }
                TokenType::OpenParen => {
                    self.advance();
                    let expr: Expr = self.parse_expression();
                    if self.consume(TokenType::CloseParen, "expected ')'").is_err() {
                        return self.create_expr_err(expr.span());
                    }
                    expr
                }
                TokenType::Identifier => {
                    let tok = self.peek().unwrap().clone();
                    let name = self.get_slice(tok.span).to_vec();
                    self.advance();
                    self.create_variable(name, tok.span)
                }
                _ => {
                    let tok = t.clone();
                    // TODO: make error handle the expected arithmetic expression cleanly to not put caret in wrong place
                    // self.error(tok, tok.span, "expected arithmetic expression");
                    self.error("unexpected token", tok.span);
                    self.sync();
                    self.create_expr_err(tok.span)
                }
            }
        } else {
            // safe recovery point if expression context hits EOF prematurely
            let fallback_span = self
                .previous
                .as_ref()
                .map(|t| t.span)
                .unwrap_or_else(|| Span::new(0, 0));
            self.error("unexpected end of input", fallback_span);
            self.create_expr_err(fallback_span)
        }
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek() {
            Some(t) if t.kind == TokenType::Minus => {
                let tok: Token = self.peek().unwrap().clone();
                self.advance();
                let right: Expr = self.parse_unary();
                let right_span: Span = right.span();

                self.create_unary_op(
                    UnaryOp::new(UnaryopType::Negate, tok.span),
                    right,
                    tok.span.join(right_span),
                )
            }
            Some(t) if t.kind == TokenType::Bang => {
                let tok = self.peek().unwrap().clone();
                self.advance();
                let right: Expr = self.parse_unary();
                let right_span: Span = right.span();

                self.create_unary_op(
                    UnaryOp::new(UnaryopType::Not, tok.span),
                    right,
                    tok.span.join(right_span),
                )
            }
            Some(_) => self.parse_primary(),
            None => {
                // safe recovery point instead of panic
                let fallback_span = self
                    .previous
                    .as_ref()
                    .map(|t| t.span)
                    .unwrap_or_else(|| Span::new(0, 0));
                self.error("unexpected end of input", fallback_span);
                self.create_expr_err(fallback_span)
            }
        }
    }

    fn parse_factor(&mut self) -> Expr {
        let mut left: Expr = self.parse_unary();
        let start_span: Span = left.span();
        while let Some(t) = self.peek() {
            match t.kind {
                TokenType::Star => {
                    let op_span: Span = self.peek().unwrap().span;
                    self.advance();
                    let right: Expr = self.parse_unary();
                    let right_span: Span = right.span();
                    left = self.create_binary_op(
                        BinaryOp::new(BinopType::Mul, op_span),
                        left,
                        right,
                        start_span.join(right_span),
                    );
                }
                TokenType::Slash => {
                    let op_span: Span = self.peek().unwrap().span;
                    self.advance();
                    let right: Expr = self.parse_unary();
                    let right_span: Span = right.span();
                    left = self.create_binary_op(
                        BinaryOp::new(BinopType::Div, op_span),
                        left,
                        right,
                        start_span.join(right_span),
                    );
                }
                _ => break,
            }
        }
        left
    }

    fn parse_term(&mut self) -> Expr {
        let mut left: Expr = self.parse_factor();
        let start_span: Span = left.span();
        while let Some(t) = self.peek() {
            match t.kind {
                TokenType::Plus => {
                    let op_span: Span = self.peek().unwrap().span;
                    self.advance();
                    let right: Expr = self.parse_factor();
                    let right_span: Span = right.span();
                    left = self.create_binary_op(
                        BinaryOp::new(BinopType::Add, op_span),
                        left,
                        right,
                        start_span.join(right_span),
                    );
                }
                TokenType::Minus => {
                    let op_span: Span = self.peek().unwrap().span;
                    self.advance();
                    let right: Expr = self.parse_factor();
                    let right_span: Span = right.span();
                    left = self.create_binary_op(
                        BinaryOp::new(BinopType::Sub, op_span),
                        left,
                        right,
                        start_span.join(right_span),
                    );
                }
                _ => break,
            }
        }
        left
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_term();
        let start_span = left.span();
        while let Some(t) = self.peek().cloned() {
            let op_span: Span = self.peek().unwrap().span;
            self.advance();
            let right = self.parse_term();
            let right_span = right.span();
            left = self.create_binary_op(
                BinaryOp::new(
                    match t.kind {
                        TokenType::GreaterEqual => BinopType::GreaterEq,
                        TokenType::Greater => BinopType::Greater,
                        TokenType::Lesser => BinopType::Lesser,
                        TokenType::LesserEqual => BinopType::LesserEq,
                        _ => break,
                    },
                    op_span,
                ),
                left,
                right,
                start_span.join(right_span),
            );
        }
        left
    }

    fn parse_equality(&mut self) -> Expr {
        let mut left = self.parse_comparison();
        let start_span = left.span();
        while let Some(t) = self.peek().cloned() {
            let op_span: Span = self.peek().unwrap().span;
            self.advance();
            let right = self.parse_comparison();
            let right_span = right.span();
            left = self.create_binary_op(
                BinaryOp::new(
                    match t.kind {
                        TokenType::EqualEqual => BinopType::EqEq,
                        TokenType::BangEqual => BinopType::BangEq,
                        _ => break,
                    },
                    op_span,
                ),
                left,
                right,
                start_span.join(right_span),
            );
        }
        left
    }

    fn parse_and(&mut self) -> Expr {
        let mut left = self.parse_equality();
        let start_span = left.span();
        while let Some(t) = self.peek().cloned() {
            let op_span = t.span;
            self.advance();
            let right = self.parse_equality();
            let right_span = right.span();
            left = self.create_binary_op(
                BinaryOp::new(
                    match t.kind {
                        TokenType::And => BinopType::And,
                        _ => break,
                    },
                    op_span,
                ),
                left,
                right,
                start_span.join(right_span),
            );
        }
        left
    }

    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();
        let start_span = left.span();
        while let Some(t) = self.peek().cloned() {
            let op_span = t.span;
            self.advance();
            let right = self.parse_and();
            let right_span = right.span();
            left = self.create_binary_op(
                BinaryOp::new(
                    match t.kind {
                        TokenType::Or => BinopType::Or,
                        _ => break,
                    },
                    op_span,
                ),
                left,
                right,
                start_span.join(right_span),
            );
        }
        left
    }

    fn parse_assignment(&mut self) -> Expr {
        let left = self.parse_or();

        if let Some(tok) = self.peek() {
            if tok.kind == TokenType::Equal {
                self.advance();

                let right = self.parse_assignment();

                let span = left.span().join(right.span());

                return self.create_var_assign(left, right, span);
            }
        }

        left
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_assignment()
    }

    fn parse_print(&mut self) -> Stmt {
        let start = self.peek().unwrap().clone();
        self.advance();

        if self.consume(TokenType::OpenParen, "expected '('").is_err() {
            return self.create_stmt_err(start.span);
        }

        let expr = self.parse_expression();
        let span = expr.span();
        let stmt = self.create_print_stmt(expr, start.span.join(span));

        if self.consume(TokenType::CloseParen, "expected ')'").is_err() {
            return self.create_stmt_err(start.span.join(span));
        }

        if self.consume(TokenType::Semicolon, "expected ';'").is_err() {
            return self.create_stmt_err(start.span.join(span));
        }

        stmt
    }

    fn parse_stmtexpr(&mut self) -> Stmt {
        let expr = self.parse_expression();
        let span = expr.span();
        let stmt = self.create_exprstmt(expr, span);

        if self.consume(TokenType::Semicolon, "expected ';'").is_err() {
            return self.create_stmt_err(span);
        }

        stmt
    }

    fn parse_var_decl(&mut self) -> Stmt {
        let start = self.peek().unwrap().clone();
        self.advance();

        let identifier = match self.consume(TokenType::Identifier, "expected identifier") {
            Ok(t) => self.get_slice(t.span).to_vec(),
            Err(_) => {
                return self.create_stmt_err(start.span);
            }
        };

        if self.peek().clone().is_none() {
            let fallback_span = self.get_fallback_span();
            self.error("unexpected end of input", fallback_span);
            return self.create_stmt_err(fallback_span);
        }

        // unwrap here because peek is guarenteed to return Some(t) as checked above
        match self.peek().unwrap().clone().kind {
            TokenType::Colon => {
                // advance past colon
                self.advance();
                let type_tok = self.peek().unwrap().clone();
                let type_span = type_tok.span;
                if !self.is_var_type(type_tok.kind) {
                    // any tokentype that is a type keyword for expected here will do
                    // because it is guarenteed here that type_tok is not a type keyword
                    let _ = self.consume(TokenType::Int, "expected type annotation");
                    return self.create_stmt_err(type_span);
                }

                let var_type = match type_tok.kind {
                    TokenType::Int => Type::Int,
                    TokenType::Bool => Type::Bool,
                    _ => {
                        unreachable!("no other types");
                    }
                };

                let eq_tok_span = match self.peek() {
                    Some(t) => t.span,
                    None => {
                        let fallback_span = self.get_fallback_span();
                        self.error("unexpected end of input", fallback_span);
                        return self.create_stmt_err(fallback_span);
                    }
                };

                self.advance();

                match self.peek().clone() {
                    Some(t) => match t.kind {
                        TokenType::Equal => {
                            self.advance();
                            let expr = self.parse_expression();
                            if self.consume(TokenType::Semicolon, "expected ';'").is_err() {
                                return self.create_stmt_err(eq_tok_span);
                            }
                            return self.create_var_decl(
                                identifier,
                                Some(var_type),
                                Some(expr),
                                start.span.join(eq_tok_span),
                            );
                        }
                        TokenType::Semicolon => {
                            self.advance();
                            return self.create_var_decl(
                                identifier,
                                Some(var_type),
                                None,
                                start.span.join(eq_tok_span),
                            );
                        }
                        _ => {
                            if self.consume(TokenType::Semicolon, "expected ';'").is_err() {
                                return self.create_stmt_err(eq_tok_span);
                            } else {
                                panic!("execution should not reach here");
                            }
                        }
                    },
                    None => {
                        let fallback_span = self.get_fallback_span();
                        self.error("unexpected end of input", fallback_span);
                        return self.create_stmt_err(fallback_span);
                    }
                }
            }
            TokenType::Equal => {
                // advance past equal
                self.advance();

                let init = self.parse_expression();
                let span = match self.peek() {
                    Some(t) => t.span,
                    None => {
                        let fallback_span = self.get_fallback_span();
                        self.error("unexpected end of input", fallback_span);
                        return self.create_stmt_err(fallback_span);
                    }
                };

                if self.consume(TokenType::Semicolon, "expected ';'").is_err() {
                    return self.create_stmt_err(span);
                }

                return self.create_var_decl(identifier, None, Some(init), span);
            }
            _ => {
                let span = self.peek().unwrap().span;
                let _ = self.consume(TokenType::Equal, "expected assignment expression");
                return  self.create_stmt_err(span);
            }
        }
    }

    fn parse_block(&mut self) -> Stmt {
        let start = self.peek().unwrap().clone();
        self.advance();

        let mut span = start.span;

        let mut stmts: Vec<Stmt> = Vec::new();

        while let Some(t) = self.peek().cloned() {
            if t.kind == TokenType::CloseBrace {
                break;
            }
            span = span.join(t.span);
            stmts.push(self.parse_stmt());
        }

        let close_brace_span = self.peek().unwrap().clone().span;

        if self
            .consume(TokenType::CloseBrace, "expected '}' at end of block")
            .is_err()
        {
            return self.create_stmt_err(close_brace_span);
        }

        self.create_block(stmts, span)
    }

    fn parse_stmt(&mut self) -> Stmt {
        let tok = self.peek().unwrap().clone();
        match tok.kind {
            TokenType::Print => self.parse_print(),
            TokenType::Let => self.parse_var_decl(),
            TokenType::IntegerLiteral | TokenType::OpenParen | TokenType::Identifier => {
                self.parse_stmtexpr()
            }
            TokenType::OpenBrace => self.parse_block(),
            _ => {
                self.error("unexpected token at statement start", tok.span);
                self.sync();
                self.create_stmt_err(tok.span)
            }
        }
    }

    pub fn generate_ast(&mut self) -> Program {
        let mut program = Program::new();
        while self.peek().is_some() {
            let stmt = self.parse_stmt();
            program.add(stmt);
        }

        program
    }
}
