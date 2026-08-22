use crate::diagnostics::{Diagnostic, Severity};
use crate::parse::ast::{
    BinOpKind, Expr, ExprKind, LitKind, NodeId, Program, Stmt, StmtKind, UnaryOpKind,
};
use crate::span::Span;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,

    Bool,

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

    pub fn analyze(&mut self, program: &Program) -> Result<AnalysisResult, ()> {
        let mut ctx = AnalysisCtx::new();

        let mut resolver = Resolver::new(self.diagnostics);
        resolver.resolve(program, &mut ctx);

        if !self.diagnostics.is_empty() {
            return Err(());
        }

        let mut typechecker = TypeChecker::new(self.diagnostics);
        typechecker.check(program, &mut ctx);

        Ok(AnalysisResult::from(ctx))
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
            StmtKind::If(cond, body, else_body) => {
                self.resolve_expr(cond, ctx);
                self.resolve_stmt(body, ctx);
                if let Some(e) = else_body {
                    self.resolve_stmt(e, ctx);
                }
            }
            StmtKind::While(cond, body) => {
                self.resolve_expr(cond, ctx);
                self.resolve_stmt(body, ctx);
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
            self.error(
                format!(
                    "redeclaration of identifer '{}'",
                    std::str::from_utf8(name).unwrap()
                ),
                span,
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

struct TypeChecker<'diag> {
    diagnostics: &'diag mut Vec<Diagnostic>,
}

impl<'diag> TypeChecker<'diag> {
    pub fn new(diagnostics: &'diag mut Vec<Diagnostic>) -> Self {
        Self { diagnostics }
    }

    pub fn check(&mut self, program: &Program, ctx: &mut AnalysisCtx) {
        for stmt in &program.body {
            self.check_stmt(stmt, ctx);
        }
    }

    fn error<T: Into<String>>(&mut self, message: T, span: Span) {
        self.diagnostics
            .push(Diagnostic::new(message.into(), span, Severity::Error));
    }

    fn check_stmt(&mut self, stmt: &Stmt, ctx: &mut AnalysisCtx) {
        match &stmt.kind {
            StmtKind::Block(body) => {
                for stmt in body {
                    self.check_stmt(stmt, ctx);
                }
            }
            StmtKind::ExprStmt(expr) => {
                self.check_expr(expr, ctx);
            }
            StmtKind::Print(expr) => {
                let expr_type = self.check_expr(expr, ctx);

                // builtin checking for the types that print supports
                // because print is not a function yet
                match expr_type {
                    Type::Error => {
                        return;
                    }
                    Type::Int | Type::Bool => {}
                }
            }
            StmtKind::VarDecl(.., type_annotation, initializer) => {
                match type_annotation {
                    Some(annotation) => {
                        // unwrap because name resolution has already checked
                        let varid = *ctx.variables.get(&stmt.id).unwrap();

                        match initializer {
                            Some(init) => {
                                let initializer_type = self.check_expr(init, ctx);

                                if !self.can_assign(initializer_type, *annotation) {
                                    self.error("mismatched types", stmt.span);
                                }

                                ctx.var_types.insert(varid, *annotation);
                            }
                            None => {
                                ctx.var_types.insert(varid, *annotation);
                            }
                        }
                    }
                    None => match initializer {
                        Some(init) => {
                            let initializer_type = self.check_expr(init, ctx);

                            let varid = *ctx.variables.get(&stmt.id).unwrap();
                            ctx.var_types.insert(varid, initializer_type);
                        }
                        None => {
                            self.error("type annotation required", stmt.span);
                        }
                    },
                }
            }
            StmtKind::If(cond, body, else_body) => {
                let cond_ty = self.check_expr(cond, ctx);

                if cond_ty != Type::Bool && cond_ty != Type::Error {
                    self.error("if statement condition must have type 'bool'", cond.span);
                    return;
                }

                self.check_stmt(body, ctx);

                if let Some(e) = else_body {
                    self.check_stmt(e, ctx);
                }
            }
            StmtKind::While(cond, body) => {
                let cond_ty = self.check_expr(cond, ctx);

                if cond_ty != Type::Bool && cond_ty != Type::Error {
                    self.error("while statement condition must have type 'bool'", cond.span);
                    return;
                }

                self.check_stmt(body, ctx);
            }
            StmtKind::Error => {
                unreachable!()
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr, ctx: &mut AnalysisCtx) -> Type {
        let ty = match &expr.kind {
            ExprKind::BinaryOp(op, left, right) => {
                let left_type = self.check_expr(left, ctx);
                let right_type = self.check_expr(right, ctx);

                let res = self.binary_result_type(op.node, left_type, right_type);
                if res == Type::Error {
                    self.error(
                        format!("invalid operands for binary operation: '{}'", op.node),
                        expr.span,
                    );
                }

                res
            }
            ExprKind::Literal(lit) => match lit {
                LitKind::Int(i) => {
                    // TODO: because i32::MAX is less than i32::min when the sign is ignored,
                    // we need to somehow account for the +1 that a negative literal needs
                    if *i <= i32::MAX as u128 {
                        Type::Int
                    } else {
                        unimplemented!("larger integer types not yet implemented")
                    }
                }
                LitKind::Bool(_b) => Type::Bool,
            },
            ExprKind::UnaryOp(op, right) => {
                let expr_type = self.check_expr(right, ctx);

                self.unary_result_type(op.node, expr_type)
            }
            ExprKind::VarAssign(target, rhs) => {
                let rhs_type = self.check_expr(rhs, ctx);
                let target_type = self.check_expr(target, ctx);

                if !self.can_assign(rhs_type, target_type) {
                    self.error("mismatched types", expr.span);
                    Type::Error
                } else if !self.is_lvalue(target) {
                    self.error("cannot assign to non-lvalue", expr.span);
                    Type::Error
                } else {
                    rhs_type
                }
            }
            ExprKind::Variable(..) => {
                // unwrap because we already know the variable exists
                // after name resolution
                let varid = ctx.variables.get(&expr.id).unwrap();
                // unwrap here because we know that this variable is already declared
                *ctx.var_types.get(varid).unwrap()
            }
            ExprKind::Error => {
                unreachable!()
            }
        };

        ctx.types.insert(expr.id, ty);

        ty
    }

    fn is_lvalue(&self, node: &Expr) -> bool {
        match node.kind {
            ExprKind::Variable(..) => true,
            _ => false,
        }
    }

    fn can_assign(&self, from: Type, to: Type) -> bool {
        if from == to {
            return true;
        }

        match (from, to) {
            (Type::Error, _) | (_, Type::Error) => {
                return true;
            }

            _ => {
                return false;
            }
        }
    }

    fn unary_result_type(&self, op: UnaryOpKind, rhs: Type) -> Type {
        if rhs == Type::Error {
            return Type::Error;
        }
        match (op, rhs) {
            (UnaryOpKind::Negate, Type::Int) => Type::Int,
            (UnaryOpKind::Negate, Type::Bool) => Type::Error,
            (UnaryOpKind::Negate, Type::Error) => unreachable!(),
        }
    }

    fn binary_result_type(&self, op: BinOpKind, lhs: Type, rhs: Type) -> Type {
        if lhs == Type::Error || rhs == Type::Error {
            return Type::Error;
        }
        match (op, lhs, rhs) {
            (BinOpKind::Add, Type::Int, Type::Int) => Type::Int,
            (BinOpKind::Sub, Type::Int, Type::Int) => Type::Int,
            (BinOpKind::Mul, Type::Int, Type::Int) => Type::Int,
            (BinOpKind::Div, Type::Int, Type::Int) => Type::Int,
            (BinOpKind::BangEq, Type::Int, Type::Int) => Type::Bool,
            (BinOpKind::BangEq, Type::Bool, Type::Bool) => Type::Bool,
            (BinOpKind::EqEq, Type::Int, Type::Int) => Type::Bool,
            (BinOpKind::EqEq, Type::Bool, Type::Bool) => Type::Bool,
            (BinOpKind::GreaterEq, Type::Int, Type::Int) => Type::Bool,
            (BinOpKind::GreaterThan, Type::Int, Type::Int) => Type::Bool,
            (BinOpKind::LessEq, Type::Int, Type::Int) => Type::Bool,
            (BinOpKind::LessThan, Type::Int, Type::Int) => Type::Bool,
            (BinOpKind::Or, Type::Bool, Type::Bool) => Type::Bool,
            (BinOpKind::And, Type::Bool, Type::Bool) => Type::Bool,
            _ => Type::Error,
        }
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
