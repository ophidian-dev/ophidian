#[derive(Copy, Clone)]
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
                ValueKind::Boolean => {
                    return self.data.boolean == other.data.boolean;
                }
                ValueKind::Uninitialized => {
                    return false;
                }
                ValueKind::Double => {
                    return self.data.double == other.data.double;
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
                ValueKind::Boolean => {
                    write!(f, "{}", self.data.boolean)
                }
                ValueKind::Double => {
                    write!(f, "{}", self.data.double)
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

    pub const fn new_bool(boolean: bool) -> Self {
        Self {
            kind: ValueKind::Boolean,
            data: ValueData { boolean: boolean },
        }
    }

    pub const fn new_double(double: f64) -> Self {
        Self {
            kind: ValueKind::Double,
            data: ValueData { double: double },
        }
    }

    const fn new_uninitialized() -> Self {
        Self {
            kind: ValueKind::Uninitialized,
            data: ValueData {
                unitialized: std::ptr::null(),
            },
        }
    }

    pub const UNINITIALIZED: Self = Self::new_uninitialized();
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ValueKind {
    Integer,

    Boolean,

    Double,

    Uninitialized,
}

#[derive(Clone, Copy)]
pub union ValueData {
    pub integer: i32,
    pub boolean: bool,
    pub double: f64,

    // any type is fine
    // just makes it ub when the vm reads it
    pub unitialized: *const std::ffi::c_void,
}
