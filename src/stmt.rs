use crate::expr::Expr;

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
}

pub trait StmtVisitor<T> {
    fn visit_stmt(&mut self, stmt: Stmt) -> T;
}
