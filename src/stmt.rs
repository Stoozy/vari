use crate::{expr::Expr, token::Token};

pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>), // name, initializer
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
}

pub trait StmtVisitor<T> {
    fn visit_stmt(&mut self, stmt: Stmt) -> T;
}
