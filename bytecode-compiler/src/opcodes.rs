// This file was auto-generated from spec/opcodes.toml
// It is not intended for manual editing

#[repr(i32)]
#[derive(Debug)]
pub enum Opcode {

    // halts execution of the virtual machine
    Halt = 0,
    // pops 2 ints off the stack and adds them then pushes the result
    Iadd = 1,
    // pops 2 ints off the stack and subtracts them then pushes the result
    Isub = 2,
    // pops 2 ints off the stack and multiplies them then pushes the result
    Imul = 3,
    // pops 2 ints off the stack and divides them then pushes the result
    Idiv = 4,
    // takes a 3 byte operand and indexes into constant pool and pushes the value at index operand
    Loadconst = 5,
    // pops 1 int off the stack and prints it
    Iprint = 6,
    // pops 1 int off the stack, negates and pushes result
    Inegate = 7,
    // pops top value off the stack
    Pop = 8,
}
