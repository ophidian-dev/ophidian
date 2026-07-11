use crate::diagnostics::{Diagnostic, Severity};
use crate::parse::ast as untyped;
use crate::semantic::ctors::{
    binary_op_from_untyped, create_binary_op, create_block, create_integer_literal, create_print,
    create_stmtexpr, create_unary_op, create_var_assign, create_var_decl, create_variable,
    unary_op_from_untyped,
};
use crate::semantic::typed;
use crate::semantic::typed::Type;
use crate::span::Span;
use common::collections::Stack;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct VarId(usize);

impl VarId {
    pub const ERROR: VarId = VarId(usize::MAX);
}

impl PartialEq<usize> for VarId {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }

    fn ne(&self, other: &usize) -> bool {
        self.0 != *other
    }
}

impl Into<u32> for VarId {
    fn into(self) -> u32 {
        assert!(self.0 <= u32::MAX as usize);
        self.0 as u32
    }
}

impl PartialOrd<usize> for VarId {
    fn ge(&self, other: &usize) -> bool {
        self.0 >= *other
    }

    fn gt(&self, other: &usize) -> bool {
        self.0 > *other
    }

    fn le(&self, other: &usize) -> bool {
        self.0 <= *other
    }

    fn lt(&self, other: &usize) -> bool {
        self.0 < *other
    }

    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl std::ops::AddAssign<usize> for VarId {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl std::ops::AddAssign<Self> for VarId {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

struct Signature {
    lhs: Type,
    rhs: Type,
    result: Type
}

static ADD_SIGS: &[Signature] = &[
    Signature::new(Type::Int, Type::Int, Type::Int),
];

static SUB_SIGS: &[Signature] = &[
    Signature::new(Type::Int, Type::Int, Type::Int),
];

static MUL_SIGS: &[Signature] = &[
    Signature::new(Type::Int, Type::Int, Type::Int),
];

static DIV_SIGS: &[Signature] = &[
    Signature::new(Type::Int, Type::Int, Type::Int),
];

static OR_SIGS: &[Signature] = &[
    Signature::new(Type::Bool, Type::Bool, Type::Bool),
];

static AND_SIGS: &[Signature] = &[
    Signature::new(Type::Bool, Type::Bool, Type::Bool),
];

static EQEQ_SIGS: &[Signature] = &[
    Signature::new(Type::Int, Type::Int, Type::Bool),
    Signature::new(Type::Bool, Type::Bool, Type::Bool),
];

static BANGEQ_SIGS: &[Signature] = &[
    Signature::new(Type::Int, Type::Int, Type::Bool),
    Signature::new(Type::Bool, Type::Bool, Type::Bool),
];

static LESSER_SIGS: &[Signature] = &[
    Signature::new(Type::Int, Type::Int, Type::Bool),
];

static GREATER_SIGS: &[Signature] = &[
    Signature::new(Type::Int, Type::Int, Type::Bool),
];

static LESSEREQ_SIGS: &[Signature] = &[
    Signature::new(Type::Int, Type::Int, Type::Bool),
];

static GREATEREQ_SIGS: &[Signature] = &[
    Signature::new(Type::Int, Type::Int, Type::Bool),
];

impl untyped::BinopType {
    fn signatures(&self) -> &'static [Signature] {
        use untyped::BinopType;

        match self {
            BinopType::Add => ADD_SIGS,
            BinopType::Sub => SUB_SIGS,
            BinopType::Mul => MUL_SIGS,
            BinopType::Div => DIV_SIGS,
            BinopType::Or => OR_SIGS,
            BinopType::And => AND_SIGS,
            BinopType::EqEq => EQEQ_SIGS,
            BinopType::BangEq => BANGEQ_SIGS,
            BinopType::Lesser => LESSER_SIGS,
            BinopType::Greater => GREATER_SIGS,
            BinopType::LesserEq => LESSEREQ_SIGS,
            BinopType::GreaterEq => GREATEREQ_SIGS, 
        }
    }
}

impl Signature {
    const fn new(lhs: Type, rhs: Type, res: Type) -> Self {
        Self { lhs, rhs, result: res }
    }
}

pub struct SemanticAnalyzer<'a> {
    scopes: Stack<Scope>,
    id_count: VarId,
    diagnostics: &'a mut Vec<Diagnostic>,
}

#[derive(Debug, PartialEq)]
struct Scope {
    symbols: HashMap<Vec<u8>, Symbol>,
}

#[derive(Debug, PartialEq, Clone)]
struct Symbol {
    pub id: VarId,
    pub ty: Type,
}

impl Scope {
    fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
}

impl Symbol {
    pub fn new(id: VarId, ty: Type) -> Self {
        Self { id, ty }
    }
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(diagnostics: &'a mut Vec<Diagnostic>) -> Self {
        let mut analyzer = Self {
            scopes: Stack::new(),
            id_count: VarId(0),
            diagnostics,
        };

        analyzer.enter_scope();
        analyzer
    }

    fn next_id(&mut self) -> VarId {
        let tmp = self.id_count;
        self.id_count += 1;
        tmp
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_var(&mut self, name: &[u8], ty: Type) -> Result<VarId, (String, VarId)> {
        if let Some(v) = self.scopes.top().unwrap().symbols.get(name) {
            return Err((
                format!(
                    "redeclaration of identifier: '{}'",
                    String::from_utf8_lossy(name)
                ),
                v.id,
            ));
        }
        let id = self.next_id();
        let symbol = Symbol::new(id, ty);
        self.scopes
            .top_mut()
            .unwrap()
            .symbols
            .insert(name.to_vec(), symbol);
        Ok(id)
    }

    fn lookup_var(&mut self, name: &[u8]) -> Option<Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(symbol.clone());
            }
        }
        None
    }

    fn error<T: Into<String>>(&mut self, msg: T, span: Span) {
        self.diagnostics
            .push(Diagnostic::new(msg.into(), span, Severity::Error));
    }

    fn can_assign(&self, target: &typed::Expr, value: &typed::Expr) -> bool {
        match target {
            typed::Expr::Variable { ty, .. } => match value.ty() {
                Type::Int => match ty {
                    Type::Int | Type::Error => return true,
                    _ => false,
                },
                Type::Bool => match ty {
                    Type::Bool | Type::Error => return true,
                    _ => false,
                },
                Type::Error => match ty {
                    _ => {
                        return true;
                    }
                },
            },
            _ => {
                return false;
            }
        }
    }

    // when a variables name is preceded by 'a_' then it refers to that Expression be 'analyzed'
    fn visit_expr(&mut self, expr: untyped::Expr) -> typed::Expr {
        match expr {
            untyped::Expr::IntegerLiteral { span, value } => {
                return create_integer_literal(value, Type::Int, span);
            }
            untyped::Expr::BinaryOp {
                span,
                op,
                left,
                right,
            } => {
                let a_left = self.visit_expr(*left);
                let a_right = self.visit_expr(*right);

                for sig in op.kind.signatures() {
                    if sig.lhs == a_left.ty() && sig.rhs == a_right.ty() {
                        
                    }
                }

            }
            untyped::Expr::UnaryOp { span, op, expr } => {
                let a_expr = self.visit_expr(*expr);
                match op.kind {
                    untyped::UnaryopType::Negate => match a_expr.ty() {
                        Type::Int => {
                            return create_unary_op(
                                unary_op_from_untyped(op),
                                Type::Int,
                                a_expr,
                                span,
                            );
                        }
                        Type::Bool => {
                            self.error("operator '-' cannot be used on type 'bool'", span);
                            return create_unary_op(
                                unary_op_from_untyped(op),
                                Type::Error,
                                a_expr,
                                span,
                            );
                        }
                        Type::Error => {
                            unreachable!("no other type except int and bool should exist")
                        }
                    },
                    untyped::UnaryopType::Not => match a_expr.ty() {
                        Type::Bool => {
                            return create_unary_op(
                                unary_op_from_untyped(op),
                                Type::Bool,
                                a_expr,
                                span,
                            );
                        }
                        Type::Int => {
                            self.error("operator '!' cannot be applied on type 'int'", span);
                            return create_unary_op(
                                unary_op_from_untyped(op),
                                Type::Error,
                                a_expr,
                                span,
                            );
                        }
                        Type::Error => {
                            unreachable!("no other type except int and bool should exist")
                        }
                    },
                }
            }
            untyped::Expr::VarAssign {
                target,
                value,
                span,
            } => {
                let a_target = self.visit_expr(*target);
                let a_value = self.visit_expr(*value);

                if !self.can_assign(&a_target, &a_value) {
                    self.error("mismatched types", a_target.span().join(a_value.span()));
                    todo!("recover from error");
                }

                let id = match a_target {
                    typed::Expr::Variable { id, .. } => id,
                    _ => VarId::ERROR,
                };

                match a_value.ty() {
                    Type::Int => {
                        return create_var_assign(a_target, a_value, Type::Int, id, span);
                    }
                    Type::Bool => {
                        return create_var_assign(a_target, a_value, Type::Bool, id, span);
                    }
                    Type::Error => {
                        return create_var_assign(
                            a_target,
                            a_value,
                            Type::Error,
                            VarId::ERROR,
                            span,
                        );
                    }
                }
            }
            untyped::Expr::Variable { name, span } => match self.lookup_var(&name) {
                Some(v) => {
                    return create_variable(v.id, v.ty, span);
                }
                None => {
                    self.error(
                        format!(
                            "use of undeclared identifier: '{}'",
                            String::from_utf8_lossy(&name)
                        ),
                        span,
                    );
                    return create_variable(VarId::ERROR, Type::Error, span);
                }
            },
            _ => {
                unreachable!("parser should have exited if error encountered");
            }
        }
    }

    fn visit_stmt(&mut self, stmt: untyped::Stmt) -> typed::Stmt {
        match stmt {
            untyped::Stmt::Block { body, span } => {
                self.enter_scope();

                let mut a_body: Vec<typed::Stmt> = Vec::new();

                let mut span = span;

                for stmt in body {
                    span = span.join(stmt.span());
                    a_body.push(self.visit_stmt(stmt));
                }

                self.exit_scope();

                return create_block(a_body, span);
            }
            untyped::Stmt::Print { expr, span } => {
                let a_expr = self.visit_expr(*expr);

                if a_expr.ty() != Type::Int {
                    self.error("statement 'print' expected type 'int'", span);
                }

                return create_print(a_expr, span);
            }
            untyped::Stmt::StmtExpr { expr, span } => {
                let a_expr = self.visit_expr(*expr);
                return create_stmtexpr(a_expr, span);
            }
            untyped::Stmt::VarDecl {
                name,
                type_annotation,
                initializer,
                span,
            } => {
                match type_annotation {
                    Some(t) => {
                        if let Some(init) = initializer {
                            let a_expr = self.visit_expr(init);

                            if t != a_expr.ty() {
                                self.error("variable type mismatch", span);
                                todo!("recover from error");
                            }

                            let id = match self.declare_var(&name, t) {
                                Ok(i) => i,
                                Err((e, i)) => {
                                    self.error(e, span);
                                    return create_var_decl(Type::Error, None, i, span);
                                }
                            };

                            return create_var_decl(t, Some(a_expr), id, span);
                        }

                        let id = match self.declare_var(&name, t) {
                            Ok(i) => i,
                            Err((e, i)) => {
                                self.error(e, span);
                                return create_var_decl(Type::Error, None, i, span);
                            }
                        };

                        return create_var_decl(t, None, id, span);
                    }
                    None => {
                        if initializer.is_none() {
                            self.error(format!("cannot infer type of identifier '{}' without initializer expression", String::from_utf8_lossy(&name)), span);
                            let id = match self.declare_var(&name, Type::Error) {
                                Ok(i) => i,
                                Err((e, i)) => {
                                    self.error(e, span);
                                    return create_var_decl(Type::Error, None, i, span);
                                }
                            };

                            return create_var_decl(Type::Error, None, id, span);
                        }

                        // initializer is guarenteed to be Some(v) after the checks above
                        let init = initializer.unwrap();

                        let a_expr = self.visit_expr(init);
                        let id = match self.declare_var(&name, a_expr.ty()) {
                            Ok(i) => i,
                            Err((e, i)) => {
                                self.error(e, span);
                                return create_var_decl(Type::Error, None, i, span);
                            }
                        };
                        return create_var_decl(a_expr.ty(), Some(a_expr), id, span);
                    }
                }
            }
            _ => {
                unreachable!("parser should have stopped after errors");
            }
        }
    }

    pub fn analyze(&mut self, program: untyped::Program) -> typed::Program {
        let mut typed_program = typed::Program::new();

        for stmt in program.stmts {
            typed_program.stmts.push(self.visit_stmt(stmt));
        }

        typed_program
    }
}
