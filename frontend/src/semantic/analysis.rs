use crate::diagnostics::Diagnostic;
use crate::parse::ast::NodeId;
use crate::parse::ast as untyped;
use std::collections::HashMap;
use common::{collections::Stack, stack};

pub fn analyze() {

}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct VarId(pub usize);

impl VarId {
    pub const ERROR: VarId = VarId(usize::MAX);
}

pub struct Resolutions {
    // uses of a variable
    // includes use as a target in an assignment expression
    pub var_uses: HashMap<NodeId, VarId>,
    // a variable declaration which introduces a new varaible id
    pub var_decls: HashMap<NodeId, VarId>
}

#[derive(Debug, PartialEq)]
struct Scope {
    vars: HashMap<Vec<u8>, VarId>
}

impl Scope {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new()
        }
    }
}

pub struct Resolver<'a> {
    scopes: Stack<Scope>,
    next_var_id: usize,
    var_uses: HashMap<NodeId, VarId>,
    var_decls: HashMap<NodeId, VarId>,
    diags: &'a mut Vec<Diagnostic>,
}

impl<'a> Resolver<'a> {
    pub fn new(diags: &'a mut Vec<Diagnostic>) -> Self {
        Self {
            scopes: stack![Scope::new()],
            next_var_id: 0,
            var_decls: HashMap::new(),
            var_uses: HashMap::new(),
            diags
        }
    }

    pub fn resolve_program(mut self, program: &untyped::Program) -> Resolutions {
        for stmt in &program.stmts {
            self.resolve_stmt(stmt);
        }

        Resolutions { var_uses: self.var_uses, var_decls: self.var_decls }
    }

    fn new_var_id(&mut self) -> VarId {
        let id = VarId(self.next_var_id);
        self.next_var_id += 1;
        id
    }

    fn declare_var(&mut self, name: &[u8]) -> VarId {
        let id = self.new_var_id();
        self.scopes.top_mut().unwrap().vars.insert(name.to_vec(), id); 
        id
    }

    fn lookup_var(&mut self, name: &[u8]) -> Option<VarId> {
        self.scopes.iter().rev().find_map(|s| s.vars.get(name).copied())
    }

    fn resolve_stmt(&mut self, stmt: &untyped::Stmt) {
        match stmt {
            untyped::Stmt::Print { expr, .. } => {
                self.resolve_expr(expr); 
            }
            untyped::Stmt::VarDecl { name, initializer, id, .. } => {
                if let Some(init) = initializer {
                    self.resolve_expr(init);
                }
                let varid = self.declare_var(name);
                self.var_decls.insert(*id, varid);
            }
            untyped::Stmt::Block { body, .. } => {
                self.scopes.push(Scope::new());
                for s in body {
                    self.resolve_stmt(s);
                }
                self.scopes.pop();
            }
            untyped::Stmt::StmtExpr { expr , .. } => {
                self.resolve_expr(expr);
            }
            untyped::Stmt::Error { .. } => {}
        }
    }

    fn resolve_expr(&mut self, expr: &untyped::Expr) {
        match expr {
            
        }
    }
}