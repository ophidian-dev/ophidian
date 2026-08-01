mod spanned;

pub use spanned::Spanned;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    start: usize,
    length: usize,
}

impl Span {
    pub const fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    pub fn retrieve_slice<'a>(text: &'a [u8], span: &Self) -> &'a [u8] {
        &text[span.start()..span.end()]
    }

    //  a placeholder span that is not meant to be used
    // and will be thrown away
    pub const fn dummy() -> Self {
        Self::new(0, 0)
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn end(&self) -> usize {
        self.length + self.start
    }
}
