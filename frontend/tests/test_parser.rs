use frontend::diagnostics::Diagnostic;
use frontend::lexer::Lexer;
use frontend::parse::Parser;
use frontend::parse::ast::{BinOpKind, Expr, ExprKind, LitKind, StmtKind};
use frontend::parse::node_id::NodeId;
use frontend::span::{Span, Spanned};

#[test]
fn test_parser_parse_arithmetic_expression() {
    let source = b"(1 + 2) * 3";
    let lexer = Lexer::new(source);
    let mut diagnostics = Vec::<Diagnostic>::new();
    let mut parser = Parser::new(lexer, &mut diagnostics, source);

    let expr = match parser.parse().stmts().get(0).unwrap().kind {
        StmtKind::ExprStmt(e) => *e,
        _ => panic!()
    };

    assert_eq!(
        expr,
        Expr {
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
        }
    );
}
