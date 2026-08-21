use crate::analysis::analyzer::Type;
use crate::lex::token::{Token, TokenKind};
use crate::span::{Span, Spanned};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub usize);

impl NodeId {
    pub const ERROR: Self = Self(usize::MAX);

    pub fn increment(&mut self) {
        *self += 1
    }
}

impl std::ops::AddAssign<usize> for NodeId {
    fn add_assign(&mut self, rhs: usize) {
        if self.0 + rhs > usize::MAX {
            self.0 = usize::MAX;
        } else {
            self.0 += rhs;
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LitKind {
    Int(u128),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,

    EqEq,
    BangEq,
    LessThan,
    GreaterThan,
    LessEq,
    GreaterEq,
}

impl From<Token> for BinOpKind {
    fn from(value: Token) -> Self {
        match value.kind {
            TokenKind::Plus => Self::Add,
            TokenKind::Minus => Self::Sub,
            TokenKind::Star => Self::Mul,
            TokenKind::Slash => Self::Div,
            TokenKind::EqualEqual => Self::EqEq,
            TokenKind::BangEqual => Self::BangEq,
            TokenKind::LessThan => Self::LessThan,
            TokenKind::GreaterThan => Self::GreaterThan,
            TokenKind::LessEq => Self::LessEq,
            TokenKind::GreaterEq => Self::GreaterEq,
            _ => panic!("not a binary operation"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

    // usage of a variable in an expression
    // e.g. print(x);
    //            ^
    // Vec<u8> being the name of the identifier
    Variable(Vec<u8>),

    // a variable assignment
    // e.g. x = 5;
    // the expression evaluates to the value that was assigned
    // the first box being the target and the second one being the value
    // that was assigned
    VarAssign(Box<Expr>, Box<Expr>),

    // this error node represents a recoverable error. this
    // node exists so that when the parser encounters an error
    // e.g. an unexpected token, it can create an error node, return it
    // from the parsing function and continue parsing. However, before
    // semantic analysis begins, if the parser has accumulated any error
    // nodes, then complilation wont continue because theres simply no point
    // in analysing and compiling a half broken ast. we will simply print the
    // parser diagnostics and exit
    Error,
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

    // a variable declaration
    //
    // Vec<u8> is the name of the identifier being declared
    // Option<Type> is optional type annotation
    // Option<Expr> is an optional init expression to initialise the variable
    VarDecl(Vec<u8>, Option<Type>, Option<Expr>),

    // a block (scope)
    // opened with a '{' and closed with '}'
    Block(Vec<Stmt>),

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
}
