use compiler::Compiler;
use runtime::{chunk::Chunk, value::Value};

#[test]
fn test_compile_stmtexpr() {
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(b"1 + 2 * 3;").unwrap();
    let constants = vec![
        Value::new_int(1),
        Value::new_int(2),
        Value::new_int(3),
        Value::new_int(0),
    ];
    let bytecode = vec![6, 0, 0, 0, 6, 1, 0, 0, 6, 2, 0, 0, 3, 1, 8, 6, 3, 0, 0, 0];

    assert_eq!(chunk, Chunk::new_from_existing(bytecode, constants));
}
