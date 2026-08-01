use crate::span::{Span, Spanned};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(usize);

impl NodeId {
    pub const ERROR: usize = usize::MAX;
}

pub enum LitKind {
    Int(u128)
}

pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

pub type BinOp = Spanned<BinOpKind>;

pub enum ExprKind {
    // a literal
    // e.g. '1'
    Literal(LitKind),

    // a binary operation like `1 + 2`
    BinaryOp(BinOp, Box<Expr>, Box<Expr>),

}

// an expression
pub struct Expr {
    id: NodeId,
    kind: ExprKind,
    span: Span,
}