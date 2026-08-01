#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub usize);

impl NodeId {
    pub const ERROR: usize = usize::MAX;

    pub fn increment(&mut self) {
        *self += 1
    }
}

impl std::ops::AddAssign<usize> for NodeId {
    fn add_assign(&mut self, rhs: usize) {
        if self.0 + rhs > usize::MAX {
            self.0 = usize::MAX;
        } else {
            self.0 += rhs;
        }
    }
}
