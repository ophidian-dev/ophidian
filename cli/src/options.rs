use crate::parser::Args;

#[derive(Debug, Default)]
pub struct Options {
    pub input: String,
    pub dump_bytecode: bool,
}

impl Options {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<Args> for Options {
    fn from(value: Args) -> Self {
        Self {
            input: value.input,
            dump_bytecode: value.dump
        } 
    }
}