use compiler::Compiler;
use runtime::{chunk::Chunk, value::Value};

#[test]
fn compile_arithmetic_exprs() {
    let mut compiler = Compiler::new();
    // unwrap cuz it shouldnt error and we dont care 
    let chunk = compiler.compile(b"(1 + 2) * 3").unwrap();

    let constants = vec![Value::new_int(1), Value::new_int(2), Value::new_int(3)];
    let bytecode = vec![6, 0, 0, 0, 6, 0, 0, 1, 1, 6, 0, 0, 2, 3, 6, 0, 0, 3, 0];

    assert_eq!(chunk, Chunk::new_from_existing(bytecode, constants));
}