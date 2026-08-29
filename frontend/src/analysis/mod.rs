pub mod analyzer;
pub mod function;
pub mod resolution;
pub mod hir;
pub mod types;

use crate::diagnostics::Diagnostic;

pub struct SemanticAnalyzer<'diag> {
    diagnostics: &'diag mut Vec<Diagnostic>,
}
