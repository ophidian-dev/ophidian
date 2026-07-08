use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Int,
}

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
pub enum TypedExpr {
    IntegerLiteral {
        span: Span,
        ty: Type,
        value: i32,
    },
    BinaryOp {
        span: Span,
        op: BinaryOp,
        ty: Type,
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    UnaryOp {
        span: Span,
        ty: Type,
        op: UnaryOp,
        expr: Box<TypedExpr>,
    },
    Variable {
        name: Vec<u8>,
        ty: Type,
        span: Span,
    },
    VarAssign {
        target: Box<TypedExpr>,
        value: Box<TypedExpr>,
        ty: Type,
        span: Span,
    },
    Error {
        span: Span,
    },
}

impl TypedExpr {
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
pub enum TypedStmt {
    Print {
        expr: Box<TypedExpr>,
        span: Span,
    },
    StmtExpr {
        expr: Box<TypedExpr>,
        span: Span,
    },
    VarDecl {
        name: Vec<u8>,
        type_annotation: Type,
        initializer: Option<TypedExpr>,
        span: Span,
    },
    Error {
        span: Span,
    },
}

impl TypedStmt {
    pub fn span(&self) -> Span {
        match self {
            Self::Print { span, .. } => *span,
            Self::StmtExpr { span, .. } => *span,
            Self::Error { span } => *span,
            Self::VarDecl { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypedProgram {
    pub stmts: Vec<TypedStmt>,
}

impl TypedProgram {
    pub fn new() -> Self {
        Self { stmts: Vec::new() }
    }

    pub fn add(&mut self, stmt: TypedStmt) {
        self.stmts.push(stmt);
    }
}
