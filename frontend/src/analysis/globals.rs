use crate::parse::ast::Program;
use crate::analysis::AnalysisCtx;

pub struct GlobalVarAnalyzer;

impl GlobalVarAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn analyze_globals(&mut self, program: &Program, ctx: &mut AnalysisCtx) {
        
    }
}