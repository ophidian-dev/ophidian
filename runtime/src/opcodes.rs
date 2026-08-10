#[repr(u8)]
pub enum OpCode {
    // halt the vm with exit code that we assume
    // the last opcode has pushed onto the stack
    Halt = 0x00,

    // i for integer
    // e.g. iadd is integer addition
    // the following opcodes pop 2 values of the type of the opcode off
    // the stack, apply the operation and pushes the result
    IAdd = 0x01,
    ISub = 0x02,
    IMul = 0x03,
    IDiv = 0x04,

    // integer negation
    INegate = 0x05,

    // load a constant from the constant pool
    // requires an operand that is 3 bytes as the index
    // encoded in little endian
    LoadConst = 0x06,

    // print an integer from the top of the stack
    IPrint = 0x07,

    // pops a value from the stack and discards it
    Pop = 0x08,
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Halt),
            1 => Ok(Self::IAdd),
            2 => Ok(Self::ISub),
            3 => Ok(Self::IMul),
            4 => Ok(Self::IDiv),
            5 => Ok(Self::INegate),
            6 => Ok(Self::LoadConst),
            7 => Ok(Self::IPrint),
            8 => Ok(Self::Pop),
            _ => Err(()),
        }
    }
}
