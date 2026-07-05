use crate::parse::ast::{BinaryOp, Expr, UnaryOp, Stmt};
use crate::parse::span::Span;

pub fn create_integer_literal(value: i32, span: Span) -> Expr {
    Expr::IntegerLiteral { value, span }
}

pub fn create_binary_op(op: BinaryOp, left: Expr, right: Expr, span: Span) -> Expr {
    Expr::BinaryOp {
        op,
        left: Box::new(left),
        right: Box::new(right),
        span,
    }
}

pub fn create_unary_op(op: UnaryOp, expr: Expr, span: Span) -> Expr {
    Expr::UnaryOp {
        op,
        expr: Box::new(expr),
        span,
    }
}

pub fn create_exprstmt(expr: Expr, span: Span) -> Stmt {
    Stmt::StmtExpr { expr: Box::new(expr), span }
}

pub fn create_print_stmt(expr: Expr, span: Span) -> Stmt {
    Stmt::Print { expr: Box::new(expr), span }
}
