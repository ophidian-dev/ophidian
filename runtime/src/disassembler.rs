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

                x if x == OpCode::IAdd as u8 => {
                    println!("IADD");
                    offset += 1;
                }

                x if x == OpCode::ISub as u8 => {
                    println!("ISUB");
                    offset += 1;
                }

                x if x == OpCode::IMul as u8 => {
                    println!("IMUL");
                    offset += 1;
                }

                x if x == OpCode::IDiv as u8 => {
                    println!("IDIV");
                    offset += 1;
                }

                x if x == OpCode::INegate as u8 => {
                    println!("INEGATE");
                    offset += 1;
                }

                x if x == OpCode::LoadConst as u8 => {
                    let bytes = &self.chunk.bytecode[offset + 1..offset + 4];
                    println!("LOADCONST {} {} {}", bytes[0], bytes[1], bytes[2]);
                    offset += 4;
                }

                x if x == OpCode::IPrint as u8 => {
                    println!("IPRINT");
                    offset += 1;
                }

                x if x == OpCode::Pop as u8 => {
                    println!("POP");
                    offset += 1;
                }

                x if x == OpCode::ILoadLocal as u8 => {
                    let bytes = &self.chunk.bytecode[offset + 1..offset + 4];
                    println!("ILOADLOCAL {} {} {}", bytes[0], bytes[1], bytes[2]);
                    offset += 4;
                }

                x if x == OpCode::IStoreLocal as u8 => {
                    let bytes = &self.chunk.bytecode[offset + 1..offset + 4];
                    println!("ISTORELOCAL {} {} {}", bytes[0], bytes[1], bytes[2]);
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

                x if x == OpCode::IEqual as u8 => {
                    println!("IEQUAL");
                    offset += 1;
                }

                x if x == OpCode::INEqual as u8 => {
                    println!("INEQUAL");
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

                x if x == OpCode::ILess as u8 => {
                    println!("ILESS");
                    offset += 1;
                }

                x if x == OpCode::ILessEq as u8 => {
                    println!("ILESSEQ");
                    offset += 1;
                }

                x if x == OpCode::IGreater as u8 => {
                    println!("IGREATER");
                    offset += 1;
                }

                x if x == OpCode::IGreaterEq as u8 => {
                    println!("IGREATEREQ");
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