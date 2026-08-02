use crate::value::Value;

pub struct Chunk {
    pub bytecode: Vec<u8>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub const fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.bytecode.push(byte);
    }

    pub fn write_constant(&mut self, constant: Value) {
        self.constants.push(constant);
    }

    pub const fn bytecode_ptr(&self) -> *const u8 {
        self.bytecode.as_ptr()
    }
}
