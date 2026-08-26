use crate::lex::token::TokenKind;
use crate::diagnostics::{Diagnostic, Severity};
use crate::parse::ast::{Program, Stmt, StmtKind, Expr, ExprKind, ForInit, LitKind, UnaryOpKind, BinOpKind, NodeId};
use crate::span::Span;
use crate::analysis::analyzer::AnalysisCtx;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Conversion {
    IntToDouble,
}

pub struct TypeChecker<'diag> {
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
            StmtKind::Return(expr) => {
                todo!()
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
            ExprKind::Call(callee, args) => {
                todo!()
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
            BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div => match (lhs, rhs) {
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
            },
            BinOpKind::BangEq | BinOpKind::EqEq => match (lhs, rhs) {
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
            },
            BinOpKind::GreaterEq
            | BinOpKind::GreaterThan
            | BinOpKind::LessEq
            | BinOpKind::LessThan => match (lhs, rhs) {
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
            },
            BinOpKind::Or | BinOpKind::And => match (lhs, rhs) {
                (Type::Bool, Type::Bool) => {
                    return Type::Bool;
                }
                _ => {
                    return Type::Error;
                }
            },
        }
    }
}