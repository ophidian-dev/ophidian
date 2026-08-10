use crate::parse::node_id::NodeId;
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

#[derive(Debug, PartialEq)]
pub enum StmtKind {
    // because right now this language does not have functions,
    // we have a built in print statement that is still called like a
    // function i.e. `print(expression);` requiring the parentheses
    Print(Box<Expr>),

    // expression statement
    ExprStmt(Box<Expr>),

    // an error node to represent a recoverable error this node exists
    // so that the parser can recover from a parsing function. Before the
    // semantic analysis phase begins, if the parser has any error nodes
    // then the driver will simply print the diagnostics and exit because
    // there is no point in analysing an ast that is half formed.
    Error,
}

#[derive(Debug, PartialEq)]
pub struct Stmt {
    pub id: NodeId,
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub const fn new(id: NodeId, kind: StmtKind, span: Span) -> Self {
        Self { id, kind, span }
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub body: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }

    pub fn add(&mut self, stmt: Stmt) {
        self.body.push(stmt);
    }

    #[deprecated]
    pub fn stmts(&self) -> &[Stmt] {
        &self.body
    }
}
