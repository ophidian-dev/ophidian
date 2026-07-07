use crate::semantic::typed::*;
use std::collections::HashMap;

pub struct SemanticAnalyzer {
    scopes: Vec<Scope>,
    id_count: usize,
}

struct Scope {
    symbols: HashMap<Vec<u8>, Symbol>
}

struct Symbol {
    id: usize,
    ty: Type
}

impl Scope {
    fn new() -> Self {
        Self {
            symbols: HashMap::new()
        }
    }
}

impl Symbol {
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
            scopes: Vec::new(),
            id_count: 0
        };

        analyzer.enter_scope();
        analyzer
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
        
    }
}