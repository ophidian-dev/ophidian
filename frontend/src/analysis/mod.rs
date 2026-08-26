pub mod analyzer;
pub mod function;
pub mod resolution;
pub mod type_check;

use crate::diagnostics::Diagnostic;

pub struct SemanticAnalyzer<'diag> {
    diagnostics: &'diag mut Vec<Diagnostic>,
}