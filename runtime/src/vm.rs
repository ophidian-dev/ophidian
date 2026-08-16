use crate::chunk::Chunk;
use crate::opcodes::OpCode;
use crate::value::Value;

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

pub struct VirtualMachine {
    // stack that bytecode operates on
    stack: Stack<Value>,
    // pointer to instruction to be executed
    ip: *const u8,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            ip: std::ptr::null_mut(),
        }
    }

    pub fn execute(&mut self, chunk: &Chunk) -> VMExitCode {
        self.ip = chunk.bytecode_ptr();

        loop {
            // we unwrap here because we assume that the bytecode is correct
            // remember to add new match cases to OpCode::try_from() when adding
            // new opcodes
            let opcode = OpCode::try_from(self.read_byte()).unwrap();
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
