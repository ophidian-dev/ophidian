use crate::lex::lexer::Lexer;
use crate::lex::token::{Token, TokenType};
use crate::parse::ast::{BinaryOp, BinopType, Expr, Program, Stmt, UnaryOp, UnaryopType};
use crate::parse::ctors;
use crate::parse::span::Span;
use owo_colors::OwoColorize;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>,
    previous: Option<Token>,
    errors: Vec<ParseError>,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEof,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current: Option<Token> = lexer.next();
        Self {
            lexer,
            current,
            previous: None,
            errors: Vec::new(),
        }
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
                self.errors.push(ParseError::UnexpectedToken(tok));

                // FIX: If a token is missing, anchor the error to the END of the previous token
                if let Some(prev) = &self.previous {
                    self.error(*prev, prev.span, error_msg);
                } else {
                    self.error(tok, tok.span, error_msg);
                }

                self.sync();
                Err(())
            }
            None => {
                self.errors.push(ParseError::UnexpectedEof);

                // FIX: Handle EOF gracefully by pointing right after the final token in the file
                if let Some(prev) = &self.previous {
                    self.error(*prev, prev.span, error_msg);
                } else {
                    // Extreme fallback if the file is completely empty
                    let fallback = Token {
                        kind: TokenType::Semicolon,
                        span: Span::new(0, 0),
                        line: 0,
                        column: 0,
                    };
                    self.error(fallback, fallback.span, error_msg);
                }

                self.sync();
                Err(())
            }
        }
    }

    fn sync(&mut self) {
        while let Some(tok) = self.peek() {
            match tok.kind {
                TokenType::Semicolon => {
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

    fn get_line(&self, tok: Token) -> &[u8] {
        let mut start = tok.span.offset();
        while start > 0 && self.lexer.source[start - 1] != b'\n' {
            start -= 1;
        }

        let mut end = tok.span.offset();
        while end < self.lexer.source.len() && self.lexer.source[end] != b'\n' {
            end += 1;
        }
        &self.lexer.source[start..end]
    }

    fn get_line_bounds(&self, tok: Token) -> (usize, usize) {
        let mut start = tok.span.offset();
        while start > 0 && self.lexer.source[start - 1] != b'\n' {
            start -= 1;
        }

        let mut end = tok.span.offset();
        while end < self.lexer.source.len() && self.lexer.source[end] != b'\n' {
            end += 1;
        }

        (start, end)
    }

    fn error(&self, tok: Token, span: Span, msg: &str) {
        let (line_start, line_end) = self.get_line_bounds(tok);

        let (caret_start, caret_end, display_column) = if msg.starts_with("expected") {
            let start = span.end();
            let end = span.end() + 1;
            let col = start - line_start + 1;
            (start, end, col)
        } else {
            (span.offset(), span.end(), tok.column + 1)
        };

        eprintln!(
            "{}{}{}{} {} {}",
            (tok.line + 1).bold(),
            ":".bold(),
            display_column.bold(),
            ": ".bold(),
            "error:".bright_red().bold(),
            msg.bold()
        );

        let count: usize = tok.line.to_string().len();
        for _ in 1..count {
            eprint!(" ");
        }
        eprintln!("  |");
        eprint!("{} | ", tok.line + 1);
        let bytes: &[u8] = self.get_line(tok);

        let s = std::str::from_utf8(bytes).unwrap_or("<invalid utf8>");
        eprintln!("{}", s);

        for _ in 1..count {
            eprint!(" ");
        }
        eprint!("  | ");

        for i in line_start..std::cmp::max(line_end, caret_end) {
            if i >= caret_start && i < caret_end {
                eprint!("{}", "^".green());
            } else {
                eprint!(" ");
            }
        }

        eprintln!();
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
                    ctors::create_integer_literal(n, tok.span)
                }
                TokenType::OpenParen => {
                    self.advance();
                    let expr: Expr = self.parse_expression();
                    if self.consume(TokenType::CloseParen, "expected ')'").is_err() {
                        return Expr::Error { span: expr.span() };
                    }
                    expr
                }
                _ => {
                    let tok = t.clone();
                    self.errors.push(ParseError::UnexpectedToken(tok));
                    self.error(tok, tok.span, "unexpected token");
                    self.sync();
                    Expr::Error { span: tok.span }
                }
            }
        } else {
            // Safe recovery point if expression context hits EOF prematurely
            let fallback_span = self
                .previous
                .as_ref()
                .map(|t| t.span)
                .unwrap_or_else(|| Span::new(0, 0));
            self.errors.push(ParseError::UnexpectedEof);
            eprintln!(
                "{}: error: unexpected end of input",
                "error".bright_red().bold()
            );
            Expr::Error {
                span: fallback_span,
            }
        }
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek() {
            Some(t) if t.kind == TokenType::Minus => {
                let tok: Token = self.peek().unwrap().clone();
                self.advance();
                let right: Expr = self.parse_unary();
                let right_span: Span = right.span();

                ctors::create_unary_op(
                    UnaryOp::new(UnaryopType::Negate, tok.span),
                    right,
                    tok.span.join(right_span),
                )
            }
            Some(_) => self.parse_primary(),
            None => {
                // Safe recovery point instead of panic
                let fallback_span = self
                    .previous
                    .as_ref()
                    .map(|t| t.span)
                    .unwrap_or_else(|| Span::new(0, 0));
                self.errors.push(ParseError::UnexpectedEof);
                eprintln!(
                    "{}: error: unexpected end of input",
                    "error".bright_red().bold()
                );
                Expr::Error {
                    span: fallback_span,
                }
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
                    left = ctors::create_binary_op(
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
                    left = ctors::create_binary_op(
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
                    left = ctors::create_binary_op(
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
                    left = ctors::create_binary_op(
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

    fn parse_expression(&mut self) -> Expr {
        self.parse_term()
    }

    fn parse_print(&mut self) -> Stmt {
        let start = self.peek().unwrap().clone();
        self.advance();

        if self.consume(TokenType::OpenParen, "expected '('").is_err() {
            return Stmt::Error { span: start.span };
        }

        let expr = self.parse_expression();
        let span = expr.span();
        let stmt = ctors::create_print_stmt(expr, start.span.join(span));

        if self.consume(TokenType::CloseParen, "expected ')'").is_err() {
            return Stmt::Error {
                span: start.span.join(span),
            };
        }

        if self.consume(TokenType::Semicolon, "expected ';'").is_err() {
            return Stmt::Error {
                span: start.span.join(span),
            };
        }

        stmt
    }

    fn parse_stmtexpr(&mut self) -> Stmt {
        let expr = self.parse_expression();
        let span = expr.span();
        let stmt = ctors::create_exprstmt(expr, span);

        if self.consume(TokenType::Semicolon, "expected ';'").is_err() {
            return Stmt::Error { span };
        }

        stmt
    }

    fn parse_stmt(&mut self) -> Stmt {
        let tok = self.peek().unwrap().clone();
        match tok.kind {
            TokenType::Print => self.parse_print(),
            TokenType::IntegerLiteral | TokenType::OpenParen => self.parse_stmtexpr(),
            _ => {
                self.error(tok, tok.span, "Unexpected token at statement start");
                self.errors.push(ParseError::UnexpectedToken(tok));
                self.sync();
                Stmt::Error { span: tok.span }
            }
        }
    }

    pub fn generate_ast(&mut self) -> Program {
        let mut program = Program::new();
        while self.peek().is_some() {
            let stmt = self.parse_stmt();
            program.add(stmt);
        }

        println!();
        if self.errors.len() == 1 {
            eprintln!("{} error generated.", self.errors.len());
        } else if self.errors.len() > 1 {
            eprintln!("{} errors generated.", self.errors.len());
        }

        if !self.errors.is_empty() {
            std::process::exit(1);
        }

        program
    }
}
