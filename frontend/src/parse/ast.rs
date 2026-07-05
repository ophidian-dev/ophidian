use crate::parse::span::Span;

#[derive(Debug, Clone, Copy)]
pub enum BinopType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy)]
pub struct BinaryOp {
    pub kind: BinopType,
    pub span: Span,
}

impl BinaryOp {
    pub fn new(kind: BinopType, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryopType {
    Negate,
}

#[derive(Debug, Clone, Copy)]
pub struct UnaryOp {
    pub kind: UnaryopType,
    pub span: Span,
}

impl UnaryOp {
    pub fn new(kind: UnaryopType, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    IntegerLiteral {
        span: Span,
        value: i32,
    },
    BinaryOp {
        span: Span,
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        span: Span,
        op: UnaryOp,
        expr: Box<Expr>,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::IntegerLiteral { span, .. } => *span,
            Expr::BinaryOp { span, .. } => *span,
            Expr::UnaryOp { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print {
        expr: Box<Expr>,
        span: Span
    },
    StmtExpr {
        expr: Box<Expr>,
        span: Span
    }
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Print { span, .. } => {
                *span
            }
            Stmt::StmtExpr { span, .. } => {
                *span
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    stmts: Vec<Stmt>    
}

impl Program {
    pub fn new() -> Self {
        Self {
            stmts: Vec::new()
        }
    }

    pub fn add(&mut self, stmt: Stmt) {
        self.stmts.push(stmt); 
    }
}