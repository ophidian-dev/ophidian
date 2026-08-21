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
                ValueKind::Uninitialized => {
                    return false;
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
                ValueKind::Uninitialized => {
                    write!(f, "unitialized value")
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

    const fn new_uninitialized() -> Self {
        Self { kind: ValueKind::Uninitialized, data: ValueData { unitialized: std::ptr::null() } }
    }

    pub const UNINITIALIZED: Self = Self::new_uninitialized();
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueKind {
    Integer,
    Uninitialized,
}

#[derive(Clone, Copy)]
pub union ValueData {
    pub integer: i32,
    // any type is fine
    // just makes it ub when the vm reads it
    pub unitialized: *const std::ffi::c_void,
}
