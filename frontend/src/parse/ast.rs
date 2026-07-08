use crate::semantic::typed::Type;
use crate::span::Span;

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
    Variable {
        name: Vec<u8>,
        span: Span,
    },
    VarAssign {
        target: Box<Expr>,
        value: Box<Expr>,
        span: Span,
    },
    Error {
        span: Span,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Self::IntegerLiteral { span, .. } => *span,
            Self::BinaryOp { span, .. } => *span,
            Self::UnaryOp { span, .. } => *span,
            Self::Error { span } => *span,
            Self::Variable { span, .. } => *span,
            Self::VarAssign { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print {
        expr: Box<Expr>,
        span: Span,
    },
    StmtExpr {
        expr: Box<Expr>,
        span: Span,
    },
    VarDecl {
        name: Vec<u8>,
        type_annotation: Type,
        initializer: Option<Expr>,
        span: Span,
    },
    Block {
        body: Vec<Stmt>,
        span: Span,
    },
    Error {
        span: Span,
    },
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Self::Print { span, .. } => *span,
            Self::StmtExpr { span, .. } => *span,
            Self::Error { span } => *span,
            Self::VarDecl { span, .. } => *span,
            Self::Block { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Self { stmts: Vec::new() }
    }

    pub fn add(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }
}
