use crate::bindings;

pub struct Chunk {
    bytecode: Vec<u8>,
    constants: Vec<bindings::Value>
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            constants: Vec::new()
        }
    }

    pub fn bytecode(&self) -> &Vec<u8> {
        &self.bytecode
    }

    pub fn constants(&self) -> &Vec<bindings::Value> {
        &self.constants
    }

    pub fn write(&mut self, byte: u8) {
        self.bytecode.push(byte);
    }

    pub fn write_constant(&mut self, value: bindings::Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn write_u24(&mut self, value: u32) {
        assert!(value <= 0xFF_FF_FF);
        
        self.write(value as u8);
        self.write((value >> 8) as u8);
        self.write((value >> 16) as u8);
    }
}