use crate::bindings;

pub struct Chunk {
    bytecode: Vec<u8>,
    constants: Vec<bindings::vm_Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn bytecode(&self) -> &Vec<u8> {
        &self.bytecode
    }

    pub fn constants(&self) -> &Vec<bindings::vm_Value> {
        &self.constants
    }

    pub fn chunk_data_as_mut_ptr(mut self) -> (*mut u8, *mut bindings::vm_Value) {
        (self.bytecode.as_mut_ptr(), self.constants.as_mut_ptr())
    }

    pub fn fmt_constants(&self) -> String {
        let mut s = String::new();
        s.push('[');

        for constant in &self.constants {
            match constant.type_ {
                bindings::vm_ValueType_VT_INT => {
                    let i: i32 = unsafe { constant.as_.integer };
                    s.push_str(&i.to_string());
                    s.push_str(", ");
                }
                _ => {
                    panic!("execution should not read here");
                }
            }
        }

        if s.len() > 1 {
            for _ in 0..2 {
                s.pop();
            }
        }

        s.push(']');
        s
    }

    pub fn write(&mut self, byte: u8) {
        self.bytecode.push(byte);
    }

    pub fn write_constant(&mut self, value: bindings::vm_Value) -> usize {
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

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "Chunk {{ \n    bytecode: {:?} , \n    constants: {} \n}}",
                self.bytecode(),
                self.fmt_constants()
            )
        } else {
            write!(
                f,
                "Chunk {{ bytecode: {:?} , constants: {} }}",
                self.bytecode(),
                self.fmt_constants()
            )
        }
    }
}
