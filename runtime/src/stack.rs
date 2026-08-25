#[derive(Debug, PartialEq)]
pub struct Stack<T> {
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

    pub fn top(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn truncate(&mut self, len: usize) {
        self.data.truncate(len);
    }
}

impl<T> std::ops::Index<usize> for Stack<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]  
    }
}

impl<T> std::ops::IndexMut<usize> for Stack<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Extend<T> for Stack<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
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
