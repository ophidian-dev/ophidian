use crate::semantic::typed::*;
use std::collections::HashMap;
use common::collections::Stack;

#[derive(Debug)]
pub struct SemanticAnalyzer {
    scopes: Stack<Scope>,
    id_count: usize,
}

#[derive(Debug)]
struct Scope {
    symbols: HashMap<Vec<u8>, Symbol>
}

#[derive(Debug)]
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
    pub fn new(id: usize, ty: Type) -> Self {
        Self {
            id,
            ty
        }
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
        if self.scopes.top().unwrap().symbols.contains_key(name) {
            todo!("implement error system for variable redeclaration");
        }
        let symbol = Symbol::new(self.next_id(), ty);
        self.scopes.top_mut().unwrap().symbols.insert(name.to_vec(), symbol);
    }

    
}