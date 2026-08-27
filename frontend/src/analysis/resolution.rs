use crate::analysis::analyzer::AnalysisCtx;
use crate::diagnostics::{Diagnostic, Severity};
use crate::parse::ast::{Expr, ExprKind, ForInit, Program, Stmt, StmtKind};
use crate::span::Span;
use std::collections::HashMap;

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

pub struct Resolver {
    curr_var_id: VarId,
    loop_depth: usize,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            curr_var_id: VarId::from(0),
            loop_depth: 0,
        }
    }

    pub fn resolve(&mut self, program: &Program, ctx: &mut AnalysisCtx) {
        self.enter_scope(ctx);

        for stmt in &program.decls {
            self.resolve_stmt(stmt, ctx);
        }

        self.exit_scope(ctx);
    }

    fn error<T: Into<String>>(&mut self, message: T, span: Span, ctx: &mut AnalysisCtx) {
        ctx.diagnostics
            .push(Diagnostic::new(message.into(), span, Severity::Error));
    }

    fn resolve_stmt(&mut self, stmt: &Stmt, ctx: &mut AnalysisCtx) {
        match &stmt.kind {
            StmtKind::VarDecl(name, .., init) => {
                self.resolve_vardecl(stmt, name, init, ctx);
            }
            StmtKind::Block(body) => {
                self.resolve_block(body, ctx);
            }
            StmtKind::ExprStmt(expr) => {
                self.resolve_exprstmt(expr, ctx);
            }
            StmtKind::Print(expr) => {
                self.resolve_print(expr, ctx);
            }
            StmtKind::If(cond, body, else_body) => {
                self.resolve_if(cond, body, else_body, ctx);
            }
            StmtKind::While(cond, body) => {
                self.resolve_while(cond, body, ctx);
            }
            StmtKind::For(init, cond, incre, body) => {
                self.resolve_for(init, cond, incre, body, ctx);
            }
            StmtKind::Break => {
                self.resolve_break(stmt, ctx);
            }
            StmtKind::Continue => {
                self.resolve_continue(stmt, ctx);
            }
            StmtKind::Return(expr) => {
                todo!()
            }
            StmtKind::Error => {
                unreachable!()
            }
        }
    }

    fn resolve_vardecl(
        &mut self,
        stmt: &Stmt,
        name: &[u8],
        init: &Option<Expr>,
        ctx: &mut AnalysisCtx,
    ) {
        if let Some(initializer) = init {
            self.resolve_expr(initializer, ctx);
        }

        let varid = self.declare_var(name, ctx, stmt.span);

        ctx.variables.insert(stmt.id, varid);
    }

    fn resolve_block(&mut self, body: &[Stmt], ctx: &mut AnalysisCtx) {
        self.enter_scope(ctx);

        for s in body {
            self.resolve_stmt(s, ctx);
        }

        self.exit_scope(ctx);
    }

    fn resolve_exprstmt(&mut self, expr: &Expr, ctx: &mut AnalysisCtx) {
        self.resolve_expr(expr, ctx);
    }

    fn resolve_print(&mut self, expr: &Expr, ctx: &mut AnalysisCtx) {
        self.resolve_expr(expr, ctx);
    }

    fn resolve_if(
        &mut self,
        cond: &Expr,
        body: &Stmt,
        else_body: &Option<Box<Stmt>>,
        ctx: &mut AnalysisCtx,
    ) {
        self.resolve_expr(cond, ctx);
        self.resolve_stmt(body, ctx);
        if let Some(e) = else_body {
            self.resolve_stmt(e, ctx);
        }
    }

    fn resolve_while(&mut self, cond: &Expr, body: &Stmt, ctx: &mut AnalysisCtx) {
        self.resolve_expr(cond, ctx);

        self.loop_depth += 1;
        self.resolve_stmt(body, ctx);
        self.loop_depth -= 1;
    }

    fn resolve_for(
        &mut self,
        init: &Option<Box<ForInit>>,
        cond: &Option<Expr>,
        incre: &Option<Expr>,
        body: &Stmt,
        ctx: &mut AnalysisCtx,
    ) {
        self.enter_scope(ctx);

        match init {
            Some(init) => match &**init {
                ForInit::Expr(expr) => {
                    self.resolve_expr(&expr, ctx);
                }
                ForInit::Decl(decl) => {
                    self.resolve_stmt(&decl, ctx);
                }
            },
            None => {
                // no name resolution needed here
            }
        }

        match cond {
            Some(cond) => {
                self.resolve_expr(cond, ctx);
            }
            None => {
                // no name resolution needed here
            }
        }

        match incre {
            Some(incre) => {
                self.resolve_expr(incre, ctx);
            }
            None => {
                // no name resolution needed here
            }
        }

        self.loop_depth += 1;

        self.resolve_stmt(body, ctx);

        self.loop_depth -= 1;

        self.exit_scope(ctx);
    }

    fn resolve_break(&mut self, stmt: &Stmt, ctx: &mut AnalysisCtx) {
        if self.loop_depth == 0 {
            self.error("use of 'break' statement outside loop", stmt.span, ctx);
        }
    }

    fn resolve_continue(&mut self, stmt: &Stmt, ctx: &mut AnalysisCtx) {
        if self.loop_depth == 0 {
            self.error("use of continue statement outside loop", stmt.span, ctx);
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
                        ctx,
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
            self.error(
                format!(
                    "redeclaration of identifer '{}'",
                    std::str::from_utf8(name).unwrap()
                ),
                span,
                ctx,
            );
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
