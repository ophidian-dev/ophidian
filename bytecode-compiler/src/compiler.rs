use crate::opcodes::Opcode;
use crate::bindings;
use crate::chunk::Chunk;
use frontend::lex::lexer::Lexer;
use frontend::parse::parser::Parser;
use frontend::parse::ast;

pub struct Compiler {
    ast: ast::Expr,
}

impl Compiler {
    pub fn new(source: &[u8]) -> Compiler {
        let lexer: Lexer<'_> = Lexer::new(source);
        let mut parser: Parser<'_> = Parser::new(lexer);
        let ast: ast::Expr = parser.generate_ast();
        Self {
            ast,
        }
    }    

    pub fn compile(&mut self) -> Chunk {
        let mut chunk: Chunk = Chunk::new();

        compile_expr(&self.ast, &mut chunk);
        chunk
    }

}

fn compile_expr(expr: &ast::Expr, chunk: &mut Chunk) {
    match expr {
        ast::Expr::IntegerLiteral { span, value } => {
            let v: bindings::Value = unsafe { 
                bindings::create_int_value(*value)
            };
            let idx = chunk.write_constant(v);
            for b in encode_u24_le(idx) {
                chunk.write(b);
            }
        }
        ast::Expr::BinaryOp { span, op, left, right } => {
            compile_expr(&*left, chunk);
            compile_expr(&*right, chunk);
            let opcode = match op.kind {
                ast::BinopType::Add => Opcode::Iadd,
                ast::BinopType::Sub => Opcode::Isub,
                ast::BinopType::Mul => Opcode::Imul,
                ast::BinopType::Div => Opcode::Idiv
            };

            chunk.write(opcode as u8); 
        }
        ast::Expr::UnaryOp { span, op, expr } => {
            compile_expr(&*expr, chunk);
            let opcode: Opcode = match op.kind {
                ast::UnaryopType::Negate => Opcode::Inegate,
            };
            chunk.write(opcode as u8);
        }
    }
}

fn encode_u24_le(value: usize) -> [u8; 3] {
    assert!(value <= 0xFF_FF_FF, "u24 overflow");

    let v = value as u32; // safe after check

    [
        v as u8,
        (v >> 8) as u8,
        (v >> 16) as u8,
    ]
}
