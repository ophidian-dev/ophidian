use crate::span::Span;

#[derive(Debug)]
pub struct Diagnostic {
    pub message: String,
    pub span: Span,
    pub severity: Severity,
}

impl Diagnostic {
    pub const fn new(message: String, span: Span, severity: Severity) -> Self {
        Self {
            message,
            span,
            severity,
        }
    }
}

#[derive(Debug)]
pub enum Severity {
    Error,
}
