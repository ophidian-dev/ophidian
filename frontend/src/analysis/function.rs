use std::collections::{HashMap, HashSet};

use crate::analysis::analyzer::AnalysisCtx;
use crate::analysis::types::Type;
use crate::diagnostics::{Diagnostic, Severity};
use crate::parse::ast::{Function as AstFunction, Param};
use crate::span::Span;

#[derive(Debug, Clone, Copy)]
pub struct FunctionId(pub usize);

impl FunctionId {
    pub const ERROR: Self = Self(usize::MAX);
}

impl std::ops::AddAssign<usize> for FunctionId {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

pub struct Function {
    return_type: Type,
    params: Vec<Param>,
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
                return;
            }
            ctx.functions.insert(function.id, self.curr_func_id);
            self.curr_func_id += 1;
        }
    }

    fn error<T: Into<String>>(&self, msg: T, span: Span, ctx: &mut AnalysisCtx) {
        ctx.diagnostics
            .push(Diagnostic::new(msg.into(), span, Severity::Error));
    }
}

pub struct FunctionAnalyzer {

}

impl FunctionAnalyzer {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn analyze(&mut self, functions: &[AstFunction], ctx: &mut AnalysisCtx) {
        for function in functions {
            self.analyze_function(function, ctx);
        }
    }

    fn analyze_function(&mut self, function: &AstFunction, ctx: &mut AnalysisCtx) {
        let return_type = function.return_type;

    }
}
