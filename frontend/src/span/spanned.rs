use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span, 
}