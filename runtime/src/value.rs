pub struct Value {
    pub kind: ValueKind,
    pub data: ValueData,
}

impl Value {
    pub const fn new_int(int: i32) -> Self {
        Self {
            kind: ValueKind::Integer,
            data: ValueData { integer: int }
        }
    }
}

pub enum ValueKind {
    Integer,
}

pub union ValueData {
    pub integer: i32,
}
