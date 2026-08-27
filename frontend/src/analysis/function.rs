use crate::diagnostics::Diagnostic;
use crate::analysis::analyzer::AnalysisCtx;
use crate::parse::ast::{Program, Stmt, Expr};

#[derive(Debug, Clone, Copy)]
pub struct FunctionId(pub usize);

impl std::ops::AddAssign<usize> for FunctionId {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

pub struct Function {
    
}

pub struct FunctionResolver<'diag> {
    diags: &'diag mut Vec<Diagnostic>,
    curr_func_id: FunctionId,
}

impl<'diag> FunctionResolver<'diag> {
    pub fn new(diags: &'diag mut Vec<Diagnostic>) -> Self {
        Self { diags, curr_func_id: FunctionId(0)}
    }

    pub fn resolve_functions(&mut self, program: &Program, ctx: &mut AnalysisCtx) {
        self.collect_decls(program, ctx);
    }

    fn collect_decls(&mut self, program: &Program, ctx: &mut AnalysisCtx) {
        for function in &program.functions {
            ctx.functions.insert(function.id, self.curr_func_id);
            self.curr_func_id += 1;
        }
    }
}