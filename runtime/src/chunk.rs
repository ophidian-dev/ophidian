use crate::value::Value;

pub struct Chunk {
    bytecode: Vec<u8>,
    constants: Vec<Value>,
}

impl Chunk {
    pub const fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.bytecode.push(byte);
    }

    pub fn emit_constant(&mut self, constant: Value) {
        self.constants.push(constant);
    }

    pub fn bytecode_ptr(&self) -> *const u8 {
        self.bytecode.as_ptr()
    }
}
