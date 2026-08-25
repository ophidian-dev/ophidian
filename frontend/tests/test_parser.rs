use frontend::diagnostics::Diagnostic;
use frontend::lex::Lexer;
use frontend::parse::Parser;
use frontend::parse::ast::NodeId;
use frontend::parse::ast::{BinOpKind, Expr, ExprKind, LitKind, Program, Stmt, StmtKind};
use frontend::span::{Span, Spanned};

#[test]
fn test_parser_parse_arithmetic_expression() {
    let source = b"(1 + 2) * 3;";
    let lexer = Lexer::new(source);
    let mut diagnostics = Vec::<Diagnostic>::new();
    let mut parser = Parser::new(lexer, &mut diagnostics, source);

    let program = parser.parse();

    let expr = match &program.body.get(0).unwrap().kind {
        StmtKind::ExprStmt(e) => e,
        _ => panic!(),
    };

    assert_eq!(
        *expr,
        Box::new(Expr {
            id: NodeId(4),
            span: Span::new(0, 11),
            kind: ExprKind::BinaryOp(
                Spanned {
                    node: BinOpKind::Mul,
                    span: Span::new(8, 1),
                },
                Box::new(Expr {
                    id: NodeId(2),
                    span: Span::new(0, 7), // "(1 + 2)"
                    kind: ExprKind::BinaryOp(
                        Spanned {
                            node: BinOpKind::Add,
                            span: Span::new(3, 1),
                        },
                        Box::new(Expr {
                            id: NodeId(0),
                            span: Span::new(1, 1),
                            kind: ExprKind::Literal(LitKind::Int(1)),
                        }),
                        Box::new(Expr {
                            id: NodeId(1),
                            span: Span::new(5, 1),
                            kind: ExprKind::Literal(LitKind::Int(2)),
                        }),
                    ),
                }),
                Box::new(Expr {
                    id: NodeId(3),
                    span: Span::new(10, 1),
                    kind: ExprKind::Literal(LitKind::Int(3)),
                }),
            ),
        })
    );
}

#[test]
fn test_parser_parse_print_stmt() {
    let source = b"print(1 + 3 * 3);";
    let lexer = Lexer::new(source);
    let mut diags = Vec::<Diagnostic>::new();
    let mut parser = Parser::new(lexer, &mut diags, source);
    let program = parser.parse();
    assert_eq!(
        program,
        Program {
            functions: vec![],
            body: vec![Stmt {
                id: NodeId(5),
                kind: StmtKind::Print(Box::new(Expr {
                    id: NodeId(4),
                    kind: ExprKind::BinaryOp(
                        Spanned::new(BinOpKind::Add, Span::new(8, 1),),
                        Box::new(Expr {
                            id: NodeId(0),
                            kind: ExprKind::Literal(LitKind::Int(1),),
                            span: Span::new(6, 1),
                        },),
                        Box::new(Expr {
                            id: NodeId(3),
                            kind: ExprKind::BinaryOp(
                                Spanned::new(BinOpKind::Mul, Span::new(12, 1),),
                                Box::new(Expr {
                                    id: NodeId(1),
                                    kind: ExprKind::Literal(LitKind::Int(3),),
                                    span: Span::new(10, 1),
                                },),
                                Box::new(Expr {
                                    id: NodeId(2),
                                    kind: ExprKind::Literal(LitKind::Int(3),),
                                    span: Span::new(14, 1),
                                },),
                            ),
                            span: Span::new(10, 5),
                        },),
                    ),
                    span: Span::new(6, 9),
                },),),
                span: Span::new(0, 17),
            },],
        }
    );
}
