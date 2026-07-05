use crate::opcodes::Opcode;
use crate::bindings;
use crate::chunk::Chunk;
use frontend::parse::ast;

pub struct Compiler {
}

impl Compiler {
    pub fn new() -> Compiler {
        Self {

        }
    }    

    pub fn compile(&mut self, ast: &ast::Program) -> Chunk {
        let mut chunk: Chunk = Chunk::new();

        for stmt in &ast.stmts {
            compile_stmt(stmt, &mut chunk);
        }

        chunk.write(Opcode::Halt as u8);
        chunk
    }

}

fn compile_stmt(stmt: &ast::Stmt, chunk: &mut Chunk) {
    match stmt {
        ast::Stmt::Print { expr, span } => {
            compile_expr(expr, chunk);
            chunk.write(Opcode::Iprint as u8);
        }
        ast::Stmt::StmtExpr { expr, span } => {
            compile_expr(expr, chunk);
            chunk.write(Opcode::Pop as u8);
        }
    }
}

fn compile_expr(expr: &ast::Expr, chunk: &mut Chunk) {
    match expr {
        ast::Expr::IntegerLiteral { span, value } => {
            let v: bindings::Value = unsafe { 
                bindings::create_int_value(*value)
            };
            chunk.write(Opcode::Loadconst as u8);
            let idx = chunk.write_constant(v);
            assert!(idx <= 0xFF_FF_FF);
            chunk.write_u24(idx as u32);
        }
        ast::Expr::BinaryOp { span, op, left, right } => {
            compile_expr(&*left, chunk);
            compile_expr(&*right, chunk);
            let opcode = match op.kind {
                ast::BinopType::Add => { 
                    Opcode::Iadd
                },
                ast::BinopType::Sub => {
                    Opcode::Isub
                },
                ast::BinopType::Mul => {
                    Opcode::Imul
                },
                ast::BinopType::Div => {
                    Opcode::Idiv
                }
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
