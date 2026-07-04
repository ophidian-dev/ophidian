// AUTO_GENERATED USING spec/opcodes.toml
// DO NOT MANUALLY EDIT

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

