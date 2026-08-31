pub mod analyzer;
pub mod declarations;
pub mod function;
pub mod hir;
pub mod resolution;
pub mod types;
pub mod ids;

use crate::diagnostics::Diagnostic;

pub struct SemanticAnalyzer<'diag> {
    diagnostics: &'diag mut Vec<Diagnostic>,
}
