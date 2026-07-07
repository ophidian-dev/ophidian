// this file contains generic collections that are not provided in the rust standard library
// some of these collections are implemented using rust based collections 
// and their purpose is mostly semantic meaning
// e.g. a vector may satisfy needs but a stack more clearly represents how that vector should be viewed

// stack implementation
pub struct Stack<T: std::fmt::Debug> {
    items: Vec<T>
}

impl<T: std::fmt::Debug> Stack<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new()
        }
    }

    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn top(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.items.last_mut()
    }

    pub const fn size(&self) -> usize {
        self.items.len()
    }

    pub const fn get_underlying(&self) -> &Vec<T> {
        &self.items
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            todo!("implement pretty print for debug formatting");
        } else {
            write!(f, "Stack ")?;
            if self.items.is_empty() {
                return write!(f, "[]");
            }
            if self.items.len() == 1 {
                return write!(f, "[{:?}]", self.items[0]);
            }
            write!(f, "[")?;

            for i in 0..self.items.len() {
                if i == self.items.len() - 1 {
                    write!(f, "{:?}", self.items[i])?;
                    break;
                }
                write!(f, "{:?}, ", self.items[i])?;
            } 

            write!(f, "]")
        }
    }
}