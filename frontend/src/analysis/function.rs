use macros::Constructor;

use crate::analysis::resolution::Resolver;
use crate::analysis::types::{Type, TypeChecker};
use crate::analysis::ids::LocalVarId;

#[derive(Debug, PartialEq, Eq, Clone, Constructor)]
pub struct Function {
    pub return_type: Type,
    pub params: Vec<Param>,
}

#[derive(Debug, PartialEq, Eq, Clone, Constructor)]
pub struct Param {
    id: LocalVarId,
    ty: Type,
}

pub struct FunctionAnalyzer {}

// impl FunctionAnalyzer {
//     pub fn new() -> Self {
//         Self {}
//     }

//     pub fn analyze(&mut self, functions: &[AstFunction], ctx: &mut AnalysisCtx) {
//         for function in functions {
//             self.analyze_function(function, ctx);
//         }
//     }

//     fn analyze_function(&mut self, function: &AstFunction, ctx: &mut AnalysisCtx) {
//         let mut resolver = Resolver::new();

//         resolver.enter_scope(ctx);
//         for param in &function.params {
//             let id = resolver.declare_var(&param.name, ctx, param.span);
//             ctx.variables.insert(param.id, id);
//             ctx.var_types.insert(id, param.ty);
//         }
//         resolver.exit_scope(ctx);

//         let block: Block = function.body.clone().try_into().expect("shouldnt happen");

//         resolver.resolve_block(&block.stmts, ctx);

//         if !ctx.diagnostics.is_empty() {
//             return;
//         }

//         let mut typechecker = TypeChecker::new();
//         typechecker.check_stmts(&block.stmts, ctx);
//     }
// }
