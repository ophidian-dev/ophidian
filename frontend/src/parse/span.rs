#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Span {
    offset: usize,
    len: usize,
}

impl Span {
    pub fn new(offset: usize, len: usize) -> Self {
        Self { offset, len }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn end(&self) -> usize {
        self.offset + self.len
    }

    pub fn join(self, other: Span) -> Self {
        let start: usize = self.offset().min(other.offset());
        let end: usize = self.end().max(other.end());
        Self {
            offset: start,
            len: end - start,
        }
    }
}
