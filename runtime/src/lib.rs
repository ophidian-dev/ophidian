
// RUNTIME
// 
// This crate contains the virtual machine used to run the bytecode generated
// by the compiler crate
// however, due to the nature of the language this interpreter is for
// and the restrictive nature of rust's memory management,
// this virtual machine implementation may include extensive
// use of raw pointers for ease of implementation rather than
// safety i.e. very c style

use crate::chunk::Chunk;

pub mod opcodes;
pub mod vm;
pub mod value;
pub mod chunk;

use vm::VirtualMachine;
use vm::VMExitCode;

fn execute(chunk: Chunk) -> VMExitCode {
    let mut vm = VirtualMachine::new();
    vm.execute(chunk) 
}