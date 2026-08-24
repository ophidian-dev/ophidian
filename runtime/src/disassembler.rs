use crate::chunk::Chunk;
use crate::opcodes::OpCode;

pub struct Disassembler<'a> {
    chunk: &'a Chunk,
}

impl<'a> Disassembler<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self { chunk }
    }

    pub fn disassemble(&self) {
        println!("Chunk:");
        println!("Bytecode:");

        let mut offset = 0;

        while offset < self.chunk.bytecode.len() {
            match self.chunk.bytecode[offset] {
                x if x == OpCode::Halt as u8 => {
                    println!("HALT");
                    offset += 1;
                }

                x if x == OpCode::I32Add as u8 => {
                    println!("I32ADD");
                    offset += 1;
                }

                x if x == OpCode::I32Sub as u8 => {
                    println!("I32SUB");
                    offset += 1;
                }

                x if x == OpCode::I32Mul as u8 => {
                    println!("I32MUL");
                    offset += 1;
                }

                x if x == OpCode::I32Div as u8 => {
                    println!("I32DIV");
                    offset += 1;
                }

                x if x == OpCode::I32Negate as u8 => {
                    println!("I32NEGATE");
                    offset += 1;
                }

                x if x == OpCode::LoadConst as u8 => {
                    let bytes = &self.chunk.bytecode[offset + 1..offset + 4];
                    println!("LOADCONST {} {} {}", bytes[0], bytes[1], bytes[2]);
                    offset += 4;
                }

                x if x == OpCode::I32Print as u8 => {
                    println!("I32PRINT");
                    offset += 1;
                }

                x if x == OpCode::Pop as u8 => {
                    println!("POP");
                    offset += 1;
                }

                x if x == OpCode::I32LoadLocal as u8 => {
                    let bytes = &self.chunk.bytecode[offset + 1..offset + 4];
                    println!("I32LOADLOCAL {} {} {}", bytes[0], bytes[1], bytes[2]);
                    offset += 4;
                }

                x if x == OpCode::I32StoreLocal as u8 => {
                    let bytes = &self.chunk.bytecode[offset + 1..offset + 4];
                    println!("I32STORELOCAL {} {} {}", bytes[0], bytes[1], bytes[2]);
                    offset += 4;
                }

                x if x == OpCode::BPrint as u8 => {
                    println!("BPRINT");
                    offset += 1;
                }

                x if x == OpCode::BLoadLocal as u8 => {
                    let bytes = &self.chunk.bytecode[offset + 1..offset + 4];
                    println!("BLOADLOCAL {} {} {}", bytes[0], bytes[1], bytes[2]);
                    offset += 4;
                }

                x if x == OpCode::BStoreLocal as u8 => {
                    let bytes = &self.chunk.bytecode[offset + 1..offset + 4];
                    println!("BSTORELOCAL {} {} {}", bytes[0], bytes[1], bytes[2]);
                    offset += 4;
                }

                x if x == OpCode::I32Equal as u8 => {
                    println!("I32EQUAL");
                    offset += 1;
                }

                x if x == OpCode::I32NEqual as u8 => {
                    println!("I32NEQUAL");
                    offset += 1;
                }

                x if x == OpCode::BEqual as u8 => {
                    println!("BEQUAL");
                    offset += 1;
                }

                x if x == OpCode::BNEqual as u8 => {
                    println!("BNEQUAL");
                    offset += 1;
                }

                x if x == OpCode::I32Less as u8 => {
                    println!("I32LESS");
                    offset += 1;
                }

                x if x == OpCode::I32LessEq as u8 => {
                    println!("I32LESSEQ");
                    offset += 1;
                }

                x if x == OpCode::I32Greater as u8 => {
                    println!("I32GREATER");
                    offset += 1;
                }

                x if x == OpCode::I32GreaterEq as u8 => {
                    println!("I32GREATEREQ");
                    offset += 1;
                }

                x if x == OpCode::Jmp as u8 => {
                    let bytes = &self.chunk.bytecode[offset + 1..offset + 5];
                    println!("JMP {} {} {} {}", bytes[0], bytes[1], bytes[2], bytes[3]);
                    offset += 5;
                }

                x if x == OpCode::JmpTrue as u8 => {
                    let bytes = &self.chunk.bytecode[offset + 1..offset + 5];
                    println!(
                        "JMPTRUE {} {} {} {}",
                        bytes[0], bytes[1], bytes[2], bytes[3]
                    );
                    offset += 5;
                }

                x if x == OpCode::JmpFalse as u8 => {
                    let bytes = &self.chunk.bytecode[offset + 1..offset + 5];
                    println!(
                        "JMPFALSE {} {} {} {}",
                        bytes[0], bytes[1], bytes[2], bytes[3]
                    );
                    offset += 5;
                }

                x if x == OpCode::Dup as u8 => {
                    println!("DUP");
                    offset += 1;
                }

                x => {
                    println!("UNKNOWN {}", x);
                    offset += 1;
                }
            }
        }

        println!("Constants:");

        for (index, constant) in self.chunk.constants.iter().enumerate() {
            println!("{} {:?}", index, constant);
        }
    }
}
