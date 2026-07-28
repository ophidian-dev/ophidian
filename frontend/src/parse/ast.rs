use crate::semantic::typed::Type;
use crate::span::Span;

// NodeId is a unique way to identify each untyped ast node
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub usize);

// defines all the possible binary operations
#[derive(Debug, Clone, Copy)]
pub enum BinopType {
    Add,
    Sub,
    Mul,
    Div,
    Or,
    And,
    EqEq,
    BangEq,
    Lesser,
    Greater,
    LesserEq,
    GreaterEq,
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

// defines all the possible unary operations
#[derive(Debug, Clone, Copy)]
pub enum UnaryopType {
    Negate,
    Not,
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
        id: NodeId,
        value: i32,
    },
    BooleanLiteral {
        span: Span,
        value: bool,
        id: NodeId,
    },
    BinaryOp {
        span: Span,
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
        id: NodeId,
    },
    UnaryOp {
        span: Span,
        op: UnaryOp,
        expr: Box<Expr>,
        id: NodeId,
    },
    Variable {
        name: Vec<u8>,
        span: Span,
        id: NodeId,
    },
    VarAssign {
        target: Box<Expr>,
        value: Box<Expr>,
        span: Span,
        id: NodeId
    },
    Error {
        span: Span,
        id: NodeId,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Self::IntegerLiteral { span, .. } => *span,
            Self::BinaryOp { span, .. } => *span,
            Self::UnaryOp { span, .. } => *span,
            Self::Error { span, .. } => *span,
            Self::Variable { span, .. } => *span,
            Self::VarAssign { span, .. } => *span,
            Self::BooleanLiteral { span, .. } => *span,
        }
    }

    pub fn id(&self) -> NodeId {
        match self {
            Self::BinaryOp { id, .. } => *id,
            Self::BooleanLiteral { id, .. } => *id,
            Self::Error { id, .. } => *id,
            Self::IntegerLiteral { id, .. } => *id,
            Self::UnaryOp { id, .. } => *id,
            Self::VarAssign { id, .. } => *id,
            Self::Variable { id, .. } => *id,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print {
        expr: Box<Expr>,
        span: Span,
        id: NodeId,
    },
    StmtExpr {
        expr: Box<Expr>,
        span: Span,
        id: NodeId,
    },
    VarDecl {
        name: Vec<u8>,
        type_annotation: Option<Type>,
        initializer: Option<Expr>,
        span: Span,
        id: NodeId,
    },
    Block {
        body: Vec<Stmt>,
        span: Span,
        id: NodeId,
    },
    Error {
        span: Span,
        id: NodeId,
    },
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Self::Print { span, .. } => *span,
            Self::StmtExpr { span, .. } => *span,
            Self::Error { span , ..} => *span,
            Self::VarDecl { span, .. } => *span,
            Self::Block { span, .. } => *span,
        }
    }

    pub fn id(&self) -> NodeId {
        match self {
            Self::Block { id, .. } => *id,
            Self::Error { id, .. } => *id,
            Self::Print { id, .. } => *id,
            Self::StmtExpr { id, .. } => *id,
            Self::VarDecl { id, .. } => *id,
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
