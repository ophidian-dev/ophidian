use crate::semantic::typed::*;
use crate::parse::ast as untyped;
use crate::span::Span;

pub fn create_integer_literal(value: i32, ty: Type, span: Span) -> Expr {
    Expr::IntegerLiteral { span, ty, value }
}

pub fn create_binary_op(op: BinaryOp, ty: Type, left: Expr, right: Expr, span: Span) -> Expr {
    Expr::BinaryOp { span, op, ty, left: Box::new(left), right: Box::new(right) }
}

pub fn binary_op_from_untyped(untyped_op: untyped::BinaryOp) -> BinaryOp {
    match untyped_op.kind {
        untyped::BinopType::Add => BinaryOp { kind: BinopType::Add, span: untyped_op.span },
        untyped::BinopType::Sub => BinaryOp { kind: BinopType::Sub, span: untyped_op.span },
        untyped::BinopType::Mul => BinaryOp { kind: BinopType::Mul, span: untyped_op.span },
        untyped::BinopType::Div => BinaryOp { kind: BinopType::Div, span: untyped_op.span }
    }
}

pub fn create_unary_op(op: UnaryOp, ty: Type, expr: Expr, span: Span) -> Expr {
    Expr::UnaryOp { span, ty, op, expr: Box::new(expr) }
}

pub fn unary_op_from_untyped(untyped_op: untyped::UnaryOp) -> UnaryOp {
    match untyped_op.kind {
        untyped::UnaryopType::Negate => UnaryOp { kind: UnaryopType::Negate, span: untyped_op.span }
    }
}

pub fn create_var_assign(target: Expr, value: Expr, ty: Type, span: Span) -> Expr {
    Expr::VarAssign { target: Box::new(target), value: Box::new(value), ty, span }
}

pub fn create_variable(name: Vec<u8>, ty: Type, span: Span) -> Expr {
    Expr::Variable { name, ty, span }
}

pub fn create_block(body: Vec<Stmt>, span: Span) -> Stmt {
    Stmt::Block { body, span } 
}

pub fn create_print(expr: Expr, span: Span) -> Stmt {
    Stmt::Print { expr: Box::new(expr), span }
}

pub fn create_stmtexpr(expr: Expr, span: Span) -> Stmt {
    Stmt::StmtExpr { expr: Box::new(expr), span }
}

pub fn create_var_decl(name: Vec<u8>, type_annotation: Type, initializer: Option<Expr>, span: Span) -> Stmt {
    Stmt::VarDecl { name, type_annotation, initializer, span }
}