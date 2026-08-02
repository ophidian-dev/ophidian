use crate::parser::node_id::NodeId;
use crate::span::{Span, Spanned};

#[derive(Debug, PartialEq)]
pub enum LitKind {
    Int(u128),
}

#[derive(Debug, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOpKind {
    Negate,
}

pub type BinOp = Spanned<BinOpKind>;

pub type UnaryOp = Spanned<UnaryOpKind>;

// all the different exprs in the language
#[derive(Debug, PartialEq)]
pub enum ExprKind {
    // a literal
    // e.g. '1'
    Literal(LitKind),

    // a binary operation like `1 + 2`
    BinaryOp(BinOp, Box<Expr>, Box<Expr>),

    // a unary operation like  `-1`
    UnaryOp(UnaryOp, Box<Expr>),
}

// an expression
#[derive(Debug, PartialEq)]
pub struct Expr {
    pub id: NodeId,
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub const fn new(id: NodeId, kind: ExprKind, span: Span) -> Self {
        Self { id, kind, span }
    }
}
