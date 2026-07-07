use crate::parse::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::semantic::typed::Type;
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
    Stmt::StmtExpr {
        expr: Box::new(expr),
        span,
    }
}

pub fn create_print_stmt(expr: Expr, span: Span) -> Stmt {
    Stmt::Print {
        expr: Box::new(expr),
        span,
    }
}

pub fn create_var_decl(
    name: Vec<u8>,
    type_annotation: Type,
    initializer: Option<Expr>,
    span: Span,
) -> Stmt {
    Stmt::VarDecl {
        name,
        type_annotation,
        initializer,
        span,
    }
}

pub fn create_variable(name: Vec<u8>, span: Span) -> Expr {
    Expr::Variable { name, span }
}

pub fn create_var_assign(target: Expr, value: Expr, span: Span) -> Expr {
    Expr::VarAssign {
        target: Box::new(target),
        value: Box::new(value),
        span,
    }
}

pub fn create_block(body: Vec<Stmt>, span: Span) -> Stmt {
    Stmt::Block { body, span }
}
