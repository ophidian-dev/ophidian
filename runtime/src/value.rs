#[derive(Clone)]
pub struct Value {
    pub kind: ValueKind,
    pub data: ValueData,
}

impl std::cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if self.kind != other.kind {
            return false;
        }

        unsafe {
            match self.kind {
                ValueKind::Integer => {
                    return self.data.integer == other.data.integer;
                }
            }
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "kind: {:?}", self.kind)?;
        write!(f, ", value: ")?;
        unsafe {
            match self.kind {
                ValueKind::Integer => {
                    write!(f, "{}", self.data.integer)
                }
            }
        }
    }
}

impl Value {
    pub const fn new_int(int: i32) -> Self {
        Self {
            kind: ValueKind::Integer,
            data: ValueData { integer: int },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueKind {
    Integer,
}

#[derive(Clone, Copy)]
pub union ValueData {
    pub integer: i32,
}
