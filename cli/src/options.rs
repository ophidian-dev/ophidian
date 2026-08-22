

#[derive(Debug, Default)]
pub struct Options {
    pub dump_bytecode: bool,
}

impl Options {
    pub fn new() -> Self {
        Self::default()
    }
}