use crate::diagnostics::{Diagnostic, Severity};
use crate::lex::token::TokenKind;
use crate::parse::ast::{
    BinOpKind, Expr, ExprKind, ForInit, LitKind, NodeId, Program, Stmt, StmtKind, UnaryOpKind,
};
use crate::span::Span;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,

    Bool,

    Double,

    // The type that allows analysis to continue if it
    // encounters an error
    Error,
}

impl From<TokenKind> for Type {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Int => Self::Int,
            TokenKind::Double => Self::Double,
            TokenKind::Bool => Self::Bool,
            _ => Self::Error,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Conversion {
    IntToDouble,
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

        for (id, value) in &ctx.types {
            if let Some(conversion_type) = ctx.conversions.get(id) {
                match conversion_type {
                    Conversion::IntToDouble => {
                        ctx.converted_types.insert(*id, Type::Double);
                    }
                }
            } else {
                ctx.converted_types.insert(*id, *value);
            }
        }

        Ok(AnalysisResult::from(ctx))
    }
}

struct Resolver<'diag> {
    curr_var_id: VarId,
    diagnostics: &'diag mut Vec<Diagnostic>,
    loop_depth: usize,
}

impl<'diag> Resolver<'diag> {
    pub fn new(diagnostics: &'diag mut Vec<Diagnostic>) -> Self {
        Self {
            curr_var_id: VarId::from(0),
            diagnostics,
            loop_depth: 0,
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
                self.resolve_break(stmt);
            }
            StmtKind::Continue => {
                self.resolve_continue(stmt);
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

    fn resolve_break(&mut self, stmt: &Stmt) {
        if self.loop_depth == 0 {
            self.error("use of 'break' statement outside loop", stmt.span);
        }
    }

    fn resolve_continue(&mut self, stmt: &Stmt) {
        if self.loop_depth == 0 {
            self.error("use of continue statement outside loop", stmt.span);
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
                    Type::Int | Type::Bool | Type::Double => {}
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

                                if !self.can_assign(initializer_type, *annotation, init, ctx) {
                                    self.error("mismatched types", stmt.span);
                                    return;
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
            StmtKind::Break => {
                // nothing to type check
            }
            StmtKind::Continue => {
                // nothing to type check
            }
            StmtKind::For(init, cond, incre, body) => {
                if let Some(init) = init {
                    match &**init {
                        ForInit::Decl(decl) => {
                            self.check_stmt(decl, ctx);
                        }
                        ForInit::Expr(expr) => {
                            self.check_expr(expr, ctx);
                        }
                    }
                }

                if let Some(cond) = cond {
                    let ty = self.check_expr(cond, ctx);

                    if ty != Type::Bool && ty != Type::Error {
                        self.error("for loop condition must have type 'bool'", cond.span);
                        return;
                    }
                }

                if let Some(incre) = incre {
                    self.check_expr(incre, ctx);
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

                let res =
                    self.binary_result_type(op.node, left_type, right_type, left.id, right.id, ctx);
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
                LitKind::Float(_f) => {
                    // no type checking needed here?
                    Type::Double        
                }
            },
            ExprKind::UnaryOp(op, right) => {
                let expr_type = self.check_expr(right, ctx);

                match op.node {
                    UnaryOpKind::Negate => {
                        // do nothin
                    }
                    UnaryOpKind::PostDecrement
                    | UnaryOpKind::PostIncrement
                    | UnaryOpKind::PreDecrement
                    | UnaryOpKind::PreIncrement => {
                        if !self.is_lvalue(right) {
                            self.error(
                                format!("cannot apply operator '{}' on non l-value", op.node),
                                expr.span,
                            );
                            return Type::Error;
                        }
                    }
                }

                self.unary_result_type(op.node, expr_type)
            }
            ExprKind::VarAssign(target, rhs) => {
                let rhs_type = self.check_expr(rhs, ctx);
                let target_type = self.check_expr(target, ctx);

                if !self.can_assign(rhs_type, target_type, rhs, ctx) {
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

    fn can_assign(&self, value: Type, target: Type, expr: &Expr, ctx: &mut AnalysisCtx) -> bool {
        if target == value {
            return true;
        }

        match (target, value) {
            (Type::Error, _) | (_, Type::Error) => {
                return true;
            }
            (Type::Double, Type::Int) => {
                ctx.conversions.insert(expr.id, Conversion::IntToDouble);
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
            (UnaryOpKind::Negate, Type::Double) => Type::Double,
            (
                UnaryOpKind::Negate
                | UnaryOpKind::PostDecrement
                | UnaryOpKind::PostIncrement
                | UnaryOpKind::PreDecrement
                | UnaryOpKind::PreIncrement,
                Type::Bool,
            ) => Type::Error,
            (UnaryOpKind::PostDecrement, Type::Int) => Type::Int,
            (UnaryOpKind::PostIncrement, Type::Int) => Type::Int,
            (UnaryOpKind::PreDecrement, Type::Int) => Type::Int,
            (UnaryOpKind::PreIncrement, Type::Int) => Type::Int,
            (UnaryOpKind::PostDecrement, Type::Double) => Type::Double,
            (UnaryOpKind::PostIncrement, Type::Double) => Type::Double,
            (UnaryOpKind::PreDecrement, Type::Double) => Type::Double,
            (UnaryOpKind::PreIncrement, Type::Double) => Type::Double,
            (_, Type::Error) => unreachable!(),
        }
    }

    fn binary_result_type(
        &self,
        op: BinOpKind,
        lhs: Type,
        rhs: Type,
        left_id: NodeId,
        right_id: NodeId,
        ctx: &mut AnalysisCtx,
    ) -> Type {
        if lhs == Type::Error || rhs == Type::Error {
            return Type::Error;
        }

        match op {
            BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div => {
                match (lhs, rhs) {
                    (Type::Int, Type::Int) => {
                        return Type::Int;
                    }
                    (Type::Double, Type::Double) => {
                        return Type::Double;
                    }
                    (Type::Int, Type::Double) => {
                        ctx.conversions.insert(left_id, Conversion::IntToDouble);
                        return Type::Double;
                    }
                    (Type::Double, Type::Int) => {
                        ctx.conversions.insert(right_id, Conversion::IntToDouble);
                        return Type::Double;
                    }
                    (Type::Bool | Type::Error, _) => {
                        return Type::Error;
                    }
                    (_, Type::Bool | Type::Error) => {
                        return Type::Error;
                    }
                }
            }
            BinOpKind::BangEq | BinOpKind::EqEq => {
                match (lhs, rhs) {
                    (Type::Int, Type::Int) => {
                        return Type::Bool;
                    }
                    (Type::Double, Type::Double) => {
                        return Type::Bool;
                    }
                    (Type::Int, Type::Double) => {
                        ctx.conversions.insert(left_id, Conversion::IntToDouble);
                        return Type::Bool;
                    }
                    (Type::Double, Type::Int) => {
                        ctx.conversions.insert(right_id, Conversion::IntToDouble);
                        return Type::Bool;
                    }
                    (Type::Bool, Type::Bool) => {
                        return Type::Bool;
                    }
                    _ => {
                        return Type::Error;
                    }
                }
            }
            BinOpKind::GreaterEq | BinOpKind::GreaterThan | BinOpKind::LessEq | BinOpKind::LessThan => {
                match (lhs, rhs) {
                    (Type::Int, Type::Int) => {
                        return Type::Bool;
                    }
                    (Type::Double, Type::Double) => {
                        return Type::Bool;
                    }
                    (Type::Int, Type::Double) => {
                        ctx.conversions.insert(left_id, Conversion::IntToDouble);
                        return Type::Bool;
                    }
                    (Type::Double, Type::Int) => {
                        ctx.conversions.insert(right_id, Conversion::IntToDouble);
                        return Type::Bool;
                    }
                    _ => {
                        return Type::Error;
                    }
                }
            }
            BinOpKind::Or | BinOpKind::And => {
                match (lhs, rhs) {
                    (Type::Bool, Type::Bool) => {
                        return Type::Bool;
                    }
                    _ => {
                        return Type::Error;
                    }
                }
            }
        }
    }
}

struct AnalysisCtx {
    scopes: Vec<Scope>,
    types: HashMap<NodeId, Type>,
    variables: HashMap<NodeId, VarId>,
    var_types: HashMap<VarId, Type>,

    conversions: HashMap<NodeId, Conversion>,

    converted_types: HashMap<NodeId, Type>,
}

impl AnalysisCtx {
    fn new() -> Self {
        Self {
            scopes: Vec::new(),
            types: HashMap::new(),
            variables: HashMap::new(),
            var_types: HashMap::new(),
            conversions: HashMap::new(),
            converted_types: HashMap::new(),
        }
    }
}

pub struct AnalysisResult {
    pub types: HashMap<NodeId, Type>,
    pub variables: HashMap<NodeId, VarId>,
    pub var_types: HashMap<VarId, Type>,

    pub conversions: HashMap<NodeId, Conversion>,

    pub converted_types: HashMap<NodeId, Type>,
}

impl From<AnalysisCtx> for AnalysisResult {
    fn from(value: AnalysisCtx) -> Self {
        Self {
            variables: value.variables,
            types: value.types,
            var_types: value.var_types,
            conversions: value.conversions,
            converted_types: value.converted_types,
        }
    }
}
