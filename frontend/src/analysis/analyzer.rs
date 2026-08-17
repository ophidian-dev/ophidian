use crate::parse::ast::{Expr, ExprKind, NodeId, Program, Stmt, StmtKind};
use crate::diagnostics::Diagnostic;
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
    pub const ERROR: usize = usize::MAX;
}

impl From<usize> for VarId {
    fn from(value: usize) -> Self {
        VarId(value)
    }
}

impl std::ops::AddAssign for VarId {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
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
        Self {
            diagnostics,
        }
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
    fn new(diagnostics: &'diag mut Vec<Diagnostic>) -> Self {
        Self {
            curr_var_id: VarId::from(0),
            diagnostics,
        }
    }
    fn resolve(&mut self, program: &Program, ctx: &mut AnalysisCtx) {
        self.enter_scope(ctx);

        for stmt in &program.body {
            self.resolve_stmt(stmt, ctx);
        }

        self.exit_scope(ctx);
    }

    fn resolve_stmt(&mut self, stmt: &Stmt, ctx: &mut AnalysisCtx) {
        match &stmt.kind {
            StmtKind::VarDecl(name, .., init) => {
                if let Some(initializer) = init {
                    self.resolve_expr(initializer, ctx);
                }

                let varid = self.declare_var(name, ctx);

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
            ExprKind::VarAssign(target, expr) => {}
            ExprKind::Variable(name) => {
                match self.lookup_var(name, ctx) {
                    Some(id) => {

                    }
                    None => {

                    }
                }
            }
            _ => return,
        }
    }

    fn declare_var(&mut self, name: &[u8], ctx: &mut AnalysisCtx) -> VarId {
        let id = self.curr_var_id;
        ctx.scopes
            .last_mut()
            .unwrap()
            .vars
            .insert(name.to_vec(), id);
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
}

impl AnalysisCtx {
    fn new() -> Self {
        Self {
            scopes: Vec::new(),
            types: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}

pub struct AnalysisResult {
    types: HashMap<NodeId, Type>,
    variables: HashMap<NodeId, VarId>,
}

impl From<AnalysisCtx> for AnalysisResult {
    fn from(value: AnalysisCtx) -> Self {
        Self {
            variables: value.variables,
            types: value.types,
        }
    }
}
