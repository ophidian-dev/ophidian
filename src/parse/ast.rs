

pub enum BinopType {
    Add,
    Sub,
    Mul,
    Div
}

pub enum Expr {
    IntegerLiteral(i64),
    BinaryOp{ op: BinopType, left: Box<Expr>, right: Box<Expr> },

}