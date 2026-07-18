use crate::parse::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::parse::Parser;
use crate::semantic::typed::Type;
use crate::span::Span;

impl<'src, 'diag> Parser<'src, 'diag> {
    pub(crate) fn create_integer_literal(&mut self, value: i32, span: Span) -> Expr {
        let id = self.next_id();
        Expr::IntegerLiteral { value, span, id }
    }

    pub(crate) fn create_binary_op(&mut self, op: BinaryOp, left: Expr, right: Expr, span: Span) -> Expr {
        let id = self.next_id();
        Expr::BinaryOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
            span,
            id,
        }
    }

    pub(crate) fn create_unary_op(&mut self, op: UnaryOp, expr: Expr, span: Span) -> Expr {
        let id = self.next_id();
        Expr::UnaryOp {
            op,
            expr: Box::new(expr),
            span,
            id,
        }
    }

    pub(crate) fn create_exprstmt(&mut self, expr: Expr, span: Span) -> Stmt {
        let id = self.next_id();
        Stmt::StmtExpr {
            expr: Box::new(expr),
            span,
            id,
        }
    }

    pub(crate) fn create_print_stmt(&mut self, expr: Expr, span: Span) -> Stmt {
        let id = self.next_id();
        Stmt::Print {
            expr: Box::new(expr),
            span,
            id,
        }
    }

    pub(crate) fn create_var_decl(
        &mut self,
        name: Vec<u8>,
        type_annotation: Option<Type>,
        initializer: Option<Expr>,
        span: Span,
    ) -> Stmt {
        let id = self.next_id();
        Stmt::VarDecl {
            name,
            type_annotation,
            initializer,
            span,
            id,
        }
    }

    pub(crate) fn create_variable(&mut self, name: Vec<u8>, span: Span) -> Expr {
        let id = self.next_id();
        Expr::Variable { name, span, id }
    }

    pub(crate) fn create_var_assign(&mut self, target: Expr, value: Expr, span: Span) -> Expr {
        let id = self.next_id();
        Expr::VarAssign {
            target: Box::new(target),
            value: Box::new(value),
            span,
            id,
        }
    }

    pub(crate) fn create_block(&mut self, body: Vec<Stmt>, span: Span) -> Stmt {
        let id = self.next_id();
        Stmt::Block { body, span, id }
    }

    pub(crate) fn create_boolean_literal(&mut self, value: bool, span: Span) -> Expr {
        let id = self.next_id();
        Expr::BooleanLiteral { span, value, id }
    }

    pub(crate) fn create_expr_err(&mut self, span: Span) -> Expr {
        let id = self.next_id();
        Expr::Error { span, id }
    }

    pub(crate)  fn create_stmt_err(&mut self, span: Span) -> Stmt {
        let id = self.next_id();
        Stmt::Error { span, id }
    }

}

