// This file was auto-generated from spec/opcodes.toml
// It is not intended for manual editing

// halts execution of the virtual machine
#define OP_HALT 0

// pops 2 ints off the stack and adds them then pushes the result
#define OP_IADD 1

// pops 2 ints off the stack and subtracts them then pushes the result
#define OP_ISUB 2

// pops 2 ints off the stack and multiplies them then pushes the result
#define OP_IMUL 3

// pops 2 ints off the stack and divides them then pushes the result
#define OP_IDIV 4

// takes a 3 byte operand and indexes into constant pool and pushes the value at index operand
#define OP_LOADCONST 5

// pops 1 int off the stack and prints it
#define OP_IPRINT 6

// pops 1 int off the stack, negates and pushes result
#define OP_INEGATE 7

// pops top value off the stack
#define OP_POP 8

