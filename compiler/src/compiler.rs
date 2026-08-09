use frontend::diagnostics::Diagnostic;
use frontend::lexer::Lexer;
use frontend::parse::Parser;
use frontend::parse::ast::{BinOpKind, Expr, ExprKind, LitKind, UnaryOpKind};
use runtime::chunk::Chunk;
use runtime::opcodes::OpCode;
use runtime::value::Value;

pub struct Compiler {}

impl Compiler {
    pub const fn new() -> Self {
        Self {}
    }

    #[must_use]
    pub fn compile(&mut self, source: &[u8]) -> Result<Chunk, Vec<Diagnostic>> {
        let mut diagnostics = Vec::<Diagnostic>::new();
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer, &mut diagnostics, source);

        let unchecked_program = parser.parse();

        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }

        let mut chunk = Chunk::new();

        for stmt in unchecked_program.stmts() {
            self.compile_stmt(stmt, &mut chunk);
        }


        chunk.write(OpCode::LoadConst as u8);
        let idx = chunk.write_constant(Value::new_int(0));
        chunk.write_u24(idx as u32);

        chunk.write(OpCode::Halt as u8);

        Ok(chunk)
    }

    fn compile_stmt(&mut self, stmt: &Stmt, chunk: &mut Chunk) {
        todo!()
    }

    fn compile_expr(&mut self, expr: &Expr, chunk: &mut Chunk) {
        match &expr.kind {
            ExprKind::Literal(litkind) => {
                match litkind {
                    LitKind::Int(i) => {
                        // we convert to i32 here because we have not implemented
                        // a type checker and hir yet
                        let value = Value::new_int(*i as i32);
                        chunk.write(OpCode::LoadConst as u8);
                        let idx = chunk.write_constant(value);
                        chunk.write_u24(idx as u32);
                    }
                }
            }
            ExprKind::BinaryOp(op, left, right) => {
                self.compile_expr(&left, chunk);
                self.compile_expr(&right, chunk);
                let opcode = match op.node {
                    // only type int exists rn so we dont needa
                    // check for different types
                    BinOpKind::Add => OpCode::IAdd,
                    BinOpKind::Sub => OpCode::ISub,
                    BinOpKind::Mul => OpCode::IMul,
                    BinOpKind::Div => OpCode::IDiv,
                };

                chunk.write(opcode as u8);
            }
            ExprKind::UnaryOp(op, right) => {
                self.compile_expr(&right, chunk);

                let opcode = match op.node {
                    // only type int exists for now so no type checks
                    // are necessary
                    UnaryOpKind::Negate => OpCode::INegate,
                };

                chunk.write(opcode as u8);
            }
        }
    }
}
