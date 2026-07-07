use common::collections::Stack;


// tests for stack

#[test]
fn push_1_value() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    assert_eq!(*stack.get_underlying(), vec![1]);
}

#[test]
fn push_multiple_values() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(2);
    stack.push(3);
    stack.push(4);
    assert_eq!(*stack.get_underlying(), vec![2 ,3 ,4])
}