use common::collections::*;
use common::stack;


// tests for stack

#[test]
fn push_1_value() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    assert_eq!(stack, stack![1]);
}

#[test]
fn push_multiple_values() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(2);
    stack.push(3);
    stack.push(4);
    assert_eq!(stack, stack![2 ,3 ,4]);
}

#[test]
fn new_stack_from() {
    let stack = Stack::from(3);
    assert_eq!(stack, stack![3]);
}

#[test]
fn stack_creation() {
    let stack: Stack<i32> = Stack::new();
    assert_eq!(stack, stack![]);
}

#[test]
fn pop_from_empty() {
    let mut stack: Stack<i32> = Stack::new();
    assert_eq!(stack.pop(), None);
}

#[test]
fn pop_from_stack_with_values() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    assert_eq!(stack.pop(), Some(1));
}

#[test]
fn top_of_empty_stack() {
    let stack: Stack<i32> = Stack::new();
    assert_eq!(stack.top(), None);
}

#[test]
fn top_of_stack_with_1_value() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1); 
    assert_eq!(stack.top(), Some(1).as_ref());
}

#[test]
fn top_of_stack_with_values() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.top(), Some(3).as_ref());
}

#[test]
fn stack_size_with_no_values() {
    let stack: Stack<i32> = Stack::new();
    assert_eq!(stack.size(), 0);
}

#[test]
fn stack_size() {
    let mut stack: Stack<i32> = Stack::new();
    for i in 0..100 {
        stack.push(i);
    }
    assert_eq!(stack.size(), 100);
}