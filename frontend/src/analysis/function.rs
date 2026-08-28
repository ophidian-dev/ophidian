use std::collections::HashSet;

use crate::analysis::analyzer::AnalysisCtx;
use crate::analysis::resolution::Resolver;
use crate::analysis::types::{Type, TypeChecker};
use crate::diagnostics::{Diagnostic, Severity};
use crate::parse::ast::{Block, Function as AstFunction, Param};
use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionId(pub usize);

impl FunctionId {
    pub const ERROR: Self = Self(usize::MAX);
}

impl std::ops::AddAssign<usize> for FunctionId {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Function {
    pub return_type: Type,
    pub params: Vec<Param>,
}

impl Function {
    pub fn new(return_type: Type, params: Vec<Param>) -> Self {
        Self {
            return_type,
            params,
        }
    }
}

pub struct FunctionResolver<'a> {
    curr_func_id: FunctionId,
    func_names: HashSet<&'a [u8]>,
}

impl<'a> FunctionResolver<'a> {
    pub fn new() -> Self {
        Self {
            curr_func_id: FunctionId(0),
            func_names: HashSet::new(),
        }
    }

    pub fn collect_functions(&mut self, functions: &'a [AstFunction], ctx: &mut AnalysisCtx) {
        for function in functions {
            if !self.func_names.insert(&function.name) {
                self.error(
                    format!(
                        "redefinition of identifier: '{}'",
                        std::str::from_utf8(&function.name).expect("invalid utf8")
                    ),
                    function.span,
                    ctx,
                );
                continue;
            }

            let id = self.curr_func_id;
            ctx.functions.insert(function.id, id);
            ctx.signatures.insert(
                id,
                Function::new(
                    function.return_type.unwrap_or(Type::Void),
                    function.params.clone(),
                ),
            );
            self.curr_func_id += 1;
        }
    }

    fn error<T: Into<String>>(&self, msg: T, span: Span, ctx: &mut AnalysisCtx) {
        ctx.diagnostics
            .push(Diagnostic::new(msg.into(), span, Severity::Error));
    }
}

pub struct FunctionAnalyzer {}

impl FunctionAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn analyze(&mut self, functions: &[AstFunction], ctx: &mut AnalysisCtx) {
        for function in functions {
            self.analyze_function(function, ctx);
        }
    }

    fn analyze_function(&mut self, function: &AstFunction, ctx: &mut AnalysisCtx) {

        let mut resolver = Resolver::new();
        for param in &function.params {
            resolver.enter_scope(ctx);
            let id = resolver.declare_var(&param.name, ctx, param.span);
            ctx.variables.insert(param.id, id);
            ctx.var_types.insert(id, param.ty);
            resolver.exit_scope(ctx);
        }

        let block: Block = function.body.clone().try_into().expect("shouldnt happen");

        resolver.resolve_block(&block.stmts, ctx);

        if !ctx.diagnostics.is_empty() {
            return;
        }

        let mut typechecker = TypeChecker::new();
        typechecker.check_stmts(&block.stmts, ctx);
    }
}
