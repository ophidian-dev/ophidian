// this file contains generic collections that are not provided in the rust standard library
// some of these collections are implemented using rust based collections 
// and their purpose is mostly semantic meaning
// e.g. a vector may satisfy needs but a stack more clearly represents how that vector should be viewed

mod stack;

pub use stack::Stack;