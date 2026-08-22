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

    // takes a 3 byte operand and indexes into local pool and pushes the integer value at index operand
    ILoadLocal = 0x09,

    // takes a 3 byte operand, then pops an int off the stack then adds the value into local pool at index
    IStoreLocal = 0x0a,

    // print a boolean from the top of the stack
    BPrint = 0x0b,

    // takes a 3 byte operand and indexes into local pool and pushes the boolean value at index operand
    BLoadLocal = 0x0c,

    // takes a 3 byte operand, then pops an bool off the stack then adds the value into local pool at index
    BStoreLocal = 0x0d,

    // integer equality
    IEqual = 0x0e,
    INEqual = 0x0f,

    // boolean equality
    BEqual = 0x10,
    BNEqual = 0x11,

    // integer comparison
    ILess = 0x12,
    ILessEq = 0x13,
    IGreater = 0x14,
    IGreaterEq = 0x15,
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Halt),
            0x01 => Ok(Self::IAdd),
            0x02 => Ok(Self::ISub),
            0x03 => Ok(Self::IMul),
            0x04 => Ok(Self::IDiv),
            0x05 => Ok(Self::INegate),
            0x06 => Ok(Self::LoadConst),
            0x07 => Ok(Self::IPrint),
            0x08 => Ok(Self::Pop),
            0x09 => Ok(Self::ILoadLocal),
            0x0a => Ok(Self::IStoreLocal),
            0x0b => Ok(Self::BPrint),
            0x0c => Ok(Self::BLoadLocal),
            0x0d => Ok(Self::BStoreLocal),
            0x0e => Ok(Self::IEqual),
            0x0f => Ok(Self::INEqual),
            0x10 => Ok(Self::BEqual),
            0x11 => Ok(Self::BNEqual),
            0x12 => Ok(Self::ILess),
            0x13 => Ok(Self::ILessEq),
            0x14 => Ok(Self::IGreater),
            0x15 => Ok(Self::IGreaterEq),
            _ => Err(()),
        }
    }
}
