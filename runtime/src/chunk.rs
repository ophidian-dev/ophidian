use crate::opcodes::OpCode;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub struct Chunk {
    pub bytecode: Vec<u8>,
    pub constants: Vec<Value>,
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
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

    pub fn write_i32(&mut self, i32: i32) {
        for byte in i32.to_le_bytes() {
            self.write(byte);
        }
    }

    pub fn write_jump(&mut self, opcode: OpCode) -> usize {
        let pos = self.bytecode.len();
        self.write(opcode as u8);
        self.write_i32(0);
        pos
    }

    pub fn write_jump_back(&mut self, opcode: OpCode, target: usize) {
        self.write(opcode as u8);

        let current = self.bytecode.len() + 4;

        let offset = target as isize - current as isize;

        self.write_i32(offset as i32);
    }

    pub fn patch_jump(&mut self, pos: usize) {
        let target = self.bytecode.len();

        let offset = target as i32 - (pos as i32 + 5);

        let bytes = offset.to_le_bytes();
        self.bytecode[pos + 1..pos + 5].copy_from_slice(&bytes);
    }

    pub fn patch_jump_to(&mut self, jump: usize, target: usize) {
        let offset = target as i32 - (jump as i32 + 5);

        let bytes = offset.to_le_bytes();
        self.bytecode[jump + 1..jump + 5].copy_from_slice(&bytes);
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
