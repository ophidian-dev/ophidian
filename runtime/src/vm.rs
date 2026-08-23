use crate::chunk::Chunk;
use crate::opcodes::OpCode;
use crate::value::{Value, ValueKind};

pub type VMExitCode = i32;

#[derive(Debug, PartialEq)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    // Panics
    // This function panics if the stack is empty
    // we chose do it here because we trust that the user provides
    // the correct code
    pub fn pop(&mut self) -> T {
        self.data.pop().expect("stack underflow")
    }
}

const LOCAL_MAX: usize = 0xFFFFFF;

pub struct VirtualMachine {
    // stack that bytecode operates on
    stack: Stack<Value>,
    // pointer to instruction to be executed
    ip: *const u8,

    locals: Vec<Value>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let mut s = Self {
            stack: Stack::new(),
            ip: std::ptr::null_mut(),
            locals: Vec::new(),
        };

        for _ in 0..LOCAL_MAX {
            s.locals.push(Value::UNINITIALIZED);
        }
        s
    }

    pub fn execute(&mut self, chunk: &Chunk) -> VMExitCode {
        self.ip = chunk.bytecode_ptr();

        loop {
            // we unwrap here because we assume that the bytecode is correct
            // remember to add new match cases to OpCode::try_from() when adding
            // new opcodes
            let opcode = OpCode::try_from(self.read_byte())
                .expect("try checking the try_from() function for Opcode in opcode.rs");
            match opcode {
                OpCode::Halt => {
                    let exit_code = self.pop();
                    // behaviour is undefined if exit code is not an integer
                    // similar to c's `exit()` function from <stdlib.h>
                    return unsafe { exit_code.data.integer };
                }
                OpCode::IAdd => {
                    let b = self.pop();
                    let a = self.pop();
                    let res = unsafe { a.data.integer + b.data.integer };
                    let value = Value::new_int(res);
                    self.push(value);
                }
                OpCode::ISub => {
                    let b = self.pop();
                    let a = self.pop();
                    let res = unsafe { a.data.integer - b.data.integer };
                    let value = Value::new_int(res);
                    self.push(value);
                }
                OpCode::IMul => {
                    let b = self.pop();
                    let a = self.pop();
                    let res = unsafe { a.data.integer * b.data.integer };
                    let value = Value::new_int(res);
                    self.push(value);
                }
                OpCode::IDiv => {
                    // division by zero is undefined behaviour
                    let b = self.pop();
                    let a = self.pop();
                    let res = unsafe { a.data.integer / b.data.integer };
                    let value = Value::new_int(res);
                    self.push(value);
                }
                OpCode::INegate => {
                    let a = self.pop();
                    let res = unsafe { -a.data.integer };
                    let value = Value::new_int(res);
                    self.push(value);
                }
                OpCode::LoadConst => {
                    let b0 = self.read_byte();
                    let b1 = self.read_byte();
                    let b2 = self.read_byte();
                    let idx = decode_u24_le([b0, b1, b2]);
                    // we unwrap here because we assume the index provided is valid
                    let constant = chunk.constants.get(idx as usize).unwrap();

                    self.push(constant.clone());
                }
                OpCode::Pop => {
                    let _ = self.pop();
                }
                OpCode::IPrint => {
                    let v = self.pop();
                    let int = unsafe { v.data.integer };
                    println!("{}", int);
                }
                OpCode::ILoadLocal => {
                    let b0 = self.read_byte();
                    let b1 = self.read_byte();
                    let b2 = self.read_byte();
                    let idx = decode_u24_le([b0, b1, b2]);
                    let value = *self.locals.get(idx as usize).expect("shouldnt happen");
                    self.push(value);
                }
                OpCode::IStoreLocal => {
                    let value = self.pop();
                    if value.kind == ValueKind::Uninitialized {
                        panic!("use of unitialized variable");
                    }
                    let b0 = self.read_byte();
                    let b1 = self.read_byte();
                    let b2 = self.read_byte();
                    let idx = decode_u24_le([b0, b1, b2]);
                    *self.locals.get_mut(idx as usize).unwrap() = value;
                }
                OpCode::BEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    let res = unsafe { a.data.boolean == b.data.boolean };
                    let v = Value::new_bool(res);
                    self.push(v);
                }
                OpCode::BLoadLocal => {
                    let b0 = self.read_byte();
                    let b1 = self.read_byte();
                    let b2 = self.read_byte();
                    let idx = decode_u24_le([b0, b1, b2]);
                    let value = *self.locals.get(idx as usize).unwrap();
                    self.push(value);
                }
                OpCode::BNEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    let res = unsafe { a.data.boolean != b.data.boolean };
                    let v = Value::new_bool(res);
                    self.push(v);
                }
                OpCode::BPrint => {
                    let v = self.pop();
                    let boolean = unsafe { v.data.boolean };
                    println!("{}", boolean);
                }
                OpCode::BStoreLocal => {
                    let value = self.pop();
                    if value.kind == ValueKind::Uninitialized {
                        panic!("use of uninitialized variable");
                    }
                    let b0 = self.read_byte();
                    let b1 = self.read_byte();
                    let b2 = self.read_byte();
                    let idx = decode_u24_le([b0, b1, b2]);
                    *self.locals.get_mut(idx as usize).unwrap() = value;
                    self.push(value);
                }
                OpCode::IEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    let res = unsafe { a.data.integer == b.data.integer };
                    let value = Value::new_bool(res);
                    self.push(value);
                }
                OpCode::IGreater => {
                    let b = self.pop();
                    let a = self.pop();
                    let res = unsafe { a.data.integer > b.data.integer };
                    let value = Value::new_bool(res);
                    self.push(value);
                }
                OpCode::IGreaterEq => {
                    let b = self.pop();
                    let a = self.pop();
                    let res = unsafe { a.data.integer >= b.data.integer };
                    let value = Value::new_bool(res);
                    self.push(value);
                }
                OpCode::ILess => {
                    let b = self.pop();
                    let a = self.pop();
                    let res = unsafe { a.data.integer < b.data.integer };
                    let value = Value::new_bool(res);
                    self.push(value);
                }
                OpCode::ILessEq => {
                    let b = self.pop();
                    let a = self.pop();
                    let res = unsafe { a.data.integer <= b.data.integer };
                    let value = Value::new_bool(res);
                    self.push(value);
                }
                OpCode::INEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    let res = unsafe { a.data.integer != b.data.integer };
                    let value = Value::new_bool(res);
                    self.push(value);
                }
                OpCode::JmpTrue => {
                    let value = self.pop();
                    let cond = unsafe { value.data.boolean };
                    let b0 = self.read_byte();
                    let b1 = self.read_byte();
                    let b2 = self.read_byte();
                    let b3 = self.read_byte();
                    if cond {
                        let offset = i32::from_le_bytes([b0, b1, b2, b3]);
                        unsafe {
                            self.jump(offset);
                        }
                    }
                }
                OpCode::Jmp => {
                    let b0 = self.read_byte();
                    let b1 = self.read_byte();
                    let b2 = self.read_byte();
                    let b3 = self.read_byte();
                    let offset = i32::from_le_bytes([b0, b1, b2, b3]);
                    unsafe {
                        self.jump(offset);
                    }
                }
                OpCode::JmpFalse => {
                    let value = self.pop();
                    let cond = unsafe { value.data.boolean };
                    let b0 = self.read_byte();
                    let b1 = self.read_byte();
                    let b2 = self.read_byte();
                    let b3 = self.read_byte();

                    if !cond {
                        let offset = i32::from_le_bytes([b0, b1, b2, b3]);
                        unsafe {
                            self.jump(offset);
                        }
                    }
                }
                OpCode::Dup => {
                    let v = self.pop();
                    self.push(v);
                    self.push(v);
                }
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop()
    }

    fn read_byte(&mut self) -> u8 {
        unsafe {
            let byte = *self.ip;
            self.ip = self.ip.add(1);
            byte
        }
    }

    unsafe fn jump(&mut self, offset: i32) {
        if offset >= 0 {
            self.ip = unsafe { self.ip.add(offset as usize) };
        } else {
            self.ip = unsafe { self.ip.sub((-offset) as usize) };
        }
    }
}

fn decode_u24_le(bytes: [u8; 3]) -> u32 {
    let padded = [bytes[0], bytes[1], bytes[2], 0];
    u32::from_le_bytes(padded)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn test_pop_from_empty_stack() {
        let mut stack = Stack::<i32>::new();
        stack.pop();
    }

    #[test]
    fn test_pop_from_stack_with_values() {
        let mut stack = Stack::<i32>::new();
        stack.push(1);
        assert_eq!(1, stack.pop());
    }
}
