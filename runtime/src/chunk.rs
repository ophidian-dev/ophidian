use crate::value::Value;

#[derive(Debug, PartialEq)]
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

    pub const fn new_from_existing(bytecode: Vec<u8>, constants: Vec<Value>) -> Self {
        Self {
            bytecode,
            constants,
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.bytecode.push(byte);
    }

    pub fn write_u24(&mut self, u24: u32) {
        assert!(u24 < 0x00FF_FFFF);

        self.write((u24 & 0xFF) as u8);
        self.write(((u24 >> 8) & 0xFF) as u8);
        self.write(((u24 >> 16) & 0xFF) as u8);
    }

    pub fn write_constant(&mut self, constant: Value) -> usize {
        let idx = self.constants.len();
        self.constants.push(constant);
        idx
    }

    pub const fn bytecode_ptr(&self) -> *const u8 {
        self.bytecode.as_ptr()
    }
}
