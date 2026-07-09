use crate::bindings;
use crate::chunk::Chunk;
use crate::opcodes::Opcode;
use frontend::diagnostics::Diagnostic;
use frontend::lex::Lexer;
use frontend::parse::Parser;
use frontend::semantic::analyzer::{SemanticAnalyzer, VarId};
use frontend::semantic::typed;
use frontend::semantic::typed::Type;

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Self {}
    }

    pub fn compile(&mut self, source: &[u8]) -> Result<Chunk, Vec<Diagnostic>> {
        let mut diagnostics: Vec<Diagnostic> = Vec::new();

        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer, &mut diagnostics);
        let unchecked_program = parser.generate_ast();

        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }

        let mut analyzer = SemanticAnalyzer::new(&mut diagnostics);
        let program = analyzer.analyze(unchecked_program);
        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }

        let mut chunk: Chunk = Chunk::new();

        for stmt in &program.stmts {
            compile_stmt(stmt, &mut chunk);
        }

        chunk.write(Opcode::Halt as u8);
        Ok(chunk)
    }
}

fn compile_stmt(stmt: &typed::Stmt, chunk: &mut Chunk) {
    match stmt {
        typed::Stmt::Print { expr, .. } => {
            compile_expr(expr, chunk);
            chunk.write(Opcode::Iprint as u8);
        }
        typed::Stmt::StmtExpr { expr, .. } => {
            compile_expr(expr, chunk);
            chunk.write(Opcode::Pop as u8);
        }
        typed::Stmt::VarDecl {
            id,
            type_annotation,
            initializer,
            ..
        } => {
            match initializer {
                Some(init) => {
                    compile_expr(init, chunk);

                    chunk.write(Opcode::IstoreLocal as u8);        
                    chunk.write_u24(<VarId as Into<u32>>::into(*id));
                }
                None => {
                    match type_annotation {
                        Type::Int => {
                            let idx = unsafe {
                                chunk.write_constant(bindings::vm_create_int_value(0))
                            };
                            chunk.write(Opcode::Loadconst as u8);
                            assert!(idx <= 0xFF_FF_FF);
                            chunk.write_u24(idx as u32);
                            
                            chunk.write(Opcode::IstoreLocal as u8);

                            chunk.write_u24(<VarId as Into<u32>>::into(*id));
                            
                        }
                        _ => {
                            unreachable!("analysis should have aborted after errors");
                        }
                    }
                }
            }
        }
        typed::Stmt::Block { body, .. } => {
            for stmt in body {
                compile_stmt(stmt, chunk);
            } 
        }
        _ => {
            unreachable!("execution should not reach here");
        }
    }
}

fn compile_expr(expr: &typed::Expr, chunk: &mut Chunk) {
    match expr {
        typed::Expr::IntegerLiteral { span: _, value, .. } => {
            let v: bindings::vm_Value = unsafe { bindings::vm_create_int_value(*value) };
            chunk.write(Opcode::Loadconst as u8);
            let idx = chunk.write_constant(v);
            assert!(idx <= 0xFF_FF_FF);
            chunk.write_u24(idx as u32);
        }
        typed::Expr::BinaryOp {
            span: _,
            op,
            left,
            right,
            ty,
        } => {
            compile_expr(&*left, chunk);
            compile_expr(&*right, chunk);
            let opcode = match ty {
                Type::Int => { 
                    let op = match op.kind {
                        typed::BinopType::Add => Opcode::Iadd,
                        typed::BinopType::Sub => Opcode::Isub,
                        typed::BinopType::Mul => Opcode::Imul,
                        typed::BinopType::Div => Opcode::Idiv,
                    };
                    op
                }   
                _ => {
                    unreachable!("parser should have exited after error");
                }
            };

            chunk.write(opcode as u8);
        }
        typed::Expr::UnaryOp {
            span: _,
            op,
            expr,
            ty,
        } => {
            compile_expr(&*expr, chunk);
            let opcode = match ty {
                Type::Int => {
                        let op: Opcode = match op.kind {
                            typed::UnaryopType::Negate => Opcode::Inegate,
                        };
                        op
                }
                _ => {
                    unreachable!("analysis should have aborted after error");
                }
            };
 
           chunk.write(opcode as u8);
        }
        typed::Expr::VarAssign { target, value, ty, .. } => {
            compile_expr(&*value, chunk);

            let var_id = match **target {
                typed::Expr::Variable { id, .. } => {
                    id
                }
                _ => {
                    unreachable!("analysis should have made sure only lvalues are assignable");
                }
            };

            match ty {
                Type::Int => {
                    chunk.write(Opcode::IstoreLocal as u8);
                    chunk.write_u24(<VarId as Into<u32>>::into(var_id));
                }
                _ => {
                    unreachable!("analysis should have aborted after error");
                }
            }
        }
        typed::Expr::Variable { id, .. } => {
            assert!(*id <= 0xFF_FF_FF);
            chunk.write(Opcode::IloadLocal as u8);
            chunk.write_u24(<VarId as Into<u32>>::into(*id));

        }
        typed::Expr::Error { span: _ } => {
            unreachable!("execution should not reach here");
        }
    }
}
