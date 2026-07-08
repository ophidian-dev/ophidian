

// stack implementation

#[derive(PartialEq)]
pub struct Stack<T: std::fmt::Debug + PartialEq> {
    items: Vec<T>
}

impl<T: std::fmt::Debug + PartialEq> Stack<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new()
        }
    }

    pub fn from(value: T) -> Self {
        let mut s: Stack<T> = Stack::new();
        s.push(value); 
        s
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

    pub const fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }

}

#[macro_export]
macro_rules! stack {
    () => {
        Stack::new(); 
    };
    ($($element:expr),+ $(,)?) => {{
        let mut s = Stack::new();
        $(
            s.push($element);
        )+
        s
    }};
}

impl<T: std::fmt::Debug + PartialEq> std::fmt::Debug for Stack<T> {
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