use crate::analysis::SemanticAnalyzer;
use crate::analysis::function::{Function, FunctionAnalyzer, FunctionId, FunctionResolver};
use crate::analysis::resolution::{Resolver, Scope, VarId};
use crate::analysis::types::{Conversion, Type, TypeChecker};
use crate::diagnostics::Diagnostic;
use crate::parse::ast::{NodeId, Program};
use std::collections::HashMap;

impl<'diag> SemanticAnalyzer<'diag> {
    pub fn new(diagnostics: &'diag mut Vec<Diagnostic>) -> Self {
        Self { diagnostics }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<AnalysisResult, ()> {
        let mut ctx = AnalysisCtx::new(self.diagnostics);

        let mut function_resolver = FunctionResolver::new();

        for _decl in &program.decls {
            todo!()
        }

        function_resolver.collect_functions(&program.functions, &mut ctx);

        let mut function_analyzer = FunctionAnalyzer::new();
        function_analyzer.analyze(&program.functions, &mut ctx);

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
    pub variables: HashMap<NodeId, VarId>,
    pub var_types: HashMap<VarId, Type>,

    pub conversions: HashMap<NodeId, Conversion>,

    pub functions: HashMap<NodeId, FunctionId>,
    pub signatures: HashMap<FunctionId, Function>,

    pub diagnostics: &'diag mut Vec<Diagnostic>,

    converted_types: HashMap<NodeId, Type>,
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
            diagnostics,
        }
    }
}

pub struct AnalysisResult {
    pub variables: HashMap<NodeId, VarId>,
    pub var_types: HashMap<VarId, Type>,

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
