use crate::span::Span;

pub struct Diagnostic {
    pub message: String,
    pub span: Span,
    pub severity: Severity,
}

pub enum Severity {
    Error,
}

impl Diagnostic {
    pub fn new(message: String, span: Span, severity: Severity) -> Self {
        Self {
            message,
            severity,
            span
        }
    }
}