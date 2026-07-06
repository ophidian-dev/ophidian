use crate::lex::lexer::Lexer;
use crate::lex::token::{Token, TokenType};
use crate::parse::ast::{BinaryOp, BinopType, Expr, Program, Stmt, UnaryOp, UnaryopType};
use crate::parse::ctors;
use crate::parse::span::Span;
use owo_colors::OwoColorize;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>,
    errors: Vec<ParseError>,
}

#[derive(Debug)]
pub enum ParseError {
    // parser encountered a token it did not expect
    UnexpectedToken(Token),
    // when the parser encounters end of file prematurely
    UnexpectedEof,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current: Option<Token> = lexer.next();
        Self {
            lexer,
            current,
            errors: Vec::new(),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.current.as_ref()
    }

    fn advance(&mut self) {
        self.current = self.lexer.next();
    }

    fn get_slice(&self, span: Span) -> &[u8] {
        &self.lexer.source[span.offset()..span.end()]
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

    fn expect(&mut self, expected: TokenType) -> Result<Token, ParseError> {
        match self.peek() {
            Some(tok) if tok.kind == expected => {
                let tok = *tok;
                self.advance();
                Ok(tok)
            }
            Some(tok) => Err(ParseError::UnexpectedToken(*tok)),
            None => Err(ParseError::UnexpectedEof),
        }
    }

    fn error(&self, tok: Token, span: Span, msg: &str) {
        eprintln!(
            "{}{}{}{} {} {}",
            (tok.line + 1).bold(),
            ":".bold(),
            (tok.column + 1).bold(),
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

        // we unwrap here because we trust the caller to pass in a file containing only valid ascii
        let s = std::str::from_utf8(bytes).unwrap();
        eprintln!("{}", s);

        for _ in 1..count {
            eprint!(" ");
        }
        eprint!("  | ");
        for i in 0..span.end() {
            if i > span.offset() {
                eprint!("{}", "^".green());
            }
        }
        eprint!("\n");
    }

    fn parse_primary(&mut self) -> Expr {
        if let Some(t) = self.peek() {
            match t.kind {
                TokenType::IntegerLiteral => {
                    // we unwrap here because peek is guarenteed to return Some(&Token)
                    let tok: Token = self.peek().unwrap().clone();
                    self.advance();
                    let value: &[u8] = self.get_slice(tok.span);
                    let s: &str = std::str::from_utf8(value).unwrap();
                    // we unwrap the value here because the string is guarenteed to be a valid number
                    // since the lexer lexes numeric literals
                    let n: i32 = s.parse().unwrap();
                    return ctors::create_integer_literal(n, tok.span);
                }
                TokenType::OpenParen => {
                    self.advance();
                    let expr: Expr = self.parse_expression();
                    let tok: Token = self.peek().unwrap().clone();
                    if let Err(e) = self.expect(TokenType::CloseParen) {
                        self.errors.push(e);
                        self.error(tok, tok.span, "expected ')'");
                    }
                    return expr;
                }
                _ => {
                    eprintln!("offending token: {:?}", t);
                    todo!("handle error");
                }
            }
        } else {
            panic!("expected end of input");
        }
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek() {
            Some(t) if t.kind == TokenType::Minus => {
                let tok: Token = self.peek().unwrap().clone();
                self.advance();
                let right: Expr = self.parse_unary();
                let right_span: Span = right.span();

                return ctors::create_unary_op(
                    UnaryOp::new(UnaryopType::Negate, tok.span),
                    right,
                    tok.span.join(right_span),
                );
            }
            Some(_) => {
                return self.parse_primary();
            }
            None => {
                panic!("unexpected end of input");
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
                _ => {
                    break;
                }
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
                _ => {
                    break;
                }
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
        if let Err(e) = self.expect(TokenType::OpenParen) {
            self.errors.push(e);
            let tok = self.peek().unwrap();
            self.error(*tok, tok.span, "expected '('");
        }


        let expr = self.parse_expression();
        let span = expr.span();
        let stmt = ctors::create_print_stmt(expr, start.span.join(span));

        if let Err(e) = self.expect(TokenType::CloseParen) {
            self.errors.push(e);
            let tok = self.peek().unwrap();
            self.error(*tok, tok.span, "expected ')'");
        }

        if let Err(e) = self.expect(TokenType::Semicolon) {
            self.errors.push(e);
            let tok = self.peek().unwrap();
            self.error(*tok, tok.span, "expected ';'");
        }

        stmt
    }

    fn parse_stmtexpr(&mut self) -> Stmt {
        let expr = self.parse_expression();
        let span = expr.span();
        let stmt = ctors::create_exprstmt(expr, span);
        if let Err(e) = self.expect(TokenType::Semicolon) {
            self.errors.push(e);
            let tok = self.peek();
            match tok {
                Some(t) => {
                    self.error(*t, t.span, "expected ';'");
                }
                None => {
                    panic!("unexpected eof")
                }
            }
        }
        stmt
    }

    fn parse_stmt(&mut self) -> Stmt {
        // we unwrap because caller guarentees that peek returns Some
        let tok = self.peek().unwrap();
        match tok.kind {
            TokenType::Print => self.parse_print(),
            TokenType::IntegerLiteral | TokenType::OpenParen => self.parse_stmtexpr(),
            _ => {
                self.error(*tok, tok.span, "Unexpected token at statement start");
                panic!("idk how to handle this error");
            }
        }
    }

    pub fn generate_ast(&mut self) -> Program {
        let mut program = Program::new();
        while let Some(_) = self.peek() {
            let stmt = self.parse_stmt();
            program.add(stmt);
        }

        println!();
        if self.errors.len() == 1 {
            eprintln!("{} error generated.", self.errors.len());
        } else if self.errors.len() > 1 {
            eprintln!("{} errors generated.", self.errors.len());
        }
        if self.errors.len() > 0 {
            std::process::exit(1);
        }


        program
    }
}
