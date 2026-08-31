
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalVarId(pub usize);

impl LocalVarId {
    pub const ERROR: Self = Self(usize::MAX);
}

impl From<usize> for LocalVarId {
    fn from(value: usize) -> Self {
        LocalVarId(value)
    }
}

impl std::ops::AddAssign<usize> for LocalVarId {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalVarId(pub usize);

impl std::ops::AddAssign<usize> for GlobalVarId {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionId(pub usize);

impl FunctionId {
    pub const ERROR: Self = Self(usize::MAX);
}

impl std::ops::AddAssign<usize> for FunctionId {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HirId(pub usize);