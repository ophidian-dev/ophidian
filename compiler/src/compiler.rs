use runtime::opcodes::OpCode;
use runtime::chunk::Chunk;
use frontend::lexer::Lexer;
use frontend::parser::Parser;
use frontend::diagnostics::Diagnostic;

pub struct Compiler {}

impl Compiler {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn compile(&mut self, source: &[u8]) -> Result<Chunk, Vec<Diagnostic>> {
        let mut diagnostics = Vec::<Diagnostic>::new();
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer, &mut diagnostics, source);

        let unchecked_program = parser.parse();

        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }
         
        let mut chunk = Chunk::new();

        chunk.write(OpCode::Halt as u8); 

        Ok(chunk)
    }
}