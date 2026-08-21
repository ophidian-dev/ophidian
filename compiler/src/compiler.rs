use frontend::analysis::analyzer::{AnalysisResult, SemanticAnalyzer, Type, VarId};
use frontend::diagnostics::Diagnostic;
use frontend::lex::Lexer;
use frontend::parse::Parser;
use frontend::parse::ast::{BinOpKind, Expr, ExprKind, LitKind, Stmt, StmtKind, UnaryOpKind};
use runtime::chunk::Chunk;
use runtime::opcodes::OpCode;
use runtime::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct LocalSlot(usize);

impl From<LocalSlot> for u32 {
    fn from(value: LocalSlot) -> Self {
        value.0.try_into().expect("local slot index exceeds u32::MAX")
    }
}

pub struct Compiler {
    locals: HashMap<VarId, LocalSlot>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
        }
    }

    #[must_use]
    pub fn compile(&mut self, source: &[u8]) -> Result<Chunk, Vec<Diagnostic>> {
        let mut diagnostics = Vec::<Diagnostic>::new();
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer, &mut diagnostics, source);

        let program = parser.parse();

        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }


        let mut analyzer = SemanticAnalyzer::new(&mut diagnostics);
        let metadata = analyzer.analyze(&program);

        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }

        let metadata = metadata.unwrap();

        let mut chunk = Chunk::new();

        for stmt in &program.body {
            self.compile_stmt(stmt, &mut chunk, &metadata);
        }

        chunk.write(OpCode::LoadConst as u8);
        let idx = chunk.write_constant(Value::new_int(0));
        chunk.write_u24(idx as u32);

        chunk.write(OpCode::Halt as u8);

        Ok(chunk)
    }

    fn compile_stmt(&mut self, stmt: &Stmt, chunk: &mut Chunk, metadata: &AnalysisResult) {
        match &stmt.kind {
            StmtKind::ExprStmt(expr) => {
                self.compile_expr(expr, chunk, metadata);

                chunk.write(OpCode::Pop as u8);
            }
            StmtKind::Print(expr) => {
                self.compile_expr(expr, chunk, metadata);

                chunk.write(OpCode::IPrint as u8);
            }
            StmtKind::VarDecl(_name, _type_annotation, initialiser) => {
                match initialiser {
                    Some(init) => {
                        self.compile_expr(init, chunk, metadata);

                        let varid = *metadata.variables.get(&stmt.id).unwrap();

                        self.locals.insert(varid, LocalSlot(varid.0));

                        match metadata.var_types.get(&varid).unwrap() {
                            Type::Int => {
                                chunk.write(OpCode::IStoreLocal as u8);
                                chunk.write_u24(varid.0.try_into().expect("hopefully this doesnt happen"));
                            }
                            Type::Error => {
                                unreachable!()
                            }
                        }
                    }
                    None => {
                        // use of a variable before its given a value is UB
                        let varid = *metadata.variables.get(&stmt.id).unwrap();

                        self.locals.insert(varid, LocalSlot(varid.0));

                        chunk.write(OpCode::LoadConst as u8);
                        let idx = chunk.write_constant(Value::UNINITIALIZED);
                        chunk.write_u24(idx as u32);

                        match metadata.var_types.get(&varid).unwrap() {
                            Type::Int => {
                                chunk.write(OpCode::IStoreLocal as u8);
                                chunk.write_u24(varid.0.try_into().expect("varid exceeds u32::MAX"));
                            }
                            Type::Error => {
                                unreachable!()
                            }
                        }
                    }
                }
            }
            StmtKind::Block(body) => {
                for stmt in body {
                    self.compile_stmt(stmt, chunk, metadata);
                }
            }
            StmtKind::Error => {
                unreachable!()
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr, chunk: &mut Chunk, metadata: &AnalysisResult) {
        match &expr.kind {
            ExprKind::Literal(litkind) => {
                match litkind {
                    LitKind::Int(i) => {
                        // we convert to i32 here because Int means i32
                        let value = Value::new_int(*i as i32);
                        chunk.write(OpCode::LoadConst as u8);
                        let idx = chunk.write_constant(value);
                        chunk.write_u24(idx as u32);
                    }
                }
            }
            ExprKind::BinaryOp(op, left, right) => {
                self.compile_expr(&left, chunk, metadata);
                self.compile_expr(&right, chunk, metadata);
                let opcode = match op.node {
                    // only type int exists rn so we dont needa
                    // check for different types
                    BinOpKind::Add => {
                        match metadata.types.get(&expr.id).unwrap() {
                            Type::Int => {
                                OpCode::IAdd
                            }
                            Type::Error => {
                                unreachable!()
                            }
                        }
                    },
                    BinOpKind::Sub => {
                        match metadata.types.get(&expr.id).unwrap() {
                            Type::Int => {
                                OpCode::ISub
                            }
                            Type::Error => {
                                unreachable!()
                            }
                        }
                    }
                    BinOpKind::Mul => {
                        match metadata.types.get(&expr.id).unwrap() {
                            Type::Int => {
                                OpCode::IMul
                            }
                            Type::Error => {
                                unreachable!()
                            }
                        }
                    }
                    BinOpKind::Div =>  {
                        match metadata.types.get(&expr.id).unwrap() {
                            Type::Int => {
                                OpCode::IDiv
                            }
                            Type::Error => {
                                unreachable!()
                            }
                        }
                    }
                };

                chunk.write(opcode as u8);
            }
            ExprKind::UnaryOp(op, right) => {
                self.compile_expr(&right, chunk, metadata);

                let opcode = match op.node {
                    // only type int exists for now so no type checks
                    // are necessary
                    UnaryOpKind::Negate => {
                        match metadata.types.get(&expr.id).unwrap() {
                            Type::Int => {
                                OpCode::INegate
                            }
                            Type::Error => {
                                unreachable!()
                            }
                        }
                    }
                };

                chunk.write(opcode as u8);
            }
            ExprKind::Error => {
                unreachable!()
            }
            ExprKind::VarAssign(target, value) => {
                    self.compile_expr(value, chunk, metadata);

                    let varid = match target.kind {
                        ExprKind::Variable(..) => {
                            metadata.variables.get(&target.id).unwrap()
                        }
                        _ => unreachable!("non lvalue?")
                    };

                    match metadata.var_types.get(varid).unwrap() {
                        Type::Int => {
                            chunk.write(OpCode::IStoreLocal as u8);
                            chunk.write_u24((*self.locals.get(varid).unwrap()).into());
                        }
                        Type::Error => unreachable!()
                    }
            }
            ExprKind::Variable(_name) => {
                chunk.write(OpCode::ILoadLocal as u8);
                chunk.write_u24((*self.locals.get(metadata.variables.get(&expr.id).unwrap()).unwrap()).try_into().expect("overflow"));
            }
        }
    }
}
