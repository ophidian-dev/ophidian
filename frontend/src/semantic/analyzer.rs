use crate::diagnostics::{Diagnostic, Severity};
use crate::parse::ast as untyped;
use crate::semantic::ctors::{binary_op_from_untyped, create_binary_op, create_block, create_integer_literal, create_print, create_stmtexpr, create_unary_op, create_var_assign, create_var_decl, create_variable, unary_op_from_untyped};
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
            diagnostics
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
            return Err((format!("redeclaration of identifier: '{}'", String::from_utf8_lossy(name)), v.id));
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
        self.diagnostics.push(Diagnostic::new(msg.into(), span, Severity::Error));
    }

    fn can_assign(&self, target: &typed::Expr, value: &typed::Expr) -> bool {
        match target {
            typed::Expr::Variable { ty, .. } => {
                match value.ty() {
                    Type::Int => {
                        match ty {
                            Type::Int | Type::Error => {
                                return true
                            }
                        }
                    } ,
                    Type::Error => {
                        match ty {
                            Type::Int | Type::Error => {
                                return true;
                            }
                        }
                    }
                } 
            }
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
            untyped::Expr::BinaryOp { span, op, left, right } => {
                let a_left = self.visit_expr(*left);
                let a_right = self.visit_expr(*right);

                if a_left.ty() == Type::Int && a_right.ty() == Type::Int {
                    match op.kind {
                        untyped::BinopType::Add | untyped::BinopType::Sub | untyped::BinopType::Mul | untyped::BinopType::Div => {
                            return create_binary_op(binary_op_from_untyped(op), Type::Int, a_left, a_right, span)
                        }
                    }
                }

                unreachable!("integer type is the only one that exists as of this moment")
            }
            untyped::Expr::UnaryOp { span, op, expr } => {
                let a_expr = self.visit_expr(*expr);
                match op.kind {
                    untyped::UnaryopType::Negate => {
                        if a_expr.ty() == Type::Int {
                            return create_unary_op(unary_op_from_untyped(op), Type::Int, a_expr, span);
                        }

                        unreachable!("no other type except int should exist")
                    }
                }
            }
            untyped::Expr::VarAssign { target, value, span } => {
                let a_target = self.visit_expr(*target);
                let a_value = self.visit_expr(*value);

                if !self.can_assign(&a_target, &a_value) {
                    self.error("mismatched types", a_target.span().join(a_value.span()));
                    todo!("recover from error");
                }

                let id = match a_target {
                    typed::Expr::Variable { id, .. } => id,
                    _ => VarId::ERROR
                };

                match a_value.ty() {
                    Type::Int => {
                        return create_var_assign(a_target, a_value, Type::Int, id, span);
                    }
                    Type::Error => {
                        return create_var_assign(a_target, a_value, Type::Error, VarId::ERROR, span)
                    }
                }
            }
            untyped::Expr::Variable { name, span } => {
                match self.lookup_var(&name) {
                    Some(v) => {
                        return create_variable(v.id, v.ty, span);
                    }
                    None => {
                        self.error(format!("use of undeclared identifier: '{}'", String::from_utf8_lossy(&name)), span);
                        return create_variable(VarId::ERROR, Type::Error, span);
                    }
                }
            } 
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
            untyped::Stmt::VarDecl { name, type_annotation, initializer, span } => {
                if let Some(init) = initializer {
                    let a_expr = self.visit_expr(init);

                    if type_annotation != a_expr.ty() {
                        self.error("variable type mismatch", span);
                        todo!("recover from error");
                    }

                    let id = match self.declare_var(&name, type_annotation) {
                        Ok(i) => {
                            i
                        }
                        Err((e, i)) => {
                            self.error(e, span);
                            return create_var_decl(Type::Error, None, i, span)
                        }
                    };

                    return create_var_decl(type_annotation, Some(a_expr), id, span);
                }

                let id = match self.declare_var(&name, type_annotation) {
                    Ok(i) => {
                        i
                    }
                    Err((e, i)) => {
                        self.error(e, span);
                        return create_var_decl(Type::Error, None, i, span)
                    }
                };

                return create_var_decl(type_annotation, None, id, span);
            }
            _ => {
                unreachable!("parser should have stopped after errors");
            }
        }
    }

    pub fn analyze(
        &mut self,
        program: untyped::Program,
    ) -> typed::Program {
        let mut typed_program = typed::Program::new();

        for stmt in program.stmts {
            typed_program.stmts.push(self.visit_stmt(stmt));
        } 

        typed_program
    }
}
