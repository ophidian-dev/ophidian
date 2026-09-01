pub mod declarations;
pub mod function;
pub mod hir;
pub mod resolution;
pub mod types;
pub mod ids;
pub mod globals;


use crate::analysis::declarations::Collecter;
use crate::analysis::function::{Function};
use crate::analysis::ids::{FunctionId, GlobalVarId, LocalVarId};
use crate::analysis::resolution::{Scope, GlobalVarDecl};
use crate::analysis::types::{Conversion, Type};
use crate::diagnostics::Diagnostic;
use crate::parse::ast::{NodeId, Program};
use std::collections::HashMap;

pub struct SemanticAnalyzer<'diag> {
    diagnostics: &'diag mut Vec<Diagnostic>,
}


impl<'diag> SemanticAnalyzer<'diag> {
    pub fn new(diagnostics: &'diag mut Vec<Diagnostic>) -> Self {
        Self { diagnostics }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<AnalysisResult, ()> {
        let mut ctx = AnalysisCtx::new(self.diagnostics);

        let mut collecter = Collecter::new();
        collecter.collect(program, &mut ctx);

        // let mut function_analyzer = FunctionAnalyzer::new();
        // function_analyzer.analyze(&program.functions, &mut ctx);

        // let mut resolver = Resolver::new();
        // resolver.resolve(program, &mut ctx);

        // if !ctx.diagnostics.is_empty() {
        //     return Err(());
        // }

        // let mut typechecker = TypeChecker::new();
        // typechecker.check(program, &mut ctx);

        // for (id, value) in &ctx.types {
        //     if let Some(conversion_type) = ctx.conversions.get(id) {
        //         match conversion_type {
        //             Conversion::IntToDouble => {
        //                 ctx.converted_types.insert(*id, Type::Double);
        //             }
        //         }
        //     } else {
        //         ctx.converted_types.insert(*id, *value);
        //     }
        // }

        Ok(AnalysisResult::from(ctx))
    }
}

pub struct AnalysisCtx<'diag> {
    pub scopes: Vec<Scope>,
    pub types: HashMap<NodeId, Type>,
    pub variables: HashMap<NodeId, LocalVarId>,
    pub var_types: HashMap<LocalVarId, Type>,

    pub conversions: HashMap<NodeId, Conversion>,

    pub functions: HashMap<NodeId, FunctionId>,
    pub signatures: HashMap<FunctionId, Function>,

    pub diagnostics: &'diag mut Vec<Diagnostic>,

    pub global_vars: HashMap<NodeId, GlobalVarId>,
    pub globalvar_data: HashMap<GlobalVarId, GlobalVarDecl>,

    converted_types: HashMap<NodeId, Type>,

    // current var id
    // local to a function
    varid: LocalVarId,
}

impl<'diag> AnalysisCtx<'diag> {
    pub fn alloc_varid(&mut self) -> LocalVarId {
        let id = self.varid;
        self.varid += 1;
        id
    }

    pub fn varid_set(&mut self, i: usize) {
        self.varid = LocalVarId(i);
    }
}

impl<'diag> AnalysisCtx<'diag> {
    fn new(diagnostics: &'diag mut Vec<Diagnostic>) -> Self {
        Self {
            scopes: Vec::new(),
            types: HashMap::new(),
            variables: HashMap::new(),
            var_types: HashMap::new(),
            conversions: HashMap::new(),
            converted_types: HashMap::new(),
            functions: HashMap::new(),
            signatures: HashMap::new(),
            global_vars: HashMap::new(),
            globalvar_data: HashMap::new(),
            diagnostics,
            varid: LocalVarId(0),
        }
    }
}

pub struct AnalysisResult {
    pub variables: HashMap<NodeId, LocalVarId>,
    pub var_types: HashMap<LocalVarId, Type>,

    pub conversions: HashMap<NodeId, Conversion>,

    pub converted_types: HashMap<NodeId, Type>,
}

impl From<AnalysisCtx<'_>> for AnalysisResult {
    fn from(value: AnalysisCtx) -> Self {
        Self {
            variables: value.variables,
            var_types: value.var_types,
            conversions: value.conversions,
            converted_types: value.converted_types,
        }
    }
}