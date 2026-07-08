use crate::diagnostics::Diagnostic;
use crate::parse::ast as untyped;
use crate::semantic::typed;
use crate::semantic::typed::Type;
use common::collections::Stack;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct SemanticAnalyzer {
    scopes: Stack<Scope>,
    id_count: usize,
}

#[derive(Debug, PartialEq)]
struct Scope {
    symbols: HashMap<Vec<u8>, Symbol>,
}

#[derive(Debug, PartialEq, Clone)]
struct Symbol {
    id: usize,
    ty: Type,
}

impl Scope {
    fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
}

impl Symbol {
    pub fn new(id: usize, ty: Type) -> Self {
        Self { id, ty }
    }

    fn id(&self) -> usize {
        self.id
    }

    fn ty(&self) -> Type {
        self.ty
    }
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            scopes: Stack::new(),
            id_count: 0,
        };

        analyzer.enter_scope();
        analyzer
    }

    pub fn analyze(
        &mut self,
        program: untyped::Program,
        diagnostics: &mut [Diagnostic],
    ) -> typed::Program {
        typed::Program { stmts: Vec::new() }
    }

    fn next_id(&mut self) -> usize {
        let tmp: usize = self.id_count;
        self.id_count += 1;
        tmp
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_var(&mut self, name: &[u8], ty: Type) {
        if self.scopes.top().unwrap().symbols.contains_key(name) {
            todo!("implement error system for variable redeclaration");
        }
        let symbol = Symbol::new(self.next_id(), ty);
        self.scopes
            .top_mut()
            .unwrap()
            .symbols
            .insert(name.to_vec(), symbol);
    }

    fn lookup_var(&mut self, name: &[u8]) -> Option<Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(symbol.clone());
            }
        }
        None
    }

    fn visit_expr(&mut self, expr: untyped::Expr) -> Type {
        match expr {
            untyped::Expr::IntegerLiteral { .. } => Type::Int,
            _ => todo!("visit other exprs"),
        }
    }
}
