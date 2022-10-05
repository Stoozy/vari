use crate::{expr::Expr, token::Token};

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>), // name, initializer
}

pub trait StmtVisitor<T> {
    fn visit_stmt(&mut self, stmt: Stmt) -> T;
}
