use crate::diagnostics::{Diagnostic, Severity};
use crate::parse::ast::{Expr, ExprKind, NodeId, Program, Stmt, StmtKind};
use crate::span::Span;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,

    // The type that allows analysis to continue if it
    // encounters an error
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarId(pub usize);

impl VarId {
    pub const ERROR: Self = Self(usize::MAX);
}

impl From<usize> for VarId {
    fn from(value: usize) -> Self {
        VarId(value)
    }
}

impl std::ops::AddAssign<usize> for VarId {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs
    }
}

pub struct SemanticAnalyzer<'diag> {
    diagnostics: &'diag mut Vec<Diagnostic>,
}

pub struct Scope {
    vars: HashMap<Vec<u8>, VarId>,
}

impl Scope {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
}

impl<'diag> SemanticAnalyzer<'diag> {
    pub fn new(diagnostics: &'diag mut Vec<Diagnostic>) -> Self {
        Self { diagnostics }
    }

    pub fn analyze(&mut self, program: &Program) -> AnalysisResult {
        let mut ctx = AnalysisCtx::new();

        let mut resolver = Resolver::new(self.diagnostics);
        resolver.resolve(program, &mut ctx);

        AnalysisResult::from(ctx)
    }
}

struct Resolver<'diag> {
    curr_var_id: VarId,
    diagnostics: &'diag mut Vec<Diagnostic>,
}

impl<'diag> Resolver<'diag> {
    pub fn new(diagnostics: &'diag mut Vec<Diagnostic>) -> Self {
        Self {
            curr_var_id: VarId::from(0),
            diagnostics,
        }
    }

    pub fn resolve(&mut self, program: &Program, ctx: &mut AnalysisCtx) {
        self.enter_scope(ctx);

        for stmt in &program.body {
            self.resolve_stmt(stmt, ctx);
        }

        self.exit_scope(ctx);
    }

    fn error<T: Into<String>>(&mut self, message: T, span: Span) {
        self.diagnostics
            .push(Diagnostic::new(message.into(), span, Severity::Error));
    }

    fn resolve_stmt(&mut self, stmt: &Stmt, ctx: &mut AnalysisCtx) {
        match &stmt.kind {
            StmtKind::VarDecl(name, .., init) => {
                if let Some(initializer) = init {
                    self.resolve_expr(initializer, ctx);
                }

                let varid = self.declare_var(name, ctx, stmt.span);

                ctx.variables.insert(stmt.id, varid);
            }
            StmtKind::Block(body) => {
                self.enter_scope(ctx);

                for s in body {
                    self.resolve_stmt(s, ctx);
                }

                self.exit_scope(ctx);
            }
            StmtKind::ExprStmt(expr) => {
                self.resolve_expr(expr, ctx);
            }
            StmtKind::Print(expr) => {
                self.resolve_expr(expr, ctx);
            }
            StmtKind::Error => {
                unreachable!()
            }
        }
    }

    fn resolve_expr(&mut self, expr: &Expr, ctx: &mut AnalysisCtx) {
        match &expr.kind {
            ExprKind::VarAssign(target, expr) => {
                self.resolve_expr(expr, ctx);
                self.resolve_expr(target, ctx);
            }
            ExprKind::Variable(name) => match self.lookup_var(name, ctx) {
                Some(id) => {
                    ctx.variables.insert(expr.id, id);
                }
                None => {
                    self.error(
                        format!(
                            "use of undeclared identifer: '{}'",
                            std::str::from_utf8(name).unwrap()
                        ),
                        expr.span,
                    );
                }
            },
            ExprKind::BinaryOp(.., left, right) => {
                self.resolve_expr(left, ctx);
                self.resolve_expr(right, ctx);
            }
            ExprKind::UnaryOp(.., right) => {
                self.resolve_expr(right, ctx);
            }
            _ => return,
        }
    }

    fn declare_var(&mut self, name: &[u8], ctx: &mut AnalysisCtx, span: Span) -> VarId {
        if ctx.scopes.last().unwrap().vars.contains_key(name) {
            self.error(format!("redeclaration of identifer '{}'", std::str::from_utf8(name).unwrap()), span);
            return VarId::ERROR;
        }

        let id = self.curr_var_id;
        ctx.scopes
            .last_mut()
            .unwrap()
            .vars
            .insert(name.to_vec(), id);
        self.curr_var_id += 1;
        id
    }

    fn lookup_var(&mut self, name: &[u8], ctx: &mut AnalysisCtx) -> Option<VarId> {
        ctx.scopes
            .iter()
            .rev()
            .find_map(|s| s.vars.get(name).copied())
    }

    fn enter_scope(&self, ctx: &mut AnalysisCtx) {
        ctx.scopes.push(Scope::new());
    }

    fn exit_scope(&self, ctx: &mut AnalysisCtx) {
        ctx.scopes.pop();
    }
}

struct AnalysisCtx {
    scopes: Vec<Scope>,
    types: HashMap<NodeId, Type>,
    variables: HashMap<NodeId, VarId>,
    var_types: HashMap<VarId, Type>,
}

impl AnalysisCtx {
    fn new() -> Self {
        Self {
            scopes: Vec::new(),
            types: HashMap::new(),
            variables: HashMap::new(),
            var_types: HashMap::new(),
        }
    }
}

pub struct AnalysisResult {
    pub types: HashMap<NodeId, Type>,
    pub variables: HashMap<NodeId, VarId>,
    pub var_types: HashMap<VarId, Type>,
}

impl From<AnalysisCtx> for AnalysisResult {
    fn from(value: AnalysisCtx) -> Self {
        Self {
            variables: value.variables,
            types: value.types,
            var_types: value.var_types,
        }
    }
}
