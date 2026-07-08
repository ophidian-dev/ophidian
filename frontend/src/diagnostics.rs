use crate::span::Span;

pub struct Diagnostics {
    pub message: String,
    pub span: Span,
    pub severity: Severity,
}

pub enum Severity {
    Error,
}
